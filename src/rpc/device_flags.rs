#[derive(Clone, Copy, Debug, Default, Deserialize, Serialize)]
pub struct DeviceFlags(u32);

impl DeviceFlags {
    pub fn wait_lock(&self) -> bool {
        (self.0 & 0x01) != 0
    }

    pub fn set_wait_lock(&mut self, bit_value: bool) {
        if bit_value == true {
            self.0 |= 0x01;
        } else {
            self.0 &= !0x01;
        }
    }

    pub fn end_indicator(&self) -> bool {
        (self.0 & 0x08) != 0
    }

    pub fn set_end_indicator(&mut self, bit_value: bool) {
        if bit_value == true {
            self.0 |= 0x08;
        } else {
            self.0 &= !0x08;
        }
    }

    pub fn termination_char_set(&self) -> bool {
        (self.0 & 0x80) != 0
    }

    pub fn set_termination_char_set(&mut self, bit_value: bool) {
        if bit_value == true {
            self.0 |= 0x80;
        } else {
            self.0 &= !0x80;
        }
    }
}
