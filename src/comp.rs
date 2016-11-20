# [ doc = "Comparator" ]
# [ repr ( C ) ]
pub struct Comp {
    # [ doc = "0x00 - Comparator 1 control and status register" ]
    pub comp1_csr: Comp1Csr,
    # [ doc = "0x04 - Comparator 2 control and status register" ]
    pub comp2_csr: Comp2Csr,
}

# [ repr ( C ) ]
pub struct Comp1Csr {
    register: ::volatile_register::RW<u32>,
}

impl Comp1Csr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Comp1CsrR, &'w mut Comp1CsrW) -> &'w mut Comp1CsrW
    {
        let bits = self.register.read();
        let r = Comp1CsrR { bits: bits };
        let mut w = Comp1CsrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Comp1CsrR {
        Comp1CsrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Comp1CsrW) -> &mut Comp1CsrW
    {
        let mut w = Comp1CsrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Comp1CsrR {
    bits: u32,
}

impl Comp1CsrR {
    # [ doc = "Bit 0 - Comparator 1 enable bit" ]
    pub fn comp1_en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:3 - Power Mode of the comparator 1" ]
    pub fn comp1_pwrmode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:6 - Comparator 1 Input Minus connection configuration bit" ]
    pub fn comp1_inmsel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Comparator1 input plus selection bit" ]
    pub fn comp1_inpsel(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Comparator 1 polarity selection bit" ]
    pub fn comp1_polarity(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 16:17 - Comparator 1 hysteresis selection bits" ]
    pub fn comp1_hyst(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 18:20 - Comparator 1 blanking source selection bits" ]
    pub fn comp1_blanking(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 22 - Scaler bridge enable" ]
    pub fn comp1_brgen(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - Voltage scaler enable bit" ]
    pub fn comp1_scalen(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Comparator 1 output status bit" ]
    pub fn comp1_value(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Comp1CsrW {
    bits: u32,
}

impl Comp1CsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Comp1CsrW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Comparator 1 enable bit" ]
    pub fn comp1_en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:3 - Power Mode of the comparator 1" ]
    pub fn comp1_pwrmode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:6 - Comparator 1 Input Minus connection configuration bit" ]
    pub fn comp1_inmsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Comparator1 input plus selection bit" ]
    pub fn comp1_inpsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Comparator 1 polarity selection bit" ]
    pub fn comp1_polarity(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 16:17 - Comparator 1 hysteresis selection bits" ]
    pub fn comp1_hyst(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 18:20 - Comparator 1 blanking source selection bits" ]
    pub fn comp1_blanking(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 22 - Scaler bridge enable" ]
    pub fn comp1_brgen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - Voltage scaler enable bit" ]
    pub fn comp1_scalen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - COMP1_CSR register lock bit" ]
    pub fn comp1_lock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Comp2Csr {
    register: ::volatile_register::RW<u32>,
}

impl Comp2Csr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Comp2CsrR, &'w mut Comp2CsrW) -> &'w mut Comp2CsrW
    {
        let bits = self.register.read();
        let r = Comp2CsrR { bits: bits };
        let mut w = Comp2CsrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Comp2CsrR {
        Comp2CsrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Comp2CsrW) -> &mut Comp2CsrW
    {
        let mut w = Comp2CsrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Comp2CsrR {
    bits: u32,
}

impl Comp2CsrR {
    # [ doc = "Bit 0 - Comparator 2 enable bit" ]
    pub fn comp2_en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:3 - Power Mode of the comparator 2" ]
    pub fn comp2_pwrmode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit" ]
    pub fn comp2_inmsel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Comparator 2 Input Plus connection configuration bit" ]
    pub fn comp2_inpsel(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Windows mode selection bit" ]
    pub fn comp2_winmode(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Comparator 2 polarity selection bit" ]
    pub fn comp2_polarity(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 16:17 - Comparator 2 hysteresis selection bits" ]
    pub fn comp2_hyst(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 18:20 - Comparator 2 blanking source selection bits" ]
    pub fn comp2_blanking(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 22 - Scaler bridge enable" ]
    pub fn comp2_brgen(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - Voltage scaler enable bit" ]
    pub fn comp2_scalen(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Comparator 2 output status bit" ]
    pub fn comp2_value(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Comp2CsrW {
    bits: u32,
}

impl Comp2CsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Comp2CsrW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Comparator 2 enable bit" ]
    pub fn comp2_en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:3 - Power Mode of the comparator 2" ]
    pub fn comp2_pwrmode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:6 - Comparator 2 Input Minus connection configuration bit" ]
    pub fn comp2_inmsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Comparator 2 Input Plus connection configuration bit" ]
    pub fn comp2_inpsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Windows mode selection bit" ]
    pub fn comp2_winmode(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Comparator 2 polarity selection bit" ]
    pub fn comp2_polarity(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 16:17 - Comparator 2 hysteresis selection bits" ]
    pub fn comp2_hyst(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 18:20 - Comparator 2 blanking source selection bits" ]
    pub fn comp2_blanking(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 22 - Scaler bridge enable" ]
    pub fn comp2_brgen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - Voltage scaler enable bit" ]
    pub fn comp2_scalen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - COMP2_CSR register lock bit" ]
    pub fn comp2_lock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}
