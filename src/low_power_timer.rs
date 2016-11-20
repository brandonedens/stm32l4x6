# [ doc = "Low power timer" ]
# [ repr ( C ) ]
pub struct LowPowerTimer {
    # [ doc = "0x00 - Interrupt and Status Register" ]
    pub isr: Isr,
    # [ doc = "0x04 - Interrupt Clear Register" ]
    pub icr: Icr,
    # [ doc = "0x08 - Interrupt Enable Register" ]
    pub ier: Ier,
    # [ doc = "0x0c - Configuration Register" ]
    pub cfgr: Cfgr,
    # [ doc = "0x10 - Control Register" ]
    pub cr: Cr,
    # [ doc = "0x14 - Compare Register" ]
    pub cmp: Cmp,
    # [ doc = "0x18 - Autoreload Register" ]
    pub arr: Arr,
    # [ doc = "0x1c - Counter Register" ]
    pub cnt: Cnt,
}

# [ repr ( C ) ]
pub struct Isr {
    register: ::volatile_register::RO<u32>,
}

impl Isr {
    pub fn read(&self) -> IsrR {
        IsrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IsrR {
    bits: u32,
}

impl IsrR {
    # [ doc = "Bit 6 - Counter direction change up to down" ]
    pub fn down(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Counter direction change down to up" ]
    pub fn up(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Autoreload register update OK" ]
    pub fn arrok(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Compare register update OK" ]
    pub fn cmpok(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - External trigger edge event" ]
    pub fn exttrig(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Autoreload match" ]
    pub fn arrm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Compare match" ]
    pub fn cmpm(&self) -> bool {
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
    # [ doc = "Bit 6 - Counter direction change up to down" ]
    pub fn down(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Counter direction change down to up" ]
    pub fn up(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Autoreload register update OK" ]
    pub fn arrok(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Compare register update OK" ]
    pub fn cmpok(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - External trigger edge event" ]
    pub fn exttrig(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Autoreload match" ]
    pub fn arrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Compare match" ]
    pub fn cmpm(&mut self, value: bool) -> &mut Self {
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
pub struct Icr {
    register: ::volatile_register::WO<u32>,
}

impl Icr {
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut IcrW) -> &mut IcrW
    {
        let mut w = IcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IcrR {
    bits: u32,
}

impl IcrR {
    # [ doc = "Bit 6 - Direction change to down Clear Flag" ]
    pub fn downcf(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Direction change to UP Clear Flag" ]
    pub fn upcf(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Autoreload register update OK Clear Flag" ]
    pub fn arrokcf(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Compare register update OK Clear Flag" ]
    pub fn cmpokcf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - External trigger valid edge Clear Flag" ]
    pub fn exttrigcf(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Autoreload match Clear Flag" ]
    pub fn arrmcf(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - compare match Clear Flag" ]
    pub fn cmpmcf(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IcrW {
    bits: u32,
}

impl IcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IcrW { bits: 0u32 }
    }
    # [ doc = "Bit 6 - Direction change to down Clear Flag" ]
    pub fn downcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Direction change to UP Clear Flag" ]
    pub fn upcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Autoreload register update OK Clear Flag" ]
    pub fn arrokcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Compare register update OK Clear Flag" ]
    pub fn cmpokcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - External trigger valid edge Clear Flag" ]
    pub fn exttrigcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Autoreload match Clear Flag" ]
    pub fn arrmcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - compare match Clear Flag" ]
    pub fn cmpmcf(&mut self, value: bool) -> &mut Self {
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
    # [ doc = "Bit 6 - Direction change to down Interrupt Enable" ]
    pub fn downie(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Direction change to UP Interrupt Enable" ]
    pub fn upie(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Autoreload register update OK Interrupt Enable" ]
    pub fn arrokie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Compare register update OK Interrupt Enable" ]
    pub fn cmpokie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - External trigger valid edge Interrupt Enable" ]
    pub fn exttrigie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Autoreload match Interrupt Enable" ]
    pub fn arrmie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Compare match Interrupt Enable" ]
    pub fn cmpmie(&self) -> bool {
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
    # [ doc = "Bit 6 - Direction change to down Interrupt Enable" ]
    pub fn downie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Direction change to UP Interrupt Enable" ]
    pub fn upie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Autoreload register update OK Interrupt Enable" ]
    pub fn arrokie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Compare register update OK Interrupt Enable" ]
    pub fn cmpokie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - External trigger valid edge Interrupt Enable" ]
    pub fn exttrigie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Autoreload match Interrupt Enable" ]
    pub fn arrmie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Compare match Interrupt Enable" ]
    pub fn cmpmie(&mut self, value: bool) -> &mut Self {
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
    # [ doc = "Bit 24 - Encoder mode enable" ]
    pub fn enc(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - counter mode enabled" ]
    pub fn countmode(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Registers update mode" ]
    pub fn preload(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Waveform shape polarity" ]
    pub fn wavpol(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Waveform shape" ]
    pub fn wave(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Timeout enable" ]
    pub fn timout(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 17:18 - Trigger enable and polarity" ]
    pub fn trigen(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 17u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 13:15 - Trigger selector" ]
    pub fn trigsel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 9:11 - Clock prescaler" ]
    pub fn presc(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 9u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 6:7 - Configurable digital filter for trigger" ]
    pub fn trgflt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 3:4 - Configurable digital filter for external clock" ]
    pub fn ckflt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 1:2 - Clock Polarity" ]
    pub fn ckpol(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 0 - Clock selector" ]
    pub fn cksel(&self) -> bool {
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
    # [ doc = "Bit 24 - Encoder mode enable" ]
    pub fn enc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - counter mode enabled" ]
    pub fn countmode(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Registers update mode" ]
    pub fn preload(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Waveform shape polarity" ]
    pub fn wavpol(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Waveform shape" ]
    pub fn wave(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Timeout enable" ]
    pub fn timout(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 17:18 - Trigger enable and polarity" ]
    pub fn trigen(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 17u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 13:15 - Trigger selector" ]
    pub fn trigsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 9:11 - Clock prescaler" ]
    pub fn presc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 9u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 6:7 - Configurable digital filter for trigger" ]
    pub fn trgflt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 3:4 - Configurable digital filter for external clock" ]
    pub fn ckflt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 1:2 - Clock Polarity" ]
    pub fn ckpol(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 0 - Clock selector" ]
    pub fn cksel(&mut self, value: bool) -> &mut Self {
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
    # [ doc = "Bit 2 - Timer start in continuous mode" ]
    pub fn cntstrt(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - LPTIM start in single mode" ]
    pub fn sngstrt(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - LPTIM Enable" ]
    pub fn enable(&self) -> bool {
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
    # [ doc = "Bit 2 - Timer start in continuous mode" ]
    pub fn cntstrt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - LPTIM start in single mode" ]
    pub fn sngstrt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - LPTIM Enable" ]
    pub fn enable(&mut self, value: bool) -> &mut Self {
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
pub struct Cmp {
    register: ::volatile_register::RW<u32>,
}

impl Cmp {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CmpR, &'w mut CmpW) -> &'w mut CmpW
    {
        let bits = self.register.read();
        let r = CmpR { bits: bits };
        let mut w = CmpW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CmpR {
        CmpR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CmpW) -> &mut CmpW
    {
        let mut w = CmpW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CmpR {
    bits: u32,
}

impl CmpR {
    # [ doc = "Bits 0:15 - Compare value" ]
    pub fn cmp(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CmpW {
    bits: u32,
}

impl CmpW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CmpW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Compare value" ]
    pub fn cmp(&mut self, value: u16) -> &mut Self {
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
    # [ doc = "Bits 0:15 - Auto reload value" ]
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
        ArrW { bits: 1u32 }
    }
    # [ doc = "Bits 0:15 - Auto reload value" ]
    pub fn arr(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cnt {
    register: ::volatile_register::RO<u32>,
}

impl Cnt {
    pub fn read(&self) -> CntR {
        CntR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CntR {
    bits: u32,
}

impl CntR {
    # [ doc = "Bits 0:15 - Counter value" ]
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
    # [ doc = "Bits 0:15 - Counter value" ]
    pub fn cnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
