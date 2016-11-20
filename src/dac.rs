# [ doc = "Digital-to-analog converter" ]
# [ repr ( C ) ]
pub struct Dac {
    # [ doc = "0x00 - control register" ]
    pub cr: Cr,
    # [ doc = "0x04 - software trigger register" ]
    pub swtrigr: Swtrigr,
    # [ doc = "0x08 - channel1 12-bit right-aligned data holding register" ]
    pub dhr12r1: Dhr12r1,
    # [ doc = "0x0c - channel1 12-bit left-aligned data holding register" ]
    pub dhr12l1: Dhr12l1,
    # [ doc = "0x10 - channel1 8-bit right-aligned data holding register" ]
    pub dhr8r1: Dhr8r1,
    # [ doc = "0x14 - channel2 12-bit right aligned data holding register" ]
    pub dhr12r2: Dhr12r2,
    # [ doc = "0x18 - channel2 12-bit left aligned data holding register" ]
    pub dhr12l2: Dhr12l2,
    # [ doc = "0x1c - channel2 8-bit right-aligned data holding register" ]
    pub dhr8r2: Dhr8r2,
    # [ doc = "0x20 - Dual DAC 12-bit right-aligned data holding register" ]
    pub dhr12rd: Dhr12rd,
    # [ doc = "0x24 - DUAL DAC 12-bit left aligned data holding register" ]
    pub dhr12ld: Dhr12ld,
    # [ doc = "0x28 - DUAL DAC 8-bit right aligned data holding register" ]
    pub dhr8rd: Dhr8rd,
    # [ doc = "0x2c - channel1 data output register" ]
    pub dor1: Dor1,
    # [ doc = "0x30 - channel2 data output register" ]
    pub dor2: Dor2,
    # [ doc = "0x34 - status register" ]
    pub sr: Sr,
    # [ doc = "0x38 - calibration control register" ]
    pub ccr: Ccr,
    # [ doc = "0x3c - mode control register" ]
    pub mcr: Mcr,
    # [ doc = "0x40 - Sample and Hold sample time register 1" ]
    pub shsr1: Shsr1,
    # [ doc = "0x44 - Sample and Hold sample time register 2" ]
    pub shsr2: Shsr2,
    # [ doc = "0x48 - Sample and Hold hold time register" ]
    pub shhr: Shhr,
    # [ doc = "0x4c - Sample and Hold refresh time register" ]
    pub shrr: Shrr,
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
    # [ doc = "Bit 0 - DAC channel1 enable" ]
    pub fn en1(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - DAC channel1 trigger enable" ]
    pub fn ten1(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 3:5 - DAC channel1 trigger selection" ]
    pub fn tsel1(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable" ]
    pub fn wave1(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - DAC channel1 mask/amplitude selector" ]
    pub fn mamp1(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 12 - DAC channel1 DMA enable" ]
    pub fn dmaen1(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable" ]
    pub fn dmaudrie1(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - DAC Channel 1 calibration enable" ]
    pub fn cen1(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - DAC channel2 enable" ]
    pub fn en2(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - DAC channel2 trigger enable" ]
    pub fn ten2(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 19:21 - DAC channel2 trigger selection" ]
    pub fn tsel2(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable" ]
    pub fn wave2(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - DAC channel2 mask/amplitude selector" ]
    pub fn mamp2(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 28 - DAC channel2 DMA enable" ]
    pub fn dmaen2(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable" ]
    pub fn dmaudrie2(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - DAC Channel 2 calibration enable" ]
    pub fn cen2(&self) -> bool {
        const OFFSET: u8 = 30u8;
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
    # [ doc = "Bit 0 - DAC channel1 enable" ]
    pub fn en1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - DAC channel1 trigger enable" ]
    pub fn ten1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 3:5 - DAC channel1 trigger selection" ]
    pub fn tsel1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 6:7 - DAC channel1 noise/triangle wave generation enable" ]
    pub fn wave1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - DAC channel1 mask/amplitude selector" ]
    pub fn mamp1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 12 - DAC channel1 DMA enable" ]
    pub fn dmaen1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - DAC channel1 DMA Underrun Interrupt enable" ]
    pub fn dmaudrie1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - DAC Channel 1 calibration enable" ]
    pub fn cen1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - DAC channel2 enable" ]
    pub fn en2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - DAC channel2 trigger enable" ]
    pub fn ten2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 19:21 - DAC channel2 trigger selection" ]
    pub fn tsel2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:23 - DAC channel2 noise/triangle wave generation enable" ]
    pub fn wave2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - DAC channel2 mask/amplitude selector" ]
    pub fn mamp2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 28 - DAC channel2 DMA enable" ]
    pub fn dmaen2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - DAC channel2 DMA underrun interrupt enable" ]
    pub fn dmaudrie2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - DAC Channel 2 calibration enable" ]
    pub fn cen2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Swtrigr {
    register: ::volatile_register::WO<u32>,
}

impl Swtrigr {
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut SwtrigrW) -> &mut SwtrigrW
    {
        let mut w = SwtrigrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SwtrigrR {
    bits: u32,
}

impl SwtrigrR {
    # [ doc = "Bit 0 - DAC channel1 software trigger" ]
    pub fn swtrig1(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - DAC channel2 software trigger" ]
    pub fn swtrig2(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SwtrigrW {
    bits: u32,
}

impl SwtrigrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        SwtrigrW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - DAC channel1 software trigger" ]
    pub fn swtrig1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - DAC channel2 software trigger" ]
    pub fn swtrig2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Dhr12r1 {
    register: ::volatile_register::RW<u32>,
}

impl Dhr12r1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dhr12r1R, &'w mut Dhr12r1W) -> &'w mut Dhr12r1W
    {
        let bits = self.register.read();
        let r = Dhr12r1R { bits: bits };
        let mut w = Dhr12r1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dhr12r1R {
        Dhr12r1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dhr12r1W) -> &mut Dhr12r1W
    {
        let mut w = Dhr12r1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dhr12r1R {
    bits: u32,
}

impl Dhr12r1R {
    # [ doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data" ]
    pub fn dacc1dhr(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dhr12r1W {
    bits: u32,
}

impl Dhr12r1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dhr12r1W { bits: 0u32 }
    }
    # [ doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data" ]
    pub fn dacc1dhr(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dhr12l1 {
    register: ::volatile_register::RW<u32>,
}

impl Dhr12l1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dhr12l1R, &'w mut Dhr12l1W) -> &'w mut Dhr12l1W
    {
        let bits = self.register.read();
        let r = Dhr12l1R { bits: bits };
        let mut w = Dhr12l1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dhr12l1R {
        Dhr12l1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dhr12l1W) -> &mut Dhr12l1W
    {
        let mut w = Dhr12l1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dhr12l1R {
    bits: u32,
}

impl Dhr12l1R {
    # [ doc = "Bits 4:15 - DAC channel1 12-bit left-aligned data" ]
    pub fn dacc1dhr(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dhr12l1W {
    bits: u32,
}

impl Dhr12l1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dhr12l1W { bits: 0u32 }
    }
    # [ doc = "Bits 4:15 - DAC channel1 12-bit left-aligned data" ]
    pub fn dacc1dhr(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dhr8r1 {
    register: ::volatile_register::RW<u32>,
}

impl Dhr8r1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dhr8r1R, &'w mut Dhr8r1W) -> &'w mut Dhr8r1W
    {
        let bits = self.register.read();
        let r = Dhr8r1R { bits: bits };
        let mut w = Dhr8r1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dhr8r1R {
        Dhr8r1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dhr8r1W) -> &mut Dhr8r1W
    {
        let mut w = Dhr8r1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dhr8r1R {
    bits: u32,
}

impl Dhr8r1R {
    # [ doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data" ]
    pub fn dacc1dhr(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dhr8r1W {
    bits: u32,
}

impl Dhr8r1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dhr8r1W { bits: 0u32 }
    }
    # [ doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data" ]
    pub fn dacc1dhr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dhr12r2 {
    register: ::volatile_register::RW<u32>,
}

impl Dhr12r2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dhr12r2R, &'w mut Dhr12r2W) -> &'w mut Dhr12r2W
    {
        let bits = self.register.read();
        let r = Dhr12r2R { bits: bits };
        let mut w = Dhr12r2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dhr12r2R {
        Dhr12r2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dhr12r2W) -> &mut Dhr12r2W
    {
        let mut w = Dhr12r2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dhr12r2R {
    bits: u32,
}

impl Dhr12r2R {
    # [ doc = "Bits 0:11 - DAC channel2 12-bit right-aligned data" ]
    pub fn dacc2dhr(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dhr12r2W {
    bits: u32,
}

impl Dhr12r2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dhr12r2W { bits: 0u32 }
    }
    # [ doc = "Bits 0:11 - DAC channel2 12-bit right-aligned data" ]
    pub fn dacc2dhr(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dhr12l2 {
    register: ::volatile_register::RW<u32>,
}

impl Dhr12l2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dhr12l2R, &'w mut Dhr12l2W) -> &'w mut Dhr12l2W
    {
        let bits = self.register.read();
        let r = Dhr12l2R { bits: bits };
        let mut w = Dhr12l2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dhr12l2R {
        Dhr12l2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dhr12l2W) -> &mut Dhr12l2W
    {
        let mut w = Dhr12l2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dhr12l2R {
    bits: u32,
}

impl Dhr12l2R {
    # [ doc = "Bits 4:15 - DAC channel2 12-bit left-aligned data" ]
    pub fn dacc2dhr(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dhr12l2W {
    bits: u32,
}

impl Dhr12l2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dhr12l2W { bits: 0u32 }
    }
    # [ doc = "Bits 4:15 - DAC channel2 12-bit left-aligned data" ]
    pub fn dacc2dhr(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dhr8r2 {
    register: ::volatile_register::RW<u32>,
}

impl Dhr8r2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dhr8r2R, &'w mut Dhr8r2W) -> &'w mut Dhr8r2W
    {
        let bits = self.register.read();
        let r = Dhr8r2R { bits: bits };
        let mut w = Dhr8r2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dhr8r2R {
        Dhr8r2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dhr8r2W) -> &mut Dhr8r2W
    {
        let mut w = Dhr8r2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dhr8r2R {
    bits: u32,
}

impl Dhr8r2R {
    # [ doc = "Bits 0:7 - DAC channel2 8-bit right-aligned data" ]
    pub fn dacc2dhr(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dhr8r2W {
    bits: u32,
}

impl Dhr8r2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dhr8r2W { bits: 0u32 }
    }
    # [ doc = "Bits 0:7 - DAC channel2 8-bit right-aligned data" ]
    pub fn dacc2dhr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dhr12rd {
    register: ::volatile_register::RW<u32>,
}

impl Dhr12rd {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dhr12rdR, &'w mut Dhr12rdW) -> &'w mut Dhr12rdW
    {
        let bits = self.register.read();
        let r = Dhr12rdR { bits: bits };
        let mut w = Dhr12rdW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dhr12rdR {
        Dhr12rdR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dhr12rdW) -> &mut Dhr12rdW
    {
        let mut w = Dhr12rdW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dhr12rdR {
    bits: u32,
}

impl Dhr12rdR {
    # [ doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data" ]
    pub fn dacc1dhr(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:27 - DAC channel2 12-bit right-aligned data" ]
    pub fn dacc2dhr(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dhr12rdW {
    bits: u32,
}

impl Dhr12rdW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dhr12rdW { bits: 0u32 }
    }
    # [ doc = "Bits 0:11 - DAC channel1 12-bit right-aligned data" ]
    pub fn dacc1dhr(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:27 - DAC channel2 12-bit right-aligned data" ]
    pub fn dacc2dhr(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dhr12ld {
    register: ::volatile_register::RW<u32>,
}

impl Dhr12ld {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dhr12ldR, &'w mut Dhr12ldW) -> &'w mut Dhr12ldW
    {
        let bits = self.register.read();
        let r = Dhr12ldR { bits: bits };
        let mut w = Dhr12ldW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dhr12ldR {
        Dhr12ldR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dhr12ldW) -> &mut Dhr12ldW
    {
        let mut w = Dhr12ldW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dhr12ldR {
    bits: u32,
}

impl Dhr12ldR {
    # [ doc = "Bits 4:15 - DAC channel1 12-bit left-aligned data" ]
    pub fn dacc1dhr(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 20:31 - DAC channel2 12-bit left-aligned data" ]
    pub fn dacc2dhr(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dhr12ldW {
    bits: u32,
}

impl Dhr12ldW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dhr12ldW { bits: 0u32 }
    }
    # [ doc = "Bits 4:15 - DAC channel1 12-bit left-aligned data" ]
    pub fn dacc1dhr(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:31 - DAC channel2 12-bit left-aligned data" ]
    pub fn dacc2dhr(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dhr8rd {
    register: ::volatile_register::RW<u32>,
}

impl Dhr8rd {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dhr8rdR, &'w mut Dhr8rdW) -> &'w mut Dhr8rdW
    {
        let bits = self.register.read();
        let r = Dhr8rdR { bits: bits };
        let mut w = Dhr8rdW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dhr8rdR {
        Dhr8rdR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dhr8rdW) -> &mut Dhr8rdW
    {
        let mut w = Dhr8rdW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dhr8rdR {
    bits: u32,
}

impl Dhr8rdR {
    # [ doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data" ]
    pub fn dacc1dhr(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - DAC channel2 8-bit right-aligned data" ]
    pub fn dacc2dhr(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dhr8rdW {
    bits: u32,
}

impl Dhr8rdW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dhr8rdW { bits: 0u32 }
    }
    # [ doc = "Bits 0:7 - DAC channel1 8-bit right-aligned data" ]
    pub fn dacc1dhr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - DAC channel2 8-bit right-aligned data" ]
    pub fn dacc2dhr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dor1 {
    register: ::volatile_register::RO<u32>,
}

impl Dor1 {
    pub fn read(&self) -> Dor1R {
        Dor1R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dor1R {
    bits: u32,
}

impl Dor1R {
    # [ doc = "Bits 0:11 - DAC channel1 data output" ]
    pub fn dacc1dor(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dor1W {
    bits: u32,
}

impl Dor1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dor1W { bits: 0u32 }
    }
    # [ doc = "Bits 0:11 - DAC channel1 data output" ]
    pub fn dacc1dor(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dor2 {
    register: ::volatile_register::RO<u32>,
}

impl Dor2 {
    pub fn read(&self) -> Dor2R {
        Dor2R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dor2R {
    bits: u32,
}

impl Dor2R {
    # [ doc = "Bits 0:11 - DAC channel2 data output" ]
    pub fn dacc2dor(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dor2W {
    bits: u32,
}

impl Dor2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dor2W { bits: 0u32 }
    }
    # [ doc = "Bits 0:11 - DAC channel2 data output" ]
    pub fn dacc2dor(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
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
    # [ doc = "Bit 13 - DAC channel1 DMA underrun flag" ]
    pub fn dmaudr1(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - DAC Channel 1 calibration offset status" ]
    pub fn cal_flag1(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - DAC Channel 1 busy writing sample time flag" ]
    pub fn bwst1(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - DAC channel2 DMA underrun flag" ]
    pub fn dmaudr2(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - DAC Channel 2 calibration offset status" ]
    pub fn cal_flag2(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - DAC Channel 2 busy writing sample time flag" ]
    pub fn bwst2(&self) -> bool {
        const OFFSET: u8 = 31u8;
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
    # [ doc = "Bit 13 - DAC channel1 DMA underrun flag" ]
    pub fn dmaudr1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - DAC channel2 DMA underrun flag" ]
    pub fn dmaudr2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
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
    # [ doc = "Bits 0:4 - DAC Channel 1 offset trimming value" ]
    pub fn otrim1(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:20 - DAC Channel 2 offset trimming value" ]
    pub fn otrim2(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
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
    # [ doc = "Bits 0:4 - DAC Channel 1 offset trimming value" ]
    pub fn otrim1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:20 - DAC Channel 2 offset trimming value" ]
    pub fn otrim2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Mcr {
    register: ::volatile_register::RW<u32>,
}

impl Mcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&McrR, &'w mut McrW) -> &'w mut McrW
    {
        let bits = self.register.read();
        let r = McrR { bits: bits };
        let mut w = McrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> McrR {
        McrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut McrW) -> &mut McrW
    {
        let mut w = McrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct McrR {
    bits: u32,
}

impl McrR {
    # [ doc = "Bits 0:2 - DAC Channel 1 mode" ]
    pub fn mode1(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:18 - DAC Channel 2 mode" ]
    pub fn mode2(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct McrW {
    bits: u32,
}

impl McrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        McrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:2 - DAC Channel 1 mode" ]
    pub fn mode1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:18 - DAC Channel 2 mode" ]
    pub fn mode2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Shsr1 {
    register: ::volatile_register::RW<u32>,
}

impl Shsr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Shsr1R, &'w mut Shsr1W) -> &'w mut Shsr1W
    {
        let bits = self.register.read();
        let r = Shsr1R { bits: bits };
        let mut w = Shsr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Shsr1R {
        Shsr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Shsr1W) -> &mut Shsr1W
    {
        let mut w = Shsr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Shsr1R {
    bits: u32,
}

impl Shsr1R {
    # [ doc = "Bits 0:9 - DAC Channel 1 sample Time" ]
    pub fn tsample1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Shsr1W {
    bits: u32,
}

impl Shsr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Shsr1W { bits: 0u32 }
    }
    # [ doc = "Bits 0:9 - DAC Channel 1 sample Time" ]
    pub fn tsample1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Shsr2 {
    register: ::volatile_register::RW<u32>,
}

impl Shsr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Shsr2R, &'w mut Shsr2W) -> &'w mut Shsr2W
    {
        let bits = self.register.read();
        let r = Shsr2R { bits: bits };
        let mut w = Shsr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Shsr2R {
        Shsr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Shsr2W) -> &mut Shsr2W
    {
        let mut w = Shsr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Shsr2R {
    bits: u32,
}

impl Shsr2R {
    # [ doc = "Bits 0:9 - DAC Channel 2 sample Time" ]
    pub fn tsample2(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Shsr2W {
    bits: u32,
}

impl Shsr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Shsr2W { bits: 0u32 }
    }
    # [ doc = "Bits 0:9 - DAC Channel 2 sample Time" ]
    pub fn tsample2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Shhr {
    register: ::volatile_register::RW<u32>,
}

impl Shhr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ShhrR, &'w mut ShhrW) -> &'w mut ShhrW
    {
        let bits = self.register.read();
        let r = ShhrR { bits: bits };
        let mut w = ShhrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ShhrR {
        ShhrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ShhrW) -> &mut ShhrW
    {
        let mut w = ShhrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ShhrR {
    bits: u32,
}

impl ShhrR {
    # [ doc = "Bits 0:9 - DAC Channel 1 hold Time" ]
    pub fn thold1(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:25 - DAC Channel 2 hold time" ]
    pub fn thold2(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ShhrW {
    bits: u32,
}

impl ShhrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ShhrW { bits: 65537u32 }
    }
    # [ doc = "Bits 0:9 - DAC Channel 1 hold Time" ]
    pub fn thold1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:25 - DAC Channel 2 hold time" ]
    pub fn thold2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Shrr {
    register: ::volatile_register::RW<u32>,
}

impl Shrr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ShrrR, &'w mut ShrrW) -> &'w mut ShrrW
    {
        let bits = self.register.read();
        let r = ShrrR { bits: bits };
        let mut w = ShrrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ShrrR {
        ShrrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ShrrW) -> &mut ShrrW
    {
        let mut w = ShrrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ShrrR {
    bits: u32,
}

impl ShrrR {
    # [ doc = "Bits 0:7 - DAC Channel 1 refresh Time" ]
    pub fn trefresh1(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - DAC Channel 2 refresh Time" ]
    pub fn trefresh2(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ShrrW {
    bits: u32,
}

impl ShrrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ShrrW { bits: 1u32 }
    }
    # [ doc = "Bits 0:7 - DAC Channel 1 refresh Time" ]
    pub fn trefresh1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - DAC Channel 2 refresh Time" ]
    pub fn trefresh2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
