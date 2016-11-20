# [ doc = "Operational amplifiers" ]
# [ repr ( C ) ]
pub struct Opamp {
    # [ doc = "0x00 - OPAMP1 control/status register" ]
    pub opamp1_csr: Opamp1Csr,
    # [ doc = "0x04 - OPAMP1 offset trimming register in normal mode" ]
    pub opamp1_otr: Opamp1Otr,
    # [ doc = "0x08 - OPAMP1 offset trimming register in low-power mode" ]
    pub opamp1_lpotr: Opamp1Lpotr,
    _reserved0: [u8; 4usize],
    # [ doc = "0x10 - OPAMP2 control/status register" ]
    pub opamp2_csr: Opamp2Csr,
    # [ doc = "0x14 - OPAMP2 offset trimming register in normal mode" ]
    pub opamp2_otr: Opamp2Otr,
    # [ doc = "0x18 - OPAMP2 offset trimming register in low-power mode" ]
    pub opamp2_lpotr: Opamp2Lpotr,
}

# [ repr ( C ) ]
pub struct Opamp1Csr {
    register: ::volatile_register::RW<u32>,
}

impl Opamp1Csr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Opamp1CsrR, &'w mut Opamp1CsrW) -> &'w mut Opamp1CsrW
    {
        let bits = self.register.read();
        let r = Opamp1CsrR { bits: bits };
        let mut w = Opamp1CsrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Opamp1CsrR {
        Opamp1CsrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Opamp1CsrW) -> &mut Opamp1CsrW
    {
        let mut w = Opamp1CsrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Opamp1CsrR {
    bits: u32,
}

impl Opamp1CsrR {
    # [ doc = "Bit 0 - Operational amplifier Enable" ]
    pub fn opaen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Operational amplifier Low Power Mode" ]
    pub fn opalpm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:3 - Operational amplifier PGA mode" ]
    pub fn opamode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:5 - Operational amplifier Programmable amplifier gain value" ]
    pub fn pga_gain(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:9 - Inverting input selection" ]
    pub fn vm_sel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 10 - Non inverted input selection" ]
    pub fn vp_sel(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Calibration mode enabled" ]
    pub fn calon(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Calibration selection" ]
    pub fn calsel(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - allows to switch from ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¹Ãƒâ€¦Ã¢â‚¬Å“factoryÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ AOP offset trimmed values to AOP offset ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¹Ãƒâ€¦Ã¢â‚¬Å“userÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢" ]
    pub fn usertrim(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Operational amplifier calibration output" ]
    pub fn calout(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Operational amplifier power supply range for stability" ]
    pub fn opa_range(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Opamp1CsrW {
    bits: u32,
}

impl Opamp1CsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Opamp1CsrW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Operational amplifier Enable" ]
    pub fn opaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Operational amplifier Low Power Mode" ]
    pub fn opalpm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:3 - Operational amplifier PGA mode" ]
    pub fn opamode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:5 - Operational amplifier Programmable amplifier gain value" ]
    pub fn pga_gain(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:9 - Inverting input selection" ]
    pub fn vm_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Non inverted input selection" ]
    pub fn vp_sel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Calibration mode enabled" ]
    pub fn calon(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Calibration selection" ]
    pub fn calsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - allows to switch from ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¹Ãƒâ€¦Ã¢â‚¬Å“factoryÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ AOP offset trimmed values to AOP offset ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¹Ãƒâ€¦Ã¢â‚¬Å“userÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢" ]
    pub fn usertrim(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Operational amplifier calibration output" ]
    pub fn calout(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Operational amplifier power supply range for stability" ]
    pub fn opa_range(&mut self, value: bool) -> &mut Self {
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
pub struct Opamp1Otr {
    register: ::volatile_register::RW<u32>,
}

impl Opamp1Otr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Opamp1OtrR, &'w mut Opamp1OtrW) -> &'w mut Opamp1OtrW
    {
        let bits = self.register.read();
        let r = Opamp1OtrR { bits: bits };
        let mut w = Opamp1OtrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Opamp1OtrR {
        Opamp1OtrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Opamp1OtrW) -> &mut Opamp1OtrW
    {
        let mut w = Opamp1OtrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Opamp1OtrR {
    bits: u32,
}

impl Opamp1OtrR {
    # [ doc = "Bits 0:4 - Trim for NMOS differential pairs" ]
    pub fn trimoffsetn(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:12 - Trim for PMOS differential pairs" ]
    pub fn trimoffsetp(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Opamp1OtrW {
    bits: u32,
}

impl Opamp1OtrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Opamp1OtrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:4 - Trim for NMOS differential pairs" ]
    pub fn trimoffsetn(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:12 - Trim for PMOS differential pairs" ]
    pub fn trimoffsetp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Opamp1Lpotr {
    register: ::volatile_register::RW<u32>,
}

impl Opamp1Lpotr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Opamp1LpotrR, &'w mut Opamp1LpotrW) -> &'w mut Opamp1LpotrW
    {
        let bits = self.register.read();
        let r = Opamp1LpotrR { bits: bits };
        let mut w = Opamp1LpotrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Opamp1LpotrR {
        Opamp1LpotrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Opamp1LpotrW) -> &mut Opamp1LpotrW
    {
        let mut w = Opamp1LpotrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Opamp1LpotrR {
    bits: u32,
}

impl Opamp1LpotrR {
    # [ doc = "Bits 0:4 - Trim for NMOS differential pairs" ]
    pub fn trimlpoffsetn(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:12 - Trim for PMOS differential pairs" ]
    pub fn trimlpoffsetp(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Opamp1LpotrW {
    bits: u32,
}

impl Opamp1LpotrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Opamp1LpotrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:4 - Trim for NMOS differential pairs" ]
    pub fn trimlpoffsetn(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:12 - Trim for PMOS differential pairs" ]
    pub fn trimlpoffsetp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Opamp2Csr {
    register: ::volatile_register::RW<u32>,
}

impl Opamp2Csr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Opamp2CsrR, &'w mut Opamp2CsrW) -> &'w mut Opamp2CsrW
    {
        let bits = self.register.read();
        let r = Opamp2CsrR { bits: bits };
        let mut w = Opamp2CsrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Opamp2CsrR {
        Opamp2CsrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Opamp2CsrW) -> &mut Opamp2CsrW
    {
        let mut w = Opamp2CsrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Opamp2CsrR {
    bits: u32,
}

impl Opamp2CsrR {
    # [ doc = "Bit 0 - Operational amplifier Enable" ]
    pub fn opaen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Operational amplifier Low Power Mode" ]
    pub fn opalpm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:3 - Operational amplifier PGA mode" ]
    pub fn opamode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:5 - Operational amplifier Programmable amplifier gain value" ]
    pub fn pga_gain(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:9 - Inverting input selection" ]
    pub fn vm_sel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 10 - Non inverted input selection" ]
    pub fn vp_sel(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Calibration mode enabled" ]
    pub fn calon(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Calibration selection" ]
    pub fn calsel(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - allows to switch from ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¹Ãƒâ€¦Ã¢â‚¬Å“factoryÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ AOP offset trimmed values to AOP offset ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¹Ãƒâ€¦Ã¢â‚¬Å“userÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢" ]
    pub fn usertrim(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Operational amplifier calibration output" ]
    pub fn calout(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Opamp2CsrW {
    bits: u32,
}

impl Opamp2CsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Opamp2CsrW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Operational amplifier Enable" ]
    pub fn opaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Operational amplifier Low Power Mode" ]
    pub fn opalpm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:3 - Operational amplifier PGA mode" ]
    pub fn opamode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:5 - Operational amplifier Programmable amplifier gain value" ]
    pub fn pga_gain(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:9 - Inverting input selection" ]
    pub fn vm_sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Non inverted input selection" ]
    pub fn vp_sel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Calibration mode enabled" ]
    pub fn calon(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Calibration selection" ]
    pub fn calsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - allows to switch from ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¹Ãƒâ€¦Ã¢â‚¬Å“factoryÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢ AOP offset trimmed values to AOP offset ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã¢â‚¬Â¹Ãƒâ€¦Ã¢â‚¬Å“userÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â€šÂ¬Ã…Â¾Ãƒâ€šÃ‚Â¢" ]
    pub fn usertrim(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Operational amplifier calibration output" ]
    pub fn calout(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Opamp2Otr {
    register: ::volatile_register::RW<u32>,
}

impl Opamp2Otr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Opamp2OtrR, &'w mut Opamp2OtrW) -> &'w mut Opamp2OtrW
    {
        let bits = self.register.read();
        let r = Opamp2OtrR { bits: bits };
        let mut w = Opamp2OtrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Opamp2OtrR {
        Opamp2OtrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Opamp2OtrW) -> &mut Opamp2OtrW
    {
        let mut w = Opamp2OtrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Opamp2OtrR {
    bits: u32,
}

impl Opamp2OtrR {
    # [ doc = "Bits 0:4 - Trim for NMOS differential pairs" ]
    pub fn trimoffsetn(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:12 - Trim for PMOS differential pairs" ]
    pub fn trimoffsetp(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Opamp2OtrW {
    bits: u32,
}

impl Opamp2OtrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Opamp2OtrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:4 - Trim for NMOS differential pairs" ]
    pub fn trimoffsetn(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:12 - Trim for PMOS differential pairs" ]
    pub fn trimoffsetp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Opamp2Lpotr {
    register: ::volatile_register::RW<u32>,
}

impl Opamp2Lpotr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Opamp2LpotrR, &'w mut Opamp2LpotrW) -> &'w mut Opamp2LpotrW
    {
        let bits = self.register.read();
        let r = Opamp2LpotrR { bits: bits };
        let mut w = Opamp2LpotrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Opamp2LpotrR {
        Opamp2LpotrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Opamp2LpotrW) -> &mut Opamp2LpotrW
    {
        let mut w = Opamp2LpotrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Opamp2LpotrR {
    bits: u32,
}

impl Opamp2LpotrR {
    # [ doc = "Bits 0:4 - Trim for NMOS differential pairs" ]
    pub fn trimlpoffsetn(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:12 - Trim for PMOS differential pairs" ]
    pub fn trimlpoffsetp(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Opamp2LpotrW {
    bits: u32,
}

impl Opamp2LpotrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Opamp2LpotrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:4 - Trim for NMOS differential pairs" ]
    pub fn trimlpoffsetn(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:12 - Trim for PMOS differential pairs" ]
    pub fn trimlpoffsetp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
