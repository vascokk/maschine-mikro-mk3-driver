use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone, Default)]
pub(crate) struct ButtonModeConfig {
    /// Map of button name to toggle mode (true = toggle, false/absent = momentary)
    /// Only buttons configured here will use toggle mode
    #[serde(default)]
    pub toggle_buttons: HashMap<String, bool>,
}

impl ButtonModeConfig {
    pub fn is_toggle(&self, button_name: &str) -> bool {
        self.toggle_buttons.get(button_name).copied().unwrap_or(false)
    }
}

#[derive(Deserialize, Debug, Clone)]
pub(crate) struct ShiftCombinationMap {
    // Navigation (2 buttons)
    #[serde(default = "default_cc_54")]
    pub left: u8,
    #[serde(default = "default_cc_55")]
    pub right: u8,
    
    // Main Control (4 buttons)
    #[serde(default = "default_cc_56")]
    pub maschine: u8,
    #[serde(default = "default_cc_57")]
    pub star: u8,
    #[serde(default = "default_cc_58")]
    pub browse: u8,
    #[serde(default = "default_cc_59")]
    pub volume: u8,
    
    // Performance (4 buttons)
    #[serde(default = "default_cc_60")]
    pub swing: u8,
    #[serde(default = "default_cc_61")]
    pub tempo: u8,
    #[serde(default = "default_cc_62")]
    pub plugin: u8,
    #[serde(default = "default_cc_63")]
    pub sampling: u8,
    
    // Pitch/Mod (2 buttons)
    #[serde(default = "default_cc_64")]
    pub pitch: u8,
    #[serde(default = "default_cc_65")]
    pub mod_button: u8,
    
    // Mode Selection (4 buttons)
    #[serde(default = "default_cc_66")]
    pub perform: u8,
    #[serde(default = "default_cc_67")]
    pub notes: u8,
    #[serde(default = "default_cc_68")]
    pub group: u8,
    #[serde(default = "default_cc_69")]
    pub auto: u8,
    
    // Recording (2 buttons)
    #[serde(default = "default_cc_70")]
    pub lock: u8,
    #[serde(default = "default_cc_71")]
    pub note_repeat: u8,
    
    // FixedVel (1 button - can be used with Shift)
    #[serde(default = "default_cc_72")]
    pub fixed_vel: u8,
    
    // Pad Modes (4 buttons)
    #[serde(default = "default_cc_73")]
    pub pad_mode: u8,
    #[serde(default = "default_cc_74")]
    pub keyboard: u8,
    #[serde(default = "default_cc_75")]
    pub chords: u8,
    #[serde(default = "default_cc_76")]
    pub step: u8,
    
    // Sequencer (5 buttons)
    #[serde(default = "default_cc_77")]
    pub scene: u8,
    #[serde(default = "default_cc_78")]
    pub pattern: u8,
    #[serde(default = "default_cc_79")]
    pub events: u8,
    #[serde(default = "default_cc_80")]
    pub variation: u8,
    #[serde(default = "default_cc_81")]
    pub duplicate: u8,
    
    // Track Control (3 buttons)
    #[serde(default = "default_cc_82")]
    pub select: u8,
    #[serde(default = "default_cc_83")]
    pub solo: u8,
    #[serde(default = "default_cc_84")]
    pub mute: u8,
    
    // Transport (7 buttons)
    #[serde(default = "default_cc_85")]
    pub play: u8,
    #[serde(default = "default_cc_86")]
    pub stop: u8,
    #[serde(default = "default_cc_87")]
    pub rec: u8,
    #[serde(default = "default_cc_88")]
    pub restart: u8,
    #[serde(default = "default_cc_89")]
    pub erase: u8,
    #[serde(default = "default_cc_90")]
    pub tap: u8,
    #[serde(default = "default_cc_91")]
    pub follow: u8,
    
    // Encoder (2 buttons)
    #[serde(default = "default_cc_92")]
    pub encoder_press: u8,
    #[serde(default = "default_cc_93")]
    pub encoder_touch: u8,
}

// Default functions for serde
fn default_cc_54() -> u8 { 54 }
fn default_cc_55() -> u8 { 55 }
fn default_cc_56() -> u8 { 56 }
fn default_cc_57() -> u8 { 57 }
fn default_cc_58() -> u8 { 58 }
fn default_cc_59() -> u8 { 59 }
fn default_cc_60() -> u8 { 60 }
fn default_cc_61() -> u8 { 61 }
fn default_cc_62() -> u8 { 62 }
fn default_cc_63() -> u8 { 63 }
fn default_cc_64() -> u8 { 64 }
fn default_cc_65() -> u8 { 65 }
fn default_cc_66() -> u8 { 66 }
fn default_cc_67() -> u8 { 67 }
fn default_cc_68() -> u8 { 68 }
fn default_cc_69() -> u8 { 69 }
fn default_cc_70() -> u8 { 70 }
fn default_cc_71() -> u8 { 71 }
fn default_cc_72() -> u8 { 72 }
fn default_cc_73() -> u8 { 73 }
fn default_cc_74() -> u8 { 74 }
fn default_cc_75() -> u8 { 75 }
fn default_cc_76() -> u8 { 76 }
fn default_cc_77() -> u8 { 77 }
fn default_cc_78() -> u8 { 78 }
fn default_cc_79() -> u8 { 79 }
fn default_cc_80() -> u8 { 80 }
fn default_cc_81() -> u8 { 81 }
fn default_cc_82() -> u8 { 82 }
fn default_cc_83() -> u8 { 83 }
fn default_cc_84() -> u8 { 84 }
fn default_cc_85() -> u8 { 85 }
fn default_cc_86() -> u8 { 86 }
fn default_cc_87() -> u8 { 87 }
fn default_cc_88() -> u8 { 88 }
fn default_cc_89() -> u8 { 89 }
fn default_cc_90() -> u8 { 90 }
fn default_cc_91() -> u8 { 91 }
fn default_cc_92() -> u8 { 92 }
fn default_cc_93() -> u8 { 93 }

impl Default for ShiftCombinationMap {
    fn default() -> Self {
        Self {
            left: 54,
            right: 55,
            maschine: 56,
            star: 57,
            browse: 58,
            volume: 59,
            swing: 60,
            tempo: 61,
            plugin: 62,
            sampling: 63,
            pitch: 64,
            mod_button: 65,
            perform: 66,
            notes: 67,
            group: 68,
            auto: 69,
            lock: 70,
            note_repeat: 71,
            fixed_vel: 72,
            pad_mode: 73,
            keyboard: 74,
            chords: 75,
            step: 76,
            scene: 77,
            pattern: 78,
            events: 79,
            variation: 80,
            duplicate: 81,
            select: 82,
            solo: 83,
            mute: 84,
            play: 85,
            stop: 86,
            rec: 87,
            restart: 88,
            erase: 89,
            tap: 90,
            follow: 91,
            encoder_press: 92,
            encoder_touch: 93,
        }
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct Settings {
    #[serde(default)]
    pub notemaps: Vec<u8>,
    #[serde(default)]
    pub client_name: String,
    #[serde(default)]
    pub port_name: String,
    #[serde(default)]
    pub transport_channel: u8,
    #[serde(default)]
    pub transport_cc_map: TransportCCMap,
    #[serde(default)]
    pub button_channel: u8,
    #[serde(default)]
    pub button_cc_map: ButtonCCMap,
    #[serde(default)]
    pub encoder_channel: u8,
    #[serde(default)]
    pub encoder_cc_map: EncoderCCMap,
    #[serde(default)]
    pub touch_strip_channel: u8,
    #[serde(default)]
    pub touch_strip_cc: u8,
    #[serde(default)]
    pub button_modes: ButtonModeConfig,
    #[serde(default)]
    pub shift_combinations: ShiftCombinationMap,
}

#[derive(Deserialize, Debug)]
pub(crate) struct TransportCCMap {
    pub play: u8,
    pub stop: u8,
    pub rec: u8,
    pub restart: u8,
    pub erase: u8,
    pub tap: u8,
    pub follow: u8,
}

impl Default for TransportCCMap {
    fn default() -> Self {
        Self {
            play: 118,
            stop: 117,
            rec: 119,
            restart: 116,
            erase: 120,
            tap: 121,
            follow: 122,
        }
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct ButtonCCMap {
    // Navigation
    pub left: u8,
    pub right: u8,
    
    // Main Control
    pub maschine: u8,
    pub star: u8,
    pub browse: u8,
    pub volume: u8,
    
    // Performance
    pub swing: u8,
    pub tempo: u8,
    pub plugin: u8,
    pub sampling: u8,
    
    // Pitch/Mod
    pub pitch: u8,
    pub mod_button: u8,
    
    // Mode Selection
    pub perform: u8,
    pub notes: u8,
    pub group: u8,
    pub auto: u8,
    
    // Recording
    pub lock: u8,
    pub note_repeat: u8,
    
    // Modifiers
    pub shift: u8,
    pub fixed_vol: u8,
    
    // Pad Modes
    pub pad_mode: u8,
    pub keyboard: u8,
    pub chords: u8,
    pub step: u8,
    
    // Sequencer
    pub scene: u8,
    pub pattern: u8,
    pub events: u8,
    pub variation: u8,
    pub duplicate: u8,
    
    // Track Control
    pub select: u8,
    pub solo: u8,
    pub mute: u8,
}

impl Default for ButtonCCMap {
    fn default() -> Self {
        Self {
            // Navigation
            left: 20,
            right: 21,
            
            // Main Control
            maschine: 22,
            star: 23,
            browse: 24,
            volume: 25,
            
            // Performance
            swing: 26,
            tempo: 27,
            plugin: 28,
            sampling: 29,
            
            // Pitch/Mod
            pitch: 30,
            mod_button: 31,
            
            // Mode Selection
            perform: 32,
            notes: 33,
            group: 34,
            auto: 35,
            
            // Recording
            lock: 36,
            note_repeat: 37,
            
            // Modifiers
            shift: 38,
            fixed_vol: 39,
            
            // Pad Modes
            pad_mode: 40,
            keyboard: 41,
            chords: 42,
            step: 43,
            
            // Sequencer
            scene: 44,
            pattern: 45,
            events: 46,
            variation: 47,
            duplicate: 48,
            
            // Track Control
            select: 49,
            solo: 50,
            mute: 51,
        }
    }
}

#[derive(Deserialize, Debug)]
pub(crate) struct EncoderCCMap {
    pub rotation: u8,
    pub press: u8,
    pub touch: u8,
}

impl Default for EncoderCCMap {
    fn default() -> Self {
        Self {
            rotation: 10,
            press: 52,
            touch: 53,
        }
    }
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            notemaps: vec![
                49, 27, 31, 57, 48, 47, 43, 59, 36, 38, 46, 51, 36, 38, 42, 44,
            ],
            client_name: "Maschine Mikro MK3".to_string(),
            port_name: "Maschine Mikro MK3 MIDI Out".to_string(),
            transport_channel: 0,
            transport_cc_map: TransportCCMap::default(),
            button_channel: 0,
            button_cc_map: ButtonCCMap::default(),
            encoder_channel: 0,
            encoder_cc_map: EncoderCCMap::default(),
            touch_strip_channel: 0,
            touch_strip_cc: 1,
            button_modes: ButtonModeConfig::default(),
            shift_combinations: ShiftCombinationMap::default(),
        }
    }
}

impl Settings {
    pub(crate) fn validate(&self) -> Result<(), String> {
        // todo: is there a better way to do it that doesn't bring too many new useless dependencies?

        let padcnt = self.notemaps.len();
        if padcnt != 16 {
            return Err(format!("The should be 16 pads exactly (found {padcnt})"));
        }

        if self.notemaps.iter().any(|x| *x >= 128) {
            return Err("MIDI notes should be 0 to 127".to_string());
        }

        if self.client_name.is_empty() {
            return Err("Client name must not be empty".to_string());
        }

        if self.port_name.is_empty() {
            return Err("Port name must not be empty".to_string());
        }

        // Validate transport channel is in valid MIDI channel range (0-15)
        if self.transport_channel > 15 {
            return Err(format!(
                "Transport MIDI channel must be 0-15 (found {})",
                self.transport_channel
            ));
        }

        // Validate all CC numbers in transport_cc_map are in valid range (0-127)
        let cc_values = [
            ("play", self.transport_cc_map.play),
            ("stop", self.transport_cc_map.stop),
            ("rec", self.transport_cc_map.rec),
            ("restart", self.transport_cc_map.restart),
            ("erase", self.transport_cc_map.erase),
            ("tap", self.transport_cc_map.tap),
            ("follow", self.transport_cc_map.follow),
        ];

        for (name, cc) in cc_values {
            if cc >= 128 {
                return Err(format!(
                    "Transport CC for {} must be 0-127 (found {})",
                    name, cc
                ));
            }
        }

        // Validate button_channel is in valid MIDI channel range (0-15)
        if self.button_channel > 15 {
            return Err(format!(
                "Button MIDI channel must be 0-15 (found {})",
                self.button_channel
            ));
        }

        // Validate encoder_channel is in valid MIDI channel range (0-15)
        if self.encoder_channel > 15 {
            return Err(format!(
                "Encoder MIDI channel must be 0-15 (found {})",
                self.encoder_channel
            ));
        }

        // Validate touch_strip_channel is in valid MIDI channel range (0-15)
        if self.touch_strip_channel > 15 {
            return Err(format!(
                "Touch strip MIDI channel must be 0-15 (found {})",
                self.touch_strip_channel
            ));
        }

        // Validate touch_strip_cc is in valid CC range (0-127)
        if self.touch_strip_cc >= 128 {
            return Err(format!(
                "Touch strip CC must be 0-127 (found {})",
                self.touch_strip_cc
            ));
        }

        // Validate all button CC numbers in ButtonCCMap are in valid range (0-127)
        let button_ccs = [
            ("left", self.button_cc_map.left),
            ("right", self.button_cc_map.right),
            ("maschine", self.button_cc_map.maschine),
            ("star", self.button_cc_map.star),
            ("browse", self.button_cc_map.browse),
            ("volume", self.button_cc_map.volume),
            ("swing", self.button_cc_map.swing),
            ("tempo", self.button_cc_map.tempo),
            ("plugin", self.button_cc_map.plugin),
            ("sampling", self.button_cc_map.sampling),
            ("pitch", self.button_cc_map.pitch),
            ("mod_button", self.button_cc_map.mod_button),
            ("perform", self.button_cc_map.perform),
            ("notes", self.button_cc_map.notes),
            ("group", self.button_cc_map.group),
            ("auto", self.button_cc_map.auto),
            ("lock", self.button_cc_map.lock),
            ("note_repeat", self.button_cc_map.note_repeat),
            ("shift", self.button_cc_map.shift),
            ("fixed_vol", self.button_cc_map.fixed_vol),
            ("pad_mode", self.button_cc_map.pad_mode),
            ("keyboard", self.button_cc_map.keyboard),
            ("chords", self.button_cc_map.chords),
            ("step", self.button_cc_map.step),
            ("scene", self.button_cc_map.scene),
            ("pattern", self.button_cc_map.pattern),
            ("events", self.button_cc_map.events),
            ("variation", self.button_cc_map.variation),
            ("duplicate", self.button_cc_map.duplicate),
            ("select", self.button_cc_map.select),
            ("solo", self.button_cc_map.solo),
            ("mute", self.button_cc_map.mute),
        ];

        for (name, cc) in button_ccs {
            if cc >= 128 {
                return Err(format!(
                    "Button CC for {} must be 0-127 (found {})",
                    name, cc
                ));
            }
        }

        // Validate all encoder CC numbers in EncoderCCMap are in valid range (0-127)
        let encoder_ccs = [
            ("rotation", self.encoder_cc_map.rotation),
            ("press", self.encoder_cc_map.press),
            ("touch", self.encoder_cc_map.touch),
        ];

        for (name, cc) in encoder_ccs {
            if cc >= 128 {
                return Err(format!(
                    "Encoder CC for {} must be 0-127 (found {})",
                    name, cc
                ));
            }
        }

        // Validate all Shift combination CC numbers are in valid range (0-127)
        let shift_ccs = [
            ("shift_combinations.left", self.shift_combinations.left),
            ("shift_combinations.right", self.shift_combinations.right),
            ("shift_combinations.maschine", self.shift_combinations.maschine),
            ("shift_combinations.star", self.shift_combinations.star),
            ("shift_combinations.browse", self.shift_combinations.browse),
            ("shift_combinations.volume", self.shift_combinations.volume),
            ("shift_combinations.swing", self.shift_combinations.swing),
            ("shift_combinations.tempo", self.shift_combinations.tempo),
            ("shift_combinations.plugin", self.shift_combinations.plugin),
            ("shift_combinations.sampling", self.shift_combinations.sampling),
            ("shift_combinations.pitch", self.shift_combinations.pitch),
            ("shift_combinations.mod_button", self.shift_combinations.mod_button),
            ("shift_combinations.perform", self.shift_combinations.perform),
            ("shift_combinations.notes", self.shift_combinations.notes),
            ("shift_combinations.group", self.shift_combinations.group),
            ("shift_combinations.auto", self.shift_combinations.auto),
            ("shift_combinations.lock", self.shift_combinations.lock),
            ("shift_combinations.note_repeat", self.shift_combinations.note_repeat),
            ("shift_combinations.fixed_vel", self.shift_combinations.fixed_vel),
            ("shift_combinations.pad_mode", self.shift_combinations.pad_mode),
            ("shift_combinations.keyboard", self.shift_combinations.keyboard),
            ("shift_combinations.chords", self.shift_combinations.chords),
            ("shift_combinations.step", self.shift_combinations.step),
            ("shift_combinations.scene", self.shift_combinations.scene),
            ("shift_combinations.pattern", self.shift_combinations.pattern),
            ("shift_combinations.events", self.shift_combinations.events),
            ("shift_combinations.variation", self.shift_combinations.variation),
            ("shift_combinations.duplicate", self.shift_combinations.duplicate),
            ("shift_combinations.select", self.shift_combinations.select),
            ("shift_combinations.solo", self.shift_combinations.solo),
            ("shift_combinations.mute", self.shift_combinations.mute),
            ("shift_combinations.play", self.shift_combinations.play),
            ("shift_combinations.stop", self.shift_combinations.stop),
            ("shift_combinations.rec", self.shift_combinations.rec),
            ("shift_combinations.restart", self.shift_combinations.restart),
            ("shift_combinations.erase", self.shift_combinations.erase),
            ("shift_combinations.tap", self.shift_combinations.tap),
            ("shift_combinations.follow", self.shift_combinations.follow),
            ("shift_combinations.encoder_press", self.shift_combinations.encoder_press),
            ("shift_combinations.encoder_touch", self.shift_combinations.encoder_touch),
        ];

        for (name, cc) in shift_ccs {
            if cc >= 128 {
                return Err(format!(
                    "Shift combination CC for {} must be 0-127 (found {})",
                    name, cc
                ));
            }
        }

        // Validate toggle button names match valid button enum variants
        // Valid button names (case-insensitive) based on Buttons enum
        let valid_buttons = [
            "maschine", "star", "browse", "volume",
            "swing", "tempo", "plugin", "sampling",
            "left", "right", "pitch", "mod",
            "perform", "notes", "group", "auto",
            "lock", "noterepeat", "restart", "erase",
            "tap", "follow", "play", "rec",
            "stop", "shift", "fixedvel", "padmode",
            "keyboard", "chords", "step", "scene",
            "pattern", "events", "variation", "duplicate",
            "select", "solo", "mute",
            "encoderpress", "encodertouch",
        ];

        for (button_name, _) in self.button_modes.toggle_buttons.iter() {
            let name_lower = button_name.to_lowercase();
            if !valid_buttons.contains(&name_lower.as_str()) {
                return Err(format!(
                    "Invalid button name in button_modes.toggle_buttons: '{}'. Valid button names are: {}",
                    button_name,
                    valid_buttons.join(", ")
                ));
            }
        }

        Ok(())
    }
}
