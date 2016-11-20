# [ doc = "Analog-to-Digital Converter" ]
# [ repr ( C ) ]
pub struct AdcCommon {
    # [ doc = "0x00 - ADC Common status register" ]
    pub csr: Csr,
    _reserved0: [u8; 4usize],
    # [ doc = "0x08 - ADC common control register" ]
    pub ccr: Ccr,
    # [ doc = "0x0c - ADC common regular data register for dual and triple modes" ]
    pub cdr: Cdr,
}

# [ repr ( C ) ]
pub struct Csr {
    register: ::volatile_register::RO<u32>,
}

impl Csr {
    pub fn read(&self) -> CsrR {
        CsrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CsrR {
    bits: u32,
}

impl CsrR {
    # [ doc = "Bit 0 - ADDRDY_MST" ]
    pub fn addrdy_mst(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - EOSMP_MST" ]
    pub fn eosmp_mst(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - EOC_MST" ]
    pub fn eoc_mst(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - EOS_MST" ]
    pub fn eos_mst(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - OVR_MST" ]
    pub fn ovr_mst(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - JEOC_MST" ]
    pub fn jeoc_mst(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - JEOS_MST" ]
    pub fn jeos_mst(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - AWD1_MST" ]
    pub fn awd1_mst(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - AWD2_MST" ]
    pub fn awd2_mst(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - AWD3_MST" ]
    pub fn awd3_mst(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - JQOVF_MST" ]
    pub fn jqovf_mst(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - ADRDY_SLV" ]
    pub fn adrdy_slv(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - EOSMP_SLV" ]
    pub fn eosmp_slv(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - End of regular conversion of the slave ADC" ]
    pub fn eoc_slv(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - End of regular sequence flag of the slave ADC" ]
    pub fn eos_slv(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Overrun flag of the slave ADC" ]
    pub fn ovr_slv(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - End of injected conversion flag of the slave ADC" ]
    pub fn jeoc_slv(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - End of injected sequence flag of the slave ADC" ]
    pub fn jeos_slv(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - Analog watchdog 1 flag of the slave ADC" ]
    pub fn awd1_slv(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Analog watchdog 2 flag of the slave ADC" ]
    pub fn awd2_slv(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Analog watchdog 3 flag of the slave ADC" ]
    pub fn awd3_slv(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Injected Context Queue Overflow flag of the slave ADC" ]
    pub fn jqovf_slv(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CsrW {
    bits: u32,
}

impl CsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CsrW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - ADDRDY_MST" ]
    pub fn addrdy_mst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - EOSMP_MST" ]
    pub fn eosmp_mst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - EOC_MST" ]
    pub fn eoc_mst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - EOS_MST" ]
    pub fn eos_mst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - OVR_MST" ]
    pub fn ovr_mst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - JEOC_MST" ]
    pub fn jeoc_mst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - JEOS_MST" ]
    pub fn jeos_mst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - AWD1_MST" ]
    pub fn awd1_mst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - AWD2_MST" ]
    pub fn awd2_mst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - AWD3_MST" ]
    pub fn awd3_mst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - JQOVF_MST" ]
    pub fn jqovf_mst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - ADRDY_SLV" ]
    pub fn adrdy_slv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - EOSMP_SLV" ]
    pub fn eosmp_slv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - End of regular conversion of the slave ADC" ]
    pub fn eoc_slv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - End of regular sequence flag of the slave ADC" ]
    pub fn eos_slv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Overrun flag of the slave ADC" ]
    pub fn ovr_slv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - End of injected conversion flag of the slave ADC" ]
    pub fn jeoc_slv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - End of injected sequence flag of the slave ADC" ]
    pub fn jeos_slv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - Analog watchdog 1 flag of the slave ADC" ]
    pub fn awd1_slv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Analog watchdog 2 flag of the slave ADC" ]
    pub fn awd2_slv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Analog watchdog 3 flag of the slave ADC" ]
    pub fn awd3_slv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Injected Context Queue Overflow flag of the slave ADC" ]
    pub fn jqovf_slv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Ccr {
    register: ::volatile_register::RW<u32>,
}

impl Ccr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CcrR, &'w mut CcrW) -> &'w mut CcrW
    {
        let bits = self.register.read();
        let r = CcrR { bits: bits };
        let mut w = CcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CcrR {
        CcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CcrW) -> &mut CcrW
    {
        let mut w = CcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CcrR {
    bits: u32,
}

impl CcrR {
    # [ doc = "Bits 0:4 - Multi ADC mode selection" ]
    pub fn mult(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - Delay between 2 sampling phases" ]
    pub fn delay(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 13 - DMA configuration (for multi-ADC mode)" ]
    pub fn dmacfg(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 14:15 - Direct memory access mode for multi ADC mode" ]
    pub fn mdma(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:17 - ADC clock mode" ]
    pub fn ckmode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 22 - VREFINT enable" ]
    pub fn vrefen(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - Temperature sensor enable" ]
    pub fn tsen(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - VBAT enable" ]
    pub fn vbaten(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CcrW {
    bits: u32,
}

impl CcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CcrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:4 - Multi ADC mode selection" ]
    pub fn mult(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - Delay between 2 sampling phases" ]
    pub fn delay(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 13 - DMA configuration (for multi-ADC mode)" ]
    pub fn dmacfg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 14:15 - Direct memory access mode for multi ADC mode" ]
    pub fn mdma(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:17 - ADC clock mode" ]
    pub fn ckmode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 22 - VREFINT enable" ]
    pub fn vrefen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - Temperature sensor enable" ]
    pub fn tsen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - VBAT enable" ]
    pub fn vbaten(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Cdr {
    register: ::volatile_register::RO<u32>,
}

impl Cdr {
    pub fn read(&self) -> CdrR {
        CdrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CdrR {
    bits: u32,
}

impl CdrR {
    # [ doc = "Bits 16:31 - Regular data of the slave ADC" ]
    pub fn rdata_slv(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:15 - Regular data of the master ADC" ]
    pub fn rdata_mst(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CdrW {
    bits: u32,
}

impl CdrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CdrW { bits: 0u32 }
    }
    # [ doc = "Bits 16:31 - Regular data of the slave ADC" ]
    pub fn rdata_slv(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:15 - Regular data of the master ADC" ]
    pub fn rdata_mst(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
