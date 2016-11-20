# [ doc = "Digital filter for sigma delta modulators" ]
# [ repr ( C ) ]
pub struct Dfsdm {
    # [ doc = "0x00 - channel configuration y register" ]
    pub chcfg0r1: Chcfg0r1,
    # [ doc = "0x04 - channel configuration y register" ]
    pub chcfg0r2: Chcfg0r2,
    # [ doc = "0x08 - analog watchdog and short-circuit detector register" ]
    pub awscd0r: Awscd0r,
    # [ doc = "0x0c - channel watchdog filter data register" ]
    pub chwdat0r: Chwdat0r,
    # [ doc = "0x10 - channel data input register" ]
    pub chdatin0r: Chdatin0r,
    _reserved0: [u8; 12usize],
    # [ doc = "0x20 - CHCFG1R1" ]
    pub chcfg1r1: Chcfg1r1,
    # [ doc = "0x24 - CHCFG1R2" ]
    pub chcfg1r2: Chcfg1r2,
    # [ doc = "0x28 - AWSCD1R" ]
    pub awscd1r: Awscd1r,
    # [ doc = "0x2c - CHWDAT1R" ]
    pub chwdat1r: Chwdat1r,
    # [ doc = "0x30 - CHDATIN1R" ]
    pub chdatin1r: Chdatin1r,
    _reserved1: [u8; 12usize],
    # [ doc = "0x40 - CHCFG2R1" ]
    pub chcfg2r1: Chcfg2r1,
    # [ doc = "0x44 - CHCFG2R2" ]
    pub chcfg2r2: Chcfg2r2,
    # [ doc = "0x48 - AWSCD2R" ]
    pub awscd2r: Awscd2r,
    # [ doc = "0x4c - CHWDAT2R" ]
    pub chwdat2r: Chwdat2r,
    # [ doc = "0x50 - CHDATIN2R" ]
    pub chdatin2r: Chdatin2r,
    _reserved2: [u8; 12usize],
    # [ doc = "0x60 - CHCFG3R1" ]
    pub chcfg3r1: Chcfg3r1,
    # [ doc = "0x64 - CHCFG3R2" ]
    pub chcfg3r2: Chcfg3r2,
    # [ doc = "0x68 - AWSCD3R" ]
    pub awscd3r: Awscd3r,
    # [ doc = "0x6c - CHWDAT3R" ]
    pub chwdat3r: Chwdat3r,
    # [ doc = "0x70 - CHDATIN3R" ]
    pub chdatin3r: Chdatin3r,
    _reserved3: [u8; 12usize],
    # [ doc = "0x80 - CHCFG4R1" ]
    pub chcfg4r1: Chcfg4r1,
    # [ doc = "0x84 - CHCFG4R2" ]
    pub chcfg4r2: Chcfg4r2,
    # [ doc = "0x88 - AWSCD4R" ]
    pub awscd4r: Awscd4r,
    # [ doc = "0x8c - CHWDAT4R" ]
    pub chwdat4r: Chwdat4r,
    # [ doc = "0x90 - CHDATIN4R" ]
    pub chdatin4r: Chdatin4r,
    _reserved4: [u8; 12usize],
    # [ doc = "0xa0 - CHCFG5R1" ]
    pub chcfg5r1: Chcfg5r1,
    # [ doc = "0xa4 - CHCFG5R2" ]
    pub chcfg5r2: Chcfg5r2,
    # [ doc = "0xa8 - AWSCD5R" ]
    pub awscd5r: Awscd5r,
    # [ doc = "0xac - CHWDAT5R" ]
    pub chwdat5r: Chwdat5r,
    # [ doc = "0xb0 - CHDATIN5R" ]
    pub chdatin5r: Chdatin5r,
    _reserved5: [u8; 12usize],
    # [ doc = "0xc0 - CHCFG6R1" ]
    pub chcfg6r1: Chcfg6r1,
    # [ doc = "0xc4 - CHCFG6R2" ]
    pub chcfg6r2: Chcfg6r2,
    # [ doc = "0xc8 - AWSCD6R" ]
    pub awscd6r: Awscd6r,
    # [ doc = "0xcc - CHWDAT6R" ]
    pub chwdat6r: Chwdat6r,
    # [ doc = "0xd0 - CHDATIN6R" ]
    pub chdatin6r: Chdatin6r,
    _reserved6: [u8; 12usize],
    # [ doc = "0xe0 - CHCFG7R1" ]
    pub chcfg7r1: Chcfg7r1,
    # [ doc = "0xe4 - CHCFG7R2" ]
    pub chcfg7r2: Chcfg7r2,
    # [ doc = "0xe8 - AWSCD7R" ]
    pub awscd7r: Awscd7r,
    # [ doc = "0xec - CHWDAT7R" ]
    pub chwdat7r: Chwdat7r,
    # [ doc = "0xf0 - CHDATIN7R" ]
    pub chdatin7r: Chdatin7r,
    _reserved7: [u8; 12usize],
    # [ doc = "0x100 - control register 1" ]
    pub dfsdm0_cr1: Dfsdm0Cr1,
    # [ doc = "0x104 - control register 2" ]
    pub dfsdm0_cr2: Dfsdm0Cr2,
    # [ doc = "0x108 - interrupt and status register" ]
    pub dfsdm0_isr: Dfsdm0Isr,
    # [ doc = "0x10c - interrupt flag clear register" ]
    pub dfsdm0_icr: Dfsdm0Icr,
    # [ doc = "0x110 - injected channel group selection register" ]
    pub dfsdm0_jchgr: Dfsdm0Jchgr,
    # [ doc = "0x114 - filter control register" ]
    pub dfsdm0_fcr: Dfsdm0Fcr,
    # [ doc = "0x118 - data register for injected group" ]
    pub dfsdm0_jdatar: Dfsdm0Jdatar,
    # [ doc = "0x11c - data register for the regular channel" ]
    pub dfsdm0_rdatar: Dfsdm0Rdatar,
    # [ doc = "0x120 - analog watchdog high threshold register" ]
    pub dfsdm0_awhtr: Dfsdm0Awhtr,
    # [ doc = "0x124 - analog watchdog low threshold register" ]
    pub dfsdm0_awltr: Dfsdm0Awltr,
    # [ doc = "0x128 - analog watchdog status register" ]
    pub dfsdm0_awsr: Dfsdm0Awsr,
    # [ doc = "0x12c - analog watchdog clear flag register" ]
    pub dfsdm0_awcfr: Dfsdm0Awcfr,
    # [ doc = "0x130 - Extremes detector maximum register" ]
    pub dfsdm0_exmax: Dfsdm0Exmax,
    # [ doc = "0x134 - Extremes detector minimum register" ]
    pub dfsdm0_exmin: Dfsdm0Exmin,
    # [ doc = "0x138 - conversion timer register" ]
    pub dfsdm0_cnvtimr: Dfsdm0Cnvtimr,
    _reserved8: [u8; 196usize],
    # [ doc = "0x200 - control register 1" ]
    pub dfsdm1_cr1: Dfsdm1Cr1,
    # [ doc = "0x204 - control register 2" ]
    pub dfsdm1_cr2: Dfsdm1Cr2,
    # [ doc = "0x208 - interrupt and status register" ]
    pub dfsdm1_isr: Dfsdm1Isr,
    # [ doc = "0x20c - interrupt flag clear register" ]
    pub dfsdm1_icr: Dfsdm1Icr,
    # [ doc = "0x210 - injected channel group selection register" ]
    pub dfsdm1_jchgr: Dfsdm1Jchgr,
    # [ doc = "0x214 - filter control register" ]
    pub dfsdm1_fcr: Dfsdm1Fcr,
    # [ doc = "0x218 - data register for injected group" ]
    pub dfsdm1_jdatar: Dfsdm1Jdatar,
    # [ doc = "0x21c - data register for the regular channel" ]
    pub dfsdm1_rdatar: Dfsdm1Rdatar,
    # [ doc = "0x220 - analog watchdog high threshold register" ]
    pub dfsdm1_awhtr: Dfsdm1Awhtr,
    # [ doc = "0x224 - analog watchdog low threshold register" ]
    pub dfsdm1_awltr: Dfsdm1Awltr,
    # [ doc = "0x228 - analog watchdog status register" ]
    pub dfsdm1_awsr: Dfsdm1Awsr,
    # [ doc = "0x22c - analog watchdog clear flag register" ]
    pub dfsdm1_awcfr: Dfsdm1Awcfr,
    # [ doc = "0x230 - Extremes detector maximum register" ]
    pub dfsdm1_exmax: Dfsdm1Exmax,
    # [ doc = "0x234 - Extremes detector minimum register" ]
    pub dfsdm1_exmin: Dfsdm1Exmin,
    # [ doc = "0x238 - conversion timer register" ]
    pub dfsdm1_cnvtimr: Dfsdm1Cnvtimr,
    _reserved9: [u8; 196usize],
    # [ doc = "0x300 - control register 1" ]
    pub dfsdm2_cr1: Dfsdm2Cr1,
    # [ doc = "0x304 - control register 2" ]
    pub dfsdm2_cr2: Dfsdm2Cr2,
    # [ doc = "0x308 - interrupt and status register" ]
    pub dfsdm2_isr: Dfsdm2Isr,
    # [ doc = "0x30c - interrupt flag clear register" ]
    pub dfsdm2_icr: Dfsdm2Icr,
    # [ doc = "0x310 - injected channel group selection register" ]
    pub dfsdm2_jchgr: Dfsdm2Jchgr,
    # [ doc = "0x314 - filter control register" ]
    pub dfsdm2_fcr: Dfsdm2Fcr,
    # [ doc = "0x318 - data register for injected group" ]
    pub dfsdm2_jdatar: Dfsdm2Jdatar,
    # [ doc = "0x31c - data register for the regular channel" ]
    pub dfsdm2_rdatar: Dfsdm2Rdatar,
    # [ doc = "0x320 - analog watchdog high threshold register" ]
    pub dfsdm2_awhtr: Dfsdm2Awhtr,
    # [ doc = "0x324 - analog watchdog low threshold register" ]
    pub dfsdm2_awltr: Dfsdm2Awltr,
    # [ doc = "0x328 - analog watchdog status register" ]
    pub dfsdm2_awsr: Dfsdm2Awsr,
    # [ doc = "0x32c - analog watchdog clear flag register" ]
    pub dfsdm2_awcfr: Dfsdm2Awcfr,
    # [ doc = "0x330 - Extremes detector maximum register" ]
    pub dfsdm2_exmax: Dfsdm2Exmax,
    # [ doc = "0x334 - Extremes detector minimum register" ]
    pub dfsdm2_exmin: Dfsdm2Exmin,
    # [ doc = "0x338 - conversion timer register" ]
    pub dfsdm2_cnvtimr: Dfsdm2Cnvtimr,
    _reserved10: [u8; 196usize],
    # [ doc = "0x400 - control register 1" ]
    pub dfsdm3_cr1: Dfsdm3Cr1,
    # [ doc = "0x404 - control register 2" ]
    pub dfsdm3_cr2: Dfsdm3Cr2,
    # [ doc = "0x408 - interrupt and status register" ]
    pub dfsdm3_isr: Dfsdm3Isr,
    # [ doc = "0x40c - interrupt flag clear register" ]
    pub dfsdm3_icr: Dfsdm3Icr,
    # [ doc = "0x410 - injected channel group selection register" ]
    pub dfsdm3_jchgr: Dfsdm3Jchgr,
    # [ doc = "0x414 - filter control register" ]
    pub dfsdm3_fcr: Dfsdm3Fcr,
    # [ doc = "0x418 - data register for injected group" ]
    pub dfsdm3_jdatar: Dfsdm3Jdatar,
    # [ doc = "0x41c - data register for the regular channel" ]
    pub dfsdm3_rdatar: Dfsdm3Rdatar,
    # [ doc = "0x420 - analog watchdog high threshold register" ]
    pub dfsdm3_awhtr: Dfsdm3Awhtr,
    # [ doc = "0x424 - analog watchdog low threshold register" ]
    pub dfsdm3_awltr: Dfsdm3Awltr,
    # [ doc = "0x428 - analog watchdog status register" ]
    pub dfsdm3_awsr: Dfsdm3Awsr,
    # [ doc = "0x42c - analog watchdog clear flag register" ]
    pub dfsdm3_awcfr: Dfsdm3Awcfr,
    # [ doc = "0x430 - Extremes detector maximum register" ]
    pub dfsdm3_exmax: Dfsdm3Exmax,
    # [ doc = "0x434 - Extremes detector minimum register" ]
    pub dfsdm3_exmin: Dfsdm3Exmin,
    # [ doc = "0x438 - conversion timer register" ]
    pub dfsdm3_cnvtimr: Dfsdm3Cnvtimr,
}

# [ repr ( C ) ]
pub struct Chcfg0r1 {
    register: ::volatile_register::RW<u32>,
}

impl Chcfg0r1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chcfg0r1R, &'w mut Chcfg0r1W) -> &'w mut Chcfg0r1W
    {
        let bits = self.register.read();
        let r = Chcfg0r1R { bits: bits };
        let mut w = Chcfg0r1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chcfg0r1R {
        Chcfg0r1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chcfg0r1W) -> &mut Chcfg0r1W
    {
        let mut w = Chcfg0r1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg0r1R {
    bits: u32,
}

impl Chcfg0r1R {
    # [ doc = "Bit 31 - DFSDMEN" ]
    pub fn dfsdmen(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - CKOUTSRC" ]
    pub fn ckoutsrc(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 16:23 - CKOUTDIV" ]
    pub fn ckoutdiv(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 14:15 - DATPACK" ]
    pub fn datpack(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:13 - DATMPX" ]
    pub fn datmpx(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 8 - CHINSEL" ]
    pub fn chinsel(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - CHEN" ]
    pub fn chen(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - CKABEN" ]
    pub fn ckaben(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - SCDEN" ]
    pub fn scden(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:3 - SPICKSEL" ]
    pub fn spicksel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:1 - SITP" ]
    pub fn sitp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg0r1W {
    bits: u32,
}

impl Chcfg0r1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chcfg0r1W { bits: 0u32 }
    }
    # [ doc = "Bit 31 - DFSDMEN" ]
    pub fn dfsdmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - CKOUTSRC" ]
    pub fn ckoutsrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 16:23 - CKOUTDIV" ]
    pub fn ckoutdiv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 14:15 - DATPACK" ]
    pub fn datpack(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:13 - DATMPX" ]
    pub fn datmpx(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 8 - CHINSEL" ]
    pub fn chinsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - CHEN" ]
    pub fn chen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - CKABEN" ]
    pub fn ckaben(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - SCDEN" ]
    pub fn scden(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:3 - SPICKSEL" ]
    pub fn spicksel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:1 - SITP" ]
    pub fn sitp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chcfg0r2 {
    register: ::volatile_register::RW<u32>,
}

impl Chcfg0r2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chcfg0r2R, &'w mut Chcfg0r2W) -> &'w mut Chcfg0r2W
    {
        let bits = self.register.read();
        let r = Chcfg0r2R { bits: bits };
        let mut w = Chcfg0r2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chcfg0r2R {
        Chcfg0r2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chcfg0r2W) -> &mut Chcfg0r2W
    {
        let mut w = Chcfg0r2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg0r2R {
    bits: u32,
}

impl Chcfg0r2R {
    # [ doc = "Bits 8:31 - OFFSET" ]
    pub fn offset(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 3:7 - DTRBS" ]
    pub fn dtrbs(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg0r2W {
    bits: u32,
}

impl Chcfg0r2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chcfg0r2W { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - OFFSET" ]
    pub fn offset(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 3:7 - DTRBS" ]
    pub fn dtrbs(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Awscd0r {
    register: ::volatile_register::RW<u32>,
}

impl Awscd0r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Awscd0rR, &'w mut Awscd0rW) -> &'w mut Awscd0rW
    {
        let bits = self.register.read();
        let r = Awscd0rR { bits: bits };
        let mut w = Awscd0rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Awscd0rR {
        Awscd0rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Awscd0rW) -> &mut Awscd0rW
    {
        let mut w = Awscd0rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awscd0rR {
    bits: u32,
}

impl Awscd0rR {
    # [ doc = "Bits 22:23 - AWFORD" ]
    pub fn awford(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:20 - AWFOSR" ]
    pub fn awfosr(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - BKSCD" ]
    pub fn bkscd(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - SCDT" ]
    pub fn scdt(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awscd0rW {
    bits: u32,
}

impl Awscd0rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Awscd0rW { bits: 0u32 }
    }
    # [ doc = "Bits 22:23 - AWFORD" ]
    pub fn awford(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:20 - AWFOSR" ]
    pub fn awfosr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - BKSCD" ]
    pub fn bkscd(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - SCDT" ]
    pub fn scdt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chwdat0r {
    register: ::volatile_register::RW<u32>,
}

impl Chwdat0r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chwdat0rR, &'w mut Chwdat0rW) -> &'w mut Chwdat0rW
    {
        let bits = self.register.read();
        let r = Chwdat0rR { bits: bits };
        let mut w = Chwdat0rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chwdat0rR {
        Chwdat0rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chwdat0rW) -> &mut Chwdat0rW
    {
        let mut w = Chwdat0rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chwdat0rR {
    bits: u32,
}

impl Chwdat0rR {
    # [ doc = "Bits 0:15 - WDATA" ]
    pub fn wdata(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chwdat0rW {
    bits: u32,
}

impl Chwdat0rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chwdat0rW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - WDATA" ]
    pub fn wdata(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chdatin0r {
    register: ::volatile_register::RW<u32>,
}

impl Chdatin0r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chdatin0rR, &'w mut Chdatin0rW) -> &'w mut Chdatin0rW
    {
        let bits = self.register.read();
        let r = Chdatin0rR { bits: bits };
        let mut w = Chdatin0rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chdatin0rR {
        Chdatin0rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chdatin0rW) -> &mut Chdatin0rW
    {
        let mut w = Chdatin0rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chdatin0rR {
    bits: u32,
}

impl Chdatin0rR {
    # [ doc = "Bits 16:31 - INDAT1" ]
    pub fn indat1(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:15 - INDAT0" ]
    pub fn indat0(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chdatin0rW {
    bits: u32,
}

impl Chdatin0rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chdatin0rW { bits: 0u32 }
    }
    # [ doc = "Bits 16:31 - INDAT1" ]
    pub fn indat1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:15 - INDAT0" ]
    pub fn indat0(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chcfg1r1 {
    register: ::volatile_register::RW<u32>,
}

impl Chcfg1r1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chcfg1r1R, &'w mut Chcfg1r1W) -> &'w mut Chcfg1r1W
    {
        let bits = self.register.read();
        let r = Chcfg1r1R { bits: bits };
        let mut w = Chcfg1r1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chcfg1r1R {
        Chcfg1r1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chcfg1r1W) -> &mut Chcfg1r1W
    {
        let mut w = Chcfg1r1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg1r1R {
    bits: u32,
}

impl Chcfg1r1R {
    # [ doc = "Bits 14:15 - DATPACK" ]
    pub fn datpack(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:13 - DATMPX" ]
    pub fn datmpx(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 8 - CHINSEL" ]
    pub fn chinsel(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - CHEN" ]
    pub fn chen(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - CKABEN" ]
    pub fn ckaben(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - SCDEN" ]
    pub fn scden(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:3 - SPICKSEL" ]
    pub fn spicksel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:1 - SITP" ]
    pub fn sitp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg1r1W {
    bits: u32,
}

impl Chcfg1r1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chcfg1r1W { bits: 0u32 }
    }
    # [ doc = "Bits 14:15 - DATPACK" ]
    pub fn datpack(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:13 - DATMPX" ]
    pub fn datmpx(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 8 - CHINSEL" ]
    pub fn chinsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - CHEN" ]
    pub fn chen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - CKABEN" ]
    pub fn ckaben(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - SCDEN" ]
    pub fn scden(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:3 - SPICKSEL" ]
    pub fn spicksel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:1 - SITP" ]
    pub fn sitp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chcfg1r2 {
    register: ::volatile_register::RW<u32>,
}

impl Chcfg1r2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chcfg1r2R, &'w mut Chcfg1r2W) -> &'w mut Chcfg1r2W
    {
        let bits = self.register.read();
        let r = Chcfg1r2R { bits: bits };
        let mut w = Chcfg1r2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chcfg1r2R {
        Chcfg1r2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chcfg1r2W) -> &mut Chcfg1r2W
    {
        let mut w = Chcfg1r2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg1r2R {
    bits: u32,
}

impl Chcfg1r2R {
    # [ doc = "Bits 8:31 - OFFSET" ]
    pub fn offset(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 3:7 - DTRBS" ]
    pub fn dtrbs(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg1r2W {
    bits: u32,
}

impl Chcfg1r2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chcfg1r2W { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - OFFSET" ]
    pub fn offset(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 3:7 - DTRBS" ]
    pub fn dtrbs(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Awscd1r {
    register: ::volatile_register::RW<u32>,
}

impl Awscd1r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Awscd1rR, &'w mut Awscd1rW) -> &'w mut Awscd1rW
    {
        let bits = self.register.read();
        let r = Awscd1rR { bits: bits };
        let mut w = Awscd1rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Awscd1rR {
        Awscd1rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Awscd1rW) -> &mut Awscd1rW
    {
        let mut w = Awscd1rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awscd1rR {
    bits: u32,
}

impl Awscd1rR {
    # [ doc = "Bits 22:23 - AWFORD" ]
    pub fn awford(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:20 - AWFOSR" ]
    pub fn awfosr(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - BKSCD" ]
    pub fn bkscd(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - SCDT" ]
    pub fn scdt(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awscd1rW {
    bits: u32,
}

impl Awscd1rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Awscd1rW { bits: 0u32 }
    }
    # [ doc = "Bits 22:23 - AWFORD" ]
    pub fn awford(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:20 - AWFOSR" ]
    pub fn awfosr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - BKSCD" ]
    pub fn bkscd(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - SCDT" ]
    pub fn scdt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chwdat1r {
    register: ::volatile_register::RW<u32>,
}

impl Chwdat1r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chwdat1rR, &'w mut Chwdat1rW) -> &'w mut Chwdat1rW
    {
        let bits = self.register.read();
        let r = Chwdat1rR { bits: bits };
        let mut w = Chwdat1rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chwdat1rR {
        Chwdat1rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chwdat1rW) -> &mut Chwdat1rW
    {
        let mut w = Chwdat1rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chwdat1rR {
    bits: u32,
}

impl Chwdat1rR {
    # [ doc = "Bits 0:15 - WDATA" ]
    pub fn wdata(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chwdat1rW {
    bits: u32,
}

impl Chwdat1rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chwdat1rW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - WDATA" ]
    pub fn wdata(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chdatin1r {
    register: ::volatile_register::RW<u32>,
}

impl Chdatin1r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chdatin1rR, &'w mut Chdatin1rW) -> &'w mut Chdatin1rW
    {
        let bits = self.register.read();
        let r = Chdatin1rR { bits: bits };
        let mut w = Chdatin1rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chdatin1rR {
        Chdatin1rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chdatin1rW) -> &mut Chdatin1rW
    {
        let mut w = Chdatin1rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chdatin1rR {
    bits: u32,
}

impl Chdatin1rR {
    # [ doc = "Bits 16:31 - INDAT1" ]
    pub fn indat1(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:15 - INDAT0" ]
    pub fn indat0(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chdatin1rW {
    bits: u32,
}

impl Chdatin1rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chdatin1rW { bits: 0u32 }
    }
    # [ doc = "Bits 16:31 - INDAT1" ]
    pub fn indat1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:15 - INDAT0" ]
    pub fn indat0(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chcfg2r1 {
    register: ::volatile_register::RW<u32>,
}

impl Chcfg2r1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chcfg2r1R, &'w mut Chcfg2r1W) -> &'w mut Chcfg2r1W
    {
        let bits = self.register.read();
        let r = Chcfg2r1R { bits: bits };
        let mut w = Chcfg2r1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chcfg2r1R {
        Chcfg2r1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chcfg2r1W) -> &mut Chcfg2r1W
    {
        let mut w = Chcfg2r1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg2r1R {
    bits: u32,
}

impl Chcfg2r1R {
    # [ doc = "Bits 14:15 - DATPACK" ]
    pub fn datpack(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:13 - DATMPX" ]
    pub fn datmpx(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 8 - CHINSEL" ]
    pub fn chinsel(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - CHEN" ]
    pub fn chen(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - CKABEN" ]
    pub fn ckaben(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - SCDEN" ]
    pub fn scden(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:3 - SPICKSEL" ]
    pub fn spicksel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:1 - SITP" ]
    pub fn sitp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg2r1W {
    bits: u32,
}

impl Chcfg2r1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chcfg2r1W { bits: 0u32 }
    }
    # [ doc = "Bits 14:15 - DATPACK" ]
    pub fn datpack(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:13 - DATMPX" ]
    pub fn datmpx(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 8 - CHINSEL" ]
    pub fn chinsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - CHEN" ]
    pub fn chen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - CKABEN" ]
    pub fn ckaben(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - SCDEN" ]
    pub fn scden(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:3 - SPICKSEL" ]
    pub fn spicksel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:1 - SITP" ]
    pub fn sitp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chcfg2r2 {
    register: ::volatile_register::RW<u32>,
}

impl Chcfg2r2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chcfg2r2R, &'w mut Chcfg2r2W) -> &'w mut Chcfg2r2W
    {
        let bits = self.register.read();
        let r = Chcfg2r2R { bits: bits };
        let mut w = Chcfg2r2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chcfg2r2R {
        Chcfg2r2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chcfg2r2W) -> &mut Chcfg2r2W
    {
        let mut w = Chcfg2r2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg2r2R {
    bits: u32,
}

impl Chcfg2r2R {
    # [ doc = "Bits 8:31 - OFFSET" ]
    pub fn offset(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 3:7 - DTRBS" ]
    pub fn dtrbs(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg2r2W {
    bits: u32,
}

impl Chcfg2r2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chcfg2r2W { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - OFFSET" ]
    pub fn offset(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 3:7 - DTRBS" ]
    pub fn dtrbs(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Awscd2r {
    register: ::volatile_register::RW<u32>,
}

impl Awscd2r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Awscd2rR, &'w mut Awscd2rW) -> &'w mut Awscd2rW
    {
        let bits = self.register.read();
        let r = Awscd2rR { bits: bits };
        let mut w = Awscd2rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Awscd2rR {
        Awscd2rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Awscd2rW) -> &mut Awscd2rW
    {
        let mut w = Awscd2rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awscd2rR {
    bits: u32,
}

impl Awscd2rR {
    # [ doc = "Bits 22:23 - AWFORD" ]
    pub fn awford(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:20 - AWFOSR" ]
    pub fn awfosr(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - BKSCD" ]
    pub fn bkscd(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - SCDT" ]
    pub fn scdt(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awscd2rW {
    bits: u32,
}

impl Awscd2rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Awscd2rW { bits: 0u32 }
    }
    # [ doc = "Bits 22:23 - AWFORD" ]
    pub fn awford(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:20 - AWFOSR" ]
    pub fn awfosr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - BKSCD" ]
    pub fn bkscd(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - SCDT" ]
    pub fn scdt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chwdat2r {
    register: ::volatile_register::RW<u32>,
}

impl Chwdat2r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chwdat2rR, &'w mut Chwdat2rW) -> &'w mut Chwdat2rW
    {
        let bits = self.register.read();
        let r = Chwdat2rR { bits: bits };
        let mut w = Chwdat2rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chwdat2rR {
        Chwdat2rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chwdat2rW) -> &mut Chwdat2rW
    {
        let mut w = Chwdat2rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chwdat2rR {
    bits: u32,
}

impl Chwdat2rR {
    # [ doc = "Bits 0:15 - WDATA" ]
    pub fn wdata(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chwdat2rW {
    bits: u32,
}

impl Chwdat2rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chwdat2rW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - WDATA" ]
    pub fn wdata(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chdatin2r {
    register: ::volatile_register::RW<u32>,
}

impl Chdatin2r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chdatin2rR, &'w mut Chdatin2rW) -> &'w mut Chdatin2rW
    {
        let bits = self.register.read();
        let r = Chdatin2rR { bits: bits };
        let mut w = Chdatin2rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chdatin2rR {
        Chdatin2rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chdatin2rW) -> &mut Chdatin2rW
    {
        let mut w = Chdatin2rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chdatin2rR {
    bits: u32,
}

impl Chdatin2rR {
    # [ doc = "Bits 16:31 - INDAT1" ]
    pub fn indat1(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:15 - INDAT0" ]
    pub fn indat0(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chdatin2rW {
    bits: u32,
}

impl Chdatin2rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chdatin2rW { bits: 0u32 }
    }
    # [ doc = "Bits 16:31 - INDAT1" ]
    pub fn indat1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:15 - INDAT0" ]
    pub fn indat0(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chcfg3r1 {
    register: ::volatile_register::RW<u32>,
}

impl Chcfg3r1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chcfg3r1R, &'w mut Chcfg3r1W) -> &'w mut Chcfg3r1W
    {
        let bits = self.register.read();
        let r = Chcfg3r1R { bits: bits };
        let mut w = Chcfg3r1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chcfg3r1R {
        Chcfg3r1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chcfg3r1W) -> &mut Chcfg3r1W
    {
        let mut w = Chcfg3r1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg3r1R {
    bits: u32,
}

impl Chcfg3r1R {
    # [ doc = "Bits 14:15 - DATPACK" ]
    pub fn datpack(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:13 - DATMPX" ]
    pub fn datmpx(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 8 - CHINSEL" ]
    pub fn chinsel(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - CHEN" ]
    pub fn chen(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - CKABEN" ]
    pub fn ckaben(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - SCDEN" ]
    pub fn scden(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:3 - SPICKSEL" ]
    pub fn spicksel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:1 - SITP" ]
    pub fn sitp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg3r1W {
    bits: u32,
}

impl Chcfg3r1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chcfg3r1W { bits: 0u32 }
    }
    # [ doc = "Bits 14:15 - DATPACK" ]
    pub fn datpack(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:13 - DATMPX" ]
    pub fn datmpx(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 8 - CHINSEL" ]
    pub fn chinsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - CHEN" ]
    pub fn chen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - CKABEN" ]
    pub fn ckaben(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - SCDEN" ]
    pub fn scden(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:3 - SPICKSEL" ]
    pub fn spicksel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:1 - SITP" ]
    pub fn sitp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chcfg3r2 {
    register: ::volatile_register::RW<u32>,
}

impl Chcfg3r2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chcfg3r2R, &'w mut Chcfg3r2W) -> &'w mut Chcfg3r2W
    {
        let bits = self.register.read();
        let r = Chcfg3r2R { bits: bits };
        let mut w = Chcfg3r2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chcfg3r2R {
        Chcfg3r2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chcfg3r2W) -> &mut Chcfg3r2W
    {
        let mut w = Chcfg3r2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg3r2R {
    bits: u32,
}

impl Chcfg3r2R {
    # [ doc = "Bits 8:31 - OFFSET" ]
    pub fn offset(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 3:7 - DTRBS" ]
    pub fn dtrbs(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg3r2W {
    bits: u32,
}

impl Chcfg3r2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chcfg3r2W { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - OFFSET" ]
    pub fn offset(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 3:7 - DTRBS" ]
    pub fn dtrbs(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Awscd3r {
    register: ::volatile_register::RW<u32>,
}

impl Awscd3r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Awscd3rR, &'w mut Awscd3rW) -> &'w mut Awscd3rW
    {
        let bits = self.register.read();
        let r = Awscd3rR { bits: bits };
        let mut w = Awscd3rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Awscd3rR {
        Awscd3rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Awscd3rW) -> &mut Awscd3rW
    {
        let mut w = Awscd3rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awscd3rR {
    bits: u32,
}

impl Awscd3rR {
    # [ doc = "Bits 22:23 - AWFORD" ]
    pub fn awford(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:20 - AWFOSR" ]
    pub fn awfosr(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - BKSCD" ]
    pub fn bkscd(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - SCDT" ]
    pub fn scdt(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awscd3rW {
    bits: u32,
}

impl Awscd3rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Awscd3rW { bits: 0u32 }
    }
    # [ doc = "Bits 22:23 - AWFORD" ]
    pub fn awford(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:20 - AWFOSR" ]
    pub fn awfosr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - BKSCD" ]
    pub fn bkscd(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - SCDT" ]
    pub fn scdt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chwdat3r {
    register: ::volatile_register::RW<u32>,
}

impl Chwdat3r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chwdat3rR, &'w mut Chwdat3rW) -> &'w mut Chwdat3rW
    {
        let bits = self.register.read();
        let r = Chwdat3rR { bits: bits };
        let mut w = Chwdat3rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chwdat3rR {
        Chwdat3rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chwdat3rW) -> &mut Chwdat3rW
    {
        let mut w = Chwdat3rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chwdat3rR {
    bits: u32,
}

impl Chwdat3rR {
    # [ doc = "Bits 0:15 - WDATA" ]
    pub fn wdata(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chwdat3rW {
    bits: u32,
}

impl Chwdat3rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chwdat3rW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - WDATA" ]
    pub fn wdata(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chdatin3r {
    register: ::volatile_register::RW<u32>,
}

impl Chdatin3r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chdatin3rR, &'w mut Chdatin3rW) -> &'w mut Chdatin3rW
    {
        let bits = self.register.read();
        let r = Chdatin3rR { bits: bits };
        let mut w = Chdatin3rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chdatin3rR {
        Chdatin3rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chdatin3rW) -> &mut Chdatin3rW
    {
        let mut w = Chdatin3rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chdatin3rR {
    bits: u32,
}

impl Chdatin3rR {
    # [ doc = "Bits 16:31 - INDAT1" ]
    pub fn indat1(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:15 - INDAT0" ]
    pub fn indat0(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chdatin3rW {
    bits: u32,
}

impl Chdatin3rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chdatin3rW { bits: 0u32 }
    }
    # [ doc = "Bits 16:31 - INDAT1" ]
    pub fn indat1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:15 - INDAT0" ]
    pub fn indat0(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chcfg4r1 {
    register: ::volatile_register::RW<u32>,
}

impl Chcfg4r1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chcfg4r1R, &'w mut Chcfg4r1W) -> &'w mut Chcfg4r1W
    {
        let bits = self.register.read();
        let r = Chcfg4r1R { bits: bits };
        let mut w = Chcfg4r1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chcfg4r1R {
        Chcfg4r1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chcfg4r1W) -> &mut Chcfg4r1W
    {
        let mut w = Chcfg4r1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg4r1R {
    bits: u32,
}

impl Chcfg4r1R {
    # [ doc = "Bits 14:15 - DATPACK" ]
    pub fn datpack(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:13 - DATMPX" ]
    pub fn datmpx(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 8 - CHINSEL" ]
    pub fn chinsel(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - CHEN" ]
    pub fn chen(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - CKABEN" ]
    pub fn ckaben(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - SCDEN" ]
    pub fn scden(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:3 - SPICKSEL" ]
    pub fn spicksel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:1 - SITP" ]
    pub fn sitp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg4r1W {
    bits: u32,
}

impl Chcfg4r1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chcfg4r1W { bits: 0u32 }
    }
    # [ doc = "Bits 14:15 - DATPACK" ]
    pub fn datpack(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:13 - DATMPX" ]
    pub fn datmpx(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 8 - CHINSEL" ]
    pub fn chinsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - CHEN" ]
    pub fn chen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - CKABEN" ]
    pub fn ckaben(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - SCDEN" ]
    pub fn scden(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:3 - SPICKSEL" ]
    pub fn spicksel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:1 - SITP" ]
    pub fn sitp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chcfg4r2 {
    register: ::volatile_register::RW<u32>,
}

impl Chcfg4r2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chcfg4r2R, &'w mut Chcfg4r2W) -> &'w mut Chcfg4r2W
    {
        let bits = self.register.read();
        let r = Chcfg4r2R { bits: bits };
        let mut w = Chcfg4r2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chcfg4r2R {
        Chcfg4r2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chcfg4r2W) -> &mut Chcfg4r2W
    {
        let mut w = Chcfg4r2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg4r2R {
    bits: u32,
}

impl Chcfg4r2R {
    # [ doc = "Bits 8:31 - OFFSET" ]
    pub fn offset(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 3:7 - DTRBS" ]
    pub fn dtrbs(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg4r2W {
    bits: u32,
}

impl Chcfg4r2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chcfg4r2W { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - OFFSET" ]
    pub fn offset(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 3:7 - DTRBS" ]
    pub fn dtrbs(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Awscd4r {
    register: ::volatile_register::RW<u32>,
}

impl Awscd4r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Awscd4rR, &'w mut Awscd4rW) -> &'w mut Awscd4rW
    {
        let bits = self.register.read();
        let r = Awscd4rR { bits: bits };
        let mut w = Awscd4rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Awscd4rR {
        Awscd4rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Awscd4rW) -> &mut Awscd4rW
    {
        let mut w = Awscd4rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awscd4rR {
    bits: u32,
}

impl Awscd4rR {
    # [ doc = "Bits 22:23 - AWFORD" ]
    pub fn awford(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:20 - AWFOSR" ]
    pub fn awfosr(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - BKSCD" ]
    pub fn bkscd(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - SCDT" ]
    pub fn scdt(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awscd4rW {
    bits: u32,
}

impl Awscd4rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Awscd4rW { bits: 0u32 }
    }
    # [ doc = "Bits 22:23 - AWFORD" ]
    pub fn awford(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:20 - AWFOSR" ]
    pub fn awfosr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - BKSCD" ]
    pub fn bkscd(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - SCDT" ]
    pub fn scdt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chwdat4r {
    register: ::volatile_register::RW<u32>,
}

impl Chwdat4r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chwdat4rR, &'w mut Chwdat4rW) -> &'w mut Chwdat4rW
    {
        let bits = self.register.read();
        let r = Chwdat4rR { bits: bits };
        let mut w = Chwdat4rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chwdat4rR {
        Chwdat4rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chwdat4rW) -> &mut Chwdat4rW
    {
        let mut w = Chwdat4rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chwdat4rR {
    bits: u32,
}

impl Chwdat4rR {
    # [ doc = "Bits 0:15 - WDATA" ]
    pub fn wdata(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chwdat4rW {
    bits: u32,
}

impl Chwdat4rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chwdat4rW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - WDATA" ]
    pub fn wdata(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chdatin4r {
    register: ::volatile_register::RW<u32>,
}

impl Chdatin4r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chdatin4rR, &'w mut Chdatin4rW) -> &'w mut Chdatin4rW
    {
        let bits = self.register.read();
        let r = Chdatin4rR { bits: bits };
        let mut w = Chdatin4rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chdatin4rR {
        Chdatin4rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chdatin4rW) -> &mut Chdatin4rW
    {
        let mut w = Chdatin4rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chdatin4rR {
    bits: u32,
}

impl Chdatin4rR {
    # [ doc = "Bits 16:31 - INDAT1" ]
    pub fn indat1(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:15 - INDAT0" ]
    pub fn indat0(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chdatin4rW {
    bits: u32,
}

impl Chdatin4rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chdatin4rW { bits: 0u32 }
    }
    # [ doc = "Bits 16:31 - INDAT1" ]
    pub fn indat1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:15 - INDAT0" ]
    pub fn indat0(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chcfg5r1 {
    register: ::volatile_register::RW<u32>,
}

impl Chcfg5r1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chcfg5r1R, &'w mut Chcfg5r1W) -> &'w mut Chcfg5r1W
    {
        let bits = self.register.read();
        let r = Chcfg5r1R { bits: bits };
        let mut w = Chcfg5r1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chcfg5r1R {
        Chcfg5r1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chcfg5r1W) -> &mut Chcfg5r1W
    {
        let mut w = Chcfg5r1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg5r1R {
    bits: u32,
}

impl Chcfg5r1R {
    # [ doc = "Bits 14:15 - DATPACK" ]
    pub fn datpack(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:13 - DATMPX" ]
    pub fn datmpx(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 8 - CHINSEL" ]
    pub fn chinsel(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - CHEN" ]
    pub fn chen(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - CKABEN" ]
    pub fn ckaben(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - SCDEN" ]
    pub fn scden(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:3 - SPICKSEL" ]
    pub fn spicksel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:1 - SITP" ]
    pub fn sitp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg5r1W {
    bits: u32,
}

impl Chcfg5r1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chcfg5r1W { bits: 0u32 }
    }
    # [ doc = "Bits 14:15 - DATPACK" ]
    pub fn datpack(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:13 - DATMPX" ]
    pub fn datmpx(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 8 - CHINSEL" ]
    pub fn chinsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - CHEN" ]
    pub fn chen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - CKABEN" ]
    pub fn ckaben(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - SCDEN" ]
    pub fn scden(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:3 - SPICKSEL" ]
    pub fn spicksel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:1 - SITP" ]
    pub fn sitp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chcfg5r2 {
    register: ::volatile_register::RW<u32>,
}

impl Chcfg5r2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chcfg5r2R, &'w mut Chcfg5r2W) -> &'w mut Chcfg5r2W
    {
        let bits = self.register.read();
        let r = Chcfg5r2R { bits: bits };
        let mut w = Chcfg5r2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chcfg5r2R {
        Chcfg5r2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chcfg5r2W) -> &mut Chcfg5r2W
    {
        let mut w = Chcfg5r2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg5r2R {
    bits: u32,
}

impl Chcfg5r2R {
    # [ doc = "Bits 8:31 - OFFSET" ]
    pub fn offset(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 3:7 - DTRBS" ]
    pub fn dtrbs(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg5r2W {
    bits: u32,
}

impl Chcfg5r2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chcfg5r2W { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - OFFSET" ]
    pub fn offset(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 3:7 - DTRBS" ]
    pub fn dtrbs(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Awscd5r {
    register: ::volatile_register::RW<u32>,
}

impl Awscd5r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Awscd5rR, &'w mut Awscd5rW) -> &'w mut Awscd5rW
    {
        let bits = self.register.read();
        let r = Awscd5rR { bits: bits };
        let mut w = Awscd5rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Awscd5rR {
        Awscd5rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Awscd5rW) -> &mut Awscd5rW
    {
        let mut w = Awscd5rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awscd5rR {
    bits: u32,
}

impl Awscd5rR {
    # [ doc = "Bits 22:23 - AWFORD" ]
    pub fn awford(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:20 - AWFOSR" ]
    pub fn awfosr(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - BKSCD" ]
    pub fn bkscd(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - SCDT" ]
    pub fn scdt(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awscd5rW {
    bits: u32,
}

impl Awscd5rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Awscd5rW { bits: 0u32 }
    }
    # [ doc = "Bits 22:23 - AWFORD" ]
    pub fn awford(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:20 - AWFOSR" ]
    pub fn awfosr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - BKSCD" ]
    pub fn bkscd(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - SCDT" ]
    pub fn scdt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chwdat5r {
    register: ::volatile_register::RW<u32>,
}

impl Chwdat5r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chwdat5rR, &'w mut Chwdat5rW) -> &'w mut Chwdat5rW
    {
        let bits = self.register.read();
        let r = Chwdat5rR { bits: bits };
        let mut w = Chwdat5rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chwdat5rR {
        Chwdat5rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chwdat5rW) -> &mut Chwdat5rW
    {
        let mut w = Chwdat5rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chwdat5rR {
    bits: u32,
}

impl Chwdat5rR {
    # [ doc = "Bits 0:15 - WDATA" ]
    pub fn wdata(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chwdat5rW {
    bits: u32,
}

impl Chwdat5rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chwdat5rW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - WDATA" ]
    pub fn wdata(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chdatin5r {
    register: ::volatile_register::RW<u32>,
}

impl Chdatin5r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chdatin5rR, &'w mut Chdatin5rW) -> &'w mut Chdatin5rW
    {
        let bits = self.register.read();
        let r = Chdatin5rR { bits: bits };
        let mut w = Chdatin5rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chdatin5rR {
        Chdatin5rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chdatin5rW) -> &mut Chdatin5rW
    {
        let mut w = Chdatin5rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chdatin5rR {
    bits: u32,
}

impl Chdatin5rR {
    # [ doc = "Bits 16:31 - INDAT1" ]
    pub fn indat1(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:15 - INDAT0" ]
    pub fn indat0(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chdatin5rW {
    bits: u32,
}

impl Chdatin5rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chdatin5rW { bits: 0u32 }
    }
    # [ doc = "Bits 16:31 - INDAT1" ]
    pub fn indat1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:15 - INDAT0" ]
    pub fn indat0(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chcfg6r1 {
    register: ::volatile_register::RW<u32>,
}

impl Chcfg6r1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chcfg6r1R, &'w mut Chcfg6r1W) -> &'w mut Chcfg6r1W
    {
        let bits = self.register.read();
        let r = Chcfg6r1R { bits: bits };
        let mut w = Chcfg6r1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chcfg6r1R {
        Chcfg6r1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chcfg6r1W) -> &mut Chcfg6r1W
    {
        let mut w = Chcfg6r1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg6r1R {
    bits: u32,
}

impl Chcfg6r1R {
    # [ doc = "Bits 14:15 - DATPACK" ]
    pub fn datpack(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:13 - DATMPX" ]
    pub fn datmpx(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 8 - CHINSEL" ]
    pub fn chinsel(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - CHEN" ]
    pub fn chen(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - CKABEN" ]
    pub fn ckaben(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - SCDEN" ]
    pub fn scden(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:3 - SPICKSEL" ]
    pub fn spicksel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:1 - SITP" ]
    pub fn sitp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg6r1W {
    bits: u32,
}

impl Chcfg6r1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chcfg6r1W { bits: 0u32 }
    }
    # [ doc = "Bits 14:15 - DATPACK" ]
    pub fn datpack(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:13 - DATMPX" ]
    pub fn datmpx(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 8 - CHINSEL" ]
    pub fn chinsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - CHEN" ]
    pub fn chen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - CKABEN" ]
    pub fn ckaben(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - SCDEN" ]
    pub fn scden(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:3 - SPICKSEL" ]
    pub fn spicksel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:1 - SITP" ]
    pub fn sitp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chcfg6r2 {
    register: ::volatile_register::RW<u32>,
}

impl Chcfg6r2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chcfg6r2R, &'w mut Chcfg6r2W) -> &'w mut Chcfg6r2W
    {
        let bits = self.register.read();
        let r = Chcfg6r2R { bits: bits };
        let mut w = Chcfg6r2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chcfg6r2R {
        Chcfg6r2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chcfg6r2W) -> &mut Chcfg6r2W
    {
        let mut w = Chcfg6r2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg6r2R {
    bits: u32,
}

impl Chcfg6r2R {
    # [ doc = "Bits 8:31 - OFFSET" ]
    pub fn offset(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 3:7 - DTRBS" ]
    pub fn dtrbs(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg6r2W {
    bits: u32,
}

impl Chcfg6r2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chcfg6r2W { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - OFFSET" ]
    pub fn offset(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 3:7 - DTRBS" ]
    pub fn dtrbs(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Awscd6r {
    register: ::volatile_register::RW<u32>,
}

impl Awscd6r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Awscd6rR, &'w mut Awscd6rW) -> &'w mut Awscd6rW
    {
        let bits = self.register.read();
        let r = Awscd6rR { bits: bits };
        let mut w = Awscd6rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Awscd6rR {
        Awscd6rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Awscd6rW) -> &mut Awscd6rW
    {
        let mut w = Awscd6rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awscd6rR {
    bits: u32,
}

impl Awscd6rR {
    # [ doc = "Bits 22:23 - AWFORD" ]
    pub fn awford(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:20 - AWFOSR" ]
    pub fn awfosr(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - BKSCD" ]
    pub fn bkscd(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - SCDT" ]
    pub fn scdt(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awscd6rW {
    bits: u32,
}

impl Awscd6rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Awscd6rW { bits: 0u32 }
    }
    # [ doc = "Bits 22:23 - AWFORD" ]
    pub fn awford(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:20 - AWFOSR" ]
    pub fn awfosr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - BKSCD" ]
    pub fn bkscd(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - SCDT" ]
    pub fn scdt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chwdat6r {
    register: ::volatile_register::RW<u32>,
}

impl Chwdat6r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chwdat6rR, &'w mut Chwdat6rW) -> &'w mut Chwdat6rW
    {
        let bits = self.register.read();
        let r = Chwdat6rR { bits: bits };
        let mut w = Chwdat6rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chwdat6rR {
        Chwdat6rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chwdat6rW) -> &mut Chwdat6rW
    {
        let mut w = Chwdat6rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chwdat6rR {
    bits: u32,
}

impl Chwdat6rR {
    # [ doc = "Bits 0:15 - WDATA" ]
    pub fn wdata(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chwdat6rW {
    bits: u32,
}

impl Chwdat6rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chwdat6rW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - WDATA" ]
    pub fn wdata(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chdatin6r {
    register: ::volatile_register::RW<u32>,
}

impl Chdatin6r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chdatin6rR, &'w mut Chdatin6rW) -> &'w mut Chdatin6rW
    {
        let bits = self.register.read();
        let r = Chdatin6rR { bits: bits };
        let mut w = Chdatin6rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chdatin6rR {
        Chdatin6rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chdatin6rW) -> &mut Chdatin6rW
    {
        let mut w = Chdatin6rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chdatin6rR {
    bits: u32,
}

impl Chdatin6rR {
    # [ doc = "Bits 16:31 - INDAT1" ]
    pub fn indat1(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:15 - INDAT0" ]
    pub fn indat0(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chdatin6rW {
    bits: u32,
}

impl Chdatin6rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chdatin6rW { bits: 0u32 }
    }
    # [ doc = "Bits 16:31 - INDAT1" ]
    pub fn indat1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:15 - INDAT0" ]
    pub fn indat0(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chcfg7r1 {
    register: ::volatile_register::RW<u32>,
}

impl Chcfg7r1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chcfg7r1R, &'w mut Chcfg7r1W) -> &'w mut Chcfg7r1W
    {
        let bits = self.register.read();
        let r = Chcfg7r1R { bits: bits };
        let mut w = Chcfg7r1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chcfg7r1R {
        Chcfg7r1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chcfg7r1W) -> &mut Chcfg7r1W
    {
        let mut w = Chcfg7r1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg7r1R {
    bits: u32,
}

impl Chcfg7r1R {
    # [ doc = "Bits 14:15 - DATPACK" ]
    pub fn datpack(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:13 - DATMPX" ]
    pub fn datmpx(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 8 - CHINSEL" ]
    pub fn chinsel(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - CHEN" ]
    pub fn chen(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - CKABEN" ]
    pub fn ckaben(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - SCDEN" ]
    pub fn scden(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:3 - SPICKSEL" ]
    pub fn spicksel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:1 - SITP" ]
    pub fn sitp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg7r1W {
    bits: u32,
}

impl Chcfg7r1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chcfg7r1W { bits: 0u32 }
    }
    # [ doc = "Bits 14:15 - DATPACK" ]
    pub fn datpack(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:13 - DATMPX" ]
    pub fn datmpx(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 8 - CHINSEL" ]
    pub fn chinsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - CHEN" ]
    pub fn chen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - CKABEN" ]
    pub fn ckaben(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - SCDEN" ]
    pub fn scden(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:3 - SPICKSEL" ]
    pub fn spicksel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:1 - SITP" ]
    pub fn sitp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chcfg7r2 {
    register: ::volatile_register::RW<u32>,
}

impl Chcfg7r2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chcfg7r2R, &'w mut Chcfg7r2W) -> &'w mut Chcfg7r2W
    {
        let bits = self.register.read();
        let r = Chcfg7r2R { bits: bits };
        let mut w = Chcfg7r2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chcfg7r2R {
        Chcfg7r2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chcfg7r2W) -> &mut Chcfg7r2W
    {
        let mut w = Chcfg7r2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg7r2R {
    bits: u32,
}

impl Chcfg7r2R {
    # [ doc = "Bits 8:31 - OFFSET" ]
    pub fn offset(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 3:7 - DTRBS" ]
    pub fn dtrbs(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chcfg7r2W {
    bits: u32,
}

impl Chcfg7r2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chcfg7r2W { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - OFFSET" ]
    pub fn offset(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 3:7 - DTRBS" ]
    pub fn dtrbs(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Awscd7r {
    register: ::volatile_register::RW<u32>,
}

impl Awscd7r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Awscd7rR, &'w mut Awscd7rW) -> &'w mut Awscd7rW
    {
        let bits = self.register.read();
        let r = Awscd7rR { bits: bits };
        let mut w = Awscd7rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Awscd7rR {
        Awscd7rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Awscd7rW) -> &mut Awscd7rW
    {
        let mut w = Awscd7rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awscd7rR {
    bits: u32,
}

impl Awscd7rR {
    # [ doc = "Bits 22:23 - AWFORD" ]
    pub fn awford(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:20 - AWFOSR" ]
    pub fn awfosr(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - BKSCD" ]
    pub fn bkscd(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - SCDT" ]
    pub fn scdt(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Awscd7rW {
    bits: u32,
}

impl Awscd7rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Awscd7rW { bits: 0u32 }
    }
    # [ doc = "Bits 22:23 - AWFORD" ]
    pub fn awford(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:20 - AWFOSR" ]
    pub fn awfosr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - BKSCD" ]
    pub fn bkscd(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - SCDT" ]
    pub fn scdt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chwdat7r {
    register: ::volatile_register::RW<u32>,
}

impl Chwdat7r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chwdat7rR, &'w mut Chwdat7rW) -> &'w mut Chwdat7rW
    {
        let bits = self.register.read();
        let r = Chwdat7rR { bits: bits };
        let mut w = Chwdat7rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chwdat7rR {
        Chwdat7rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chwdat7rW) -> &mut Chwdat7rW
    {
        let mut w = Chwdat7rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chwdat7rR {
    bits: u32,
}

impl Chwdat7rR {
    # [ doc = "Bits 0:15 - WDATA" ]
    pub fn wdata(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chwdat7rW {
    bits: u32,
}

impl Chwdat7rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chwdat7rW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - WDATA" ]
    pub fn wdata(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Chdatin7r {
    register: ::volatile_register::RW<u32>,
}

impl Chdatin7r {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Chdatin7rR, &'w mut Chdatin7rW) -> &'w mut Chdatin7rW
    {
        let bits = self.register.read();
        let r = Chdatin7rR { bits: bits };
        let mut w = Chdatin7rW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Chdatin7rR {
        Chdatin7rR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Chdatin7rW) -> &mut Chdatin7rW
    {
        let mut w = Chdatin7rW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chdatin7rR {
    bits: u32,
}

impl Chdatin7rR {
    # [ doc = "Bits 16:31 - INDAT1" ]
    pub fn indat1(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:15 - INDAT0" ]
    pub fn indat0(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Chdatin7rW {
    bits: u32,
}

impl Chdatin7rW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Chdatin7rW { bits: 0u32 }
    }
    # [ doc = "Bits 16:31 - INDAT1" ]
    pub fn indat1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:15 - INDAT0" ]
    pub fn indat0(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm0Cr1 {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm0Cr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm0Cr1R, &'w mut Dfsdm0Cr1W) -> &'w mut Dfsdm0Cr1W
    {
        let bits = self.register.read();
        let r = Dfsdm0Cr1R { bits: bits };
        let mut w = Dfsdm0Cr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm0Cr1R {
        Dfsdm0Cr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm0Cr1W) -> &mut Dfsdm0Cr1W
    {
        let mut w = Dfsdm0Cr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0Cr1R {
    bits: u32,
}

impl Dfsdm0Cr1R {
    # [ doc = "Bit 30 - Analog watchdog fast mode select" ]
    pub fn awfsel(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - Fast conversion mode selection for regular conversions" ]
    pub fn fast(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 24:26 - Regular channel selection" ]
    pub fn rch(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 21 - DMA channel enabled to read data for the regular conversion" ]
    pub fn rdmaen(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Launch regular conversion synchronously with DFSDM0" ]
    pub fn rsync(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Continuous mode selection for regular conversions" ]
    pub fn rcont(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Software start of a conversion on the regular channel" ]
    pub fn rswstart(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions" ]
    pub fn jexten(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:10 - Trigger signal selection for launching injected conversions" ]
    pub fn jextsel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 5 - DMA channel enabled to read data for the injected channel group" ]
    pub fn jdmaen(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Scanning conversion mode for injected conversions" ]
    pub fn jscan(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger" ]
    pub fn jsync(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Start a conversion of the injected group of channels" ]
    pub fn jswstart(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - DFSDM enable" ]
    pub fn dfen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0Cr1W {
    bits: u32,
}

impl Dfsdm0Cr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm0Cr1W { bits: 0u32 }
    }
    # [ doc = "Bit 30 - Analog watchdog fast mode select" ]
    pub fn awfsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Fast conversion mode selection for regular conversions" ]
    pub fn fast(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 24:26 - Regular channel selection" ]
    pub fn rch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 21 - DMA channel enabled to read data for the regular conversion" ]
    pub fn rdmaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Launch regular conversion synchronously with DFSDM0" ]
    pub fn rsync(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Continuous mode selection for regular conversions" ]
    pub fn rcont(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Software start of a conversion on the regular channel" ]
    pub fn rswstart(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions" ]
    pub fn jexten(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:10 - Trigger signal selection for launching injected conversions" ]
    pub fn jextsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 5 - DMA channel enabled to read data for the injected channel group" ]
    pub fn jdmaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Scanning conversion mode for injected conversions" ]
    pub fn jscan(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger" ]
    pub fn jsync(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Start a conversion of the injected group of channels" ]
    pub fn jswstart(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - DFSDM enable" ]
    pub fn dfen(&mut self, value: bool) -> &mut Self {
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
pub struct Dfsdm0Cr2 {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm0Cr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm0Cr2R, &'w mut Dfsdm0Cr2W) -> &'w mut Dfsdm0Cr2W
    {
        let bits = self.register.read();
        let r = Dfsdm0Cr2R { bits: bits };
        let mut w = Dfsdm0Cr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm0Cr2R {
        Dfsdm0Cr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm0Cr2W) -> &mut Dfsdm0Cr2W
    {
        let mut w = Dfsdm0Cr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0Cr2R {
    bits: u32,
}

impl Dfsdm0Cr2R {
    # [ doc = "Bits 16:23 - Analog watchdog channel selection" ]
    pub fn awdch(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - Extremes detector channel selection" ]
    pub fn exch(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 6 - Clock absence interrupt enable" ]
    pub fn ckabie(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Short-circuit detector interrupt enable" ]
    pub fn scdie(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Analog watchdog interrupt enable" ]
    pub fn awdie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Regular data overrun interrupt enable" ]
    pub fn rovrie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Injected data overrun interrupt enable" ]
    pub fn jovrie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Regular end of conversion interrupt enable" ]
    pub fn reocie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Injected end of conversion interrupt enable" ]
    pub fn jeocie(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0Cr2W {
    bits: u32,
}

impl Dfsdm0Cr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm0Cr2W { bits: 0u32 }
    }
    # [ doc = "Bits 16:23 - Analog watchdog channel selection" ]
    pub fn awdch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - Extremes detector channel selection" ]
    pub fn exch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 6 - Clock absence interrupt enable" ]
    pub fn ckabie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Short-circuit detector interrupt enable" ]
    pub fn scdie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Analog watchdog interrupt enable" ]
    pub fn awdie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Regular data overrun interrupt enable" ]
    pub fn rovrie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Injected data overrun interrupt enable" ]
    pub fn jovrie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Regular end of conversion interrupt enable" ]
    pub fn reocie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Injected end of conversion interrupt enable" ]
    pub fn jeocie(&mut self, value: bool) -> &mut Self {
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
pub struct Dfsdm0Isr {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm0Isr {
    pub fn read(&self) -> Dfsdm0IsrR {
        Dfsdm0IsrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0IsrR {
    bits: u32,
}

impl Dfsdm0IsrR {
    # [ doc = "Bits 24:31 - short-circuit detector flag" ]
    pub fn scdf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - Clock absence flag" ]
    pub fn ckabf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 14 - Regular conversion in progress status" ]
    pub fn rcip(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Injected conversion in progress status" ]
    pub fn jcip(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Analog watchdog" ]
    pub fn awdf(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Regular conversion overrun flag" ]
    pub fn rovrf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Injected conversion overrun flag" ]
    pub fn jovrf(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - End of regular conversion flag" ]
    pub fn reocf(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - End of injected conversion flag" ]
    pub fn jeocf(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0IsrW {
    bits: u32,
}

impl Dfsdm0IsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm0IsrW { bits: 16711680u32 }
    }
    # [ doc = "Bits 24:31 - short-circuit detector flag" ]
    pub fn scdf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - Clock absence flag" ]
    pub fn ckabf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 14 - Regular conversion in progress status" ]
    pub fn rcip(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Injected conversion in progress status" ]
    pub fn jcip(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Analog watchdog" ]
    pub fn awdf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Regular conversion overrun flag" ]
    pub fn rovrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Injected conversion overrun flag" ]
    pub fn jovrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - End of regular conversion flag" ]
    pub fn reocf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - End of injected conversion flag" ]
    pub fn jeocf(&mut self, value: bool) -> &mut Self {
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
pub struct Dfsdm0Icr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm0Icr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm0IcrR, &'w mut Dfsdm0IcrW) -> &'w mut Dfsdm0IcrW
    {
        let bits = self.register.read();
        let r = Dfsdm0IcrR { bits: bits };
        let mut w = Dfsdm0IcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm0IcrR {
        Dfsdm0IcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm0IcrW) -> &mut Dfsdm0IcrW
    {
        let mut w = Dfsdm0IcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0IcrR {
    bits: u32,
}

impl Dfsdm0IcrR {
    # [ doc = "Bits 24:31 - Clear the short-circuit detector flag" ]
    pub fn clrscdf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - Clear the clock absence flag" ]
    pub fn clrckabf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 3 - Clear the regular conversion overrun flag" ]
    pub fn clrrovrf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Clear the injected conversion overrun flag" ]
    pub fn clrjovrf(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0IcrW {
    bits: u32,
}

impl Dfsdm0IcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm0IcrW { bits: 0u32 }
    }
    # [ doc = "Bits 24:31 - Clear the short-circuit detector flag" ]
    pub fn clrscdf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - Clear the clock absence flag" ]
    pub fn clrckabf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 3 - Clear the regular conversion overrun flag" ]
    pub fn clrrovrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Clear the injected conversion overrun flag" ]
    pub fn clrjovrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm0Jchgr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm0Jchgr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm0JchgrR, &'w mut Dfsdm0JchgrW) -> &'w mut Dfsdm0JchgrW
    {
        let bits = self.register.read();
        let r = Dfsdm0JchgrR { bits: bits };
        let mut w = Dfsdm0JchgrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm0JchgrR {
        Dfsdm0JchgrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm0JchgrW) -> &mut Dfsdm0JchgrW
    {
        let mut w = Dfsdm0JchgrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0JchgrR {
    bits: u32,
}

impl Dfsdm0JchgrR {
    # [ doc = "Bits 0:7 - Injected channel group selection" ]
    pub fn jchg(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0JchgrW {
    bits: u32,
}

impl Dfsdm0JchgrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm0JchgrW { bits: 1u32 }
    }
    # [ doc = "Bits 0:7 - Injected channel group selection" ]
    pub fn jchg(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm0Fcr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm0Fcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm0FcrR, &'w mut Dfsdm0FcrW) -> &'w mut Dfsdm0FcrW
    {
        let bits = self.register.read();
        let r = Dfsdm0FcrR { bits: bits };
        let mut w = Dfsdm0FcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm0FcrR {
        Dfsdm0FcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm0FcrW) -> &mut Dfsdm0FcrW
    {
        let mut w = Dfsdm0FcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0FcrR {
    bits: u32,
}

impl Dfsdm0FcrR {
    # [ doc = "Bits 29:31 - Sinc filter order" ]
    pub fn ford(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)" ]
    pub fn fosr(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:7 - Integrator oversampling ratio (averaging length)" ]
    pub fn iosr(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0FcrW {
    bits: u32,
}

impl Dfsdm0FcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm0FcrW { bits: 0u32 }
    }
    # [ doc = "Bits 29:31 - Sinc filter order" ]
    pub fn ford(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)" ]
    pub fn fosr(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Integrator oversampling ratio (averaging length)" ]
    pub fn iosr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm0Jdatar {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm0Jdatar {
    pub fn read(&self) -> Dfsdm0JdatarR {
        Dfsdm0JdatarR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0JdatarR {
    bits: u32,
}

impl Dfsdm0JdatarR {
    # [ doc = "Bits 8:31 - Injected group conversion data" ]
    pub fn jdata(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:2 - Injected channel most recently converted" ]
    pub fn jdatach(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0JdatarW {
    bits: u32,
}

impl Dfsdm0JdatarW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm0JdatarW { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - Injected group conversion data" ]
    pub fn jdata(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:2 - Injected channel most recently converted" ]
    pub fn jdatach(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm0Rdatar {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm0Rdatar {
    pub fn read(&self) -> Dfsdm0RdatarR {
        Dfsdm0RdatarR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0RdatarR {
    bits: u32,
}

impl Dfsdm0RdatarR {
    # [ doc = "Bits 8:31 - Regular channel conversion data" ]
    pub fn rdata(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bit 4 - Regular channel pending data" ]
    pub fn rpend(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:2 - Regular channel most recently converted" ]
    pub fn rdatach(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0RdatarW {
    bits: u32,
}

impl Dfsdm0RdatarW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm0RdatarW { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - Regular channel conversion data" ]
    pub fn rdata(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 4 - Regular channel pending data" ]
    pub fn rpend(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:2 - Regular channel most recently converted" ]
    pub fn rdatach(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm0Awhtr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm0Awhtr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm0AwhtrR, &'w mut Dfsdm0AwhtrW) -> &'w mut Dfsdm0AwhtrW
    {
        let bits = self.register.read();
        let r = Dfsdm0AwhtrR { bits: bits };
        let mut w = Dfsdm0AwhtrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm0AwhtrR {
        Dfsdm0AwhtrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm0AwhtrW) -> &mut Dfsdm0AwhtrW
    {
        let mut w = Dfsdm0AwhtrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0AwhtrR {
    bits: u32,
}

impl Dfsdm0AwhtrR {
    # [ doc = "Bits 8:31 - Analog watchdog high threshold" ]
    pub fn awht(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event" ]
    pub fn bkawh(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0AwhtrW {
    bits: u32,
}

impl Dfsdm0AwhtrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm0AwhtrW { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - Analog watchdog high threshold" ]
    pub fn awht(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event" ]
    pub fn bkawh(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm0Awltr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm0Awltr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm0AwltrR, &'w mut Dfsdm0AwltrW) -> &'w mut Dfsdm0AwltrW
    {
        let bits = self.register.read();
        let r = Dfsdm0AwltrR { bits: bits };
        let mut w = Dfsdm0AwltrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm0AwltrR {
        Dfsdm0AwltrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm0AwltrW) -> &mut Dfsdm0AwltrW
    {
        let mut w = Dfsdm0AwltrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0AwltrR {
    bits: u32,
}

impl Dfsdm0AwltrR {
    # [ doc = "Bits 8:31 - Analog watchdog low threshold" ]
    pub fn awlt(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event" ]
    pub fn bkawl(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0AwltrW {
    bits: u32,
}

impl Dfsdm0AwltrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm0AwltrW { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - Analog watchdog low threshold" ]
    pub fn awlt(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event" ]
    pub fn bkawl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm0Awsr {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm0Awsr {
    pub fn read(&self) -> Dfsdm0AwsrR {
        Dfsdm0AwsrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0AwsrR {
    bits: u32,
}

impl Dfsdm0AwsrR {
    # [ doc = "Bits 8:15 - Analog watchdog high threshold flag" ]
    pub fn awhtf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - Analog watchdog low threshold flag" ]
    pub fn awltf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0AwsrW {
    bits: u32,
}

impl Dfsdm0AwsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm0AwsrW { bits: 0u32 }
    }
    # [ doc = "Bits 8:15 - Analog watchdog high threshold flag" ]
    pub fn awhtf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Analog watchdog low threshold flag" ]
    pub fn awltf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm0Awcfr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm0Awcfr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm0AwcfrR, &'w mut Dfsdm0AwcfrW) -> &'w mut Dfsdm0AwcfrW
    {
        let bits = self.register.read();
        let r = Dfsdm0AwcfrR { bits: bits };
        let mut w = Dfsdm0AwcfrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm0AwcfrR {
        Dfsdm0AwcfrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm0AwcfrW) -> &mut Dfsdm0AwcfrW
    {
        let mut w = Dfsdm0AwcfrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0AwcfrR {
    bits: u32,
}

impl Dfsdm0AwcfrR {
    # [ doc = "Bits 8:15 - Clear the analog watchdog high threshold flag" ]
    pub fn clrawhtf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - Clear the analog watchdog low threshold flag" ]
    pub fn clrawltf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0AwcfrW {
    bits: u32,
}

impl Dfsdm0AwcfrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm0AwcfrW { bits: 0u32 }
    }
    # [ doc = "Bits 8:15 - Clear the analog watchdog high threshold flag" ]
    pub fn clrawhtf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Clear the analog watchdog low threshold flag" ]
    pub fn clrawltf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm0Exmax {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm0Exmax {
    pub fn read(&self) -> Dfsdm0ExmaxR {
        Dfsdm0ExmaxR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0ExmaxR {
    bits: u32,
}

impl Dfsdm0ExmaxR {
    # [ doc = "Bits 8:31 - Extremes detector maximum value" ]
    pub fn exmax(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:2 - Extremes detector maximum data channel" ]
    pub fn exmaxch(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0ExmaxW {
    bits: u32,
}

impl Dfsdm0ExmaxW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm0ExmaxW { bits: 2147483648u32 }
    }
    # [ doc = "Bits 8:31 - Extremes detector maximum value" ]
    pub fn exmax(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:2 - Extremes detector maximum data channel" ]
    pub fn exmaxch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm0Exmin {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm0Exmin {
    pub fn read(&self) -> Dfsdm0ExminR {
        Dfsdm0ExminR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0ExminR {
    bits: u32,
}

impl Dfsdm0ExminR {
    # [ doc = "Bits 8:31 - EXMIN" ]
    pub fn exmin(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:2 - Extremes detector minimum data channel" ]
    pub fn exminch(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0ExminW {
    bits: u32,
}

impl Dfsdm0ExminW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm0ExminW { bits: 2147483392u32 }
    }
    # [ doc = "Bits 8:31 - EXMIN" ]
    pub fn exmin(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:2 - Extremes detector minimum data channel" ]
    pub fn exminch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm0Cnvtimr {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm0Cnvtimr {
    pub fn read(&self) -> Dfsdm0CnvtimrR {
        Dfsdm0CnvtimrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0CnvtimrR {
    bits: u32,
}

impl Dfsdm0CnvtimrR {
    # [ doc = "Bits 4:31 - 28-bit timer counting conversion time t = CNVCNT[27:0] / fDFSDM_CKIN" ]
    pub fn cnvcnt(&self) -> u32 {
        const MASK: u32 = 268435455;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm0CnvtimrW {
    bits: u32,
}

impl Dfsdm0CnvtimrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm0CnvtimrW { bits: 0u32 }
    }
    # [ doc = "Bits 4:31 - 28-bit timer counting conversion time t = CNVCNT[27:0] / fDFSDM_CKIN" ]
    pub fn cnvcnt(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u32 = 268435455;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm1Cr1 {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm1Cr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm1Cr1R, &'w mut Dfsdm1Cr1W) -> &'w mut Dfsdm1Cr1W
    {
        let bits = self.register.read();
        let r = Dfsdm1Cr1R { bits: bits };
        let mut w = Dfsdm1Cr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm1Cr1R {
        Dfsdm1Cr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm1Cr1W) -> &mut Dfsdm1Cr1W
    {
        let mut w = Dfsdm1Cr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1Cr1R {
    bits: u32,
}

impl Dfsdm1Cr1R {
    # [ doc = "Bit 30 - Analog watchdog fast mode select" ]
    pub fn awfsel(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - Fast conversion mode selection for regular conversions" ]
    pub fn fast(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 24:26 - Regular channel selection" ]
    pub fn rch(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 21 - DMA channel enabled to read data for the regular conversion" ]
    pub fn rdmaen(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Launch regular conversion synchronously with DFSDM0" ]
    pub fn rsync(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Continuous mode selection for regular conversions" ]
    pub fn rcont(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Software start of a conversion on the regular channel" ]
    pub fn rswstart(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions" ]
    pub fn jexten(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:10 - Trigger signal selection for launching injected conversions" ]
    pub fn jextsel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 5 - DMA channel enabled to read data for the injected channel group" ]
    pub fn jdmaen(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Scanning conversion mode for injected conversions" ]
    pub fn jscan(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger" ]
    pub fn jsync(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Start a conversion of the injected group of channels" ]
    pub fn jswstart(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - DFSDM enable" ]
    pub fn dfen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1Cr1W {
    bits: u32,
}

impl Dfsdm1Cr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm1Cr1W { bits: 0u32 }
    }
    # [ doc = "Bit 30 - Analog watchdog fast mode select" ]
    pub fn awfsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Fast conversion mode selection for regular conversions" ]
    pub fn fast(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 24:26 - Regular channel selection" ]
    pub fn rch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 21 - DMA channel enabled to read data for the regular conversion" ]
    pub fn rdmaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Launch regular conversion synchronously with DFSDM0" ]
    pub fn rsync(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Continuous mode selection for regular conversions" ]
    pub fn rcont(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Software start of a conversion on the regular channel" ]
    pub fn rswstart(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions" ]
    pub fn jexten(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:10 - Trigger signal selection for launching injected conversions" ]
    pub fn jextsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 5 - DMA channel enabled to read data for the injected channel group" ]
    pub fn jdmaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Scanning conversion mode for injected conversions" ]
    pub fn jscan(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger" ]
    pub fn jsync(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Start a conversion of the injected group of channels" ]
    pub fn jswstart(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - DFSDM enable" ]
    pub fn dfen(&mut self, value: bool) -> &mut Self {
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
pub struct Dfsdm1Cr2 {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm1Cr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm1Cr2R, &'w mut Dfsdm1Cr2W) -> &'w mut Dfsdm1Cr2W
    {
        let bits = self.register.read();
        let r = Dfsdm1Cr2R { bits: bits };
        let mut w = Dfsdm1Cr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm1Cr2R {
        Dfsdm1Cr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm1Cr2W) -> &mut Dfsdm1Cr2W
    {
        let mut w = Dfsdm1Cr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1Cr2R {
    bits: u32,
}

impl Dfsdm1Cr2R {
    # [ doc = "Bits 16:23 - Analog watchdog channel selection" ]
    pub fn awdch(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - Extremes detector channel selection" ]
    pub fn exch(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 6 - Clock absence interrupt enable" ]
    pub fn ckabie(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Short-circuit detector interrupt enable" ]
    pub fn scdie(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Analog watchdog interrupt enable" ]
    pub fn awdie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Regular data overrun interrupt enable" ]
    pub fn rovrie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Injected data overrun interrupt enable" ]
    pub fn jovrie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Regular end of conversion interrupt enable" ]
    pub fn reocie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Injected end of conversion interrupt enable" ]
    pub fn jeocie(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1Cr2W {
    bits: u32,
}

impl Dfsdm1Cr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm1Cr2W { bits: 0u32 }
    }
    # [ doc = "Bits 16:23 - Analog watchdog channel selection" ]
    pub fn awdch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - Extremes detector channel selection" ]
    pub fn exch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 6 - Clock absence interrupt enable" ]
    pub fn ckabie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Short-circuit detector interrupt enable" ]
    pub fn scdie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Analog watchdog interrupt enable" ]
    pub fn awdie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Regular data overrun interrupt enable" ]
    pub fn rovrie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Injected data overrun interrupt enable" ]
    pub fn jovrie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Regular end of conversion interrupt enable" ]
    pub fn reocie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Injected end of conversion interrupt enable" ]
    pub fn jeocie(&mut self, value: bool) -> &mut Self {
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
pub struct Dfsdm1Isr {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm1Isr {
    pub fn read(&self) -> Dfsdm1IsrR {
        Dfsdm1IsrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1IsrR {
    bits: u32,
}

impl Dfsdm1IsrR {
    # [ doc = "Bits 24:31 - short-circuit detector flag" ]
    pub fn scdf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - Clock absence flag" ]
    pub fn ckabf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 14 - Regular conversion in progress status" ]
    pub fn rcip(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Injected conversion in progress status" ]
    pub fn jcip(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Analog watchdog" ]
    pub fn awdf(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Regular conversion overrun flag" ]
    pub fn rovrf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Injected conversion overrun flag" ]
    pub fn jovrf(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - End of regular conversion flag" ]
    pub fn reocf(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - End of injected conversion flag" ]
    pub fn jeocf(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1IsrW {
    bits: u32,
}

impl Dfsdm1IsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm1IsrW { bits: 16711680u32 }
    }
    # [ doc = "Bits 24:31 - short-circuit detector flag" ]
    pub fn scdf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - Clock absence flag" ]
    pub fn ckabf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 14 - Regular conversion in progress status" ]
    pub fn rcip(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Injected conversion in progress status" ]
    pub fn jcip(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Analog watchdog" ]
    pub fn awdf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Regular conversion overrun flag" ]
    pub fn rovrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Injected conversion overrun flag" ]
    pub fn jovrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - End of regular conversion flag" ]
    pub fn reocf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - End of injected conversion flag" ]
    pub fn jeocf(&mut self, value: bool) -> &mut Self {
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
pub struct Dfsdm1Icr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm1Icr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm1IcrR, &'w mut Dfsdm1IcrW) -> &'w mut Dfsdm1IcrW
    {
        let bits = self.register.read();
        let r = Dfsdm1IcrR { bits: bits };
        let mut w = Dfsdm1IcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm1IcrR {
        Dfsdm1IcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm1IcrW) -> &mut Dfsdm1IcrW
    {
        let mut w = Dfsdm1IcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1IcrR {
    bits: u32,
}

impl Dfsdm1IcrR {
    # [ doc = "Bits 24:31 - Clear the short-circuit detector flag" ]
    pub fn clrscdf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - Clear the clock absence flag" ]
    pub fn clrckabf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 3 - Clear the regular conversion overrun flag" ]
    pub fn clrrovrf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Clear the injected conversion overrun flag" ]
    pub fn clrjovrf(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1IcrW {
    bits: u32,
}

impl Dfsdm1IcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm1IcrW { bits: 0u32 }
    }
    # [ doc = "Bits 24:31 - Clear the short-circuit detector flag" ]
    pub fn clrscdf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - Clear the clock absence flag" ]
    pub fn clrckabf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 3 - Clear the regular conversion overrun flag" ]
    pub fn clrrovrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Clear the injected conversion overrun flag" ]
    pub fn clrjovrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm1Jchgr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm1Jchgr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm1JchgrR, &'w mut Dfsdm1JchgrW) -> &'w mut Dfsdm1JchgrW
    {
        let bits = self.register.read();
        let r = Dfsdm1JchgrR { bits: bits };
        let mut w = Dfsdm1JchgrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm1JchgrR {
        Dfsdm1JchgrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm1JchgrW) -> &mut Dfsdm1JchgrW
    {
        let mut w = Dfsdm1JchgrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1JchgrR {
    bits: u32,
}

impl Dfsdm1JchgrR {
    # [ doc = "Bits 0:7 - Injected channel group selection" ]
    pub fn jchg(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1JchgrW {
    bits: u32,
}

impl Dfsdm1JchgrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm1JchgrW { bits: 1u32 }
    }
    # [ doc = "Bits 0:7 - Injected channel group selection" ]
    pub fn jchg(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm1Fcr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm1Fcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm1FcrR, &'w mut Dfsdm1FcrW) -> &'w mut Dfsdm1FcrW
    {
        let bits = self.register.read();
        let r = Dfsdm1FcrR { bits: bits };
        let mut w = Dfsdm1FcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm1FcrR {
        Dfsdm1FcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm1FcrW) -> &mut Dfsdm1FcrW
    {
        let mut w = Dfsdm1FcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1FcrR {
    bits: u32,
}

impl Dfsdm1FcrR {
    # [ doc = "Bits 29:31 - Sinc filter order" ]
    pub fn ford(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)" ]
    pub fn fosr(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:7 - Integrator oversampling ratio (averaging length)" ]
    pub fn iosr(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1FcrW {
    bits: u32,
}

impl Dfsdm1FcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm1FcrW { bits: 0u32 }
    }
    # [ doc = "Bits 29:31 - Sinc filter order" ]
    pub fn ford(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)" ]
    pub fn fosr(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Integrator oversampling ratio (averaging length)" ]
    pub fn iosr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm1Jdatar {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm1Jdatar {
    pub fn read(&self) -> Dfsdm1JdatarR {
        Dfsdm1JdatarR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1JdatarR {
    bits: u32,
}

impl Dfsdm1JdatarR {
    # [ doc = "Bits 8:31 - Injected group conversion data" ]
    pub fn jdata(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:2 - Injected channel most recently converted" ]
    pub fn jdatach(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1JdatarW {
    bits: u32,
}

impl Dfsdm1JdatarW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm1JdatarW { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - Injected group conversion data" ]
    pub fn jdata(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:2 - Injected channel most recently converted" ]
    pub fn jdatach(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm1Rdatar {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm1Rdatar {
    pub fn read(&self) -> Dfsdm1RdatarR {
        Dfsdm1RdatarR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1RdatarR {
    bits: u32,
}

impl Dfsdm1RdatarR {
    # [ doc = "Bits 8:31 - Regular channel conversion data" ]
    pub fn rdata(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bit 4 - Regular channel pending data" ]
    pub fn rpend(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:2 - Regular channel most recently converted" ]
    pub fn rdatach(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1RdatarW {
    bits: u32,
}

impl Dfsdm1RdatarW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm1RdatarW { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - Regular channel conversion data" ]
    pub fn rdata(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 4 - Regular channel pending data" ]
    pub fn rpend(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:2 - Regular channel most recently converted" ]
    pub fn rdatach(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm1Awhtr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm1Awhtr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm1AwhtrR, &'w mut Dfsdm1AwhtrW) -> &'w mut Dfsdm1AwhtrW
    {
        let bits = self.register.read();
        let r = Dfsdm1AwhtrR { bits: bits };
        let mut w = Dfsdm1AwhtrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm1AwhtrR {
        Dfsdm1AwhtrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm1AwhtrW) -> &mut Dfsdm1AwhtrW
    {
        let mut w = Dfsdm1AwhtrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1AwhtrR {
    bits: u32,
}

impl Dfsdm1AwhtrR {
    # [ doc = "Bits 8:31 - Analog watchdog high threshold" ]
    pub fn awht(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event" ]
    pub fn bkawh(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1AwhtrW {
    bits: u32,
}

impl Dfsdm1AwhtrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm1AwhtrW { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - Analog watchdog high threshold" ]
    pub fn awht(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event" ]
    pub fn bkawh(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm1Awltr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm1Awltr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm1AwltrR, &'w mut Dfsdm1AwltrW) -> &'w mut Dfsdm1AwltrW
    {
        let bits = self.register.read();
        let r = Dfsdm1AwltrR { bits: bits };
        let mut w = Dfsdm1AwltrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm1AwltrR {
        Dfsdm1AwltrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm1AwltrW) -> &mut Dfsdm1AwltrW
    {
        let mut w = Dfsdm1AwltrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1AwltrR {
    bits: u32,
}

impl Dfsdm1AwltrR {
    # [ doc = "Bits 8:31 - Analog watchdog low threshold" ]
    pub fn awlt(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event" ]
    pub fn bkawl(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1AwltrW {
    bits: u32,
}

impl Dfsdm1AwltrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm1AwltrW { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - Analog watchdog low threshold" ]
    pub fn awlt(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event" ]
    pub fn bkawl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm1Awsr {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm1Awsr {
    pub fn read(&self) -> Dfsdm1AwsrR {
        Dfsdm1AwsrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1AwsrR {
    bits: u32,
}

impl Dfsdm1AwsrR {
    # [ doc = "Bits 8:15 - Analog watchdog high threshold flag" ]
    pub fn awhtf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - Analog watchdog low threshold flag" ]
    pub fn awltf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1AwsrW {
    bits: u32,
}

impl Dfsdm1AwsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm1AwsrW { bits: 0u32 }
    }
    # [ doc = "Bits 8:15 - Analog watchdog high threshold flag" ]
    pub fn awhtf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Analog watchdog low threshold flag" ]
    pub fn awltf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm1Awcfr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm1Awcfr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm1AwcfrR, &'w mut Dfsdm1AwcfrW) -> &'w mut Dfsdm1AwcfrW
    {
        let bits = self.register.read();
        let r = Dfsdm1AwcfrR { bits: bits };
        let mut w = Dfsdm1AwcfrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm1AwcfrR {
        Dfsdm1AwcfrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm1AwcfrW) -> &mut Dfsdm1AwcfrW
    {
        let mut w = Dfsdm1AwcfrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1AwcfrR {
    bits: u32,
}

impl Dfsdm1AwcfrR {
    # [ doc = "Bits 8:15 - Clear the analog watchdog high threshold flag" ]
    pub fn clrawhtf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - Clear the analog watchdog low threshold flag" ]
    pub fn clrawltf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1AwcfrW {
    bits: u32,
}

impl Dfsdm1AwcfrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm1AwcfrW { bits: 0u32 }
    }
    # [ doc = "Bits 8:15 - Clear the analog watchdog high threshold flag" ]
    pub fn clrawhtf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Clear the analog watchdog low threshold flag" ]
    pub fn clrawltf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm1Exmax {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm1Exmax {
    pub fn read(&self) -> Dfsdm1ExmaxR {
        Dfsdm1ExmaxR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1ExmaxR {
    bits: u32,
}

impl Dfsdm1ExmaxR {
    # [ doc = "Bits 8:31 - Extremes detector maximum value" ]
    pub fn exmax(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:2 - Extremes detector maximum data channel" ]
    pub fn exmaxch(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1ExmaxW {
    bits: u32,
}

impl Dfsdm1ExmaxW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm1ExmaxW { bits: 2147483648u32 }
    }
    # [ doc = "Bits 8:31 - Extremes detector maximum value" ]
    pub fn exmax(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:2 - Extremes detector maximum data channel" ]
    pub fn exmaxch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm1Exmin {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm1Exmin {
    pub fn read(&self) -> Dfsdm1ExminR {
        Dfsdm1ExminR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1ExminR {
    bits: u32,
}

impl Dfsdm1ExminR {
    # [ doc = "Bits 8:31 - EXMIN" ]
    pub fn exmin(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:2 - Extremes detector minimum data channel" ]
    pub fn exminch(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1ExminW {
    bits: u32,
}

impl Dfsdm1ExminW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm1ExminW { bits: 2147483392u32 }
    }
    # [ doc = "Bits 8:31 - EXMIN" ]
    pub fn exmin(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:2 - Extremes detector minimum data channel" ]
    pub fn exminch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm1Cnvtimr {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm1Cnvtimr {
    pub fn read(&self) -> Dfsdm1CnvtimrR {
        Dfsdm1CnvtimrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1CnvtimrR {
    bits: u32,
}

impl Dfsdm1CnvtimrR {
    # [ doc = "Bits 4:31 - 28-bit timer counting conversion time t = CNVCNT[27:0] / fDFSDM_CKIN" ]
    pub fn cnvcnt(&self) -> u32 {
        const MASK: u32 = 268435455;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm1CnvtimrW {
    bits: u32,
}

impl Dfsdm1CnvtimrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm1CnvtimrW { bits: 0u32 }
    }
    # [ doc = "Bits 4:31 - 28-bit timer counting conversion time t = CNVCNT[27:0] / fDFSDM_CKIN" ]
    pub fn cnvcnt(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u32 = 268435455;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm2Cr1 {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm2Cr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm2Cr1R, &'w mut Dfsdm2Cr1W) -> &'w mut Dfsdm2Cr1W
    {
        let bits = self.register.read();
        let r = Dfsdm2Cr1R { bits: bits };
        let mut w = Dfsdm2Cr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm2Cr1R {
        Dfsdm2Cr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm2Cr1W) -> &mut Dfsdm2Cr1W
    {
        let mut w = Dfsdm2Cr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2Cr1R {
    bits: u32,
}

impl Dfsdm2Cr1R {
    # [ doc = "Bit 30 - Analog watchdog fast mode select" ]
    pub fn awfsel(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - Fast conversion mode selection for regular conversions" ]
    pub fn fast(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 24:26 - Regular channel selection" ]
    pub fn rch(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 21 - DMA channel enabled to read data for the regular conversion" ]
    pub fn rdmaen(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Launch regular conversion synchronously with DFSDM0" ]
    pub fn rsync(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Continuous mode selection for regular conversions" ]
    pub fn rcont(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Software start of a conversion on the regular channel" ]
    pub fn rswstart(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions" ]
    pub fn jexten(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:10 - Trigger signal selection for launching injected conversions" ]
    pub fn jextsel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 5 - DMA channel enabled to read data for the injected channel group" ]
    pub fn jdmaen(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Scanning conversion mode for injected conversions" ]
    pub fn jscan(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger" ]
    pub fn jsync(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Start a conversion of the injected group of channels" ]
    pub fn jswstart(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - DFSDM enable" ]
    pub fn dfen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2Cr1W {
    bits: u32,
}

impl Dfsdm2Cr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm2Cr1W { bits: 0u32 }
    }
    # [ doc = "Bit 30 - Analog watchdog fast mode select" ]
    pub fn awfsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Fast conversion mode selection for regular conversions" ]
    pub fn fast(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 24:26 - Regular channel selection" ]
    pub fn rch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 21 - DMA channel enabled to read data for the regular conversion" ]
    pub fn rdmaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Launch regular conversion synchronously with DFSDM0" ]
    pub fn rsync(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Continuous mode selection for regular conversions" ]
    pub fn rcont(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Software start of a conversion on the regular channel" ]
    pub fn rswstart(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions" ]
    pub fn jexten(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:10 - Trigger signal selection for launching injected conversions" ]
    pub fn jextsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 5 - DMA channel enabled to read data for the injected channel group" ]
    pub fn jdmaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Scanning conversion mode for injected conversions" ]
    pub fn jscan(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger" ]
    pub fn jsync(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Start a conversion of the injected group of channels" ]
    pub fn jswstart(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - DFSDM enable" ]
    pub fn dfen(&mut self, value: bool) -> &mut Self {
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
pub struct Dfsdm2Cr2 {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm2Cr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm2Cr2R, &'w mut Dfsdm2Cr2W) -> &'w mut Dfsdm2Cr2W
    {
        let bits = self.register.read();
        let r = Dfsdm2Cr2R { bits: bits };
        let mut w = Dfsdm2Cr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm2Cr2R {
        Dfsdm2Cr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm2Cr2W) -> &mut Dfsdm2Cr2W
    {
        let mut w = Dfsdm2Cr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2Cr2R {
    bits: u32,
}

impl Dfsdm2Cr2R {
    # [ doc = "Bits 16:23 - Analog watchdog channel selection" ]
    pub fn awdch(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - Extremes detector channel selection" ]
    pub fn exch(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 6 - Clock absence interrupt enable" ]
    pub fn ckabie(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Short-circuit detector interrupt enable" ]
    pub fn scdie(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Analog watchdog interrupt enable" ]
    pub fn awdie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Regular data overrun interrupt enable" ]
    pub fn rovrie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Injected data overrun interrupt enable" ]
    pub fn jovrie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Regular end of conversion interrupt enable" ]
    pub fn reocie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Injected end of conversion interrupt enable" ]
    pub fn jeocie(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2Cr2W {
    bits: u32,
}

impl Dfsdm2Cr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm2Cr2W { bits: 0u32 }
    }
    # [ doc = "Bits 16:23 - Analog watchdog channel selection" ]
    pub fn awdch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - Extremes detector channel selection" ]
    pub fn exch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 6 - Clock absence interrupt enable" ]
    pub fn ckabie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Short-circuit detector interrupt enable" ]
    pub fn scdie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Analog watchdog interrupt enable" ]
    pub fn awdie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Regular data overrun interrupt enable" ]
    pub fn rovrie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Injected data overrun interrupt enable" ]
    pub fn jovrie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Regular end of conversion interrupt enable" ]
    pub fn reocie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Injected end of conversion interrupt enable" ]
    pub fn jeocie(&mut self, value: bool) -> &mut Self {
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
pub struct Dfsdm2Isr {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm2Isr {
    pub fn read(&self) -> Dfsdm2IsrR {
        Dfsdm2IsrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2IsrR {
    bits: u32,
}

impl Dfsdm2IsrR {
    # [ doc = "Bits 24:31 - short-circuit detector flag" ]
    pub fn scdf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - Clock absence flag" ]
    pub fn ckabf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 14 - Regular conversion in progress status" ]
    pub fn rcip(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Injected conversion in progress status" ]
    pub fn jcip(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Analog watchdog" ]
    pub fn awdf(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Regular conversion overrun flag" ]
    pub fn rovrf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Injected conversion overrun flag" ]
    pub fn jovrf(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - End of regular conversion flag" ]
    pub fn reocf(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - End of injected conversion flag" ]
    pub fn jeocf(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2IsrW {
    bits: u32,
}

impl Dfsdm2IsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm2IsrW { bits: 16711680u32 }
    }
    # [ doc = "Bits 24:31 - short-circuit detector flag" ]
    pub fn scdf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - Clock absence flag" ]
    pub fn ckabf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 14 - Regular conversion in progress status" ]
    pub fn rcip(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Injected conversion in progress status" ]
    pub fn jcip(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Analog watchdog" ]
    pub fn awdf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Regular conversion overrun flag" ]
    pub fn rovrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Injected conversion overrun flag" ]
    pub fn jovrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - End of regular conversion flag" ]
    pub fn reocf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - End of injected conversion flag" ]
    pub fn jeocf(&mut self, value: bool) -> &mut Self {
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
pub struct Dfsdm2Icr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm2Icr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm2IcrR, &'w mut Dfsdm2IcrW) -> &'w mut Dfsdm2IcrW
    {
        let bits = self.register.read();
        let r = Dfsdm2IcrR { bits: bits };
        let mut w = Dfsdm2IcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm2IcrR {
        Dfsdm2IcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm2IcrW) -> &mut Dfsdm2IcrW
    {
        let mut w = Dfsdm2IcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2IcrR {
    bits: u32,
}

impl Dfsdm2IcrR {
    # [ doc = "Bits 24:31 - Clear the short-circuit detector flag" ]
    pub fn clrscdf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - Clear the clock absence flag" ]
    pub fn clrckabf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 3 - Clear the regular conversion overrun flag" ]
    pub fn clrrovrf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Clear the injected conversion overrun flag" ]
    pub fn clrjovrf(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2IcrW {
    bits: u32,
}

impl Dfsdm2IcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm2IcrW { bits: 0u32 }
    }
    # [ doc = "Bits 24:31 - Clear the short-circuit detector flag" ]
    pub fn clrscdf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - Clear the clock absence flag" ]
    pub fn clrckabf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 3 - Clear the regular conversion overrun flag" ]
    pub fn clrrovrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Clear the injected conversion overrun flag" ]
    pub fn clrjovrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm2Jchgr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm2Jchgr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm2JchgrR, &'w mut Dfsdm2JchgrW) -> &'w mut Dfsdm2JchgrW
    {
        let bits = self.register.read();
        let r = Dfsdm2JchgrR { bits: bits };
        let mut w = Dfsdm2JchgrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm2JchgrR {
        Dfsdm2JchgrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm2JchgrW) -> &mut Dfsdm2JchgrW
    {
        let mut w = Dfsdm2JchgrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2JchgrR {
    bits: u32,
}

impl Dfsdm2JchgrR {
    # [ doc = "Bits 0:7 - Injected channel group selection" ]
    pub fn jchg(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2JchgrW {
    bits: u32,
}

impl Dfsdm2JchgrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm2JchgrW { bits: 1u32 }
    }
    # [ doc = "Bits 0:7 - Injected channel group selection" ]
    pub fn jchg(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm2Fcr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm2Fcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm2FcrR, &'w mut Dfsdm2FcrW) -> &'w mut Dfsdm2FcrW
    {
        let bits = self.register.read();
        let r = Dfsdm2FcrR { bits: bits };
        let mut w = Dfsdm2FcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm2FcrR {
        Dfsdm2FcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm2FcrW) -> &mut Dfsdm2FcrW
    {
        let mut w = Dfsdm2FcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2FcrR {
    bits: u32,
}

impl Dfsdm2FcrR {
    # [ doc = "Bits 29:31 - Sinc filter order" ]
    pub fn ford(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)" ]
    pub fn fosr(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:7 - Integrator oversampling ratio (averaging length)" ]
    pub fn iosr(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2FcrW {
    bits: u32,
}

impl Dfsdm2FcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm2FcrW { bits: 0u32 }
    }
    # [ doc = "Bits 29:31 - Sinc filter order" ]
    pub fn ford(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)" ]
    pub fn fosr(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Integrator oversampling ratio (averaging length)" ]
    pub fn iosr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm2Jdatar {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm2Jdatar {
    pub fn read(&self) -> Dfsdm2JdatarR {
        Dfsdm2JdatarR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2JdatarR {
    bits: u32,
}

impl Dfsdm2JdatarR {
    # [ doc = "Bits 8:31 - Injected group conversion data" ]
    pub fn jdata(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:2 - Injected channel most recently converted" ]
    pub fn jdatach(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2JdatarW {
    bits: u32,
}

impl Dfsdm2JdatarW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm2JdatarW { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - Injected group conversion data" ]
    pub fn jdata(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:2 - Injected channel most recently converted" ]
    pub fn jdatach(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm2Rdatar {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm2Rdatar {
    pub fn read(&self) -> Dfsdm2RdatarR {
        Dfsdm2RdatarR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2RdatarR {
    bits: u32,
}

impl Dfsdm2RdatarR {
    # [ doc = "Bits 8:31 - Regular channel conversion data" ]
    pub fn rdata(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bit 4 - Regular channel pending data" ]
    pub fn rpend(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:2 - Regular channel most recently converted" ]
    pub fn rdatach(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2RdatarW {
    bits: u32,
}

impl Dfsdm2RdatarW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm2RdatarW { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - Regular channel conversion data" ]
    pub fn rdata(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 4 - Regular channel pending data" ]
    pub fn rpend(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:2 - Regular channel most recently converted" ]
    pub fn rdatach(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm2Awhtr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm2Awhtr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm2AwhtrR, &'w mut Dfsdm2AwhtrW) -> &'w mut Dfsdm2AwhtrW
    {
        let bits = self.register.read();
        let r = Dfsdm2AwhtrR { bits: bits };
        let mut w = Dfsdm2AwhtrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm2AwhtrR {
        Dfsdm2AwhtrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm2AwhtrW) -> &mut Dfsdm2AwhtrW
    {
        let mut w = Dfsdm2AwhtrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2AwhtrR {
    bits: u32,
}

impl Dfsdm2AwhtrR {
    # [ doc = "Bits 8:31 - Analog watchdog high threshold" ]
    pub fn awht(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event" ]
    pub fn bkawh(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2AwhtrW {
    bits: u32,
}

impl Dfsdm2AwhtrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm2AwhtrW { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - Analog watchdog high threshold" ]
    pub fn awht(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event" ]
    pub fn bkawh(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm2Awltr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm2Awltr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm2AwltrR, &'w mut Dfsdm2AwltrW) -> &'w mut Dfsdm2AwltrW
    {
        let bits = self.register.read();
        let r = Dfsdm2AwltrR { bits: bits };
        let mut w = Dfsdm2AwltrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm2AwltrR {
        Dfsdm2AwltrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm2AwltrW) -> &mut Dfsdm2AwltrW
    {
        let mut w = Dfsdm2AwltrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2AwltrR {
    bits: u32,
}

impl Dfsdm2AwltrR {
    # [ doc = "Bits 8:31 - Analog watchdog low threshold" ]
    pub fn awlt(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event" ]
    pub fn bkawl(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2AwltrW {
    bits: u32,
}

impl Dfsdm2AwltrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm2AwltrW { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - Analog watchdog low threshold" ]
    pub fn awlt(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event" ]
    pub fn bkawl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm2Awsr {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm2Awsr {
    pub fn read(&self) -> Dfsdm2AwsrR {
        Dfsdm2AwsrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2AwsrR {
    bits: u32,
}

impl Dfsdm2AwsrR {
    # [ doc = "Bits 8:15 - Analog watchdog high threshold flag" ]
    pub fn awhtf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - Analog watchdog low threshold flag" ]
    pub fn awltf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2AwsrW {
    bits: u32,
}

impl Dfsdm2AwsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm2AwsrW { bits: 0u32 }
    }
    # [ doc = "Bits 8:15 - Analog watchdog high threshold flag" ]
    pub fn awhtf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Analog watchdog low threshold flag" ]
    pub fn awltf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm2Awcfr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm2Awcfr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm2AwcfrR, &'w mut Dfsdm2AwcfrW) -> &'w mut Dfsdm2AwcfrW
    {
        let bits = self.register.read();
        let r = Dfsdm2AwcfrR { bits: bits };
        let mut w = Dfsdm2AwcfrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm2AwcfrR {
        Dfsdm2AwcfrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm2AwcfrW) -> &mut Dfsdm2AwcfrW
    {
        let mut w = Dfsdm2AwcfrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2AwcfrR {
    bits: u32,
}

impl Dfsdm2AwcfrR {
    # [ doc = "Bits 8:15 - Clear the analog watchdog high threshold flag" ]
    pub fn clrawhtf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - Clear the analog watchdog low threshold flag" ]
    pub fn clrawltf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2AwcfrW {
    bits: u32,
}

impl Dfsdm2AwcfrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm2AwcfrW { bits: 0u32 }
    }
    # [ doc = "Bits 8:15 - Clear the analog watchdog high threshold flag" ]
    pub fn clrawhtf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Clear the analog watchdog low threshold flag" ]
    pub fn clrawltf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm2Exmax {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm2Exmax {
    pub fn read(&self) -> Dfsdm2ExmaxR {
        Dfsdm2ExmaxR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2ExmaxR {
    bits: u32,
}

impl Dfsdm2ExmaxR {
    # [ doc = "Bits 8:31 - Extremes detector maximum value" ]
    pub fn exmax(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:2 - Extremes detector maximum data channel" ]
    pub fn exmaxch(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2ExmaxW {
    bits: u32,
}

impl Dfsdm2ExmaxW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm2ExmaxW { bits: 2147483648u32 }
    }
    # [ doc = "Bits 8:31 - Extremes detector maximum value" ]
    pub fn exmax(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:2 - Extremes detector maximum data channel" ]
    pub fn exmaxch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm2Exmin {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm2Exmin {
    pub fn read(&self) -> Dfsdm2ExminR {
        Dfsdm2ExminR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2ExminR {
    bits: u32,
}

impl Dfsdm2ExminR {
    # [ doc = "Bits 8:31 - EXMIN" ]
    pub fn exmin(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:2 - Extremes detector minimum data channel" ]
    pub fn exminch(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2ExminW {
    bits: u32,
}

impl Dfsdm2ExminW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm2ExminW { bits: 2147483392u32 }
    }
    # [ doc = "Bits 8:31 - EXMIN" ]
    pub fn exmin(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:2 - Extremes detector minimum data channel" ]
    pub fn exminch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm2Cnvtimr {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm2Cnvtimr {
    pub fn read(&self) -> Dfsdm2CnvtimrR {
        Dfsdm2CnvtimrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2CnvtimrR {
    bits: u32,
}

impl Dfsdm2CnvtimrR {
    # [ doc = "Bits 4:31 - 28-bit timer counting conversion time t = CNVCNT[27:0] / fDFSDM_CKIN" ]
    pub fn cnvcnt(&self) -> u32 {
        const MASK: u32 = 268435455;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm2CnvtimrW {
    bits: u32,
}

impl Dfsdm2CnvtimrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm2CnvtimrW { bits: 0u32 }
    }
    # [ doc = "Bits 4:31 - 28-bit timer counting conversion time t = CNVCNT[27:0] / fDFSDM_CKIN" ]
    pub fn cnvcnt(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u32 = 268435455;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm3Cr1 {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm3Cr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm3Cr1R, &'w mut Dfsdm3Cr1W) -> &'w mut Dfsdm3Cr1W
    {
        let bits = self.register.read();
        let r = Dfsdm3Cr1R { bits: bits };
        let mut w = Dfsdm3Cr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm3Cr1R {
        Dfsdm3Cr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm3Cr1W) -> &mut Dfsdm3Cr1W
    {
        let mut w = Dfsdm3Cr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3Cr1R {
    bits: u32,
}

impl Dfsdm3Cr1R {
    # [ doc = "Bit 30 - Analog watchdog fast mode select" ]
    pub fn awfsel(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - Fast conversion mode selection for regular conversions" ]
    pub fn fast(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 24:26 - Regular channel selection" ]
    pub fn rch(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 21 - DMA channel enabled to read data for the regular conversion" ]
    pub fn rdmaen(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Launch regular conversion synchronously with DFSDM0" ]
    pub fn rsync(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Continuous mode selection for regular conversions" ]
    pub fn rcont(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Software start of a conversion on the regular channel" ]
    pub fn rswstart(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions" ]
    pub fn jexten(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:10 - Trigger signal selection for launching injected conversions" ]
    pub fn jextsel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 5 - DMA channel enabled to read data for the injected channel group" ]
    pub fn jdmaen(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Scanning conversion mode for injected conversions" ]
    pub fn jscan(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger" ]
    pub fn jsync(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Start a conversion of the injected group of channels" ]
    pub fn jswstart(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - DFSDM enable" ]
    pub fn dfen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3Cr1W {
    bits: u32,
}

impl Dfsdm3Cr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm3Cr1W { bits: 0u32 }
    }
    # [ doc = "Bit 30 - Analog watchdog fast mode select" ]
    pub fn awfsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Fast conversion mode selection for regular conversions" ]
    pub fn fast(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 24:26 - Regular channel selection" ]
    pub fn rch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 21 - DMA channel enabled to read data for the regular conversion" ]
    pub fn rdmaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Launch regular conversion synchronously with DFSDM0" ]
    pub fn rsync(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Continuous mode selection for regular conversions" ]
    pub fn rcont(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Software start of a conversion on the regular channel" ]
    pub fn rswstart(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 13:14 - Trigger enable and trigger edge selection for injected conversions" ]
    pub fn jexten(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:10 - Trigger signal selection for launching injected conversions" ]
    pub fn jextsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 5 - DMA channel enabled to read data for the injected channel group" ]
    pub fn jdmaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Scanning conversion mode for injected conversions" ]
    pub fn jscan(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Launch an injected conversion synchronously with the DFSDM0 JSWSTART trigger" ]
    pub fn jsync(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Start a conversion of the injected group of channels" ]
    pub fn jswstart(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - DFSDM enable" ]
    pub fn dfen(&mut self, value: bool) -> &mut Self {
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
pub struct Dfsdm3Cr2 {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm3Cr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm3Cr2R, &'w mut Dfsdm3Cr2W) -> &'w mut Dfsdm3Cr2W
    {
        let bits = self.register.read();
        let r = Dfsdm3Cr2R { bits: bits };
        let mut w = Dfsdm3Cr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm3Cr2R {
        Dfsdm3Cr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm3Cr2W) -> &mut Dfsdm3Cr2W
    {
        let mut w = Dfsdm3Cr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3Cr2R {
    bits: u32,
}

impl Dfsdm3Cr2R {
    # [ doc = "Bits 16:23 - Analog watchdog channel selection" ]
    pub fn awdch(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - Extremes detector channel selection" ]
    pub fn exch(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 6 - Clock absence interrupt enable" ]
    pub fn ckabie(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Short-circuit detector interrupt enable" ]
    pub fn scdie(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Analog watchdog interrupt enable" ]
    pub fn awdie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Regular data overrun interrupt enable" ]
    pub fn rovrie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Injected data overrun interrupt enable" ]
    pub fn jovrie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Regular end of conversion interrupt enable" ]
    pub fn reocie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Injected end of conversion interrupt enable" ]
    pub fn jeocie(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3Cr2W {
    bits: u32,
}

impl Dfsdm3Cr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm3Cr2W { bits: 0u32 }
    }
    # [ doc = "Bits 16:23 - Analog watchdog channel selection" ]
    pub fn awdch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - Extremes detector channel selection" ]
    pub fn exch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 6 - Clock absence interrupt enable" ]
    pub fn ckabie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Short-circuit detector interrupt enable" ]
    pub fn scdie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Analog watchdog interrupt enable" ]
    pub fn awdie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Regular data overrun interrupt enable" ]
    pub fn rovrie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Injected data overrun interrupt enable" ]
    pub fn jovrie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Regular end of conversion interrupt enable" ]
    pub fn reocie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Injected end of conversion interrupt enable" ]
    pub fn jeocie(&mut self, value: bool) -> &mut Self {
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
pub struct Dfsdm3Isr {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm3Isr {
    pub fn read(&self) -> Dfsdm3IsrR {
        Dfsdm3IsrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3IsrR {
    bits: u32,
}

impl Dfsdm3IsrR {
    # [ doc = "Bits 24:31 - short-circuit detector flag" ]
    pub fn scdf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - Clock absence flag" ]
    pub fn ckabf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 14 - Regular conversion in progress status" ]
    pub fn rcip(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Injected conversion in progress status" ]
    pub fn jcip(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Analog watchdog" ]
    pub fn awdf(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Regular conversion overrun flag" ]
    pub fn rovrf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Injected conversion overrun flag" ]
    pub fn jovrf(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - End of regular conversion flag" ]
    pub fn reocf(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - End of injected conversion flag" ]
    pub fn jeocf(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3IsrW {
    bits: u32,
}

impl Dfsdm3IsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm3IsrW { bits: 16711680u32 }
    }
    # [ doc = "Bits 24:31 - short-circuit detector flag" ]
    pub fn scdf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - Clock absence flag" ]
    pub fn ckabf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 14 - Regular conversion in progress status" ]
    pub fn rcip(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Injected conversion in progress status" ]
    pub fn jcip(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Analog watchdog" ]
    pub fn awdf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Regular conversion overrun flag" ]
    pub fn rovrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Injected conversion overrun flag" ]
    pub fn jovrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - End of regular conversion flag" ]
    pub fn reocf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - End of injected conversion flag" ]
    pub fn jeocf(&mut self, value: bool) -> &mut Self {
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
pub struct Dfsdm3Icr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm3Icr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm3IcrR, &'w mut Dfsdm3IcrW) -> &'w mut Dfsdm3IcrW
    {
        let bits = self.register.read();
        let r = Dfsdm3IcrR { bits: bits };
        let mut w = Dfsdm3IcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm3IcrR {
        Dfsdm3IcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm3IcrW) -> &mut Dfsdm3IcrW
    {
        let mut w = Dfsdm3IcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3IcrR {
    bits: u32,
}

impl Dfsdm3IcrR {
    # [ doc = "Bits 24:31 - Clear the short-circuit detector flag" ]
    pub fn clrscdf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - Clear the clock absence flag" ]
    pub fn clrckabf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 3 - Clear the regular conversion overrun flag" ]
    pub fn clrrovrf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Clear the injected conversion overrun flag" ]
    pub fn clrjovrf(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3IcrW {
    bits: u32,
}

impl Dfsdm3IcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm3IcrW { bits: 0u32 }
    }
    # [ doc = "Bits 24:31 - Clear the short-circuit detector flag" ]
    pub fn clrscdf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - Clear the clock absence flag" ]
    pub fn clrckabf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 3 - Clear the regular conversion overrun flag" ]
    pub fn clrrovrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Clear the injected conversion overrun flag" ]
    pub fn clrjovrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm3Jchgr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm3Jchgr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm3JchgrR, &'w mut Dfsdm3JchgrW) -> &'w mut Dfsdm3JchgrW
    {
        let bits = self.register.read();
        let r = Dfsdm3JchgrR { bits: bits };
        let mut w = Dfsdm3JchgrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm3JchgrR {
        Dfsdm3JchgrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm3JchgrW) -> &mut Dfsdm3JchgrW
    {
        let mut w = Dfsdm3JchgrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3JchgrR {
    bits: u32,
}

impl Dfsdm3JchgrR {
    # [ doc = "Bits 0:7 - Injected channel group selection" ]
    pub fn jchg(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3JchgrW {
    bits: u32,
}

impl Dfsdm3JchgrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm3JchgrW { bits: 1u32 }
    }
    # [ doc = "Bits 0:7 - Injected channel group selection" ]
    pub fn jchg(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm3Fcr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm3Fcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm3FcrR, &'w mut Dfsdm3FcrW) -> &'w mut Dfsdm3FcrW
    {
        let bits = self.register.read();
        let r = Dfsdm3FcrR { bits: bits };
        let mut w = Dfsdm3FcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm3FcrR {
        Dfsdm3FcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm3FcrW) -> &mut Dfsdm3FcrW
    {
        let mut w = Dfsdm3FcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3FcrR {
    bits: u32,
}

impl Dfsdm3FcrR {
    # [ doc = "Bits 29:31 - Sinc filter order" ]
    pub fn ford(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)" ]
    pub fn fosr(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:7 - Integrator oversampling ratio (averaging length)" ]
    pub fn iosr(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3FcrW {
    bits: u32,
}

impl Dfsdm3FcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm3FcrW { bits: 0u32 }
    }
    # [ doc = "Bits 29:31 - Sinc filter order" ]
    pub fn ford(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:25 - Sinc filter oversampling ratio (decimation rate)" ]
    pub fn fosr(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Integrator oversampling ratio (averaging length)" ]
    pub fn iosr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm3Jdatar {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm3Jdatar {
    pub fn read(&self) -> Dfsdm3JdatarR {
        Dfsdm3JdatarR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3JdatarR {
    bits: u32,
}

impl Dfsdm3JdatarR {
    # [ doc = "Bits 8:31 - Injected group conversion data" ]
    pub fn jdata(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:2 - Injected channel most recently converted" ]
    pub fn jdatach(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3JdatarW {
    bits: u32,
}

impl Dfsdm3JdatarW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm3JdatarW { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - Injected group conversion data" ]
    pub fn jdata(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:2 - Injected channel most recently converted" ]
    pub fn jdatach(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm3Rdatar {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm3Rdatar {
    pub fn read(&self) -> Dfsdm3RdatarR {
        Dfsdm3RdatarR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3RdatarR {
    bits: u32,
}

impl Dfsdm3RdatarR {
    # [ doc = "Bits 8:31 - Regular channel conversion data" ]
    pub fn rdata(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bit 4 - Regular channel pending data" ]
    pub fn rpend(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:2 - Regular channel most recently converted" ]
    pub fn rdatach(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3RdatarW {
    bits: u32,
}

impl Dfsdm3RdatarW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm3RdatarW { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - Regular channel conversion data" ]
    pub fn rdata(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 4 - Regular channel pending data" ]
    pub fn rpend(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:2 - Regular channel most recently converted" ]
    pub fn rdatach(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm3Awhtr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm3Awhtr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm3AwhtrR, &'w mut Dfsdm3AwhtrW) -> &'w mut Dfsdm3AwhtrW
    {
        let bits = self.register.read();
        let r = Dfsdm3AwhtrR { bits: bits };
        let mut w = Dfsdm3AwhtrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm3AwhtrR {
        Dfsdm3AwhtrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm3AwhtrW) -> &mut Dfsdm3AwhtrW
    {
        let mut w = Dfsdm3AwhtrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3AwhtrR {
    bits: u32,
}

impl Dfsdm3AwhtrR {
    # [ doc = "Bits 8:31 - Analog watchdog high threshold" ]
    pub fn awht(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event" ]
    pub fn bkawh(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3AwhtrW {
    bits: u32,
}

impl Dfsdm3AwhtrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm3AwhtrW { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - Analog watchdog high threshold" ]
    pub fn awht(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - Break signal assignment to analog watchdog high threshold event" ]
    pub fn bkawh(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm3Awltr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm3Awltr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm3AwltrR, &'w mut Dfsdm3AwltrW) -> &'w mut Dfsdm3AwltrW
    {
        let bits = self.register.read();
        let r = Dfsdm3AwltrR { bits: bits };
        let mut w = Dfsdm3AwltrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm3AwltrR {
        Dfsdm3AwltrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm3AwltrW) -> &mut Dfsdm3AwltrW
    {
        let mut w = Dfsdm3AwltrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3AwltrR {
    bits: u32,
}

impl Dfsdm3AwltrR {
    # [ doc = "Bits 8:31 - Analog watchdog low threshold" ]
    pub fn awlt(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event" ]
    pub fn bkawl(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3AwltrW {
    bits: u32,
}

impl Dfsdm3AwltrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm3AwltrW { bits: 0u32 }
    }
    # [ doc = "Bits 8:31 - Analog watchdog low threshold" ]
    pub fn awlt(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - Break signal assignment to analog watchdog low threshold event" ]
    pub fn bkawl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm3Awsr {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm3Awsr {
    pub fn read(&self) -> Dfsdm3AwsrR {
        Dfsdm3AwsrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3AwsrR {
    bits: u32,
}

impl Dfsdm3AwsrR {
    # [ doc = "Bits 8:15 - Analog watchdog high threshold flag" ]
    pub fn awhtf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - Analog watchdog low threshold flag" ]
    pub fn awltf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3AwsrW {
    bits: u32,
}

impl Dfsdm3AwsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm3AwsrW { bits: 0u32 }
    }
    # [ doc = "Bits 8:15 - Analog watchdog high threshold flag" ]
    pub fn awhtf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Analog watchdog low threshold flag" ]
    pub fn awltf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm3Awcfr {
    register: ::volatile_register::RW<u32>,
}

impl Dfsdm3Awcfr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dfsdm3AwcfrR, &'w mut Dfsdm3AwcfrW) -> &'w mut Dfsdm3AwcfrW
    {
        let bits = self.register.read();
        let r = Dfsdm3AwcfrR { bits: bits };
        let mut w = Dfsdm3AwcfrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dfsdm3AwcfrR {
        Dfsdm3AwcfrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dfsdm3AwcfrW) -> &mut Dfsdm3AwcfrW
    {
        let mut w = Dfsdm3AwcfrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3AwcfrR {
    bits: u32,
}

impl Dfsdm3AwcfrR {
    # [ doc = "Bits 8:15 - Clear the analog watchdog high threshold flag" ]
    pub fn clrawhtf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - Clear the analog watchdog low threshold flag" ]
    pub fn clrawltf(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3AwcfrW {
    bits: u32,
}

impl Dfsdm3AwcfrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm3AwcfrW { bits: 0u32 }
    }
    # [ doc = "Bits 8:15 - Clear the analog watchdog high threshold flag" ]
    pub fn clrawhtf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Clear the analog watchdog low threshold flag" ]
    pub fn clrawltf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm3Exmax {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm3Exmax {
    pub fn read(&self) -> Dfsdm3ExmaxR {
        Dfsdm3ExmaxR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3ExmaxR {
    bits: u32,
}

impl Dfsdm3ExmaxR {
    # [ doc = "Bits 8:31 - Extremes detector maximum value" ]
    pub fn exmax(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:2 - Extremes detector maximum data channel" ]
    pub fn exmaxch(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3ExmaxW {
    bits: u32,
}

impl Dfsdm3ExmaxW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm3ExmaxW { bits: 2147483648u32 }
    }
    # [ doc = "Bits 8:31 - Extremes detector maximum value" ]
    pub fn exmax(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:2 - Extremes detector maximum data channel" ]
    pub fn exmaxch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm3Exmin {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm3Exmin {
    pub fn read(&self) -> Dfsdm3ExminR {
        Dfsdm3ExminR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3ExminR {
    bits: u32,
}

impl Dfsdm3ExminR {
    # [ doc = "Bits 8:31 - EXMIN" ]
    pub fn exmin(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 0:2 - Extremes detector minimum data channel" ]
    pub fn exminch(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3ExminW {
    bits: u32,
}

impl Dfsdm3ExminW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm3ExminW { bits: 2147483392u32 }
    }
    # [ doc = "Bits 8:31 - EXMIN" ]
    pub fn exmin(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:2 - Extremes detector minimum data channel" ]
    pub fn exminch(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dfsdm3Cnvtimr {
    register: ::volatile_register::RO<u32>,
}

impl Dfsdm3Cnvtimr {
    pub fn read(&self) -> Dfsdm3CnvtimrR {
        Dfsdm3CnvtimrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3CnvtimrR {
    bits: u32,
}

impl Dfsdm3CnvtimrR {
    # [ doc = "Bits 4:31 - 28-bit timer counting conversion time t = CNVCNT[27:0] / fDFSDM_CKIN" ]
    pub fn cnvcnt(&self) -> u32 {
        const MASK: u32 = 268435455;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dfsdm3CnvtimrW {
    bits: u32,
}

impl Dfsdm3CnvtimrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dfsdm3CnvtimrW { bits: 0u32 }
    }
    # [ doc = "Bits 4:31 - 28-bit timer counting conversion time t = CNVCNT[27:0] / fDFSDM_CKIN" ]
    pub fn cnvcnt(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u32 = 268435455;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
