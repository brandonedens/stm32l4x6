# [ doc = "General purpose timers" ]
# [ repr ( C ) ]
pub struct General1ChannelTimer {
    # [ doc = "0x00 - control register 1" ]
    pub cr1: Cr1,
    # [ doc = "0x04 - control register 2" ]
    pub cr2: Cr2,
    _reserved0: [u8; 4usize],
    # [ doc = "0x0c - DMA/Interrupt enable register" ]
    pub dier: Dier,
    # [ doc = "0x10 - status register" ]
    pub sr: Sr,
    # [ doc = "0x14 - event generation register" ]
    pub egr: Egr,
    # [ doc = "0x18 - capture/compare mode register (output mode)" ]
    pub ccmr1_output: Ccmr1Output,
    _reserved1: [u8; 4usize],
    # [ doc = "0x20 - capture/compare enable register" ]
    pub ccer: Ccer,
    # [ doc = "0x24 - counter" ]
    pub cnt: Cnt,
    # [ doc = "0x28 - prescaler" ]
    pub psc: Psc,
    # [ doc = "0x2c - auto-reload register" ]
    pub arr: Arr,
    # [ doc = "0x30 - repetition counter register" ]
    pub rcr: Rcr,
    # [ doc = "0x34 - capture/compare register 1" ]
    pub ccr1: Ccr1,
    _reserved2: [u8; 12usize],
    # [ doc = "0x44 - break and dead-time register" ]
    pub bdtr: Bdtr,
    # [ doc = "0x48 - DMA control register" ]
    pub dcr: Dcr,
    # [ doc = "0x4c - DMA address for full transfer" ]
    pub dmar: Dmar,
    # [ doc = "0x50 - TIM16 option register 1" ]
    pub or1: Or1,
    _reserved3: [u8; 12usize],
    # [ doc = "0x60 - TIM17 option register 1" ]
    pub or2: Or2,
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
    # [ doc = "Bit 0 - Counter enable" ]
    pub fn cen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Update disable" ]
    pub fn udis(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Update request source" ]
    pub fn urs(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - One-pulse mode" ]
    pub fn opm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Auto-reload preload enable" ]
    pub fn arpe(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:9 - Clock division" ]
    pub fn ckd(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 11 - UIF status bit remapping" ]
    pub fn uifremap(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
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
        Cr1W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Counter enable" ]
    pub fn cen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Update disable" ]
    pub fn udis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Update request source" ]
    pub fn urs(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - One-pulse mode" ]
    pub fn opm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Auto-reload preload enable" ]
    pub fn arpe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 8:9 - Clock division" ]
    pub fn ckd(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - UIF status bit remapping" ]
    pub fn uifremap(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
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
    # [ doc = "Bit 9 - Output Idle state 1" ]
    pub fn ois1n(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Output Idle state 1" ]
    pub fn ois1(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Capture/compare DMA selection" ]
    pub fn ccds(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Capture/compare control update selection" ]
    pub fn ccus(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Capture/compare preloaded control" ]
    pub fn ccpc(&self) -> bool {
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
    # [ doc = "Bit 9 - Output Idle state 1" ]
    pub fn ois1n(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Output Idle state 1" ]
    pub fn ois1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Capture/compare DMA selection" ]
    pub fn ccds(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Capture/compare control update selection" ]
    pub fn ccus(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Capture/compare preloaded control" ]
    pub fn ccpc(&mut self, value: bool) -> &mut Self {
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
pub struct Dier {
    register: ::volatile_register::RW<u32>,
}

impl Dier {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DierR, &'w mut DierW) -> &'w mut DierW
    {
        let bits = self.register.read();
        let r = DierR { bits: bits };
        let mut w = DierW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DierR {
        DierR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DierW) -> &mut DierW
    {
        let mut w = DierW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DierR {
    bits: u32,
}

impl DierR {
    # [ doc = "Bit 14 - Trigger DMA request enable" ]
    pub fn tde(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - COM DMA request enable" ]
    pub fn comde(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Capture/Compare 1 DMA request enable" ]
    pub fn cc1de(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Update DMA request enable" ]
    pub fn ude(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Break interrupt enable" ]
    pub fn bie(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Trigger interrupt enable" ]
    pub fn tie(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - COM interrupt enable" ]
    pub fn comie(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Capture/Compare 1 interrupt enable" ]
    pub fn cc1ie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Update interrupt enable" ]
    pub fn uie(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DierW {
    bits: u32,
}

impl DierW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DierW { bits: 0u32 }
    }
    # [ doc = "Bit 14 - Trigger DMA request enable" ]
    pub fn tde(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - COM DMA request enable" ]
    pub fn comde(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Capture/Compare 1 DMA request enable" ]
    pub fn cc1de(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Update DMA request enable" ]
    pub fn ude(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Break interrupt enable" ]
    pub fn bie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Trigger interrupt enable" ]
    pub fn tie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - COM interrupt enable" ]
    pub fn comie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Capture/Compare 1 interrupt enable" ]
    pub fn cc1ie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Update interrupt enable" ]
    pub fn uie(&mut self, value: bool) -> &mut Self {
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
    # [ doc = "Bit 9 - Capture/Compare 1 overcapture flag" ]
    pub fn cc1of(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Break interrupt flag" ]
    pub fn bif(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Trigger interrupt flag" ]
    pub fn tif(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - COM interrupt flag" ]
    pub fn comif(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Capture/compare 1 interrupt flag" ]
    pub fn cc1if(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Update interrupt flag" ]
    pub fn uif(&self) -> bool {
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
    # [ doc = "Bit 9 - Capture/Compare 1 overcapture flag" ]
    pub fn cc1of(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Break interrupt flag" ]
    pub fn bif(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Trigger interrupt flag" ]
    pub fn tif(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - COM interrupt flag" ]
    pub fn comif(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Capture/compare 1 interrupt flag" ]
    pub fn cc1if(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Update interrupt flag" ]
    pub fn uif(&mut self, value: bool) -> &mut Self {
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
pub struct Egr {
    register: ::volatile_register::WO<u32>,
}

impl Egr {
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut EgrW) -> &mut EgrW
    {
        let mut w = EgrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct EgrR {
    bits: u32,
}

impl EgrR {
    # [ doc = "Bit 7 - Break generation" ]
    pub fn bg(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Trigger generation" ]
    pub fn tg(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Capture/Compare control update generation" ]
    pub fn comg(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Capture/compare 1 generation" ]
    pub fn cc1g(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Update generation" ]
    pub fn ug(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct EgrW {
    bits: u32,
}

impl EgrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        EgrW { bits: 0u32 }
    }
    # [ doc = "Bit 7 - Break generation" ]
    pub fn bg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Trigger generation" ]
    pub fn tg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Capture/Compare control update generation" ]
    pub fn comg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Capture/compare 1 generation" ]
    pub fn cc1g(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Update generation" ]
    pub fn ug(&mut self, value: bool) -> &mut Self {
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
pub struct Ccmr1Output {
    register: ::volatile_register::RW<u32>,
}

impl Ccmr1Output {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ccmr1OutputR, &'w mut Ccmr1OutputW) -> &'w mut Ccmr1OutputW
    {
        let bits = self.register.read();
        let r = Ccmr1OutputR { bits: bits };
        let mut w = Ccmr1OutputW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ccmr1OutputR {
        Ccmr1OutputR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ccmr1OutputW) -> &mut Ccmr1OutputW
    {
        let mut w = Ccmr1OutputW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccmr1OutputR {
    bits: u32,
}

impl Ccmr1OutputR {
    # [ doc = "Bit 16 - Output Compare 1 mode" ]
    pub fn oc1m_2(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:6 - Output Compare 1 mode" ]
    pub fn oc1m(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 3 - Output Compare 1 preload enable" ]
    pub fn oc1pe(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Output Compare 1 fast enable" ]
    pub fn oc1fe(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
    pub fn cc1s(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccmr1OutputW {
    bits: u32,
}

impl Ccmr1OutputW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ccmr1OutputW { bits: 0u32 }
    }
    # [ doc = "Bit 16 - Output Compare 1 mode" ]
    pub fn oc1m_2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:6 - Output Compare 1 mode" ]
    pub fn oc1m(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 3 - Output Compare 1 preload enable" ]
    pub fn oc1pe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Output Compare 1 fast enable" ]
    pub fn oc1fe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
    pub fn cc1s(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ccmr1Input {
    register: ::volatile_register::RW<u32>,
}

impl Ccmr1Input {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ccmr1InputR, &'w mut Ccmr1InputW) -> &'w mut Ccmr1InputW
    {
        let bits = self.register.read();
        let r = Ccmr1InputR { bits: bits };
        let mut w = Ccmr1InputW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ccmr1InputR {
        Ccmr1InputR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ccmr1InputW) -> &mut Ccmr1InputW
    {
        let mut w = Ccmr1InputW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccmr1InputR {
    bits: u32,
}

impl Ccmr1InputR {
    # [ doc = "Bits 4:7 - Input capture 1 filter" ]
    pub fn ic1f(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 2:3 - Input capture 1 prescaler" ]
    pub fn ic1psc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
    pub fn cc1s(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccmr1InputW {
    bits: u32,
}

impl Ccmr1InputW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ccmr1InputW { bits: 0u32 }
    }
    # [ doc = "Bits 4:7 - Input capture 1 filter" ]
    pub fn ic1f(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 2:3 - Input capture 1 prescaler" ]
    pub fn ic1psc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:1 - Capture/Compare 1 selection" ]
    pub fn cc1s(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ccer {
    register: ::volatile_register::RW<u32>,
}

impl Ccer {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CcerR, &'w mut CcerW) -> &'w mut CcerW
    {
        let bits = self.register.read();
        let r = CcerR { bits: bits };
        let mut w = CcerW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CcerR {
        CcerR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CcerW) -> &mut CcerW
    {
        let mut w = CcerW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CcerR {
    bits: u32,
}

impl CcerR {
    # [ doc = "Bit 3 - Capture/Compare 1 output Polarity" ]
    pub fn cc1np(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Capture/Compare 1 complementary output enable" ]
    pub fn cc1ne(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Capture/Compare 1 output Polarity" ]
    pub fn cc1p(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Capture/Compare 1 output enable" ]
    pub fn cc1e(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CcerW {
    bits: u32,
}

impl CcerW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CcerW { bits: 0u32 }
    }
    # [ doc = "Bit 3 - Capture/Compare 1 output Polarity" ]
    pub fn cc1np(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Capture/Compare 1 complementary output enable" ]
    pub fn cc1ne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Capture/Compare 1 output Polarity" ]
    pub fn cc1p(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Capture/Compare 1 output enable" ]
    pub fn cc1e(&mut self, value: bool) -> &mut Self {
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
pub struct Cnt {
    register: ::volatile_register::RW<u32>,
}

impl Cnt {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CntR, &'w mut CntW) -> &'w mut CntW
    {
        let bits = self.register.read();
        let r = CntR { bits: bits };
        let mut w = CntW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CntR {
        CntR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CntW) -> &mut CntW
    {
        let mut w = CntW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CntR {
    bits: u32,
}

impl CntR {
    # [ doc = "Bits 0:15 - counter value" ]
    pub fn cnt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 31 - UIF Copy" ]
    pub fn uifcpy(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CntW {
    bits: u32,
}

impl CntW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CntW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - counter value" ]
    pub fn cnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Psc {
    register: ::volatile_register::RW<u32>,
}

impl Psc {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PscR, &'w mut PscW) -> &'w mut PscW
    {
        let bits = self.register.read();
        let r = PscR { bits: bits };
        let mut w = PscW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PscR {
        PscR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PscW) -> &mut PscW
    {
        let mut w = PscW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PscR {
    bits: u32,
}

impl PscR {
    # [ doc = "Bits 0:15 - Prescaler value" ]
    pub fn psc(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PscW {
    bits: u32,
}

impl PscW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PscW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Prescaler value" ]
    pub fn psc(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Arr {
    register: ::volatile_register::RW<u32>,
}

impl Arr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ArrR, &'w mut ArrW) -> &'w mut ArrW
    {
        let bits = self.register.read();
        let r = ArrR { bits: bits };
        let mut w = ArrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ArrR {
        ArrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ArrW) -> &mut ArrW
    {
        let mut w = ArrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ArrR {
    bits: u32,
}

impl ArrR {
    # [ doc = "Bits 0:15 - Auto-reload value" ]
    pub fn arr(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ArrW {
    bits: u32,
}

impl ArrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ArrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Auto-reload value" ]
    pub fn arr(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Rcr {
    register: ::volatile_register::RW<u32>,
}

impl Rcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&RcrR, &'w mut RcrW) -> &'w mut RcrW
    {
        let bits = self.register.read();
        let r = RcrR { bits: bits };
        let mut w = RcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> RcrR {
        RcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut RcrW) -> &mut RcrW
    {
        let mut w = RcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RcrR {
    bits: u32,
}

impl RcrR {
    # [ doc = "Bits 0:7 - Repetition counter value" ]
    pub fn rep(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RcrW {
    bits: u32,
}

impl RcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        RcrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:7 - Repetition counter value" ]
    pub fn rep(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ccr1 {
    register: ::volatile_register::RW<u32>,
}

impl Ccr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ccr1R, &'w mut Ccr1W) -> &'w mut Ccr1W
    {
        let bits = self.register.read();
        let r = Ccr1R { bits: bits };
        let mut w = Ccr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ccr1R {
        Ccr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ccr1W) -> &mut Ccr1W
    {
        let mut w = Ccr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr1R {
    bits: u32,
}

impl Ccr1R {
    # [ doc = "Bits 0:15 - Capture/Compare 1 value" ]
    pub fn ccr1(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr1W {
    bits: u32,
}

impl Ccr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ccr1W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Capture/Compare 1 value" ]
    pub fn ccr1(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bdtr {
    register: ::volatile_register::RW<u32>,
}

impl Bdtr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BdtrR, &'w mut BdtrW) -> &'w mut BdtrW
    {
        let bits = self.register.read();
        let r = BdtrR { bits: bits };
        let mut w = BdtrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BdtrR {
        BdtrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BdtrW) -> &mut BdtrW
    {
        let mut w = BdtrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BdtrR {
    bits: u32,
}

impl BdtrR {
    # [ doc = "Bits 0:7 - Dead-time generator setup" ]
    pub fn dtg(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:9 - Lock configuration" ]
    pub fn lock(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 10 - Off-state selection for Idle mode" ]
    pub fn ossi(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Off-state selection for Run mode" ]
    pub fn ossr(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Break enable" ]
    pub fn bke(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Break polarity" ]
    pub fn bkp(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Automatic output enable" ]
    pub fn aoe(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Main output enable" ]
    pub fn moe(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 16:19 - Break filter" ]
    pub fn bkf(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BdtrW {
    bits: u32,
}

impl BdtrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BdtrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:7 - Dead-time generator setup" ]
    pub fn dtg(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:9 - Lock configuration" ]
    pub fn lock(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Off-state selection for Idle mode" ]
    pub fn ossi(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Off-state selection for Run mode" ]
    pub fn ossr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Break enable" ]
    pub fn bke(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Break polarity" ]
    pub fn bkp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Automatic output enable" ]
    pub fn aoe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Main output enable" ]
    pub fn moe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 16:19 - Break filter" ]
    pub fn bkf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dcr {
    register: ::volatile_register::RW<u32>,
}

impl Dcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DcrR, &'w mut DcrW) -> &'w mut DcrW
    {
        let bits = self.register.read();
        let r = DcrR { bits: bits };
        let mut w = DcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DcrR {
        DcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DcrW) -> &mut DcrW
    {
        let mut w = DcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DcrR {
    bits: u32,
}

impl DcrR {
    # [ doc = "Bits 8:12 - DMA burst length" ]
    pub fn dbl(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:4 - DMA base address" ]
    pub fn dba(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DcrW {
    bits: u32,
}

impl DcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DcrW { bits: 0u32 }
    }
    # [ doc = "Bits 8:12 - DMA burst length" ]
    pub fn dbl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:4 - DMA base address" ]
    pub fn dba(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dmar {
    register: ::volatile_register::RW<u32>,
}

impl Dmar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DmarR, &'w mut DmarW) -> &'w mut DmarW
    {
        let bits = self.register.read();
        let r = DmarR { bits: bits };
        let mut w = DmarW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DmarR {
        DmarR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DmarW) -> &mut DmarW
    {
        let mut w = DmarW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DmarR {
    bits: u32,
}

impl DmarR {
    # [ doc = "Bits 0:15 - DMA register for burst accesses" ]
    pub fn dmab(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DmarW {
    bits: u32,
}

impl DmarW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DmarW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - DMA register for burst accesses" ]
    pub fn dmab(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Or1 {
    register: ::volatile_register::RW<u32>,
}

impl Or1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Or1R, &'w mut Or1W) -> &'w mut Or1W
    {
        let bits = self.register.read();
        let r = Or1R { bits: bits };
        let mut w = Or1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Or1R {
        Or1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Or1W) -> &mut Or1W
    {
        let mut w = Or1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Or1R {
    bits: u32,
}

impl Or1R {
    # [ doc = "Bits 0:1 - Input capture 1 remap" ]
    pub fn ti1_rmp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Or1W {
    bits: u32,
}

impl Or1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Or1W { bits: 0u32 }
    }
    # [ doc = "Bits 0:1 - Input capture 1 remap" ]
    pub fn ti1_rmp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Or2 {
    register: ::volatile_register::RW<u32>,
}

impl Or2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Or2R, &'w mut Or2W) -> &'w mut Or2W
    {
        let bits = self.register.read();
        let r = Or2R { bits: bits };
        let mut w = Or2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Or2R {
        Or2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Or2W) -> &mut Or2W
    {
        let mut w = Or2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Or2R {
    bits: u32,
}

impl Or2R {
    # [ doc = "Bit 0 - BRK BKIN input enable" ]
    pub fn bkine(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - BRK COMP1 enable" ]
    pub fn bkcmp1e(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - BRK COMP2 enable" ]
    pub fn bkcmp2e(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - BRK DFSDM_BREAK1 enable" ]
    pub fn bkdfbk1e(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - BRK BKIN input polarity" ]
    pub fn bkinp(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - BRK COMP1 input polarity" ]
    pub fn bkcmp1p(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - BRK COMP2 input polarit" ]
    pub fn bkcmp2p(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Or2W {
    bits: u32,
}

impl Or2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Or2W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - BRK BKIN input enable" ]
    pub fn bkine(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - BRK COMP1 enable" ]
    pub fn bkcmp1e(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - BRK COMP2 enable" ]
    pub fn bkcmp2e(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - BRK DFSDM_BREAK1 enable" ]
    pub fn bkdfbk1e(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - BRK BKIN input polarity" ]
    pub fn bkinp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - BRK COMP1 input polarity" ]
    pub fn bkcmp1p(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - BRK COMP2 input polarit" ]
    pub fn bkcmp2p(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}
