# [ doc = "Power control" ]
# [ repr ( C ) ]
pub struct Pwr {
    # [ doc = "0x00 - Power control register 1" ]
    pub cr1: Cr1,
    # [ doc = "0x04 - Power control register 2" ]
    pub cr2: Cr2,
    # [ doc = "0x08 - Power control register 3" ]
    pub cr3: Cr3,
    # [ doc = "0x0c - Power control register 4" ]
    pub cr4: Cr4,
    # [ doc = "0x10 - Power status register 1" ]
    pub sr1: Sr1,
    # [ doc = "0x14 - Power status register 2" ]
    pub sr2: Sr2,
    # [ doc = "0x18 - Power status clear register" ]
    pub scr: Scr,
    _reserved0: [u8; 4usize],
    # [ doc = "0x20 - Power Port A pull-up control register" ]
    pub pucra: Pucra,
    # [ doc = "0x24 - Power Port A pull-down control register" ]
    pub pdcra: Pdcra,
    # [ doc = "0x28 - Power Port B pull-up control register" ]
    pub pucrb: Pucrb,
    # [ doc = "0x2c - Power Port B pull-down control register" ]
    pub pdcrb: Pdcrb,
    # [ doc = "0x30 - Power Port C pull-up control register" ]
    pub pucrc: Pucrc,
    # [ doc = "0x34 - Power Port C pull-down control register" ]
    pub pdcrc: Pdcrc,
    # [ doc = "0x38 - Power Port D pull-up control register" ]
    pub pucrd: Pucrd,
    # [ doc = "0x3c - Power Port D pull-down control register" ]
    pub pdcrd: Pdcrd,
    # [ doc = "0x40 - Power Port E pull-up control register" ]
    pub pucre: Pucre,
    # [ doc = "0x44 - Power Port E pull-down control register" ]
    pub pdcre: Pdcre,
    # [ doc = "0x48 - Power Port F pull-up control register" ]
    pub pucrf: Pucrf,
    # [ doc = "0x4c - Power Port F pull-down control register" ]
    pub pdcrf: Pdcrf,
    # [ doc = "0x50 - Power Port G pull-up control register" ]
    pub pucrg: Pucrg,
    # [ doc = "0x54 - Power Port G pull-down control register" ]
    pub pdcrg: Pdcrg,
    # [ doc = "0x58 - Power Port H pull-up control register" ]
    pub pucrh: Pucrh,
    # [ doc = "0x5c - Power Port H pull-down control register" ]
    pub pdcrh: Pdcrh,
}

# [ repr ( C ) ]
pub struct Cr1 {
    register: ::volatile_register::RW<u32>,
}

impl Cr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cr1R, &'w mut Cr1W) -> &'w mut Cr1W
    {
        let bits = self.register.read();
        let r = Cr1R { bits: bits };
        let mut w = Cr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cr1R {
        Cr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cr1W) -> &mut Cr1W
    {
        let mut w = Cr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cr1R {
    bits: u32,
}

impl Cr1R {
    # [ doc = "Bit 14 - Low-power run" ]
    pub fn lpr(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 9:10 - Voltage scaling range selection" ]
    pub fn vos(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 9u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 8 - Disable backup domain write protection" ]
    pub fn dbp(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:2 - Low-power mode selection" ]
    pub fn lpms(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cr1W {
    bits: u32,
}

impl Cr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cr1W { bits: 512u32 }
    }
    # [ doc = "Bit 14 - Low-power run" ]
    pub fn lpr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 9:10 - Voltage scaling range selection" ]
    pub fn vos(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 9u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 8 - Disable backup domain write protection" ]
    pub fn dbp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:2 - Low-power mode selection" ]
    pub fn lpms(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cr2 {
    register: ::volatile_register::RW<u32>,
}

impl Cr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cr2R, &'w mut Cr2W) -> &'w mut Cr2W
    {
        let bits = self.register.read();
        let r = Cr2R { bits: bits };
        let mut w = Cr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cr2R {
        Cr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cr2W) -> &mut Cr2W
    {
        let mut w = Cr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cr2R {
    bits: u32,
}

impl Cr2R {
    # [ doc = "Bit 10 - VDDUSB USB supply valid" ]
    pub fn usv(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - VDDIO2 Independent I/Os supply valid" ]
    pub fn iosv(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V" ]
    pub fn pvme4(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V" ]
    pub fn pvme3(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V" ]
    pub fn pvme2(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V" ]
    pub fn pvme1(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:3 - Power voltage detector level selection" ]
    pub fn pls(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 0 - Power voltage detector enable" ]
    pub fn pvde(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cr2W {
    bits: u32,
}

impl Cr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cr2W { bits: 0u32 }
    }
    # [ doc = "Bit 10 - VDDUSB USB supply valid" ]
    pub fn usv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - VDDIO2 Independent I/Os supply valid" ]
    pub fn iosv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Peripheral voltage monitoring 4 enable: VDDA vs. 2.2V" ]
    pub fn pvme4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Peripheral voltage monitoring 3 enable: VDDA vs. 1.62V" ]
    pub fn pvme3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Peripheral voltage monitoring 2 enable: VDDIO2 vs. 0.9V" ]
    pub fn pvme2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Peripheral voltage monitoring 1 enable: VDDUSB vs. 1.2V" ]
    pub fn pvme1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:3 - Power voltage detector level selection" ]
    pub fn pls(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 0 - Power voltage detector enable" ]
    pub fn pvde(&mut self, value: bool) -> &mut Self {
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
pub struct Cr3 {
    register: ::volatile_register::RW<u32>,
}

impl Cr3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cr3R, &'w mut Cr3W) -> &'w mut Cr3W
    {
        let bits = self.register.read();
        let r = Cr3R { bits: bits };
        let mut w = Cr3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cr3R {
        Cr3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cr3W) -> &mut Cr3W
    {
        let mut w = Cr3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cr3R {
    bits: u32,
}

impl Cr3R {
    # [ doc = "Bit 15 - Enable internal wakeup line" ]
    pub fn ewf(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Apply pull-up and pull-down configuration" ]
    pub fn apc(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - SRAM2 retention in Standby mode" ]
    pub fn rrs(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Enable Wakeup pin WKUP5" ]
    pub fn ewup5(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Enable Wakeup pin WKUP4" ]
    pub fn ewup4(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Enable Wakeup pin WKUP3" ]
    pub fn ewup3(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Enable Wakeup pin WKUP2" ]
    pub fn ewup2(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Enable Wakeup pin WKUP1" ]
    pub fn ewup1(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cr3W {
    bits: u32,
}

impl Cr3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cr3W { bits: 32768u32 }
    }
    # [ doc = "Bit 15 - Enable internal wakeup line" ]
    pub fn ewf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Apply pull-up and pull-down configuration" ]
    pub fn apc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - SRAM2 retention in Standby mode" ]
    pub fn rrs(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Enable Wakeup pin WKUP5" ]
    pub fn ewup5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Enable Wakeup pin WKUP4" ]
    pub fn ewup4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Enable Wakeup pin WKUP3" ]
    pub fn ewup3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Enable Wakeup pin WKUP2" ]
    pub fn ewup2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Enable Wakeup pin WKUP1" ]
    pub fn ewup1(&mut self, value: bool) -> &mut Self {
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
pub struct Cr4 {
    register: ::volatile_register::RW<u32>,
}

impl Cr4 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cr4R, &'w mut Cr4W) -> &'w mut Cr4W
    {
        let bits = self.register.read();
        let r = Cr4R { bits: bits };
        let mut w = Cr4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cr4R {
        Cr4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cr4W) -> &mut Cr4W
    {
        let mut w = Cr4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cr4R {
    bits: u32,
}

impl Cr4R {
    # [ doc = "Bit 9 - VBAT battery charging resistor selection" ]
    pub fn vbrs(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - VBAT battery charging enable" ]
    pub fn vbe(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Wakeup pin WKUP5 polarity" ]
    pub fn wp5(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Wakeup pin WKUP4 polarity" ]
    pub fn wp4(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wakeup pin WKUP3 polarity" ]
    pub fn wp3(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Wakeup pin WKUP2 polarity" ]
    pub fn wp2(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Wakeup pin WKUP1 polarity" ]
    pub fn wp1(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cr4W {
    bits: u32,
}

impl Cr4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cr4W { bits: 0u32 }
    }
    # [ doc = "Bit 9 - VBAT battery charging resistor selection" ]
    pub fn vbrs(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - VBAT battery charging enable" ]
    pub fn vbe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Wakeup pin WKUP5 polarity" ]
    pub fn wp5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Wakeup pin WKUP4 polarity" ]
    pub fn wp4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wakeup pin WKUP3 polarity" ]
    pub fn wp3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Wakeup pin WKUP2 polarity" ]
    pub fn wp2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Wakeup pin WKUP1 polarity" ]
    pub fn wp1(&mut self, value: bool) -> &mut Self {
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
pub struct Sr1 {
    register: ::volatile_register::RO<u32>,
}

impl Sr1 {
    pub fn read(&self) -> Sr1R {
        Sr1R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sr1R {
    bits: u32,
}

impl Sr1R {
    # [ doc = "Bit 15 - Wakeup flag internal" ]
    pub fn wufi(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Standby flag" ]
    pub fn csbf(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Wakeup flag 5" ]
    pub fn cwuf5(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Wakeup flag 4" ]
    pub fn cwuf4(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wakeup flag 3" ]
    pub fn cwuf3(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Wakeup flag 2" ]
    pub fn cwuf2(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Wakeup flag 1" ]
    pub fn cwuf1(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sr1W {
    bits: u32,
}

impl Sr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Sr1W { bits: 0u32 }
    }
    # [ doc = "Bit 15 - Wakeup flag internal" ]
    pub fn wufi(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Standby flag" ]
    pub fn csbf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Wakeup flag 5" ]
    pub fn cwuf5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Wakeup flag 4" ]
    pub fn cwuf4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wakeup flag 3" ]
    pub fn cwuf3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Wakeup flag 2" ]
    pub fn cwuf2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Wakeup flag 1" ]
    pub fn cwuf1(&mut self, value: bool) -> &mut Self {
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
pub struct Sr2 {
    register: ::volatile_register::RO<u32>,
}

impl Sr2 {
    pub fn read(&self) -> Sr2R {
        Sr2R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sr2R {
    bits: u32,
}

impl Sr2R {
    # [ doc = "Bit 15 - Peripheral voltage monitoring output: VDDA vs. 2.2 V" ]
    pub fn pvmo4(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Peripheral voltage monitoring output: VDDA vs. 1.62 V" ]
    pub fn pvmo3(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Peripheral voltage monitoring output: VDDIO2 vs. 0.9 V" ]
    pub fn pvmo2(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Peripheral voltage monitoring output: VDDUSB vs. 1.2 V" ]
    pub fn pvmo1(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Power voltage detector output" ]
    pub fn pvdo(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Voltage scaling flag" ]
    pub fn vosf(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Low-power regulator flag" ]
    pub fn reglpf(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Low-power regulator started" ]
    pub fn reglps(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Sr2W {
    bits: u32,
}

impl Sr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Sr2W { bits: 0u32 }
    }
    # [ doc = "Bit 15 - Peripheral voltage monitoring output: VDDA vs. 2.2 V" ]
    pub fn pvmo4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Peripheral voltage monitoring output: VDDA vs. 1.62 V" ]
    pub fn pvmo3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Peripheral voltage monitoring output: VDDIO2 vs. 0.9 V" ]
    pub fn pvmo2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Peripheral voltage monitoring output: VDDUSB vs. 1.2 V" ]
    pub fn pvmo1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Power voltage detector output" ]
    pub fn pvdo(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Voltage scaling flag" ]
    pub fn vosf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Low-power regulator flag" ]
    pub fn reglpf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Low-power regulator started" ]
    pub fn reglps(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Scr {
    register: ::volatile_register::WO<u32>,
}

impl Scr {
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut ScrW) -> &mut ScrW
    {
        let mut w = ScrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ScrR {
    bits: u32,
}

impl ScrR {
    # [ doc = "Bit 8 - Clear standby flag" ]
    pub fn sbf(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Clear wakeup flag 5" ]
    pub fn wuf5(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Clear wakeup flag 4" ]
    pub fn wuf4(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Clear wakeup flag 3" ]
    pub fn wuf3(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Clear wakeup flag 2" ]
    pub fn wuf2(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Clear wakeup flag 1" ]
    pub fn wuf1(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ScrW {
    bits: u32,
}

impl ScrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ScrW { bits: 0u32 }
    }
    # [ doc = "Bit 8 - Clear standby flag" ]
    pub fn sbf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Clear wakeup flag 5" ]
    pub fn wuf5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Clear wakeup flag 4" ]
    pub fn wuf4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Clear wakeup flag 3" ]
    pub fn wuf3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Clear wakeup flag 2" ]
    pub fn wuf2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Clear wakeup flag 1" ]
    pub fn wuf1(&mut self, value: bool) -> &mut Self {
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
pub struct Pucra {
    register: ::volatile_register::RW<u32>,
}

impl Pucra {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PucraR, &'w mut PucraW) -> &'w mut PucraW
    {
        let bits = self.register.read();
        let r = PucraR { bits: bits };
        let mut w = PucraW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PucraR {
        PucraR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PucraW) -> &mut PucraW
    {
        let mut w = PucraW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PucraR {
    bits: u32,
}

impl PucraR {
    # [ doc = "Bit 15 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PucraW {
    bits: u32,
}

impl PucraW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PucraW { bits: 0u32 }
    }
    # [ doc = "Bit 15 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Port A pull-up bit y (y=0..15)" ]
    pub fn pu0(&mut self, value: bool) -> &mut Self {
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
pub struct Pdcra {
    register: ::volatile_register::RW<u32>,
}

impl Pdcra {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PdcraR, &'w mut PdcraW) -> &'w mut PdcraW
    {
        let bits = self.register.read();
        let r = PdcraR { bits: bits };
        let mut w = PdcraW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PdcraR {
        PdcraR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PdcraW) -> &mut PdcraW
    {
        let mut w = PdcraW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdcraR {
    bits: u32,
}

impl PdcraR {
    # [ doc = "Bit 15 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdcraW {
    bits: u32,
}

impl PdcraW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PdcraW { bits: 0u32 }
    }
    # [ doc = "Bit 15 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Port A pull-down bit y (y=0..15)" ]
    pub fn pd0(&mut self, value: bool) -> &mut Self {
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
pub struct Pucrb {
    register: ::volatile_register::RW<u32>,
}

impl Pucrb {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PucrbR, &'w mut PucrbW) -> &'w mut PucrbW
    {
        let bits = self.register.read();
        let r = PucrbR { bits: bits };
        let mut w = PucrbW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PucrbR {
        PucrbR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PucrbW) -> &mut PucrbW
    {
        let mut w = PucrbW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PucrbR {
    bits: u32,
}

impl PucrbR {
    # [ doc = "Bit 15 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PucrbW {
    bits: u32,
}

impl PucrbW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PucrbW { bits: 0u32 }
    }
    # [ doc = "Bit 15 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Port B pull-up bit y (y=0..15)" ]
    pub fn pu0(&mut self, value: bool) -> &mut Self {
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
pub struct Pdcrb {
    register: ::volatile_register::RW<u32>,
}

impl Pdcrb {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PdcrbR, &'w mut PdcrbW) -> &'w mut PdcrbW
    {
        let bits = self.register.read();
        let r = PdcrbR { bits: bits };
        let mut w = PdcrbW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PdcrbR {
        PdcrbR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PdcrbW) -> &mut PdcrbW
    {
        let mut w = PdcrbW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdcrbR {
    bits: u32,
}

impl PdcrbR {
    # [ doc = "Bit 15 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdcrbW {
    bits: u32,
}

impl PdcrbW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PdcrbW { bits: 0u32 }
    }
    # [ doc = "Bit 15 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Port B pull-down bit y (y=0..15)" ]
    pub fn pd0(&mut self, value: bool) -> &mut Self {
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
pub struct Pucrc {
    register: ::volatile_register::RW<u32>,
}

impl Pucrc {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PucrcR, &'w mut PucrcW) -> &'w mut PucrcW
    {
        let bits = self.register.read();
        let r = PucrcR { bits: bits };
        let mut w = PucrcW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PucrcR {
        PucrcR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PucrcW) -> &mut PucrcW
    {
        let mut w = PucrcW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PucrcR {
    bits: u32,
}

impl PucrcR {
    # [ doc = "Bit 15 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PucrcW {
    bits: u32,
}

impl PucrcW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PucrcW { bits: 0u32 }
    }
    # [ doc = "Bit 15 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Port C pull-up bit y (y=0..15)" ]
    pub fn pu0(&mut self, value: bool) -> &mut Self {
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
pub struct Pdcrc {
    register: ::volatile_register::RW<u32>,
}

impl Pdcrc {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PdcrcR, &'w mut PdcrcW) -> &'w mut PdcrcW
    {
        let bits = self.register.read();
        let r = PdcrcR { bits: bits };
        let mut w = PdcrcW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PdcrcR {
        PdcrcR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PdcrcW) -> &mut PdcrcW
    {
        let mut w = PdcrcW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdcrcR {
    bits: u32,
}

impl PdcrcR {
    # [ doc = "Bit 15 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdcrcW {
    bits: u32,
}

impl PdcrcW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PdcrcW { bits: 0u32 }
    }
    # [ doc = "Bit 15 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Port C pull-down bit y (y=0..15)" ]
    pub fn pd0(&mut self, value: bool) -> &mut Self {
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
pub struct Pucrd {
    register: ::volatile_register::RW<u32>,
}

impl Pucrd {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PucrdR, &'w mut PucrdW) -> &'w mut PucrdW
    {
        let bits = self.register.read();
        let r = PucrdR { bits: bits };
        let mut w = PucrdW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PucrdR {
        PucrdR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PucrdW) -> &mut PucrdW
    {
        let mut w = PucrdW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PucrdR {
    bits: u32,
}

impl PucrdR {
    # [ doc = "Bit 15 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PucrdW {
    bits: u32,
}

impl PucrdW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PucrdW { bits: 0u32 }
    }
    # [ doc = "Bit 15 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Port D pull-up bit y (y=0..15)" ]
    pub fn pu0(&mut self, value: bool) -> &mut Self {
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
pub struct Pdcrd {
    register: ::volatile_register::RW<u32>,
}

impl Pdcrd {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PdcrdR, &'w mut PdcrdW) -> &'w mut PdcrdW
    {
        let bits = self.register.read();
        let r = PdcrdR { bits: bits };
        let mut w = PdcrdW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PdcrdR {
        PdcrdR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PdcrdW) -> &mut PdcrdW
    {
        let mut w = PdcrdW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdcrdR {
    bits: u32,
}

impl PdcrdR {
    # [ doc = "Bit 15 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdcrdW {
    bits: u32,
}

impl PdcrdW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PdcrdW { bits: 0u32 }
    }
    # [ doc = "Bit 15 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Port D pull-down bit y (y=0..15)" ]
    pub fn pd0(&mut self, value: bool) -> &mut Self {
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
pub struct Pucre {
    register: ::volatile_register::RW<u32>,
}

impl Pucre {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PucreR, &'w mut PucreW) -> &'w mut PucreW
    {
        let bits = self.register.read();
        let r = PucreR { bits: bits };
        let mut w = PucreW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PucreR {
        PucreR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PucreW) -> &mut PucreW
    {
        let mut w = PucreW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PucreR {
    bits: u32,
}

impl PucreR {
    # [ doc = "Bit 15 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PucreW {
    bits: u32,
}

impl PucreW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PucreW { bits: 0u32 }
    }
    # [ doc = "Bit 15 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Port E pull-up bit y (y=0..15)" ]
    pub fn pu0(&mut self, value: bool) -> &mut Self {
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
pub struct Pdcre {
    register: ::volatile_register::RW<u32>,
}

impl Pdcre {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PdcreR, &'w mut PdcreW) -> &'w mut PdcreW
    {
        let bits = self.register.read();
        let r = PdcreR { bits: bits };
        let mut w = PdcreW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PdcreR {
        PdcreR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PdcreW) -> &mut PdcreW
    {
        let mut w = PdcreW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdcreR {
    bits: u32,
}

impl PdcreR {
    # [ doc = "Bit 15 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdcreW {
    bits: u32,
}

impl PdcreW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PdcreW { bits: 0u32 }
    }
    # [ doc = "Bit 15 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Port E pull-down bit y (y=0..15)" ]
    pub fn pd0(&mut self, value: bool) -> &mut Self {
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
pub struct Pucrf {
    register: ::volatile_register::RW<u32>,
}

impl Pucrf {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PucrfR, &'w mut PucrfW) -> &'w mut PucrfW
    {
        let bits = self.register.read();
        let r = PucrfR { bits: bits };
        let mut w = PucrfW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PucrfR {
        PucrfR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PucrfW) -> &mut PucrfW
    {
        let mut w = PucrfW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PucrfR {
    bits: u32,
}

impl PucrfR {
    # [ doc = "Bit 15 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PucrfW {
    bits: u32,
}

impl PucrfW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PucrfW { bits: 0u32 }
    }
    # [ doc = "Bit 15 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Port F pull-up bit y (y=0..15)" ]
    pub fn pu0(&mut self, value: bool) -> &mut Self {
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
pub struct Pdcrf {
    register: ::volatile_register::RW<u32>,
}

impl Pdcrf {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PdcrfR, &'w mut PdcrfW) -> &'w mut PdcrfW
    {
        let bits = self.register.read();
        let r = PdcrfR { bits: bits };
        let mut w = PdcrfW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PdcrfR {
        PdcrfR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PdcrfW) -> &mut PdcrfW
    {
        let mut w = PdcrfW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdcrfR {
    bits: u32,
}

impl PdcrfR {
    # [ doc = "Bit 15 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdcrfW {
    bits: u32,
}

impl PdcrfW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PdcrfW { bits: 0u32 }
    }
    # [ doc = "Bit 15 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Port F pull-down bit y (y=0..15)" ]
    pub fn pd0(&mut self, value: bool) -> &mut Self {
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
pub struct Pucrg {
    register: ::volatile_register::RW<u32>,
}

impl Pucrg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PucrgR, &'w mut PucrgW) -> &'w mut PucrgW
    {
        let bits = self.register.read();
        let r = PucrgR { bits: bits };
        let mut w = PucrgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PucrgR {
        PucrgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PucrgW) -> &mut PucrgW
    {
        let mut w = PucrgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PucrgR {
    bits: u32,
}

impl PucrgR {
    # [ doc = "Bit 15 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PucrgW {
    bits: u32,
}

impl PucrgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PucrgW { bits: 0u32 }
    }
    # [ doc = "Bit 15 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Port G pull-up bit y (y=0..15)" ]
    pub fn pu0(&mut self, value: bool) -> &mut Self {
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
pub struct Pdcrg {
    register: ::volatile_register::RW<u32>,
}

impl Pdcrg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PdcrgR, &'w mut PdcrgW) -> &'w mut PdcrgW
    {
        let bits = self.register.read();
        let r = PdcrgR { bits: bits };
        let mut w = PdcrgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PdcrgR {
        PdcrgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PdcrgW) -> &mut PdcrgW
    {
        let mut w = PdcrgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdcrgR {
    bits: u32,
}

impl PdcrgR {
    # [ doc = "Bit 15 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd15(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd14(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd13(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd12(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd11(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd10(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd9(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd8(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd7(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd6(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd5(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd4(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd3(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd2(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdcrgW {
    bits: u32,
}

impl PdcrgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PdcrgW { bits: 0u32 }
    }
    # [ doc = "Bit 15 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd15(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd14(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd13(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd12(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd11(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd10(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd9(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd8(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Port G pull-down bit y (y=0..15)" ]
    pub fn pd0(&mut self, value: bool) -> &mut Self {
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
pub struct Pucrh {
    register: ::volatile_register::RW<u32>,
}

impl Pucrh {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PucrhR, &'w mut PucrhW) -> &'w mut PucrhW
    {
        let bits = self.register.read();
        let r = PucrhR { bits: bits };
        let mut w = PucrhW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PucrhR {
        PucrhR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PucrhW) -> &mut PucrhW
    {
        let mut w = PucrhW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PucrhR {
    bits: u32,
}

impl PucrhR {
    # [ doc = "Bit 1 - Port H pull-up bit y (y=0..1)" ]
    pub fn pu1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Port H pull-up bit y (y=0..1)" ]
    pub fn pu0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PucrhW {
    bits: u32,
}

impl PucrhW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PucrhW { bits: 0u32 }
    }
    # [ doc = "Bit 1 - Port H pull-up bit y (y=0..1)" ]
    pub fn pu1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Port H pull-up bit y (y=0..1)" ]
    pub fn pu0(&mut self, value: bool) -> &mut Self {
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
pub struct Pdcrh {
    register: ::volatile_register::RW<u32>,
}

impl Pdcrh {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PdcrhR, &'w mut PdcrhW) -> &'w mut PdcrhW
    {
        let bits = self.register.read();
        let r = PdcrhR { bits: bits };
        let mut w = PdcrhW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PdcrhR {
        PdcrhR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PdcrhW) -> &mut PdcrhW
    {
        let mut w = PdcrhW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdcrhR {
    bits: u32,
}

impl PdcrhR {
    # [ doc = "Bit 1 - Port H pull-down bit y (y=0..1)" ]
    pub fn pd1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Port H pull-down bit y (y=0..1)" ]
    pub fn pd0(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdcrhW {
    bits: u32,
}

impl PdcrhW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PdcrhW { bits: 0u32 }
    }
    # [ doc = "Bit 1 - Port H pull-down bit y (y=0..1)" ]
    pub fn pd1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Port H pull-down bit y (y=0..1)" ]
    pub fn pd0(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}
