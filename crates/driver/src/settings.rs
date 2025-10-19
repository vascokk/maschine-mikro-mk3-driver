use serde::Deserialize;

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

        Ok(())
    }
}
