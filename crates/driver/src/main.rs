mod self_test;
mod settings;

use crate::self_test::self_test;
use crate::settings::Settings;
use clap::Parser;
use config::Config;
use hidapi::{HidDevice, HidResult};
use maschine_library::controls::{Buttons, PadEventType};
use maschine_library::lights::{Brightness, Lights, PadColors};
use maschine_library::screen::Screen;
use midir::os::unix::VirtualOutput;
use midir::{MidiOutput, MidiOutputConnection};
use midly::{MidiMessage, live::LiveEvent};
use std::collections::HashMap;

#[derive(Debug, Clone, Copy)]
struct ButtonState {
    physical_pressed: bool,
    toggle_state: bool,
    last_sent_value: u8,
}

impl Default for ButtonState {
    fn default() -> Self {
        Self {
            physical_pressed: false,
            toggle_state: false,
            last_sent_value: 0,
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
struct ShiftState {
    pressed: bool,
    used_as_modifier: bool, // Track if Shift was used in a combination
}

#[derive(Parser, Debug)]
#[clap(
    name = "Maschine Mikro MK3 Userspace MIDI driver",
    version = env!("CARGO_PKG_VERSION"),
    author = env!("CARGO_PKG_AUTHORS"),
)]
struct Args {
    #[clap(short, long, help = "Config file (see example_config.toml)")]
    config: Option<String>,
}

fn main() -> HidResult<()> {
    let args = Args::parse();

    let mut cfg = Config::builder();

    if let Some(config_fn) = args.config {
        cfg = cfg.add_source(config::File::with_name(config_fn.as_str()));
    }

    let cfg = cfg.build().expect("Can't create settings");
    let settings: Settings = cfg.try_deserialize().expect("Can't parse settings");

    settings.validate().unwrap();

    println!("Running with settings:");
    println!("{settings:?}");

    let output = MidiOutput::new(&settings.client_name).expect("Couldn't open MIDI output");
    let mut port = output
        .create_virtual(&settings.port_name)
        .expect("Couldn't create virtual port");

    let api = hidapi::HidApi::new()?;
    #[allow(non_snake_case)]
    let (VID, PID) = (0x17cc, 0x1700);
    let device = api.open(VID, PID)?;

    device.set_blocking_mode(false)?;

    let mut screen = Screen::new();
    let mut lights = Lights::new();

    self_test(&device, &mut screen, &mut lights)?;

    main_loop(&device, &mut screen, &mut lights, &mut port, &settings)?;

    Ok(())
}

fn process_button_event(
    button: Buttons,
    pressed: bool,
    button_states: &mut HashMap<Buttons, ButtonState>,
    shift_state: &mut ShiftState,
    settings: &Settings,
    port: &mut MidiOutputConnection,
) {
    // Get or create button state
    let state = button_states.entry(button).or_insert(ButtonState::default());
    
    // Check if physical state changed
    if state.physical_pressed == pressed {
        return; // No change, ignore
    }
    
    state.physical_pressed = pressed;
    
    // Handle Shift button specially to prevent CC conflicts
    // The Shift button acts as a modifier key:
    // - When pressed alone and released: sends its own CC (127 then 0)
    // - When used in a combination: doesn't send its own CC
    // This prevents the Shift CC from overwriting combination CCs
    if button == Buttons::Shift {
        if pressed {
            // Shift pressed - mark as not yet used as modifier
            shift_state.pressed = true;
            shift_state.used_as_modifier = false;
            // Don't send MIDI yet - wait to see if it's used as a modifier
            return;
        } else {
            // Shift released
            shift_state.pressed = false;
            // Only send Shift CC if it wasn't used as a modifier
            if !shift_state.used_as_modifier {
                // Send Shift press and release as a single action
                let cc_number = get_single_button_cc(button, settings);
                let channel = settings.button_channel;
                send_button_cc(port, cc_number, 127, channel);
                send_button_cc(port, cc_number, 0, channel);
            }
            shift_state.used_as_modifier = false;
            return;
        }
    }
    
    // For non-Shift buttons, check if Shift is held
    if shift_state.pressed && pressed {
        // Mark that Shift is being used as a modifier
        shift_state.used_as_modifier = true;
    }
    
    // Determine if this is a toggle button
    let button_name = format!("{:?}", button).to_lowercase();
    let is_toggle = settings.button_modes.is_toggle(&button_name);
    
    // Determine CC number based on shift state
    let cc_number = if shift_state.pressed {
        get_shift_combination_cc(button, settings)
    } else {
        get_single_button_cc(button, settings)
    };
    
    // Calculate MIDI value to send
    let midi_value = if is_toggle {
        if pressed {
            // Toggle the state on press
            state.toggle_state = !state.toggle_state;
            if state.toggle_state { 127 } else { 0 }
        } else {
            // Don't send anything on release for toggle buttons
            return;
        }
    } else {
        // Momentary mode: send 127 on press, 0 on release
        if pressed { 127 } else { 0 }
    };
    
    // Avoid duplicate messages
    if state.last_sent_value == midi_value {
        return;
    }
    
    state.last_sent_value = midi_value;
    
    // Determine which channel to use
    let channel = match button {
        Buttons::Play | Buttons::Stop | Buttons::Rec | 
        Buttons::Restart | Buttons::Erase | Buttons::Tap | Buttons::Follow 
            => settings.transport_channel,
        Buttons::EncoderPress | Buttons::EncoderTouch 
            => settings.encoder_channel,
        _ => settings.button_channel,
    };
    
    // Send MIDI message
    send_button_cc(port, cc_number, midi_value, channel);
}

fn send_button_cc(port: &mut MidiOutputConnection, cc_number: u8, value: u8, channel: u8) {
    let message = MidiMessage::Controller {
        controller: cc_number.into(),
        value: value.into(),
    };
    let event = LiveEvent::Midi {
        channel: channel.into(),
        message,
    };
    let mut buf = Vec::new();
    event.write(&mut buf).unwrap();
    port.send(&buf[..]).unwrap();
}

fn get_single_button_cc(button: Buttons, settings: &Settings) -> u8 {
    match button {
        // Navigation buttons
        Buttons::Left => settings.button_cc_map.left,
        Buttons::Right => settings.button_cc_map.right,
        
        // Main Control buttons
        Buttons::Maschine => settings.button_cc_map.maschine,
        Buttons::Star => settings.button_cc_map.star,
        Buttons::Browse => settings.button_cc_map.browse,
        Buttons::Volume => settings.button_cc_map.volume,
        
        // Performance buttons
        Buttons::Swing => settings.button_cc_map.swing,
        Buttons::Tempo => settings.button_cc_map.tempo,
        Buttons::Plugin => settings.button_cc_map.plugin,
        Buttons::Sampling => settings.button_cc_map.sampling,
        
        // Pitch/Mod buttons
        Buttons::Pitch => settings.button_cc_map.pitch,
        Buttons::Mod => settings.button_cc_map.mod_button,
        
        // Mode Selection buttons
        Buttons::Perform => settings.button_cc_map.perform,
        Buttons::Notes => settings.button_cc_map.notes,
        Buttons::Group => settings.button_cc_map.group,
        Buttons::Auto => settings.button_cc_map.auto,
        
        // Recording buttons
        Buttons::Lock => settings.button_cc_map.lock,
        Buttons::NoteRepeat => settings.button_cc_map.note_repeat,
        
        // Modifier buttons
        Buttons::Shift => settings.button_cc_map.shift,
        Buttons::FixedVel => settings.button_cc_map.fixed_vol,
        
        // Pad Mode buttons
        Buttons::PadMode => settings.button_cc_map.pad_mode,
        Buttons::Keyboard => settings.button_cc_map.keyboard,
        Buttons::Chords => settings.button_cc_map.chords,
        Buttons::Step => settings.button_cc_map.step,
        
        // Sequencer buttons
        Buttons::Scene => settings.button_cc_map.scene,
        Buttons::Pattern => settings.button_cc_map.pattern,
        Buttons::Events => settings.button_cc_map.events,
        Buttons::Variation => settings.button_cc_map.variation,
        Buttons::Duplicate => settings.button_cc_map.duplicate,
        
        // Track Control buttons
        Buttons::Select => settings.button_cc_map.select,
        Buttons::Solo => settings.button_cc_map.solo,
        Buttons::Mute => settings.button_cc_map.mute,
        
        // Transport buttons
        Buttons::Play => settings.transport_cc_map.play,
        Buttons::Stop => settings.transport_cc_map.stop,
        Buttons::Rec => settings.transport_cc_map.rec,
        Buttons::Restart => settings.transport_cc_map.restart,
        Buttons::Erase => settings.transport_cc_map.erase,
        Buttons::Tap => settings.transport_cc_map.tap,
        Buttons::Follow => settings.transport_cc_map.follow,
        
        // Encoder buttons
        Buttons::EncoderPress => settings.encoder_cc_map.press,
        Buttons::EncoderTouch => settings.encoder_cc_map.touch,
    }
}

fn get_shift_combination_cc(button: Buttons, settings: &Settings) -> u8 {
    match button {
        // Navigation buttons
        Buttons::Left => settings.shift_combinations.left,
        Buttons::Right => settings.shift_combinations.right,
        
        // Main Control buttons
        Buttons::Maschine => settings.shift_combinations.maschine,
        Buttons::Star => settings.shift_combinations.star,
        Buttons::Browse => settings.shift_combinations.browse,
        Buttons::Volume => settings.shift_combinations.volume,
        
        // Performance buttons
        Buttons::Swing => settings.shift_combinations.swing,
        Buttons::Tempo => settings.shift_combinations.tempo,
        Buttons::Plugin => settings.shift_combinations.plugin,
        Buttons::Sampling => settings.shift_combinations.sampling,
        
        // Pitch/Mod buttons
        Buttons::Pitch => settings.shift_combinations.pitch,
        Buttons::Mod => settings.shift_combinations.mod_button,
        
        // Mode Selection buttons
        Buttons::Perform => settings.shift_combinations.perform,
        Buttons::Notes => settings.shift_combinations.notes,
        Buttons::Group => settings.shift_combinations.group,
        Buttons::Auto => settings.shift_combinations.auto,
        
        // Recording buttons
        Buttons::Lock => settings.shift_combinations.lock,
        Buttons::NoteRepeat => settings.shift_combinations.note_repeat,
        
        // FixedVel button
        Buttons::FixedVel => settings.shift_combinations.fixed_vel,
        
        // Pad Mode buttons
        Buttons::PadMode => settings.shift_combinations.pad_mode,
        Buttons::Keyboard => settings.shift_combinations.keyboard,
        Buttons::Chords => settings.shift_combinations.chords,
        Buttons::Step => settings.shift_combinations.step,
        
        // Sequencer buttons
        Buttons::Scene => settings.shift_combinations.scene,
        Buttons::Pattern => settings.shift_combinations.pattern,
        Buttons::Events => settings.shift_combinations.events,
        Buttons::Variation => settings.shift_combinations.variation,
        Buttons::Duplicate => settings.shift_combinations.duplicate,
        
        // Track Control buttons
        Buttons::Select => settings.shift_combinations.select,
        Buttons::Solo => settings.shift_combinations.solo,
        Buttons::Mute => settings.shift_combinations.mute,
        
        // Transport buttons
        Buttons::Play => settings.shift_combinations.play,
        Buttons::Stop => settings.shift_combinations.stop,
        Buttons::Rec => settings.shift_combinations.rec,
        Buttons::Restart => settings.shift_combinations.restart,
        Buttons::Erase => settings.shift_combinations.erase,
        Buttons::Tap => settings.shift_combinations.tap,
        Buttons::Follow => settings.shift_combinations.follow,
        
        // Encoder buttons
        Buttons::EncoderPress => settings.shift_combinations.encoder_press,
        Buttons::EncoderTouch => settings.shift_combinations.encoder_touch,
        
        // Shift+Shift case: returns single Shift CC
        Buttons::Shift => get_single_button_cc(button, settings),
    }
}

fn main_loop(
    device: &HidDevice,
    _screen: &mut Screen,
    lights: &mut Lights,
    port: &mut MidiOutputConnection,
    settings: &Settings,
) -> HidResult<()> {
    let mut buf = [0u8; 64];
    let mut button_states: HashMap<Buttons, ButtonState> = HashMap::new();
    let mut shift_state = ShiftState::default();
    let mut prev_encoder_val: u8 = 0; // Track previous encoder value
    let mut prev_slider_val: u8 = 0; // Track previous touch strip value
    loop {
        let size = device.read_timeout(&mut buf, 10)?;
        if size < 1 {
            continue;
        }
        println!("DEBUG: {:?}", &buf[..]);
        let mut changed_lights = false;
        if buf[0] == 0x01 {
            // button mode
            for i in 0..6 {
                // bytes
                for j in 0..8 {
                    // bits
                    let idx = i * 8 + j;
                    let button: Option<Buttons> = num::FromPrimitive::from_usize(idx);
                    let button = match button {
                        Some(val) => val,
                        None => continue,
                    };
                    let status = buf[i + 1] & (1 << j);
                    let status = status > 0;
                    
                    // Check if button state changed for debug output
                    let state = button_states.get(&button);
                    let button_state_changed = state.map_or(true, |s| s.physical_pressed != status);
                    
                    if button_state_changed && status {
                        println!("New status: {:?}", button);
                    }
                    
                    // Update button lights
                    if lights.button_has_light(button) {
                        let light_status = lights.get_button(button) != Brightness::Off;
                        if status != light_status {
                            lights.set_button(
                                button,
                                if status {
                                    Brightness::Normal
                                } else {
                                    Brightness::Off
                                },
                            );
                            changed_lights = true;
                        }
                    }
                    
                    // Process button event through unified handler
                    process_button_event(button, status, &mut button_states, &mut shift_state, settings, port);
                }
            }
            let encoder_val = buf[7];
            println!("Encoder: {}", encoder_val);
            
            // Send MIDI CC for encoder rotation when value changes
            if encoder_val != prev_encoder_val && encoder_val != 0 {
                prev_encoder_val = encoder_val;
                
                // Send relative CC value (encoder_val as-is from hardware)
                let message = MidiMessage::Controller {
                    controller: settings.encoder_cc_map.rotation.into(),
                    value: encoder_val.into(),
                };
                let event = LiveEvent::Midi {
                    channel: settings.encoder_channel.into(),
                    message,
                };
                let mut midi_buf = Vec::new();
                event.write(&mut midi_buf).unwrap();
                port.send(&midi_buf[..]).unwrap();
            }
            
            let slider_val = buf[10];
            
            // Send MIDI CC for touch strip when value changes
            if slider_val != prev_slider_val {
                prev_slider_val = slider_val;
                
                // Convert strip value (0-200 range) to MIDI range (0-127)
                // Send value 0 when strip is released (slider_val == 0)
                let midi_val = if slider_val == 0 {
                    0
                } else {
                    ((slider_val as u16 * 127) / 200) as u8
                };
                
                let message = MidiMessage::Controller {
                    controller: settings.touch_strip_cc.into(),
                    value: midi_val.into(),
                };
                let event = LiveEvent::Midi {
                    channel: settings.touch_strip_channel.into(),
                    message,
                };
                let mut midi_buf = Vec::new();
                event.write(&mut midi_buf).unwrap();
                port.send(&midi_buf[..]).unwrap();
            }
            
            if slider_val != 0 {
                println!("Slider: {}", slider_val);
                let cnt = (slider_val as i32 - 1 + 5) * 25 / 200 - 1;
                for i in 0..25 {
                    let b = match cnt - i {
                        0 => Brightness::Normal,
                        1..=25 => Brightness::Dim,
                        _ => Brightness::Off,
                    };
                    lights.set_slider(i as usize, b);
                }
                changed_lights = true;
            }
        } else if buf[0] == 0x02 {
            // pad mode
            for i in (1..buf.len()).step_by(3) {
                let idx = buf[i];
                let evt = buf[i + 1] & 0xf0;
                let val = ((buf[i + 1] as u16 & 0x0f) << 8) + buf[i + 2] as u16;
                if i > 1 && idx == 0 && evt == 0 && val == 0 {
                    break;
                }
                let pad_evt: PadEventType = num::FromPrimitive::from_u8(evt).unwrap();
                // if evt != PadEventType::Aftertouch {
                println!("Pad {}: {:?} @ {}", idx, pad_evt, val);
                // }
                let (_, prev_b) = lights.get_pad(idx as usize);
                let b = match pad_evt {
                    PadEventType::NoteOn | PadEventType::PressOn => Brightness::Normal,
                    PadEventType::NoteOff | PadEventType::PressOff => Brightness::Off,
                    PadEventType::Aftertouch => {
                        if val > 0 {
                            Brightness::Normal
                        } else {
                            Brightness::Off
                        }
                    }
                    #[allow(unreachable_patterns)]
                    _ => prev_b,
                };
                if prev_b != b {
                    lights.set_pad(idx as usize, PadColors::Blue, b);
                    changed_lights = true;
                }

                let note = settings.notemaps[idx as usize];
                println!("Sending MIDI note: {} for pad {}", note, idx);

                let mut velocity = (val >> 5) as u8;
                if val > 0 && velocity == 0 {
                    velocity = 1;
                }

                let event = match pad_evt {
                    PadEventType::NoteOn | PadEventType::PressOn => Some(MidiMessage::NoteOn {
                        key: note.into(),
                        vel: velocity.into(),
                    }),
                    PadEventType::NoteOff | PadEventType::PressOff => Some(MidiMessage::NoteOff {
                        key: note.into(),
                        vel: velocity.into(),
                    }),
                    _ => None,
                };

                if let Some(evt) = event {
                    let l_ev = LiveEvent::Midi {
                        channel: 0.into(),
                        message: evt,
                    };

                    let mut buf = Vec::new();
                    l_ev.write(&mut buf).unwrap();
                    port.send(&buf[..]).unwrap()
                }
            }
        }
        if changed_lights {
            lights.write(device)?;
        }
    }
}
