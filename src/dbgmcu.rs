# [ doc = "Debug support" ]
# [ repr ( C ) ]
pub struct Dbgmcu {
    # [ doc = "0x00 - MCU Device ID Code Register" ]
    pub idcode: Idcode,
    # [ doc = "0x04 - Debug MCU Configuration Register" ]
    pub cr: Cr,
    # [ doc = "0x08 - APB Low Freeze Register 1" ]
    pub apb1_fzr1: Apb1Fzr1,
    # [ doc = "0x0c - APB Low Freeze Register 2" ]
    pub apb1_fzr2: Apb1Fzr2,
    # [ doc = "0x10 - APB High Freeze Register" ]
    pub apb2_fzr: Apb2Fzr,
}

# [ repr ( C ) ]
pub struct Idcode {
    register: ::volatile_register::RO<u32>,
}

impl Idcode {
    pub fn read(&self) -> IdcodeR {
        IdcodeR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdcodeR {
    bits: u32,
}

impl IdcodeR {
    # [ doc = "Bits 0:15 - Device Identifier" ]
    pub fn dev_id(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - Revision Identifier" ]
    pub fn rev_id(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdcodeW {
    bits: u32,
}

impl IdcodeW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IdcodeW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Device Identifier" ]
    pub fn dev_id(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - Revision Identifier" ]
    pub fn rev_id(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
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
    # [ doc = "Bit 0 - Debug Sleep Mode" ]
    pub fn dbg_sleep(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Debug Stop Mode" ]
    pub fn dbg_stop(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Debug Standby Mode" ]
    pub fn dbg_standby(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Trace pin assignment control" ]
    pub fn trace_ioen(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 6:7 - Trace pin assignment control" ]
    pub fn trace_mode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
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
    # [ doc = "Bit 0 - Debug Sleep Mode" ]
    pub fn dbg_sleep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Debug Stop Mode" ]
    pub fn dbg_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Debug Standby Mode" ]
    pub fn dbg_standby(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Trace pin assignment control" ]
    pub fn trace_ioen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 6:7 - Trace pin assignment control" ]
    pub fn trace_mode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Apb1Fzr1 {
    register: ::volatile_register::RW<u32>,
}

impl Apb1Fzr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb1Fzr1R, &'w mut Apb1Fzr1W) -> &'w mut Apb1Fzr1W
    {
        let bits = self.register.read();
        let r = Apb1Fzr1R { bits: bits };
        let mut w = Apb1Fzr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb1Fzr1R {
        Apb1Fzr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb1Fzr1W) -> &mut Apb1Fzr1W
    {
        let mut w = Apb1Fzr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1Fzr1R {
    bits: u32,
}

impl Apb1Fzr1R {
    # [ doc = "Bit 0 - Debug Timer 2 stopped when Core is halted" ]
    pub fn dbg_timer2_stop(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - TIM3 counter stopped when core is halted" ]
    pub fn dbg_tim3_stop(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - TIM4 counter stopped when core is halted" ]
    pub fn dbg_tim4_stop(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - TIM5 counter stopped when core is halted" ]
    pub fn dbg_tim5_stop(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Debug Timer 6 stopped when Core is halted" ]
    pub fn dbg_timer6_stop(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - TIM7 counter stopped when core is halted" ]
    pub fn dbg_tim7_stop(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Debug RTC stopped when Core is halted" ]
    pub fn dbg_rtc_stop(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted" ]
    pub fn dbg_wwdg_stop(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted" ]
    pub fn dbg_iwdg_stop(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted" ]
    pub fn dbg_i2c1_stop(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - I2C2 SMBUS timeout mode stopped when core is halted" ]
    pub fn dbg_i2c2_stop(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - I2C3 SMBUS timeout counter stopped when core is halted" ]
    pub fn dbg_i2c3_stop(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - bxCAN stopped when core is halted" ]
    pub fn dbg_can_stop(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - LPTIM1 counter stopped when core is halted" ]
    pub fn dbg_lptimer_stop(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1Fzr1W {
    bits: u32,
}

impl Apb1Fzr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb1Fzr1W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Debug Timer 2 stopped when Core is halted" ]
    pub fn dbg_timer2_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - TIM3 counter stopped when core is halted" ]
    pub fn dbg_tim3_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - TIM4 counter stopped when core is halted" ]
    pub fn dbg_tim4_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - TIM5 counter stopped when core is halted" ]
    pub fn dbg_tim5_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Debug Timer 6 stopped when Core is halted" ]
    pub fn dbg_timer6_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - TIM7 counter stopped when core is halted" ]
    pub fn dbg_tim7_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Debug RTC stopped when Core is halted" ]
    pub fn dbg_rtc_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Debug Window Wachdog stopped when Core is halted" ]
    pub fn dbg_wwdg_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Debug Independent Wachdog stopped when Core is halted" ]
    pub fn dbg_iwdg_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - I2C1 SMBUS timeout mode stopped when core is halted" ]
    pub fn dbg_i2c1_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - I2C2 SMBUS timeout mode stopped when core is halted" ]
    pub fn dbg_i2c2_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - I2C3 SMBUS timeout counter stopped when core is halted" ]
    pub fn dbg_i2c3_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - bxCAN stopped when core is halted" ]
    pub fn dbg_can_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - LPTIM1 counter stopped when core is halted" ]
    pub fn dbg_lptimer_stop(&mut self, value: bool) -> &mut Self {
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
pub struct Apb1Fzr2 {
    register: ::volatile_register::RW<u32>,
}

impl Apb1Fzr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb1Fzr2R, &'w mut Apb1Fzr2W) -> &'w mut Apb1Fzr2W
    {
        let bits = self.register.read();
        let r = Apb1Fzr2R { bits: bits };
        let mut w = Apb1Fzr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb1Fzr2R {
        Apb1Fzr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb1Fzr2W) -> &mut Apb1Fzr2W
    {
        let mut w = Apb1Fzr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1Fzr2R {
    bits: u32,
}

impl Apb1Fzr2R {
    # [ doc = "Bit 5 - LPTIM2 counter stopped when core is halted" ]
    pub fn dbg_lptim2_stop(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1Fzr2W {
    bits: u32,
}

impl Apb1Fzr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb1Fzr2W { bits: 0u32 }
    }
    # [ doc = "Bit 5 - LPTIM2 counter stopped when core is halted" ]
    pub fn dbg_lptim2_stop(&mut self, value: bool) -> &mut Self {
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
pub struct Apb2Fzr {
    register: ::volatile_register::RW<u32>,
}

impl Apb2Fzr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb2FzrR, &'w mut Apb2FzrW) -> &'w mut Apb2FzrW
    {
        let bits = self.register.read();
        let r = Apb2FzrR { bits: bits };
        let mut w = Apb2FzrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb2FzrR {
        Apb2FzrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb2FzrW) -> &mut Apb2FzrW
    {
        let mut w = Apb2FzrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb2FzrR {
    bits: u32,
}

impl Apb2FzrR {
    # [ doc = "Bit 11 - TIM1 counter stopped when core is halted" ]
    pub fn dbg_tim1_stop(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - TIM8 counter stopped when core is halted" ]
    pub fn dbg_tim8_stop(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - TIM15 counter stopped when core is halted" ]
    pub fn dbg_tim15_stop(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - TIM16 counter stopped when core is halted" ]
    pub fn dbg_tim16_stop(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - TIM17 counter stopped when core is halted" ]
    pub fn dbg_tim17_stop(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb2FzrW {
    bits: u32,
}

impl Apb2FzrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb2FzrW { bits: 0u32 }
    }
    # [ doc = "Bit 11 - TIM1 counter stopped when core is halted" ]
    pub fn dbg_tim1_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - TIM8 counter stopped when core is halted" ]
    pub fn dbg_tim8_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - TIM15 counter stopped when core is halted" ]
    pub fn dbg_tim15_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - TIM16 counter stopped when core is halted" ]
    pub fn dbg_tim16_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - TIM17 counter stopped when core is halted" ]
    pub fn dbg_tim17_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}
