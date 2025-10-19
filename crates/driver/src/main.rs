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

fn send_transport_cc(
    port: &mut MidiOutputConnection,
    cc_number: u8,
    pressed: bool,
    channel: u8,
) {
    let value = if pressed { 127 } else { 0 };
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

fn main_loop(
    device: &HidDevice,
    _screen: &mut Screen,
    lights: &mut Lights,
    port: &mut MidiOutputConnection,
    settings: &Settings,
) -> HidResult<()> {
    let mut buf = [0u8; 64];
    let mut button_states = [false; 48]; // Track previous state of all buttons
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
                    
                    // Check if button state changed
                    let prev_status = button_states[idx];
                    let button_state_changed = status != prev_status;
                    
                    if button_state_changed {
                        button_states[idx] = status;
                        if status {
                            println!("{:?}", button);
                        }
                    }
                    
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
                    
                    // Send MIDI CC for buttons only when state changes
                    if button_state_changed {
                        match button {
                            // Transport buttons
                            Buttons::Play => send_transport_cc(port, settings.transport_cc_map.play, status, settings.transport_channel),
                            Buttons::Stop => send_transport_cc(port, settings.transport_cc_map.stop, status, settings.transport_channel),
                            Buttons::Rec => send_transport_cc(port, settings.transport_cc_map.rec, status, settings.transport_channel),
                            Buttons::Restart => send_transport_cc(port, settings.transport_cc_map.restart, status, settings.transport_channel),
                            Buttons::Erase => send_transport_cc(port, settings.transport_cc_map.erase, status, settings.transport_channel),
                            Buttons::Tap => send_transport_cc(port, settings.transport_cc_map.tap, status, settings.transport_channel),
                            Buttons::Follow => send_transport_cc(port, settings.transport_cc_map.follow, status, settings.transport_channel),
                            
                            // Navigation buttons
                            Buttons::Left => send_transport_cc(port, settings.button_cc_map.left, status, settings.button_channel),
                            Buttons::Right => send_transport_cc(port, settings.button_cc_map.right, status, settings.button_channel),
                            
                            // Main Control buttons
                            Buttons::Maschine => send_transport_cc(port, settings.button_cc_map.maschine, status, settings.button_channel),
                            Buttons::Star => send_transport_cc(port, settings.button_cc_map.star, status, settings.button_channel),
                            Buttons::Browse => send_transport_cc(port, settings.button_cc_map.browse, status, settings.button_channel),
                            Buttons::Volume => send_transport_cc(port, settings.button_cc_map.volume, status, settings.button_channel),
                            
                            // Performance buttons
                            Buttons::Swing => send_transport_cc(port, settings.button_cc_map.swing, status, settings.button_channel),
                            Buttons::Tempo => send_transport_cc(port, settings.button_cc_map.tempo, status, settings.button_channel),
                            Buttons::Plugin => send_transport_cc(port, settings.button_cc_map.plugin, status, settings.button_channel),
                            Buttons::Sampling => send_transport_cc(port, settings.button_cc_map.sampling, status, settings.button_channel),
                            
                            // Pitch/Mod buttons
                            Buttons::Pitch => send_transport_cc(port, settings.button_cc_map.pitch, status, settings.button_channel),
                            Buttons::Mod => send_transport_cc(port, settings.button_cc_map.mod_button, status, settings.button_channel),
                            
                            // Mode Selection buttons
                            Buttons::Perform => send_transport_cc(port, settings.button_cc_map.perform, status, settings.button_channel),
                            Buttons::Notes => send_transport_cc(port, settings.button_cc_map.notes, status, settings.button_channel),
                            Buttons::Group => send_transport_cc(port, settings.button_cc_map.group, status, settings.button_channel),
                            Buttons::Auto => send_transport_cc(port, settings.button_cc_map.auto, status, settings.button_channel),
                            
                            // Recording buttons
                            Buttons::Lock => send_transport_cc(port, settings.button_cc_map.lock, status, settings.button_channel),
                            Buttons::NoteRepeat => send_transport_cc(port, settings.button_cc_map.note_repeat, status, settings.button_channel),
                            
                            // Modifier buttons
                            Buttons::Shift => send_transport_cc(port, settings.button_cc_map.shift, status, settings.button_channel),
                            Buttons::FixedVol => send_transport_cc(port, settings.button_cc_map.fixed_vol, status, settings.button_channel),
                            
                            // Pad Mode buttons
                            Buttons::PadMode => send_transport_cc(port, settings.button_cc_map.pad_mode, status, settings.button_channel),
                            Buttons::Keyboard => send_transport_cc(port, settings.button_cc_map.keyboard, status, settings.button_channel),
                            Buttons::Chords => send_transport_cc(port, settings.button_cc_map.chords, status, settings.button_channel),
                            Buttons::Step => send_transport_cc(port, settings.button_cc_map.step, status, settings.button_channel),
                            
                            // Sequencer buttons
                            Buttons::Scene => send_transport_cc(port, settings.button_cc_map.scene, status, settings.button_channel),
                            Buttons::Pattern => send_transport_cc(port, settings.button_cc_map.pattern, status, settings.button_channel),
                            Buttons::Events => send_transport_cc(port, settings.button_cc_map.events, status, settings.button_channel),
                            Buttons::Variation => send_transport_cc(port, settings.button_cc_map.variation, status, settings.button_channel),
                            Buttons::Duplicate => send_transport_cc(port, settings.button_cc_map.duplicate, status, settings.button_channel),
                            
                            // Track Control buttons
                            Buttons::Select => send_transport_cc(port, settings.button_cc_map.select, status, settings.button_channel),
                            Buttons::Solo => send_transport_cc(port, settings.button_cc_map.solo, status, settings.button_channel),
                            Buttons::Mute => send_transport_cc(port, settings.button_cc_map.mute, status, settings.button_channel),
                            
                            // Encoder buttons (will be handled in task 4, but included for completeness)
                            Buttons::EncoderPress => send_transport_cc(port, settings.encoder_cc_map.press, status, settings.encoder_channel),
                            Buttons::EncoderTouch => send_transport_cc(port, settings.encoder_cc_map.touch, status, settings.encoder_channel),
                        }
                    }
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
