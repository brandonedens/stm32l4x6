# [ doc = "Analog-to-Digital Converter" ]
# [ repr ( C ) ]
pub struct Adc {
    # [ doc = "0x00 - interrupt and status register" ]
    pub isr: Isr,
    # [ doc = "0x04 - interrupt enable register" ]
    pub ier: Ier,
    # [ doc = "0x08 - control register" ]
    pub cr: Cr,
    # [ doc = "0x0c - configuration register" ]
    pub cfgr: Cfgr,
    # [ doc = "0x10 - configuration register" ]
    pub cfgr2: Cfgr2,
    # [ doc = "0x14 - sample time register 1" ]
    pub smpr1: Smpr1,
    # [ doc = "0x18 - sample time register 2" ]
    pub smpr2: Smpr2,
    _reserved0: [u8; 4usize],
    # [ doc = "0x20 - watchdog threshold register 1" ]
    pub tr1: Tr1,
    # [ doc = "0x24 - watchdog threshold register" ]
    pub tr2: Tr2,
    # [ doc = "0x28 - watchdog threshold register 3" ]
    pub tr3: Tr3,
    _reserved1: [u8; 4usize],
    # [ doc = "0x30 - regular sequence register 1" ]
    pub sqr1: Sqr1,
    # [ doc = "0x34 - regular sequence register 2" ]
    pub sqr2: Sqr2,
    # [ doc = "0x38 - regular sequence register 3" ]
    pub sqr3: Sqr3,
    # [ doc = "0x3c - regular sequence register 4" ]
    pub sqr4: Sqr4,
    # [ doc = "0x40 - regular Data Register" ]
    pub dr: Dr,
    _reserved2: [u8; 8usize],
    # [ doc = "0x4c - injected sequence register" ]
    pub jsqr: Jsqr,
    _reserved3: [u8; 16usize],
    # [ doc = "0x60 - offset register 1" ]
    pub ofr1: Ofr1,
    # [ doc = "0x64 - offset register 2" ]
    pub ofr2: Ofr2,
    # [ doc = "0x68 - offset register 3" ]
    pub ofr3: Ofr3,
    # [ doc = "0x6c - offset register 4" ]
    pub ofr4: Ofr4,
    _reserved4: [u8; 16usize],
    # [ doc = "0x80 - injected data register 1" ]
    pub jdr1: Jdr1,
    # [ doc = "0x84 - injected data register 2" ]
    pub jdr2: Jdr2,
    # [ doc = "0x88 - injected data register 3" ]
    pub jdr3: Jdr3,
    # [ doc = "0x8c - injected data register 4" ]
    pub jdr4: Jdr4,
    _reserved5: [u8; 16usize],
    # [ doc = "0xa0 - Analog Watchdog 2 Configuration Register" ]
    pub awd2cr: Awd2cr,
    # [ doc = "0xa4 - Analog Watchdog 3 Configuration Register" ]
    pub awd3cr: Awd3cr,
    _reserved6: [u8; 8usize],
    # [ doc = "0xb0 - Differential Mode Selection Register 2" ]
    pub difsel: Difsel,
    # [ doc = "0xb4 - Calibration Factors" ]
    pub calfact: Calfact,
}

# [ repr ( C ) ]
pub struct Isr {
    register: ::volatile_register::RW<u32>,
}

impl Isr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&IsrR, &'w mut IsrW) -> &'w mut IsrW
    {
        let bits = self.register.read();
        let r = IsrR { bits: bits };
        let mut w = IsrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IsrR {
        IsrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IsrW) -> &mut IsrW
    {
        let mut w = IsrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IsrR {
    bits: u32,
}

impl IsrR {
    # [ doc = "Bit 10 - JQOVF" ]
    pub fn jqovf(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - AWD3" ]
    pub fn awd3(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - AWD2" ]
    pub fn awd2(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - AWD1" ]
    pub fn awd1(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - JEOS" ]
    pub fn jeos(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - JEOC" ]
    pub fn jeoc(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - OVR" ]
    pub fn ovr(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - EOS" ]
    pub fn eos(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - EOC" ]
    pub fn eoc(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - EOSMP" ]
    pub fn eosmp(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - ADRDY" ]
    pub fn adrdy(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IsrW {
    bits: u32,
}

impl IsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IsrW { bits: 0u32 }
    }
    # [ doc = "Bit 10 - JQOVF" ]
    pub fn jqovf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - AWD3" ]
    pub fn awd3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - AWD2" ]
    pub fn awd2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - AWD1" ]
    pub fn awd1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - JEOS" ]
    pub fn jeos(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - JEOC" ]
    pub fn jeoc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - OVR" ]
    pub fn ovr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - EOS" ]
    pub fn eos(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - EOC" ]
    pub fn eoc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - EOSMP" ]
    pub fn eosmp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - ADRDY" ]
    pub fn adrdy(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Ier {
    register: ::volatile_register::RW<u32>,
}

impl Ier {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&IerR, &'w mut IerW) -> &'w mut IerW
    {
        let bits = self.register.read();
        let r = IerR { bits: bits };
        let mut w = IerW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IerR {
        IerR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IerW) -> &mut IerW
    {
        let mut w = IerW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IerR {
    bits: u32,
}

impl IerR {
    # [ doc = "Bit 10 - JQOVFIE" ]
    pub fn jqovfie(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - AWD3IE" ]
    pub fn awd3ie(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - AWD2IE" ]
    pub fn awd2ie(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - AWD1IE" ]
    pub fn awd1ie(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - JEOSIE" ]
    pub fn jeosie(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - JEOCIE" ]
    pub fn jeocie(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - OVRIE" ]
    pub fn ovrie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - EOSIE" ]
    pub fn eosie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - EOCIE" ]
    pub fn eocie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - EOSMPIE" ]
    pub fn eosmpie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - ADRDYIE" ]
    pub fn adrdyie(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IerW {
    bits: u32,
}

impl IerW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IerW { bits: 0u32 }
    }
    # [ doc = "Bit 10 - JQOVFIE" ]
    pub fn jqovfie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - AWD3IE" ]
    pub fn awd3ie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - AWD2IE" ]
    pub fn awd2ie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - AWD1IE" ]
    pub fn awd1ie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - JEOSIE" ]
    pub fn jeosie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - JEOCIE" ]
    pub fn jeocie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - OVRIE" ]
    pub fn ovrie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - EOSIE" ]
    pub fn eosie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - EOCIE" ]
    pub fn eocie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - EOSMPIE" ]
    pub fn eosmpie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - ADRDYIE" ]
    pub fn adrdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
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
    # [ doc = "Bit 31 - ADCAL" ]
    pub fn adcal(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - ADCALDIF" ]
    pub fn adcaldif(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - DEEPPWD" ]
    pub fn deeppwd(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - ADVREGEN" ]
    pub fn advregen(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - JADSTP" ]
    pub fn jadstp(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - ADSTP" ]
    pub fn adstp(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - JADSTART" ]
    pub fn jadstart(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - ADSTART" ]
    pub fn adstart(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - ADDIS" ]
    pub fn addis(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - ADEN" ]
    pub fn aden(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
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
        CrW { bits: 0u32 }
    }
    # [ doc = "Bit 31 - ADCAL" ]
    pub fn adcal(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - ADCALDIF" ]
    pub fn adcaldif(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - DEEPPWD" ]
    pub fn deeppwd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - ADVREGEN" ]
    pub fn advregen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - JADSTP" ]
    pub fn jadstp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - ADSTP" ]
    pub fn adstp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - JADSTART" ]
    pub fn jadstart(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - ADSTART" ]
    pub fn adstart(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - ADDIS" ]
    pub fn addis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - ADEN" ]
    pub fn aden(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Cfgr {
    register: ::volatile_register::RW<u32>,
}

impl Cfgr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CfgrR, &'w mut CfgrW) -> &'w mut CfgrW
    {
        let bits = self.register.read();
        let r = CfgrR { bits: bits };
        let mut w = CfgrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CfgrR {
        CfgrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CfgrW) -> &mut CfgrW
    {
        let mut w = CfgrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CfgrR {
    bits: u32,
}

impl CfgrR {
    # [ doc = "Bits 26:30 - AWDCH1CH" ]
    pub fn awdch1ch(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 26u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 25 - JAUTO" ]
    pub fn jauto(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - JAWD1EN" ]
    pub fn jawd1en(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - AWD1EN" ]
    pub fn awd1en(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - AWD1SGL" ]
    pub fn awd1sgl(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - JQM" ]
    pub fn jqm(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - JDISCEN" ]
    pub fn jdiscen(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 17:19 - DISCNUM" ]
    pub fn discnum(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 17u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 16 - DISCEN" ]
    pub fn discen(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - AUTOFF" ]
    pub fn autoff(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - AUTDLY" ]
    pub fn autdly(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - CONT" ]
    pub fn cont(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - OVRMOD" ]
    pub fn ovrmod(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 10:11 - EXTEN" ]
    pub fn exten(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 6:9 - EXTSEL" ]
    pub fn extsel(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 5 - ALIGN" ]
    pub fn align(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 3:4 - RES" ]
    pub fn res(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 1 - DMACFG" ]
    pub fn dmacfg(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - DMAEN" ]
    pub fn dmaen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CfgrW {
    bits: u32,
}

impl CfgrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CfgrW { bits: 0u32 }
    }
    # [ doc = "Bits 26:30 - AWDCH1CH" ]
    pub fn awdch1ch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 26u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 25 - JAUTO" ]
    pub fn jauto(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - JAWD1EN" ]
    pub fn jawd1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - AWD1EN" ]
    pub fn awd1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - AWD1SGL" ]
    pub fn awd1sgl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - JQM" ]
    pub fn jqm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - JDISCEN" ]
    pub fn jdiscen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 17:19 - DISCNUM" ]
    pub fn discnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 17u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 16 - DISCEN" ]
    pub fn discen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - AUTOFF" ]
    pub fn autoff(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - AUTDLY" ]
    pub fn autdly(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - CONT" ]
    pub fn cont(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - OVRMOD" ]
    pub fn ovrmod(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 10:11 - EXTEN" ]
    pub fn exten(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 6:9 - EXTSEL" ]
    pub fn extsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 5 - ALIGN" ]
    pub fn align(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 3:4 - RES" ]
    pub fn res(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 1 - DMACFG" ]
    pub fn dmacfg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - DMAEN" ]
    pub fn dmaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Cfgr2 {
    register: ::volatile_register::RW<u32>,
}

impl Cfgr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cfgr2R, &'w mut Cfgr2W) -> &'w mut Cfgr2W
    {
        let bits = self.register.read();
        let r = Cfgr2R { bits: bits };
        let mut w = Cfgr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cfgr2R {
        Cfgr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cfgr2W) -> &mut Cfgr2W
    {
        let mut w = Cfgr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cfgr2R {
    bits: u32,
}

impl Cfgr2R {
    # [ doc = "Bit 10 - EXTEN" ]
    pub fn rovsm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - EXTSEL" ]
    pub fn tovs(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 5:8 - ALIGN" ]
    pub fn ovss(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 5u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 2:4 - RES" ]
    pub fn ovsr(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 1 - DMACFG" ]
    pub fn jovse(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - DMAEN" ]
    pub fn rovse(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cfgr2W {
    bits: u32,
}

impl Cfgr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cfgr2W { bits: 0u32 }
    }
    # [ doc = "Bit 10 - EXTEN" ]
    pub fn rovsm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - EXTSEL" ]
    pub fn tovs(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 5:8 - ALIGN" ]
    pub fn ovss(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 5u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 2:4 - RES" ]
    pub fn ovsr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 1 - DMACFG" ]
    pub fn jovse(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - DMAEN" ]
    pub fn rovse(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Smpr1 {
    register: ::volatile_register::RW<u32>,
}

impl Smpr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Smpr1R, &'w mut Smpr1W) -> &'w mut Smpr1W
    {
        let bits = self.register.read();
        let r = Smpr1R { bits: bits };
        let mut w = Smpr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Smpr1R {
        Smpr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Smpr1W) -> &mut Smpr1W
    {
        let mut w = Smpr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Smpr1R {
    bits: u32,
}

impl Smpr1R {
    # [ doc = "Bits 27:29 - SMP9" ]
    pub fn smp9(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 27u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:26 - SMP8" ]
    pub fn smp8(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 21:23 - SMP7" ]
    pub fn smp7(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 21u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 18:20 - SMP6" ]
    pub fn smp6(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 15:17 - SMP5" ]
    pub fn smp5(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 15u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:14 - SMP4" ]
    pub fn smp4(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 9:11 - SMP3" ]
    pub fn smp3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 9u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 6:8 - SMP2" ]
    pub fn smp2(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 3:5 - SMP1" ]
    pub fn smp1(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Smpr1W {
    bits: u32,
}

impl Smpr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Smpr1W { bits: 0u32 }
    }
    # [ doc = "Bits 27:29 - SMP9" ]
    pub fn smp9(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 27u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:26 - SMP8" ]
    pub fn smp8(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 21:23 - SMP7" ]
    pub fn smp7(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 21u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 18:20 - SMP6" ]
    pub fn smp6(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 15:17 - SMP5" ]
    pub fn smp5(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 15u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:14 - SMP4" ]
    pub fn smp4(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 9:11 - SMP3" ]
    pub fn smp3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 9u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 6:8 - SMP2" ]
    pub fn smp2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 3:5 - SMP1" ]
    pub fn smp1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Smpr2 {
    register: ::volatile_register::RW<u32>,
}

impl Smpr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Smpr2R, &'w mut Smpr2W) -> &'w mut Smpr2W
    {
        let bits = self.register.read();
        let r = Smpr2R { bits: bits };
        let mut w = Smpr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Smpr2R {
        Smpr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Smpr2W) -> &mut Smpr2W
    {
        let mut w = Smpr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Smpr2R {
    bits: u32,
}

impl Smpr2R {
    # [ doc = "Bits 24:26 - SMP18" ]
    pub fn smp18(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 21:23 - SMP17" ]
    pub fn smp17(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 21u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 18:20 - SMP16" ]
    pub fn smp16(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 15:17 - SMP15" ]
    pub fn smp15(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 15u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:14 - SMP14" ]
    pub fn smp14(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 9:11 - SMP13" ]
    pub fn smp13(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 9u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 6:8 - SMP12" ]
    pub fn smp12(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 3:5 - SMP11" ]
    pub fn smp11(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:2 - SMP10" ]
    pub fn smp10(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Smpr2W {
    bits: u32,
}

impl Smpr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Smpr2W { bits: 0u32 }
    }
    # [ doc = "Bits 24:26 - SMP18" ]
    pub fn smp18(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 21:23 - SMP17" ]
    pub fn smp17(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 21u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 18:20 - SMP16" ]
    pub fn smp16(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 15:17 - SMP15" ]
    pub fn smp15(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 15u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:14 - SMP14" ]
    pub fn smp14(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 9:11 - SMP13" ]
    pub fn smp13(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 9u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 6:8 - SMP12" ]
    pub fn smp12(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 3:5 - SMP11" ]
    pub fn smp11(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:2 - SMP10" ]
    pub fn smp10(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Tr1 {
    register: ::volatile_register::RW<u32>,
}

impl Tr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Tr1R, &'w mut Tr1W) -> &'w mut Tr1W
    {
        let bits = self.register.read();
        let r = Tr1R { bits: bits };
        let mut w = Tr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Tr1R {
        Tr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Tr1W) -> &mut Tr1W
    {
        let mut w = Tr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Tr1R {
    bits: u32,
}

impl Tr1R {
    # [ doc = "Bits 16:27 - HT1" ]
    pub fn ht1(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:11 - LT1" ]
    pub fn lt1(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Tr1W {
    bits: u32,
}

impl Tr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Tr1W { bits: 268369920u32 }
    }
    # [ doc = "Bits 16:27 - HT1" ]
    pub fn ht1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:11 - LT1" ]
    pub fn lt1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Tr2 {
    register: ::volatile_register::RW<u32>,
}

impl Tr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Tr2R, &'w mut Tr2W) -> &'w mut Tr2W
    {
        let bits = self.register.read();
        let r = Tr2R { bits: bits };
        let mut w = Tr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Tr2R {
        Tr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Tr2W) -> &mut Tr2W
    {
        let mut w = Tr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Tr2R {
    bits: u32,
}

impl Tr2R {
    # [ doc = "Bits 16:23 - HT2" ]
    pub fn ht2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - LT2" ]
    pub fn lt2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Tr2W {
    bits: u32,
}

impl Tr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Tr2W { bits: 268369920u32 }
    }
    # [ doc = "Bits 16:23 - HT2" ]
    pub fn ht2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - LT2" ]
    pub fn lt2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Tr3 {
    register: ::volatile_register::RW<u32>,
}

impl Tr3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Tr3R, &'w mut Tr3W) -> &'w mut Tr3W
    {
        let bits = self.register.read();
        let r = Tr3R { bits: bits };
        let mut w = Tr3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Tr3R {
        Tr3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Tr3W) -> &mut Tr3W
    {
        let mut w = Tr3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Tr3R {
    bits: u32,
}

impl Tr3R {
    # [ doc = "Bits 16:23 - HT3" ]
    pub fn ht3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - LT3" ]
    pub fn lt3(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Tr3W {
    bits: u32,
}

impl Tr3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Tr3W { bits: 268369920u32 }
    }
    # [ doc = "Bits 16:23 - HT3" ]
    pub fn ht3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - LT3" ]
    pub fn lt3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Sqr1 {
    register: ::volatile_register::RW<u32>,
}

impl Sqr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Sqr1R, &'w mut Sqr1W) -> &'w mut Sqr1W
    {
        let bits = self.register.read();
        let r = Sqr1R { bits: bits };
        let mut w = Sqr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Sqr1R {
        Sqr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Sqr1W) -> &mut Sqr1W
    {
        let mut w = Sqr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sqr1R {
    bits: u32,
}

impl Sqr1R {
    # [ doc = "Bits 24:28 - SQ4" ]
    pub fn sq4(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 18:22 - SQ3" ]
    pub fn sq3(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:16 - SQ2" ]
    pub fn sq2(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 6:10 - SQ1" ]
    pub fn sq1(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - L3" ]
    pub fn l3(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sqr1W {
    bits: u32,
}

impl Sqr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Sqr1W { bits: 0u32 }
    }
    # [ doc = "Bits 24:28 - SQ4" ]
    pub fn sq4(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 18:22 - SQ3" ]
    pub fn sq3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:16 - SQ2" ]
    pub fn sq2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 6:10 - SQ1" ]
    pub fn sq1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - L3" ]
    pub fn l3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Sqr2 {
    register: ::volatile_register::RW<u32>,
}

impl Sqr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Sqr2R, &'w mut Sqr2W) -> &'w mut Sqr2W
    {
        let bits = self.register.read();
        let r = Sqr2R { bits: bits };
        let mut w = Sqr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Sqr2R {
        Sqr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Sqr2W) -> &mut Sqr2W
    {
        let mut w = Sqr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sqr2R {
    bits: u32,
}

impl Sqr2R {
    # [ doc = "Bits 24:28 - SQ9" ]
    pub fn sq9(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 18:22 - SQ8" ]
    pub fn sq8(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:16 - SQ7" ]
    pub fn sq7(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 6:10 - SQ6" ]
    pub fn sq6(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:4 - SQ5" ]
    pub fn sq5(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sqr2W {
    bits: u32,
}

impl Sqr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Sqr2W { bits: 0u32 }
    }
    # [ doc = "Bits 24:28 - SQ9" ]
    pub fn sq9(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 18:22 - SQ8" ]
    pub fn sq8(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:16 - SQ7" ]
    pub fn sq7(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 6:10 - SQ6" ]
    pub fn sq6(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:4 - SQ5" ]
    pub fn sq5(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Sqr3 {
    register: ::volatile_register::RW<u32>,
}

impl Sqr3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Sqr3R, &'w mut Sqr3W) -> &'w mut Sqr3W
    {
        let bits = self.register.read();
        let r = Sqr3R { bits: bits };
        let mut w = Sqr3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Sqr3R {
        Sqr3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Sqr3W) -> &mut Sqr3W
    {
        let mut w = Sqr3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sqr3R {
    bits: u32,
}

impl Sqr3R {
    # [ doc = "Bits 24:28 - SQ14" ]
    pub fn sq14(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 18:22 - SQ13" ]
    pub fn sq13(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:16 - SQ12" ]
    pub fn sq12(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 6:10 - SQ11" ]
    pub fn sq11(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:4 - SQ10" ]
    pub fn sq10(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sqr3W {
    bits: u32,
}

impl Sqr3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Sqr3W { bits: 0u32 }
    }
    # [ doc = "Bits 24:28 - SQ14" ]
    pub fn sq14(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 18:22 - SQ13" ]
    pub fn sq13(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:16 - SQ12" ]
    pub fn sq12(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 6:10 - SQ11" ]
    pub fn sq11(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:4 - SQ10" ]
    pub fn sq10(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Sqr4 {
    register: ::volatile_register::RW<u32>,
}

impl Sqr4 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Sqr4R, &'w mut Sqr4W) -> &'w mut Sqr4W
    {
        let bits = self.register.read();
        let r = Sqr4R { bits: bits };
        let mut w = Sqr4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Sqr4R {
        Sqr4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Sqr4W) -> &mut Sqr4W
    {
        let mut w = Sqr4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sqr4R {
    bits: u32,
}

impl Sqr4R {
    # [ doc = "Bits 6:10 - SQ16" ]
    pub fn sq16(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:4 - SQ15" ]
    pub fn sq15(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sqr4W {
    bits: u32,
}

impl Sqr4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Sqr4W { bits: 0u32 }
    }
    # [ doc = "Bits 6:10 - SQ16" ]
    pub fn sq16(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:4 - SQ15" ]
    pub fn sq15(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dr {
    register: ::volatile_register::RO<u32>,
}

impl Dr {
    pub fn read(&self) -> DrR {
        DrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DrR {
    bits: u32,
}

impl DrR {
    # [ doc = "Bits 0:15 - regularDATA" ]
    pub fn regular_data(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DrW {
    bits: u32,
}

impl DrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - regularDATA" ]
    pub fn regular_data(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Jsqr {
    register: ::volatile_register::RW<u32>,
}

impl Jsqr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&JsqrR, &'w mut JsqrW) -> &'w mut JsqrW
    {
        let bits = self.register.read();
        let r = JsqrR { bits: bits };
        let mut w = JsqrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> JsqrR {
        JsqrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut JsqrW) -> &mut JsqrW
    {
        let mut w = JsqrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct JsqrR {
    bits: u32,
}

impl JsqrR {
    # [ doc = "Bits 26:30 - JSQ4" ]
    pub fn jsq4(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 26u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:24 - JSQ3" ]
    pub fn jsq3(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 14:18 - JSQ2" ]
    pub fn jsq2(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:12 - JSQ1" ]
    pub fn jsq1(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 6:7 - JEXTEN" ]
    pub fn jexten(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 2:5 - JEXTSEL" ]
    pub fn jextsel(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:1 - JL" ]
    pub fn jl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct JsqrW {
    bits: u32,
}

impl JsqrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        JsqrW { bits: 0u32 }
    }
    # [ doc = "Bits 26:30 - JSQ4" ]
    pub fn jsq4(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 26u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:24 - JSQ3" ]
    pub fn jsq3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 14:18 - JSQ2" ]
    pub fn jsq2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:12 - JSQ1" ]
    pub fn jsq1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 6:7 - JEXTEN" ]
    pub fn jexten(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 2:5 - JEXTSEL" ]
    pub fn jextsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:1 - JL" ]
    pub fn jl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ofr1 {
    register: ::volatile_register::RW<u32>,
}

impl Ofr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ofr1R, &'w mut Ofr1W) -> &'w mut Ofr1W
    {
        let bits = self.register.read();
        let r = Ofr1R { bits: bits };
        let mut w = Ofr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ofr1R {
        Ofr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ofr1W) -> &mut Ofr1W
    {
        let mut w = Ofr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ofr1R {
    bits: u32,
}

impl Ofr1R {
    # [ doc = "Bit 31 - OFFSET1_EN" ]
    pub fn offset1_en(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 26:30 - OFFSET1_CH" ]
    pub fn offset1_ch(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 26u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:11 - OFFSET1" ]
    pub fn offset1(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ofr1W {
    bits: u32,
}

impl Ofr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ofr1W { bits: 0u32 }
    }
    # [ doc = "Bit 31 - OFFSET1_EN" ]
    pub fn offset1_en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 26:30 - OFFSET1_CH" ]
    pub fn offset1_ch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 26u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:11 - OFFSET1" ]
    pub fn offset1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ofr2 {
    register: ::volatile_register::RW<u32>,
}

impl Ofr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ofr2R, &'w mut Ofr2W) -> &'w mut Ofr2W
    {
        let bits = self.register.read();
        let r = Ofr2R { bits: bits };
        let mut w = Ofr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ofr2R {
        Ofr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ofr2W) -> &mut Ofr2W
    {
        let mut w = Ofr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ofr2R {
    bits: u32,
}

impl Ofr2R {
    # [ doc = "Bit 31 - OFFSET2_EN" ]
    pub fn offset2_en(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 26:30 - OFFSET2_CH" ]
    pub fn offset2_ch(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 26u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:11 - OFFSET2" ]
    pub fn offset2(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ofr2W {
    bits: u32,
}

impl Ofr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ofr2W { bits: 0u32 }
    }
    # [ doc = "Bit 31 - OFFSET2_EN" ]
    pub fn offset2_en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 26:30 - OFFSET2_CH" ]
    pub fn offset2_ch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 26u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:11 - OFFSET2" ]
    pub fn offset2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ofr3 {
    register: ::volatile_register::RW<u32>,
}

impl Ofr3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ofr3R, &'w mut Ofr3W) -> &'w mut Ofr3W
    {
        let bits = self.register.read();
        let r = Ofr3R { bits: bits };
        let mut w = Ofr3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ofr3R {
        Ofr3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ofr3W) -> &mut Ofr3W
    {
        let mut w = Ofr3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ofr3R {
    bits: u32,
}

impl Ofr3R {
    # [ doc = "Bit 31 - OFFSET3_EN" ]
    pub fn offset3_en(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 26:30 - OFFSET3_CH" ]
    pub fn offset3_ch(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 26u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:11 - OFFSET3" ]
    pub fn offset3(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ofr3W {
    bits: u32,
}

impl Ofr3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ofr3W { bits: 0u32 }
    }
    # [ doc = "Bit 31 - OFFSET3_EN" ]
    pub fn offset3_en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 26:30 - OFFSET3_CH" ]
    pub fn offset3_ch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 26u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:11 - OFFSET3" ]
    pub fn offset3(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ofr4 {
    register: ::volatile_register::RW<u32>,
}

impl Ofr4 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ofr4R, &'w mut Ofr4W) -> &'w mut Ofr4W
    {
        let bits = self.register.read();
        let r = Ofr4R { bits: bits };
        let mut w = Ofr4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ofr4R {
        Ofr4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ofr4W) -> &mut Ofr4W
    {
        let mut w = Ofr4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ofr4R {
    bits: u32,
}

impl Ofr4R {
    # [ doc = "Bit 31 - OFFSET4_EN" ]
    pub fn offset4_en(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 26:30 - OFFSET4_CH" ]
    pub fn offset4_ch(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 26u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:11 - OFFSET4" ]
    pub fn offset4(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ofr4W {
    bits: u32,
}

impl Ofr4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ofr4W { bits: 0u32 }
    }
    # [ doc = "Bit 31 - OFFSET4_EN" ]
    pub fn offset4_en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 26:30 - OFFSET4_CH" ]
    pub fn offset4_ch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 26u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:11 - OFFSET4" ]
    pub fn offset4(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Jdr1 {
    register: ::volatile_register::RO<u32>,
}

impl Jdr1 {
    pub fn read(&self) -> Jdr1R {
        Jdr1R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jdr1R {
    bits: u32,
}

impl Jdr1R {
    # [ doc = "Bits 0:15 - JDATA1" ]
    pub fn jdata1(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jdr1W {
    bits: u32,
}

impl Jdr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Jdr1W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - JDATA1" ]
    pub fn jdata1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Jdr2 {
    register: ::volatile_register::RO<u32>,
}

impl Jdr2 {
    pub fn read(&self) -> Jdr2R {
        Jdr2R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jdr2R {
    bits: u32,
}

impl Jdr2R {
    # [ doc = "Bits 0:15 - JDATA2" ]
    pub fn jdata2(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jdr2W {
    bits: u32,
}

impl Jdr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Jdr2W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - JDATA2" ]
    pub fn jdata2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Jdr3 {
    register: ::volatile_register::RO<u32>,
}

impl Jdr3 {
    pub fn read(&self) -> Jdr3R {
        Jdr3R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jdr3R {
    bits: u32,
}

impl Jdr3R {
    # [ doc = "Bits 0:15 - JDATA3" ]
    pub fn jdata3(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jdr3W {
    bits: u32,
}

impl Jdr3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Jdr3W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - JDATA3" ]
    pub fn jdata3(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Jdr4 {
    register: ::volatile_register::RO<u32>,
}

impl Jdr4 {
    pub fn read(&self) -> Jdr4R {
        Jdr4R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jdr4R {
    bits: u32,
}

impl Jdr4R {
    # [ doc = "Bits 0:15 - JDATA4" ]
    pub fn jdata4(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Jdr4W {
    bits: u32,
}

impl Jdr4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Jdr4W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - JDATA4" ]
    pub fn jdata4(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Awd2cr {
    register: ::volatile_register::RW<u32>,
}

impl Awd2cr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Awd2crR, &'w mut Awd2crW) -> &'w mut Awd2crW
    {
        let bits = self.register.read();
        let r = Awd2crR { bits: bits };
        let mut w = Awd2crW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Awd2crR {
        Awd2crR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Awd2crW) -> &mut Awd2crW
    {
        let mut w = Awd2crW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awd2crR {
    bits: u32,
}

impl Awd2crR {
    # [ doc = "Bits 1:18 - AWD2CH" ]
    pub fn awd2ch(&self) -> u32 {
        const MASK: u32 = 262143;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awd2crW {
    bits: u32,
}

impl Awd2crW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Awd2crW { bits: 0u32 }
    }
    # [ doc = "Bits 1:18 - AWD2CH" ]
    pub fn awd2ch(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u32 = 262143;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Awd3cr {
    register: ::volatile_register::RW<u32>,
}

impl Awd3cr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Awd3crR, &'w mut Awd3crW) -> &'w mut Awd3crW
    {
        let bits = self.register.read();
        let r = Awd3crR { bits: bits };
        let mut w = Awd3crW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Awd3crR {
        Awd3crR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Awd3crW) -> &mut Awd3crW
    {
        let mut w = Awd3crW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awd3crR {
    bits: u32,
}

impl Awd3crR {
    # [ doc = "Bits 1:18 - AWD3CH" ]
    pub fn awd3ch(&self) -> u32 {
        const MASK: u32 = 262143;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awd3crW {
    bits: u32,
}

impl Awd3crW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Awd3crW { bits: 0u32 }
    }
    # [ doc = "Bits 1:18 - AWD3CH" ]
    pub fn awd3ch(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u32 = 262143;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Difsel {
    register: ::volatile_register::RW<u32>,
}

impl Difsel {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DifselR, &'w mut DifselW) -> &'w mut DifselW
    {
        let bits = self.register.read();
        let r = DifselR { bits: bits };
        let mut w = DifselW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DifselR {
        DifselR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DifselW) -> &mut DifselW
    {
        let mut w = DifselW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DifselR {
    bits: u32,
}

impl DifselR {
    # [ doc = "Bits 1:15 - Differential mode for channels 15 to 1" ]
    pub fn difsel_1_15(&self) -> u16 {
        const MASK: u32 = 32767;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:18 - Differential mode for channels 18 to 16" ]
    pub fn difsel_16_18(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DifselW {
    bits: u32,
}

impl DifselW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DifselW { bits: 0u32 }
    }
    # [ doc = "Bits 1:15 - Differential mode for channels 15 to 1" ]
    pub fn difsel_1_15(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u16 = 32767;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Calfact {
    register: ::volatile_register::RW<u32>,
}

impl Calfact {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CalfactR, &'w mut CalfactW) -> &'w mut CalfactW
    {
        let bits = self.register.read();
        let r = CalfactR { bits: bits };
        let mut w = CalfactW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CalfactR {
        CalfactR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CalfactW) -> &mut CalfactW
    {
        let mut w = CalfactW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CalfactR {
    bits: u32,
}

impl CalfactR {
    # [ doc = "Bits 16:22 - CALFACT_D" ]
    pub fn calfact_d(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:6 - CALFACT_S" ]
    pub fn calfact_s(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CalfactW {
    bits: u32,
}

impl CalfactW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CalfactW { bits: 0u32 }
    }
    # [ doc = "Bits 16:22 - CALFACT_D" ]
    pub fn calfact_d(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:6 - CALFACT_S" ]
    pub fn calfact_s(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
