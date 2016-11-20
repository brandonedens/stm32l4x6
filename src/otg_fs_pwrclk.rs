# [ doc = "USB on the go full speed" ]
# [ repr ( C ) ]
pub struct OtgFsPwrclk {
    # [ doc = "0x00 - OTG_FS power and clock gating control register (OTG_FS_PCGCCTL)" ]
    pub fs_pcgcctl: FsPcgcctl,
}

# [ repr ( C ) ]
pub struct FsPcgcctl {
    register: ::volatile_register::RW<u32>,
}

impl FsPcgcctl {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsPcgcctlR, &'w mut FsPcgcctlW) -> &'w mut FsPcgcctlW
    {
        let bits = self.register.read();
        let r = FsPcgcctlR { bits: bits };
        let mut w = FsPcgcctlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsPcgcctlR {
        FsPcgcctlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsPcgcctlW) -> &mut FsPcgcctlW
    {
        let mut w = FsPcgcctlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsPcgcctlR {
    bits: u32,
}

impl FsPcgcctlR {
    # [ doc = "Bit 0 - Stop PHY clock" ]
    pub fn stppclk(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Gate HCLK" ]
    pub fn gatehclk(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - PHY Suspended" ]
    pub fn physusp(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsPcgcctlW {
    bits: u32,
}

impl FsPcgcctlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsPcgcctlW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Stop PHY clock" ]
    pub fn stppclk(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Gate HCLK" ]
    pub fn gatehclk(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - PHY Suspended" ]
    pub fn physusp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}
