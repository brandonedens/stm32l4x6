# [ doc = "System window watchdog" ]
# [ repr ( C ) ]
pub struct Wwdg {
    # [ doc = "0x00 - Control register" ]
    pub cr: Cr,
    # [ doc = "0x04 - Configuration register" ]
    pub cfr: Cfr,
    # [ doc = "0x08 - Status register" ]
    pub sr: Sr,
}

# [ repr ( C ) ]
pub struct Cr {
    register: ::volatile_register::RW<u32>,
}

impl Cr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CrR, &'w mut CrW) -> &'w mut CrW
    {
        let bits = self.register.read();
        let r = CrR { bits: bits };
        let mut w = CrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CrR {
        CrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CrW) -> &mut CrW
    {
        let mut w = CrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CrR {
    bits: u32,
}

impl CrR {
    # [ doc = "Bit 7 - Activation bit" ]
    pub fn wdga(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:6 - 7-bit counter (MSB to LSB)" ]
    pub fn t(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CrW {
    bits: u32,
}

impl CrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CrW { bits: 127u32 }
    }
    # [ doc = "Bit 7 - Activation bit" ]
    pub fn wdga(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:6 - 7-bit counter (MSB to LSB)" ]
    pub fn t(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cfr {
    register: ::volatile_register::RW<u32>,
}

impl Cfr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CfrR, &'w mut CfrW) -> &'w mut CfrW
    {
        let bits = self.register.read();
        let r = CfrR { bits: bits };
        let mut w = CfrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CfrR {
        CfrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CfrW) -> &mut CfrW
    {
        let mut w = CfrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CfrR {
    bits: u32,
}

impl CfrR {
    # [ doc = "Bit 9 - Early wakeup interrupt" ]
    pub fn ewi(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 7:8 - Timer base" ]
    pub fn wdgtb(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 7u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:6 - 7-bit window value" ]
    pub fn w(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CfrW {
    bits: u32,
}

impl CfrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CfrW { bits: 127u32 }
    }
    # [ doc = "Bit 9 - Early wakeup interrupt" ]
    pub fn ewi(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 7:8 - Timer base" ]
    pub fn wdgtb(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 7u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:6 - 7-bit window value" ]
    pub fn w(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Sr {
    register: ::volatile_register::RW<u32>,
}

impl Sr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&SrR, &'w mut SrW) -> &'w mut SrW
    {
        let bits = self.register.read();
        let r = SrR { bits: bits };
        let mut w = SrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> SrR {
        SrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut SrW) -> &mut SrW
    {
        let mut w = SrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SrR {
    bits: u32,
}

impl SrR {
    # [ doc = "Bit 0 - Early wakeup interrupt flag" ]
    pub fn ewif(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SrW {
    bits: u32,
}

impl SrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        SrW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Early wakeup interrupt flag" ]
    pub fn ewif(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}
