# [ doc = "Advanced-timers" ]
# [ repr ( C ) ]
pub struct AdvancedTimer {
    # [ doc = "0x00 - control register 1" ]
    pub cr1: Cr1,
    # [ doc = "0x04 - control register 2" ]
    pub cr2: Cr2,
    # [ doc = "0x08 - slave mode control register" ]
    pub smcr: Smcr,
    # [ doc = "0x0c - DMA/Interrupt enable register" ]
    pub dier: Dier,
    # [ doc = "0x10 - status register" ]
    pub sr: Sr,
    # [ doc = "0x14 - event generation register" ]
    pub egr: Egr,
    # [ doc = "0x18 - capture/compare mode register 1 (output mode)" ]
    pub ccmr1_output: Ccmr1Output,
    # [ doc = "0x1c - capture/compare mode register 2 (output mode)" ]
    pub ccmr2_output: Ccmr2Output,
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
    # [ doc = "0x38 - capture/compare register 2" ]
    pub ccr2: Ccr2,
    # [ doc = "0x3c - capture/compare register 3" ]
    pub ccr3: Ccr3,
    # [ doc = "0x40 - capture/compare register 4" ]
    pub ccr4: Ccr4,
    # [ doc = "0x44 - break and dead-time register" ]
    pub bdtr: Bdtr,
    # [ doc = "0x48 - DMA control register" ]
    pub dcr: Dcr,
    # [ doc = "0x4c - DMA address for full transfer" ]
    pub dmar: Dmar,
    # [ doc = "0x50 - DMA address for full transfer" ]
    pub or1: Or1,
    # [ doc = "0x54 - capture/compare mode register 2 (output mode)" ]
    pub ccmr3_output: Ccmr3Output,
    # [ doc = "0x58 - capture/compare register 4" ]
    pub ccr5: Ccr5,
    # [ doc = "0x5c - capture/compare register 4" ]
    pub ccr6: Ccr6,
    # [ doc = "0x60 - DMA address for full transfer" ]
    pub or2: Or2,
    # [ doc = "0x64 - DMA address for full transfer" ]
    pub or3: Or3,
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
    # [ doc = "Bits 8:9 - Clock division" ]
    pub fn ckd(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Auto-reload preload enable" ]
    pub fn arpe(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 5:6 - Center-aligned mode selection" ]
    pub fn cms(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 5u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 4 - Direction" ]
    pub fn dir(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - One-pulse mode" ]
    pub fn opm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Update request source" ]
    pub fn urs(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Update disable" ]
    pub fn udis(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Counter enable" ]
    pub fn cen(&self) -> bool {
        const OFFSET: u8 = 0u8;
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
    # [ doc = "Bits 8:9 - Clock division" ]
    pub fn ckd(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
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
    # [ doc = "Bits 5:6 - Center-aligned mode selection" ]
    pub fn cms(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 5u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 4 - Direction" ]
    pub fn dir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
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
    # [ doc = "Bit 14 - Output Idle state 4" ]
    pub fn ois4(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Output Idle state 3" ]
    pub fn ois3n(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Output Idle state 3" ]
    pub fn ois3(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Output Idle state 2" ]
    pub fn ois2n(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Output Idle state 2" ]
    pub fn ois2(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
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
    # [ doc = "Bit 7 - TI1 selection" ]
    pub fn ti1s(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:6 - Master mode selection" ]
    pub fn mms(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
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
    # [ doc = "Bit 14 - Output Idle state 4" ]
    pub fn ois4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Output Idle state 3" ]
    pub fn ois3n(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Output Idle state 3" ]
    pub fn ois3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Output Idle state 2" ]
    pub fn ois2n(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Output Idle state 2" ]
    pub fn ois2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
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
    # [ doc = "Bit 7 - TI1 selection" ]
    pub fn ti1s(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:6 - Master mode selection" ]
    pub fn mms(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
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
pub struct Smcr {
    register: ::volatile_register::RW<u32>,
}

impl Smcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&SmcrR, &'w mut SmcrW) -> &'w mut SmcrW
    {
        let bits = self.register.read();
        let r = SmcrR { bits: bits };
        let mut w = SmcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> SmcrR {
        SmcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut SmcrW) -> &mut SmcrW
    {
        let mut w = SmcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SmcrR {
    bits: u32,
}

impl SmcrR {
    # [ doc = "Bit 15 - External trigger polarity" ]
    pub fn etp(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - External clock enable" ]
    pub fn ece(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:13 - External trigger prescaler" ]
    pub fn etps(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - External trigger filter" ]
    pub fn etf(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Master/Slave mode" ]
    pub fn msm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:6 - Trigger selection" ]
    pub fn ts(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:2 - Slave mode selection" ]
    pub fn sms(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SmcrW {
    bits: u32,
}

impl SmcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        SmcrW { bits: 0u32 }
    }
    # [ doc = "Bit 15 - External trigger polarity" ]
    pub fn etp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - External clock enable" ]
    pub fn ece(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:13 - External trigger prescaler" ]
    pub fn etps(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - External trigger filter" ]
    pub fn etf(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Master/Slave mode" ]
    pub fn msm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:6 - Trigger selection" ]
    pub fn ts(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:2 - Slave mode selection" ]
    pub fn sms(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
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
    # [ doc = "Bit 12 - Capture/Compare 4 DMA request enable" ]
    pub fn cc4de(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Capture/Compare 3 DMA request enable" ]
    pub fn cc3de(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Capture/Compare 2 DMA request enable" ]
    pub fn cc2de(&self) -> bool {
        const OFFSET: u8 = 10u8;
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
    # [ doc = "Bit 6 - Trigger interrupt enable" ]
    pub fn tie(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Capture/Compare 4 interrupt enable" ]
    pub fn cc4ie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Capture/Compare 3 interrupt enable" ]
    pub fn cc3ie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Capture/Compare 2 interrupt enable" ]
    pub fn cc2ie(&self) -> bool {
        const OFFSET: u8 = 2u8;
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
    # [ doc = "Bit 7 - Break interrupt enable" ]
    pub fn bie(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - COM interrupt enable" ]
    pub fn comie(&self) -> bool {
        const OFFSET: u8 = 5u8;
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
    # [ doc = "Bit 12 - Capture/Compare 4 DMA request enable" ]
    pub fn cc4de(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Capture/Compare 3 DMA request enable" ]
    pub fn cc3de(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Capture/Compare 2 DMA request enable" ]
    pub fn cc2de(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
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
    # [ doc = "Bit 4 - Capture/Compare 4 interrupt enable" ]
    pub fn cc4ie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Capture/Compare 3 interrupt enable" ]
    pub fn cc3ie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Capture/Compare 2 interrupt enable" ]
    pub fn cc2ie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
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
    # [ doc = "Bit 12 - Capture/Compare 4 overcapture flag" ]
    pub fn cc4of(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Capture/Compare 3 overcapture flag" ]
    pub fn cc3of(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Capture/compare 2 overcapture flag" ]
    pub fn cc2of(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
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
    # [ doc = "Bit 4 - Capture/Compare 4 interrupt flag" ]
    pub fn cc4if(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Capture/Compare 3 interrupt flag" ]
    pub fn cc3if(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Capture/Compare 2 interrupt flag" ]
    pub fn cc2if(&self) -> bool {
        const OFFSET: u8 = 2u8;
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
    # [ doc = "Bit 12 - Capture/Compare 4 overcapture flag" ]
    pub fn cc4of(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Capture/Compare 3 overcapture flag" ]
    pub fn cc3of(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Capture/compare 2 overcapture flag" ]
    pub fn cc2of(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
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
    # [ doc = "Bit 4 - Capture/Compare 4 interrupt flag" ]
    pub fn cc4if(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Capture/Compare 3 interrupt flag" ]
    pub fn cc3if(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Capture/Compare 2 interrupt flag" ]
    pub fn cc2if(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
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
    # [ doc = "Bit 4 - Capture/compare 4 generation" ]
    pub fn cc4g(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Capture/compare 3 generation" ]
    pub fn cc3g(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Capture/compare 2 generation" ]
    pub fn cc2g(&self) -> bool {
        const OFFSET: u8 = 2u8;
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
    # [ doc = "Bit 4 - Capture/compare 4 generation" ]
    pub fn cc4g(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Capture/compare 3 generation" ]
    pub fn cc3g(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Capture/compare 2 generation" ]
    pub fn cc2g(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
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
    # [ doc = "Bit 15 - Output Compare 2 clear enable" ]
    pub fn oc2ce(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:14 - Output Compare 2 mode" ]
    pub fn oc2m(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 11 - Output Compare 2 preload enable" ]
    pub fn oc2pe(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Output Compare 2 fast enable" ]
    pub fn oc2fe(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:9 - Capture/Compare 2 selection" ]
    pub fn cc2s(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Output Compare 1 clear enable" ]
    pub fn oc1ce(&self) -> bool {
        const OFFSET: u8 = 7u8;
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
    # [ doc = "Bit 15 - Output Compare 2 clear enable" ]
    pub fn oc2ce(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:14 - Output Compare 2 mode" ]
    pub fn oc2m(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Output Compare 2 preload enable" ]
    pub fn oc2pe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Output Compare 2 fast enable" ]
    pub fn oc2fe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 8:9 - Capture/Compare 2 selection" ]
    pub fn cc2s(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Output Compare 1 clear enable" ]
    pub fn oc1ce(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
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
    # [ doc = "Bits 12:15 - Input capture 2 filter" ]
    pub fn ic2f(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 10:11 - Input capture 2 prescaler" ]
    pub fn ic2pcs(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:9 - Capture/Compare 2 selection" ]
    pub fn cc2s(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - Input capture 1 filter" ]
    pub fn ic1f(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 2:3 - Input capture 1 prescaler" ]
    pub fn icpcs(&self) -> u8 {
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
    # [ doc = "Bits 12:15 - Input capture 2 filter" ]
    pub fn ic2f(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 10:11 - Input capture 2 prescaler" ]
    pub fn ic2pcs(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:9 - Capture/Compare 2 selection" ]
    pub fn cc2s(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
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
    pub fn icpcs(&mut self, value: u8) -> &mut Self {
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
pub struct Ccmr2Output {
    register: ::volatile_register::RW<u32>,
}

impl Ccmr2Output {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ccmr2OutputR, &'w mut Ccmr2OutputW) -> &'w mut Ccmr2OutputW
    {
        let bits = self.register.read();
        let r = Ccmr2OutputR { bits: bits };
        let mut w = Ccmr2OutputW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ccmr2OutputR {
        Ccmr2OutputR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ccmr2OutputW) -> &mut Ccmr2OutputW
    {
        let mut w = Ccmr2OutputW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccmr2OutputR {
    bits: u32,
}

impl Ccmr2OutputR {
    # [ doc = "Bit 15 - Output compare 4 clear enable" ]
    pub fn oc4ce(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:14 - Output compare 4 mode" ]
    pub fn oc4m(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 11 - Output compare 4 preload enable" ]
    pub fn oc4pe(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Output compare 4 fast enable" ]
    pub fn oc4fe(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:9 - Capture/Compare 4 selection" ]
    pub fn cc4s(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Output compare 3 clear enable" ]
    pub fn oc3ce(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:6 - Output compare 3 mode" ]
    pub fn oc3m(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 3 - Output compare 3 preload enable" ]
    pub fn oc3pe(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Output compare 3 fast enable" ]
    pub fn oc3fe(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:1 - Capture/Compare 3 selection" ]
    pub fn cc3s(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccmr2OutputW {
    bits: u32,
}

impl Ccmr2OutputW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ccmr2OutputW { bits: 0u32 }
    }
    # [ doc = "Bit 15 - Output compare 4 clear enable" ]
    pub fn oc4ce(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:14 - Output compare 4 mode" ]
    pub fn oc4m(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Output compare 4 preload enable" ]
    pub fn oc4pe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Output compare 4 fast enable" ]
    pub fn oc4fe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 8:9 - Capture/Compare 4 selection" ]
    pub fn cc4s(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Output compare 3 clear enable" ]
    pub fn oc3ce(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:6 - Output compare 3 mode" ]
    pub fn oc3m(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 3 - Output compare 3 preload enable" ]
    pub fn oc3pe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Output compare 3 fast enable" ]
    pub fn oc3fe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:1 - Capture/Compare 3 selection" ]
    pub fn cc3s(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ccmr2Input {
    register: ::volatile_register::RW<u32>,
}

impl Ccmr2Input {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ccmr2InputR, &'w mut Ccmr2InputW) -> &'w mut Ccmr2InputW
    {
        let bits = self.register.read();
        let r = Ccmr2InputR { bits: bits };
        let mut w = Ccmr2InputW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ccmr2InputR {
        Ccmr2InputR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ccmr2InputW) -> &mut Ccmr2InputW
    {
        let mut w = Ccmr2InputW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccmr2InputR {
    bits: u32,
}

impl Ccmr2InputR {
    # [ doc = "Bits 12:15 - Input capture 4 filter" ]
    pub fn ic4f(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 10:11 - Input capture 4 prescaler" ]
    pub fn ic4psc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:9 - Capture/Compare 4 selection" ]
    pub fn cc4s(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - Input capture 3 filter" ]
    pub fn ic3f(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 2:3 - Input capture 3 prescaler" ]
    pub fn ic3psc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:1 - Capture/compare 3 selection" ]
    pub fn cc3s(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccmr2InputW {
    bits: u32,
}

impl Ccmr2InputW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ccmr2InputW { bits: 0u32 }
    }
    # [ doc = "Bits 12:15 - Input capture 4 filter" ]
    pub fn ic4f(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 10:11 - Input capture 4 prescaler" ]
    pub fn ic4psc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:9 - Capture/Compare 4 selection" ]
    pub fn cc4s(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - Input capture 3 filter" ]
    pub fn ic3f(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 2:3 - Input capture 3 prescaler" ]
    pub fn ic3psc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:1 - Capture/compare 3 selection" ]
    pub fn cc3s(&mut self, value: u8) -> &mut Self {
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
    # [ doc = "Bit 13 - Capture/Compare 3 output Polarity" ]
    pub fn cc4p(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Capture/Compare 4 output enable" ]
    pub fn cc4e(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Capture/Compare 3 output Polarity" ]
    pub fn cc3np(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Capture/Compare 3 complementary output enable" ]
    pub fn cc3ne(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Capture/Compare 3 output Polarity" ]
    pub fn cc3p(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Capture/Compare 3 output enable" ]
    pub fn cc3e(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Capture/Compare 2 output Polarity" ]
    pub fn cc2np(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Capture/Compare 2 complementary output enable" ]
    pub fn cc2ne(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Capture/Compare 2 output Polarity" ]
    pub fn cc2p(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Capture/Compare 2 output enable" ]
    pub fn cc2e(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
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
    # [ doc = "Bit 13 - Capture/Compare 3 output Polarity" ]
    pub fn cc4p(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Capture/Compare 4 output enable" ]
    pub fn cc4e(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Capture/Compare 3 output Polarity" ]
    pub fn cc3np(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Capture/Compare 3 complementary output enable" ]
    pub fn cc3ne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Capture/Compare 3 output Polarity" ]
    pub fn cc3p(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Capture/Compare 3 output enable" ]
    pub fn cc3e(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Capture/Compare 2 output Polarity" ]
    pub fn cc2np(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Capture/Compare 2 complementary output enable" ]
    pub fn cc2ne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Capture/Compare 2 output Polarity" ]
    pub fn cc2p(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Capture/Compare 2 output enable" ]
    pub fn cc2e(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
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
pub struct Ccr2 {
    register: ::volatile_register::RW<u32>,
}

impl Ccr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ccr2R, &'w mut Ccr2W) -> &'w mut Ccr2W
    {
        let bits = self.register.read();
        let r = Ccr2R { bits: bits };
        let mut w = Ccr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ccr2R {
        Ccr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ccr2W) -> &mut Ccr2W
    {
        let mut w = Ccr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr2R {
    bits: u32,
}

impl Ccr2R {
    # [ doc = "Bits 0:15 - Capture/Compare 2 value" ]
    pub fn ccr2(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr2W {
    bits: u32,
}

impl Ccr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ccr2W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Capture/Compare 2 value" ]
    pub fn ccr2(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ccr3 {
    register: ::volatile_register::RW<u32>,
}

impl Ccr3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ccr3R, &'w mut Ccr3W) -> &'w mut Ccr3W
    {
        let bits = self.register.read();
        let r = Ccr3R { bits: bits };
        let mut w = Ccr3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ccr3R {
        Ccr3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ccr3W) -> &mut Ccr3W
    {
        let mut w = Ccr3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr3R {
    bits: u32,
}

impl Ccr3R {
    # [ doc = "Bits 0:15 - Capture/Compare value" ]
    pub fn ccr3(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr3W {
    bits: u32,
}

impl Ccr3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ccr3W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Capture/Compare value" ]
    pub fn ccr3(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ccr4 {
    register: ::volatile_register::RW<u32>,
}

impl Ccr4 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ccr4R, &'w mut Ccr4W) -> &'w mut Ccr4W
    {
        let bits = self.register.read();
        let r = Ccr4R { bits: bits };
        let mut w = Ccr4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ccr4R {
        Ccr4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ccr4W) -> &mut Ccr4W
    {
        let mut w = Ccr4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr4R {
    bits: u32,
}

impl Ccr4R {
    # [ doc = "Bits 0:15 - Capture/Compare value" ]
    pub fn ccr4(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr4W {
    bits: u32,
}

impl Ccr4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ccr4W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Capture/Compare value" ]
    pub fn ccr4(&mut self, value: u16) -> &mut Self {
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
    # [ doc = "Bit 15 - Main output enable" ]
    pub fn moe(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Automatic output enable" ]
    pub fn aoe(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Break polarity" ]
    pub fn bkp(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Break enable" ]
    pub fn bke(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Off-state selection for Run mode" ]
    pub fn ossr(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Off-state selection for Idle mode" ]
    pub fn ossi(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:9 - Lock configuration" ]
    pub fn lock(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - Dead-time generator setup" ]
    pub fn dtg(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
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
    # [ doc = "Bits 8:9 - Lock configuration" ]
    pub fn lock(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Dead-time generator setup" ]
    pub fn dtg(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
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
    # [ doc = "Bits 0:1 - External trigger remap on ADC1 analog watchdog" ]
    pub fn etr_adc1_rmp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 2:3 - External trigger remap on ADC3 analog watchdog" ]
    pub fn etr_adc3_rmp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 4 - Input Capture 1 remap" ]
    pub fn ti1_rmp(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
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
    # [ doc = "Bits 0:1 - External trigger remap on ADC1 analog watchdog" ]
    pub fn etr_adc1_rmp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 2:3 - External trigger remap on ADC3 analog watchdog" ]
    pub fn etr_adc3_rmp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 4 - Input Capture 1 remap" ]
    pub fn ti1_rmp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Ccmr3Output {
    register: ::volatile_register::RW<u32>,
}

impl Ccmr3Output {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ccmr3OutputR, &'w mut Ccmr3OutputW) -> &'w mut Ccmr3OutputW
    {
        let bits = self.register.read();
        let r = Ccmr3OutputR { bits: bits };
        let mut w = Ccmr3OutputW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ccmr3OutputR {
        Ccmr3OutputR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ccmr3OutputW) -> &mut Ccmr3OutputW
    {
        let mut w = Ccmr3OutputW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccmr3OutputR {
    bits: u32,
}

impl Ccmr3OutputR {
    # [ doc = "Bit 24 - Output Compare 6 mode bit 3" ]
    pub fn oc6m_bit3(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 16:18 - Output Compare 5 mode bit 3" ]
    pub fn oc5m_bit3(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Output compare 6 clear enable" ]
    pub fn oc6ce(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:14 - Output compare 6 mode" ]
    pub fn oc6m(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 11 - Output compare 6 preload enable" ]
    pub fn oc6pe(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Output compare 6 fast enable" ]
    pub fn oc6fe(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Output compare 5 clear enable" ]
    pub fn oc5ce(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:6 - Output compare 5 mode" ]
    pub fn oc5m(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 3 - Output compare 5 preload enable" ]
    pub fn oc5pe(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Output compare 5 fast enable" ]
    pub fn oc5fe(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccmr3OutputW {
    bits: u32,
}

impl Ccmr3OutputW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ccmr3OutputW { bits: 0u32 }
    }
    # [ doc = "Bit 24 - Output Compare 6 mode bit 3" ]
    pub fn oc6m_bit3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 16:18 - Output Compare 5 mode bit 3" ]
    pub fn oc5m_bit3(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Output compare 6 clear enable" ]
    pub fn oc6ce(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:14 - Output compare 6 mode" ]
    pub fn oc6m(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Output compare 6 preload enable" ]
    pub fn oc6pe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Output compare 6 fast enable" ]
    pub fn oc6fe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Output compare 5 clear enable" ]
    pub fn oc5ce(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:6 - Output compare 5 mode" ]
    pub fn oc5m(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 3 - Output compare 5 preload enable" ]
    pub fn oc5pe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Output compare 5 fast enable" ]
    pub fn oc5fe(&mut self, value: bool) -> &mut Self {
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
pub struct Ccr5 {
    register: ::volatile_register::RW<u32>,
}

impl Ccr5 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ccr5R, &'w mut Ccr5W) -> &'w mut Ccr5W
    {
        let bits = self.register.read();
        let r = Ccr5R { bits: bits };
        let mut w = Ccr5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ccr5R {
        Ccr5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ccr5W) -> &mut Ccr5W
    {
        let mut w = Ccr5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr5R {
    bits: u32,
}

impl Ccr5R {
    # [ doc = "Bits 0:15 - Capture/Compare value" ]
    pub fn ccr5(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 29 - Group Channel 5 and Channel 1" ]
    pub fn gc5c1(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Group Channel 5 and Channel 2" ]
    pub fn gc5c2(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Group Channel 5 and Channel 3" ]
    pub fn gc5c3(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr5W {
    bits: u32,
}

impl Ccr5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ccr5W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Capture/Compare value" ]
    pub fn ccr5(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Group Channel 5 and Channel 1" ]
    pub fn gc5c1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Group Channel 5 and Channel 2" ]
    pub fn gc5c2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Group Channel 5 and Channel 3" ]
    pub fn gc5c3(&mut self, value: bool) -> &mut Self {
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
pub struct Ccr6 {
    register: ::volatile_register::RW<u32>,
}

impl Ccr6 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ccr6R, &'w mut Ccr6W) -> &'w mut Ccr6W
    {
        let bits = self.register.read();
        let r = Ccr6R { bits: bits };
        let mut w = Ccr6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ccr6R {
        Ccr6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ccr6W) -> &mut Ccr6W
    {
        let mut w = Ccr6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr6R {
    bits: u32,
}

impl Ccr6R {
    # [ doc = "Bits 0:15 - Capture/Compare value" ]
    pub fn ccr6(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr6W {
    bits: u32,
}

impl Ccr6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ccr6W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Capture/Compare value" ]
    pub fn ccr6(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
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
    # [ doc = "Bit 8 - BRK DFSDM_BREAK0 enable" ]
    pub fn bkdfbk0e(&self) -> bool {
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
    # [ doc = "Bit 11 - BRK COMP2 input polarity" ]
    pub fn bkcmp2p(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 14:16 - ETR source selection" ]
    pub fn etrsel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
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
        Or2W { bits: 1u32 }
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
    # [ doc = "Bit 8 - BRK DFSDM_BREAK0 enable" ]
    pub fn bkdfbk0e(&mut self, value: bool) -> &mut Self {
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
    # [ doc = "Bit 11 - BRK COMP2 input polarity" ]
    pub fn bkcmp2p(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 14:16 - ETR source selection" ]
    pub fn etrsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Or3 {
    register: ::volatile_register::RW<u32>,
}

impl Or3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Or3R, &'w mut Or3W) -> &'w mut Or3W
    {
        let bits = self.register.read();
        let r = Or3R { bits: bits };
        let mut w = Or3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Or3R {
        Or3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Or3W) -> &mut Or3W
    {
        let mut w = Or3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Or3R {
    bits: u32,
}

impl Or3R {
    # [ doc = "Bit 0 - BRK2 BKIN input enable" ]
    pub fn bk2ine(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - BRK2 COMP1 enable" ]
    pub fn bk2cmp1e(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - BRK2 COMP2 enable" ]
    pub fn bk2cmp2e(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - BRK2 DFSDM_BREAK0 enable" ]
    pub fn bk2dfbk0e(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - BRK2 BKIN input polarity" ]
    pub fn bk2inp(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - BRK2 COMP1 input polarity" ]
    pub fn bk2cmp1p(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - BRK2 COMP2 input polarity" ]
    pub fn bk2cmp2p(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Or3W {
    bits: u32,
}

impl Or3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Or3W { bits: 1u32 }
    }
    # [ doc = "Bit 0 - BRK2 BKIN input enable" ]
    pub fn bk2ine(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - BRK2 COMP1 enable" ]
    pub fn bk2cmp1e(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - BRK2 COMP2 enable" ]
    pub fn bk2cmp2e(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - BRK2 DFSDM_BREAK0 enable" ]
    pub fn bk2dfbk0e(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - BRK2 BKIN input polarity" ]
    pub fn bk2inp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - BRK2 COMP1 input polarity" ]
    pub fn bk2cmp1p(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - BRK2 COMP2 input polarity" ]
    pub fn bk2cmp2p(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}
