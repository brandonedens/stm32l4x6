# [ doc = "Reset and clock control" ]
# [ repr ( C ) ]
pub struct Rcc {
    # [ doc = "0x00 - Clock control register" ]
    pub cr: Cr,
    # [ doc = "0x04 - Internal clock sources calibration register" ]
    pub icscr: Icscr,
    # [ doc = "0x08 - Clock configuration register" ]
    pub cfgr: Cfgr,
    # [ doc = "0x0c - PLL configuration register" ]
    pub pllcfgr: Pllcfgr,
    # [ doc = "0x10 - PLLSAI1 configuration register" ]
    pub pllsai1cfgr: Pllsai1cfgr,
    # [ doc = "0x14 - PLLSAI2 configuration register" ]
    pub pllsai2cfgr: Pllsai2cfgr,
    # [ doc = "0x18 - Clock interrupt enable register" ]
    pub cier: Cier,
    # [ doc = "0x1c - Clock interrupt flag register" ]
    pub cifr: Cifr,
    # [ doc = "0x20 - Clock interrupt clear register" ]
    pub cicr: Cicr,
    _reserved0: [u8; 4usize],
    # [ doc = "0x28 - AHB1 peripheral reset register" ]
    pub ahb1rstr: Ahb1rstr,
    # [ doc = "0x2c - AHB2 peripheral reset register" ]
    pub ahb2rstr: Ahb2rstr,
    # [ doc = "0x30 - AHB3 peripheral reset register" ]
    pub ahb3rstr: Ahb3rstr,
    _reserved1: [u8; 4usize],
    # [ doc = "0x38 - APB1 peripheral reset register 1" ]
    pub apb1rstr1: Apb1rstr1,
    # [ doc = "0x3c - APB1 peripheral reset register 2" ]
    pub apb1rstr2: Apb1rstr2,
    # [ doc = "0x40 - APB2 peripheral reset register" ]
    pub apb2rstr: Apb2rstr,
    _reserved2: [u8; 4usize],
    # [ doc = "0x48 - AHB1 peripheral clock enable register" ]
    pub ahb1enr: Ahb1enr,
    # [ doc = "0x4c - AHB2 peripheral clock enable register" ]
    pub ahb2enr: Ahb2enr,
    # [ doc = "0x50 - AHB3 peripheral clock enable register" ]
    pub ahb3enr: Ahb3enr,
    _reserved3: [u8; 4usize],
    # [ doc = "0x58 - APB1ENR1" ]
    pub apb1enr1: Apb1enr1,
    # [ doc = "0x5c - APB1 peripheral clock enable register 2" ]
    pub apb1enr2: Apb1enr2,
    # [ doc = "0x60 - APB2ENR" ]
    pub apb2enr: Apb2enr,
    _reserved4: [u8; 4usize],
    # [ doc = "0x68 - AHB1 peripheral clocks enable in Sleep and Stop modes register" ]
    pub ahb1smenr: Ahb1smenr,
    # [ doc = "0x6c - AHB2 peripheral clocks enable in Sleep and Stop modes register" ]
    pub ahb2smenr: Ahb2smenr,
    # [ doc = "0x70 - AHB3 peripheral clocks enable in Sleep and Stop modes register" ]
    pub ahb3smenr: Ahb3smenr,
    _reserved5: [u8; 4usize],
    # [ doc = "0x78 - APB1SMENR1" ]
    pub apb1smenr1: Apb1smenr1,
    # [ doc = "0x7c - APB1 peripheral clocks enable in Sleep and Stop modes register 2" ]
    pub apb1smenr2: Apb1smenr2,
    # [ doc = "0x80 - APB2SMENR" ]
    pub apb2smenr: Apb2smenr,
    _reserved6: [u8; 4usize],
    # [ doc = "0x88 - CCIPR" ]
    pub ccipr: Ccipr,
    _reserved7: [u8; 4usize],
    # [ doc = "0x90 - BDCR" ]
    pub bdcr: Bdcr,
    # [ doc = "0x94 - CSR" ]
    pub csr: Csr,
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
    # [ doc = "Bit 29 - SAI2 PLL clock ready flag" ]
    pub fn pllsai2rdy(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - SAI2 PLL enable" ]
    pub fn pllsai2on(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 27 - SAI1 PLL clock ready flag" ]
    pub fn pllsai1rdy(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - SAI1 PLL enable" ]
    pub fn pllsai1on(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Main PLL clock ready flag" ]
    pub fn pllrdy(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Main PLL enable" ]
    pub fn pllon(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - HSE crystal oscillator bypass" ]
    pub fn hsebyp(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - HSE clock ready flag" ]
    pub fn hserdy(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - HSE clock enable" ]
    pub fn hseon(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - HSI automatic start from Stop" ]
    pub fn hsiasfs(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - HSI clock ready flag" ]
    pub fn hsirdy(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - HSI always enable for peripheral kernels" ]
    pub fn hsikeron(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - HSI clock enable" ]
    pub fn hsion(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:7 - MSI clock ranges" ]
    pub fn msirange(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 2 - MSI clock PLL enable" ]
    pub fn msipllen(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - MSI clock ready flag" ]
    pub fn msirdy(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - MSI clock enable" ]
    pub fn msion(&self) -> bool {
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
        CrW { bits: 99u32 }
    }
    # [ doc = "Bit 28 - SAI2 PLL enable" ]
    pub fn pllsai2on(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - SAI1 PLL enable" ]
    pub fn pllsai1on(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Main PLL enable" ]
    pub fn pllon(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Clock security system enable" ]
    pub fn csson(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - HSE crystal oscillator bypass" ]
    pub fn hsebyp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - HSE clock enable" ]
    pub fn hseon(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - HSI automatic start from Stop" ]
    pub fn hsiasfs(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - HSI always enable for peripheral kernels" ]
    pub fn hsikeron(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - HSI clock enable" ]
    pub fn hsion(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:7 - MSI clock ranges" ]
    pub fn msirange(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 3 - MSI clock range selection" ]
    pub fn msirgsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - MSI clock PLL enable" ]
    pub fn msipllen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - MSI clock enable" ]
    pub fn msion(&mut self, value: bool) -> &mut Self {
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
pub struct Icscr {
    register: ::volatile_register::RW<u32>,
}

impl Icscr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&IcscrR, &'w mut IcscrW) -> &'w mut IcscrW
    {
        let bits = self.register.read();
        let r = IcscrR { bits: bits };
        let mut w = IcscrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IcscrR {
        IcscrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IcscrW) -> &mut IcscrW
    {
        let mut w = IcscrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IcscrR {
    bits: u32,
}

impl IcscrR {
    # [ doc = "Bits 24:28 - HSI clock trimming" ]
    pub fn hsitrim(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - HSI clock calibration" ]
    pub fn hsical(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - MSI clock trimming" ]
    pub fn msitrim(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - MSI clock calibration" ]
    pub fn msical(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IcscrW {
    bits: u32,
}

impl IcscrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IcscrW { bits: 268435456u32 }
    }
    # [ doc = "Bits 24:28 - HSI clock trimming" ]
    pub fn hsitrim(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - MSI clock trimming" ]
    pub fn msitrim(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
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
    # [ doc = "Bits 28:30 - Microcontroller clock output prescaler" ]
    pub fn mcopre(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:26 - Microcontroller clock output" ]
    pub fn mcosel(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Wakeup from Stop and CSS backup clock selection" ]
    pub fn stopwuck(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 11:13 - APB high-speed prescaler (APB2)" ]
    pub fn ppre2(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:10 - PB low-speed prescaler (APB1)" ]
    pub fn ppre1(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - AHB prescaler" ]
    pub fn hpre(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 2:3 - System clock switch status" ]
    pub fn sws(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:1 - System clock switch" ]
    pub fn sw(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
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
    # [ doc = "Bits 24:26 - Microcontroller clock output" ]
    pub fn mcosel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Wakeup from Stop and CSS backup clock selection" ]
    pub fn stopwuck(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 11:13 - APB high-speed prescaler (APB2)" ]
    pub fn ppre2(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:10 - PB low-speed prescaler (APB1)" ]
    pub fn ppre1(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - AHB prescaler" ]
    pub fn hpre(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:1 - System clock switch" ]
    pub fn sw(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pllcfgr {
    register: ::volatile_register::RW<u32>,
}

impl Pllcfgr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PllcfgrR, &'w mut PllcfgrW) -> &'w mut PllcfgrW
    {
        let bits = self.register.read();
        let r = PllcfgrR { bits: bits };
        let mut w = PllcfgrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PllcfgrR {
        PllcfgrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PllcfgrW) -> &mut PllcfgrW
    {
        let mut w = PllcfgrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PllcfgrR {
    bits: u32,
}

impl PllcfgrR {
    # [ doc = "Bits 25:26 - Main PLL division factor for PLLCLK (system clock)" ]
    pub fn pllr(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 25u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 24 - Main PLL PLLCLK output enable" ]
    pub fn pllren(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 21:22 - Main PLL division factor for PLLUSB1CLK(48 MHz clock)" ]
    pub fn pllq(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 21u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 20 - Main PLL PLLUSB1CLK output enable" ]
    pub fn pllqen(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)" ]
    pub fn pllp(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Main PLL PLLSAI3CLK output enable" ]
    pub fn pllpen(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:14 - Main PLL multiplication factor for VCO" ]
    pub fn plln(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:6 - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock" ]
    pub fn pllm(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source" ]
    pub fn pllsrc(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PllcfgrW {
    bits: u32,
}

impl PllcfgrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PllcfgrW { bits: 4096u32 }
    }
    # [ doc = "Bits 25:26 - Main PLL division factor for PLLCLK (system clock)" ]
    pub fn pllr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 25u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 24 - Main PLL PLLCLK output enable" ]
    pub fn pllren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 21:22 - Main PLL division factor for PLLUSB1CLK(48 MHz clock)" ]
    pub fn pllq(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 21u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 20 - Main PLL PLLUSB1CLK output enable" ]
    pub fn pllqen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Main PLL division factor for PLLSAI3CLK (SAI1 and SAI2 clock)" ]
    pub fn pllp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Main PLL PLLSAI3CLK output enable" ]
    pub fn pllpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 8:14 - Main PLL multiplication factor for VCO" ]
    pub fn plln(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:6 - Division factor for the main PLL and audio PLL (PLLSAI1 and PLLSAI2) input clock" ]
    pub fn pllm(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:1 - Main PLL, PLLSAI1 and PLLSAI2 entry clock source" ]
    pub fn pllsrc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pllsai1cfgr {
    register: ::volatile_register::RW<u32>,
}

impl Pllsai1cfgr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Pllsai1cfgrR, &'w mut Pllsai1cfgrW) -> &'w mut Pllsai1cfgrW
    {
        let bits = self.register.read();
        let r = Pllsai1cfgrR { bits: bits };
        let mut w = Pllsai1cfgrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Pllsai1cfgrR {
        Pllsai1cfgrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Pllsai1cfgrW) -> &mut Pllsai1cfgrW
    {
        let mut w = Pllsai1cfgrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pllsai1cfgrR {
    bits: u32,
}

impl Pllsai1cfgrR {
    # [ doc = "Bits 25:26 - PLLSAI1 division factor for PLLADC1CLK (ADC clock)" ]
    pub fn pllsai1r(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 25u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 24 - PLLSAI1 PLLADC1CLK output enable" ]
    pub fn pllsai1ren(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 21:22 - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)" ]
    pub fn pllsai1q(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 21u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 20 - SAI1PLL PLLUSB2CLK output enable" ]
    pub fn pllsai1qen(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)" ]
    pub fn pllsai1p(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - SAI1PLL PLLSAI1CLK output enable" ]
    pub fn pllsai1pen(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:14 - SAI1PLL multiplication factor for VCO" ]
    pub fn pllsai1n(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pllsai1cfgrW {
    bits: u32,
}

impl Pllsai1cfgrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Pllsai1cfgrW { bits: 4096u32 }
    }
    # [ doc = "Bits 25:26 - PLLSAI1 division factor for PLLADC1CLK (ADC clock)" ]
    pub fn pllsai1r(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 25u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 24 - PLLSAI1 PLLADC1CLK output enable" ]
    pub fn pllsai1ren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 21:22 - SAI1PLL division factor for PLLUSB2CLK (48 MHz clock)" ]
    pub fn pllsai1q(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 21u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 20 - SAI1PLL PLLUSB2CLK output enable" ]
    pub fn pllsai1qen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - SAI1PLL division factor for PLLSAI1CLK (SAI1 or SAI2 clock)" ]
    pub fn pllsai1p(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - SAI1PLL PLLSAI1CLK output enable" ]
    pub fn pllsai1pen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 8:14 - SAI1PLL multiplication factor for VCO" ]
    pub fn pllsai1n(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pllsai2cfgr {
    register: ::volatile_register::RW<u32>,
}

impl Pllsai2cfgr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Pllsai2cfgrR, &'w mut Pllsai2cfgrW) -> &'w mut Pllsai2cfgrW
    {
        let bits = self.register.read();
        let r = Pllsai2cfgrR { bits: bits };
        let mut w = Pllsai2cfgrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Pllsai2cfgrR {
        Pllsai2cfgrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Pllsai2cfgrW) -> &mut Pllsai2cfgrW
    {
        let mut w = Pllsai2cfgrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pllsai2cfgrR {
    bits: u32,
}

impl Pllsai2cfgrR {
    # [ doc = "Bits 25:26 - PLLSAI2 division factor for PLLADC2CLK (ADC clock)" ]
    pub fn pllsai2r(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 25u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 24 - PLLSAI2 PLLADC2CLK output enable" ]
    pub fn pllsai2ren(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)" ]
    pub fn pllsai2p(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - SAI2PLL PLLSAI2CLK output enable" ]
    pub fn pllsai2pen(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:14 - SAI2PLL multiplication factor for VCO" ]
    pub fn pllsai2n(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pllsai2cfgrW {
    bits: u32,
}

impl Pllsai2cfgrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Pllsai2cfgrW { bits: 4096u32 }
    }
    # [ doc = "Bits 25:26 - PLLSAI2 division factor for PLLADC2CLK (ADC clock)" ]
    pub fn pllsai2r(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 25u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 24 - PLLSAI2 PLLADC2CLK output enable" ]
    pub fn pllsai2ren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - SAI1PLL division factor for PLLSAI2CLK (SAI1 or SAI2 clock)" ]
    pub fn pllsai2p(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - SAI2PLL PLLSAI2CLK output enable" ]
    pub fn pllsai2pen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 8:14 - SAI2PLL multiplication factor for VCO" ]
    pub fn pllsai2n(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cier {
    register: ::volatile_register::RW<u32>,
}

impl Cier {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CierR, &'w mut CierW) -> &'w mut CierW
    {
        let bits = self.register.read();
        let r = CierR { bits: bits };
        let mut w = CierW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CierR {
        CierR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CierW) -> &mut CierW
    {
        let mut w = CierW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CierR {
    bits: u32,
}

impl CierR {
    # [ doc = "Bit 9 - LSE clock security system interrupt enable" ]
    pub fn lsecssie(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - PLLSAI2 ready interrupt enable" ]
    pub fn pllsai2rdyie(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - PLLSAI1 ready interrupt enable" ]
    pub fn pllsai1rdyie(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - PLL ready interrupt enable" ]
    pub fn pllrdyie(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - HSE ready interrupt enable" ]
    pub fn hserdyie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - HSI ready interrupt enable" ]
    pub fn hsirdyie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - MSI ready interrupt enable" ]
    pub fn msirdyie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - LSE ready interrupt enable" ]
    pub fn lserdyie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - LSI ready interrupt enable" ]
    pub fn lsirdyie(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CierW {
    bits: u32,
}

impl CierW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CierW { bits: 0u32 }
    }
    # [ doc = "Bit 9 - LSE clock security system interrupt enable" ]
    pub fn lsecssie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - PLLSAI2 ready interrupt enable" ]
    pub fn pllsai2rdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - PLLSAI1 ready interrupt enable" ]
    pub fn pllsai1rdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - PLL ready interrupt enable" ]
    pub fn pllrdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - HSE ready interrupt enable" ]
    pub fn hserdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - HSI ready interrupt enable" ]
    pub fn hsirdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - MSI ready interrupt enable" ]
    pub fn msirdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - LSE ready interrupt enable" ]
    pub fn lserdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - LSI ready interrupt enable" ]
    pub fn lsirdyie(&mut self, value: bool) -> &mut Self {
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
pub struct Cifr {
    register: ::volatile_register::RO<u32>,
}

impl Cifr {
    pub fn read(&self) -> CifrR {
        CifrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CifrR {
    bits: u32,
}

impl CifrR {
    # [ doc = "Bit 9 - LSE Clock security system interrupt flag" ]
    pub fn lsecssf(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Clock security system interrupt flag" ]
    pub fn cssf(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - PLLSAI2 ready interrupt flag" ]
    pub fn pllsai2rdyf(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - PLLSAI1 ready interrupt flag" ]
    pub fn pllsai1rdyf(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - PLL ready interrupt flag" ]
    pub fn pllrdyf(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - HSE ready interrupt flag" ]
    pub fn hserdyf(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - HSI ready interrupt flag" ]
    pub fn hsirdyf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - MSI ready interrupt flag" ]
    pub fn msirdyf(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - LSE ready interrupt flag" ]
    pub fn lserdyf(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - LSI ready interrupt flag" ]
    pub fn lsirdyf(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CifrW {
    bits: u32,
}

impl CifrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CifrW { bits: 0u32 }
    }
    # [ doc = "Bit 9 - LSE Clock security system interrupt flag" ]
    pub fn lsecssf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Clock security system interrupt flag" ]
    pub fn cssf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - PLLSAI2 ready interrupt flag" ]
    pub fn pllsai2rdyf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - PLLSAI1 ready interrupt flag" ]
    pub fn pllsai1rdyf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - PLL ready interrupt flag" ]
    pub fn pllrdyf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - HSE ready interrupt flag" ]
    pub fn hserdyf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - HSI ready interrupt flag" ]
    pub fn hsirdyf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - MSI ready interrupt flag" ]
    pub fn msirdyf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - LSE ready interrupt flag" ]
    pub fn lserdyf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - LSI ready interrupt flag" ]
    pub fn lsirdyf(&mut self, value: bool) -> &mut Self {
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
pub struct Cicr {
    register: ::volatile_register::WO<u32>,
}

impl Cicr {
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut CicrW) -> &mut CicrW
    {
        let mut w = CicrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CicrR {
    bits: u32,
}

impl CicrR {
    # [ doc = "Bit 9 - LSE Clock security system interrupt clear" ]
    pub fn lsecssc(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Clock security system interrupt clear" ]
    pub fn cssc(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - PLLSAI2 ready interrupt clear" ]
    pub fn pllsai2rdyc(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - PLLSAI1 ready interrupt clear" ]
    pub fn pllsai1rdyc(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - PLL ready interrupt clear" ]
    pub fn pllrdyc(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - HSE ready interrupt clear" ]
    pub fn hserdyc(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - HSI ready interrupt clear" ]
    pub fn hsirdyc(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - MSI ready interrupt clear" ]
    pub fn msirdyc(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - LSE ready interrupt clear" ]
    pub fn lserdyc(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - LSI ready interrupt clear" ]
    pub fn lsirdyc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CicrW {
    bits: u32,
}

impl CicrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CicrW { bits: 0u32 }
    }
    # [ doc = "Bit 9 - LSE Clock security system interrupt clear" ]
    pub fn lsecssc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Clock security system interrupt clear" ]
    pub fn cssc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - PLLSAI2 ready interrupt clear" ]
    pub fn pllsai2rdyc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - PLLSAI1 ready interrupt clear" ]
    pub fn pllsai1rdyc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - PLL ready interrupt clear" ]
    pub fn pllrdyc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - HSE ready interrupt clear" ]
    pub fn hserdyc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - HSI ready interrupt clear" ]
    pub fn hsirdyc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - MSI ready interrupt clear" ]
    pub fn msirdyc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - LSE ready interrupt clear" ]
    pub fn lserdyc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - LSI ready interrupt clear" ]
    pub fn lsirdyc(&mut self, value: bool) -> &mut Self {
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
pub struct Ahb1rstr {
    register: ::volatile_register::RW<u32>,
}

impl Ahb1rstr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ahb1rstrR, &'w mut Ahb1rstrW) -> &'w mut Ahb1rstrW
    {
        let bits = self.register.read();
        let r = Ahb1rstrR { bits: bits };
        let mut w = Ahb1rstrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ahb1rstrR {
        Ahb1rstrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ahb1rstrW) -> &mut Ahb1rstrW
    {
        let mut w = Ahb1rstrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb1rstrR {
    bits: u32,
}

impl Ahb1rstrR {
    # [ doc = "Bit 16 - Touch Sensing Controller reset" ]
    pub fn tscrst(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Reserved" ]
    pub fn crcrst(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Flash memory interface reset" ]
    pub fn flashrst(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - DMA2 reset" ]
    pub fn dma2rst(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - DMA1 reset" ]
    pub fn dma1rst(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb1rstrW {
    bits: u32,
}

impl Ahb1rstrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ahb1rstrW { bits: 0u32 }
    }
    # [ doc = "Bit 16 - Touch Sensing Controller reset" ]
    pub fn tscrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Reserved" ]
    pub fn crcrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Flash memory interface reset" ]
    pub fn flashrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - DMA2 reset" ]
    pub fn dma2rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - DMA1 reset" ]
    pub fn dma1rst(&mut self, value: bool) -> &mut Self {
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
pub struct Ahb2rstr {
    register: ::volatile_register::RW<u32>,
}

impl Ahb2rstr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ahb2rstrR, &'w mut Ahb2rstrW) -> &'w mut Ahb2rstrW
    {
        let bits = self.register.read();
        let r = Ahb2rstrR { bits: bits };
        let mut w = Ahb2rstrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ahb2rstrR {
        Ahb2rstrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ahb2rstrW) -> &mut Ahb2rstrW
    {
        let mut w = Ahb2rstrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb2rstrR {
    bits: u32,
}

impl Ahb2rstrR {
    # [ doc = "Bit 18 - Random number generator reset" ]
    pub fn rngrst(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - AES hardware accelerator reset" ]
    pub fn aesrst(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - ADC reset" ]
    pub fn adcrst(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - USB OTG FS reset" ]
    pub fn otgfsrst(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - IO port H reset" ]
    pub fn gpiohrst(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - IO port G reset" ]
    pub fn gpiogrst(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - IO port F reset" ]
    pub fn gpiofrst(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - IO port E reset" ]
    pub fn gpioerst(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - IO port D reset" ]
    pub fn gpiodrst(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - IO port C reset" ]
    pub fn gpiocrst(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - IO port B reset" ]
    pub fn gpiobrst(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - IO port A reset" ]
    pub fn gpioarst(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb2rstrW {
    bits: u32,
}

impl Ahb2rstrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ahb2rstrW { bits: 0u32 }
    }
    # [ doc = "Bit 18 - Random number generator reset" ]
    pub fn rngrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - AES hardware accelerator reset" ]
    pub fn aesrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - ADC reset" ]
    pub fn adcrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - USB OTG FS reset" ]
    pub fn otgfsrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - IO port H reset" ]
    pub fn gpiohrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - IO port G reset" ]
    pub fn gpiogrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - IO port F reset" ]
    pub fn gpiofrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - IO port E reset" ]
    pub fn gpioerst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - IO port D reset" ]
    pub fn gpiodrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - IO port C reset" ]
    pub fn gpiocrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - IO port B reset" ]
    pub fn gpiobrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - IO port A reset" ]
    pub fn gpioarst(&mut self, value: bool) -> &mut Self {
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
pub struct Ahb3rstr {
    register: ::volatile_register::RW<u32>,
}

impl Ahb3rstr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ahb3rstrR, &'w mut Ahb3rstrW) -> &'w mut Ahb3rstrW
    {
        let bits = self.register.read();
        let r = Ahb3rstrR { bits: bits };
        let mut w = Ahb3rstrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ahb3rstrR {
        Ahb3rstrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ahb3rstrW) -> &mut Ahb3rstrW
    {
        let mut w = Ahb3rstrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb3rstrR {
    bits: u32,
}

impl Ahb3rstrR {
    # [ doc = "Bit 8 - Quad SPI memory interface reset" ]
    pub fn qspirst(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Flexible memory controller reset" ]
    pub fn fmcrst(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb3rstrW {
    bits: u32,
}

impl Ahb3rstrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ahb3rstrW { bits: 0u32 }
    }
    # [ doc = "Bit 8 - Quad SPI memory interface reset" ]
    pub fn qspirst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Flexible memory controller reset" ]
    pub fn fmcrst(&mut self, value: bool) -> &mut Self {
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
pub struct Apb1rstr1 {
    register: ::volatile_register::RW<u32>,
}

impl Apb1rstr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb1rstr1R, &'w mut Apb1rstr1W) -> &'w mut Apb1rstr1W
    {
        let bits = self.register.read();
        let r = Apb1rstr1R { bits: bits };
        let mut w = Apb1rstr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb1rstr1R {
        Apb1rstr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb1rstr1W) -> &mut Apb1rstr1W
    {
        let mut w = Apb1rstr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1rstr1R {
    bits: u32,
}

impl Apb1rstr1R {
    # [ doc = "Bit 31 - Low Power Timer 1 reset" ]
    pub fn lptim1rst(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - OPAMP interface reset" ]
    pub fn opamprst(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - DAC1 interface reset" ]
    pub fn dac1rst(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - Power interface reset" ]
    pub fn pwrrst(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - CAN1 reset" ]
    pub fn can1rst(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - I2C3 reset" ]
    pub fn i2c3rst(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - I2C2 reset" ]
    pub fn i2c2rst(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - I2C1 reset" ]
    pub fn i2c1rst(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - UART5 reset" ]
    pub fn uart5rst(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - UART4 reset" ]
    pub fn uart4rst(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - USART3 reset" ]
    pub fn usart3rst(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - USART2 reset" ]
    pub fn usart2rst(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - SPI3 reset" ]
    pub fn spi3rst(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - SPI2 reset" ]
    pub fn spi2rst(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - LCD interface reset" ]
    pub fn lcdrst(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - TIM7 timer reset" ]
    pub fn tim7rst(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - TIM6 timer reset" ]
    pub fn tim6rst(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - TIM5 timer reset" ]
    pub fn tim5rst(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - TIM3 timer reset" ]
    pub fn tim4rst(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - TIM3 timer reset" ]
    pub fn tim3rst(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - TIM2 timer reset" ]
    pub fn tim2rst(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1rstr1W {
    bits: u32,
}

impl Apb1rstr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb1rstr1W { bits: 0u32 }
    }
    # [ doc = "Bit 31 - Low Power Timer 1 reset" ]
    pub fn lptim1rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - OPAMP interface reset" ]
    pub fn opamprst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - DAC1 interface reset" ]
    pub fn dac1rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Power interface reset" ]
    pub fn pwrrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - CAN1 reset" ]
    pub fn can1rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - I2C3 reset" ]
    pub fn i2c3rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - I2C2 reset" ]
    pub fn i2c2rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - I2C1 reset" ]
    pub fn i2c1rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - UART5 reset" ]
    pub fn uart5rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - UART4 reset" ]
    pub fn uart4rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - USART3 reset" ]
    pub fn usart3rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - USART2 reset" ]
    pub fn usart2rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - SPI3 reset" ]
    pub fn spi3rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - SPI2 reset" ]
    pub fn spi2rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - LCD interface reset" ]
    pub fn lcdrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - TIM7 timer reset" ]
    pub fn tim7rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - TIM6 timer reset" ]
    pub fn tim6rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - TIM5 timer reset" ]
    pub fn tim5rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - TIM3 timer reset" ]
    pub fn tim4rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - TIM3 timer reset" ]
    pub fn tim3rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - TIM2 timer reset" ]
    pub fn tim2rst(&mut self, value: bool) -> &mut Self {
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
pub struct Apb1rstr2 {
    register: ::volatile_register::RW<u32>,
}

impl Apb1rstr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb1rstr2R, &'w mut Apb1rstr2W) -> &'w mut Apb1rstr2W
    {
        let bits = self.register.read();
        let r = Apb1rstr2R { bits: bits };
        let mut w = Apb1rstr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb1rstr2R {
        Apb1rstr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb1rstr2W) -> &mut Apb1rstr2W
    {
        let mut w = Apb1rstr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1rstr2R {
    bits: u32,
}

impl Apb1rstr2R {
    # [ doc = "Bit 5 - Low-power timer 2 reset" ]
    pub fn lptim2rst(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Single wire protocol reset" ]
    pub fn swpmi1rst(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Low-power UART 1 reset" ]
    pub fn lpuart1rst(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1rstr2W {
    bits: u32,
}

impl Apb1rstr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb1rstr2W { bits: 0u32 }
    }
    # [ doc = "Bit 5 - Low-power timer 2 reset" ]
    pub fn lptim2rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Single wire protocol reset" ]
    pub fn swpmi1rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Low-power UART 1 reset" ]
    pub fn lpuart1rst(&mut self, value: bool) -> &mut Self {
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
pub struct Apb2rstr {
    register: ::volatile_register::RW<u32>,
}

impl Apb2rstr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb2rstrR, &'w mut Apb2rstrW) -> &'w mut Apb2rstrW
    {
        let bits = self.register.read();
        let r = Apb2rstrR { bits: bits };
        let mut w = Apb2rstrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb2rstrR {
        Apb2rstrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb2rstrW) -> &mut Apb2rstrW
    {
        let mut w = Apb2rstrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb2rstrR {
    bits: u32,
}

impl Apb2rstrR {
    # [ doc = "Bit 24 - Digital filters for sigma-delata modulators (DFSDM) reset" ]
    pub fn dfsdmrst(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Serial audio interface 2 (SAI2) reset" ]
    pub fn sai2rst(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Serial audio interface 1 (SAI1) reset" ]
    pub fn sai1rst(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - TIM17 timer reset" ]
    pub fn tim17rst(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - TIM16 timer reset" ]
    pub fn tim16rst(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - TIM15 timer reset" ]
    pub fn tim15rst(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - USART1 reset" ]
    pub fn usart1rst(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - TIM8 timer reset" ]
    pub fn tim8rst(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - SPI1 reset" ]
    pub fn spi1rst(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - TIM1 timer reset" ]
    pub fn tim1rst(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - SDMMC reset" ]
    pub fn sdmmcrst(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - System configuration (SYSCFG) reset" ]
    pub fn syscfgrst(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb2rstrW {
    bits: u32,
}

impl Apb2rstrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb2rstrW { bits: 0u32 }
    }
    # [ doc = "Bit 24 - Digital filters for sigma-delata modulators (DFSDM) reset" ]
    pub fn dfsdmrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Serial audio interface 2 (SAI2) reset" ]
    pub fn sai2rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Serial audio interface 1 (SAI1) reset" ]
    pub fn sai1rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - TIM17 timer reset" ]
    pub fn tim17rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - TIM16 timer reset" ]
    pub fn tim16rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - TIM15 timer reset" ]
    pub fn tim15rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - USART1 reset" ]
    pub fn usart1rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - TIM8 timer reset" ]
    pub fn tim8rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - SPI1 reset" ]
    pub fn spi1rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - TIM1 timer reset" ]
    pub fn tim1rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - SDMMC reset" ]
    pub fn sdmmcrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - System configuration (SYSCFG) reset" ]
    pub fn syscfgrst(&mut self, value: bool) -> &mut Self {
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
pub struct Ahb1enr {
    register: ::volatile_register::RW<u32>,
}

impl Ahb1enr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ahb1enrR, &'w mut Ahb1enrW) -> &'w mut Ahb1enrW
    {
        let bits = self.register.read();
        let r = Ahb1enrR { bits: bits };
        let mut w = Ahb1enrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ahb1enrR {
        Ahb1enrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ahb1enrW) -> &mut Ahb1enrW
    {
        let mut w = Ahb1enrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb1enrR {
    bits: u32,
}

impl Ahb1enrR {
    # [ doc = "Bit 16 - Touch Sensing Controller clock enable" ]
    pub fn tscen(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Reserved" ]
    pub fn crcen(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Flash memory interface clock enable" ]
    pub fn flashen(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - DMA2 clock enable" ]
    pub fn dma2en(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - DMA1 clock enable" ]
    pub fn dma1en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb1enrW {
    bits: u32,
}

impl Ahb1enrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ahb1enrW { bits: 256u32 }
    }
    # [ doc = "Bit 16 - Touch Sensing Controller clock enable" ]
    pub fn tscen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Reserved" ]
    pub fn crcen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Flash memory interface clock enable" ]
    pub fn flashen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - DMA2 clock enable" ]
    pub fn dma2en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - DMA1 clock enable" ]
    pub fn dma1en(&mut self, value: bool) -> &mut Self {
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
pub struct Ahb2enr {
    register: ::volatile_register::RW<u32>,
}

impl Ahb2enr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ahb2enrR, &'w mut Ahb2enrW) -> &'w mut Ahb2enrW
    {
        let bits = self.register.read();
        let r = Ahb2enrR { bits: bits };
        let mut w = Ahb2enrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ahb2enrR {
        Ahb2enrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ahb2enrW) -> &mut Ahb2enrW
    {
        let mut w = Ahb2enrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb2enrR {
    bits: u32,
}

impl Ahb2enrR {
    # [ doc = "Bit 18 - Random Number Generator clock enable" ]
    pub fn rngen(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - AES accelerator clock enable" ]
    pub fn aesen(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - ADC clock enable" ]
    pub fn adcen(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - OTG full speed clock enable" ]
    pub fn otgfsen(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - IO port H clock enable" ]
    pub fn gpiohen(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - IO port G clock enable" ]
    pub fn gpiogen(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - IO port F clock enable" ]
    pub fn gpiofen(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - IO port E clock enable" ]
    pub fn gpioeen(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - IO port D clock enable" ]
    pub fn gpioden(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - IO port C clock enable" ]
    pub fn gpiocen(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - IO port B clock enable" ]
    pub fn gpioben(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - IO port A clock enable" ]
    pub fn gpioaen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb2enrW {
    bits: u32,
}

impl Ahb2enrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ahb2enrW { bits: 0u32 }
    }
    # [ doc = "Bit 18 - Random Number Generator clock enable" ]
    pub fn rngen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - AES accelerator clock enable" ]
    pub fn aesen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - ADC clock enable" ]
    pub fn adcen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - OTG full speed clock enable" ]
    pub fn otgfsen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - IO port H clock enable" ]
    pub fn gpiohen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - IO port G clock enable" ]
    pub fn gpiogen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - IO port F clock enable" ]
    pub fn gpiofen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - IO port E clock enable" ]
    pub fn gpioeen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - IO port D clock enable" ]
    pub fn gpioden(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - IO port C clock enable" ]
    pub fn gpiocen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - IO port B clock enable" ]
    pub fn gpioben(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - IO port A clock enable" ]
    pub fn gpioaen(&mut self, value: bool) -> &mut Self {
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
pub struct Ahb3enr {
    register: ::volatile_register::RW<u32>,
}

impl Ahb3enr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ahb3enrR, &'w mut Ahb3enrW) -> &'w mut Ahb3enrW
    {
        let bits = self.register.read();
        let r = Ahb3enrR { bits: bits };
        let mut w = Ahb3enrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ahb3enrR {
        Ahb3enrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ahb3enrW) -> &mut Ahb3enrW
    {
        let mut w = Ahb3enrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb3enrR {
    bits: u32,
}

impl Ahb3enrR {
    # [ doc = "Bit 8 - QSPIEN" ]
    pub fn qspien(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Flexible memory controller clock enable" ]
    pub fn fmcen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb3enrW {
    bits: u32,
}

impl Ahb3enrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ahb3enrW { bits: 0u32 }
    }
    # [ doc = "Bit 8 - QSPIEN" ]
    pub fn qspien(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Flexible memory controller clock enable" ]
    pub fn fmcen(&mut self, value: bool) -> &mut Self {
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
pub struct Apb1enr1 {
    register: ::volatile_register::RW<u32>,
}

impl Apb1enr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb1enr1R, &'w mut Apb1enr1W) -> &'w mut Apb1enr1W
    {
        let bits = self.register.read();
        let r = Apb1enr1R { bits: bits };
        let mut w = Apb1enr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb1enr1R {
        Apb1enr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb1enr1W) -> &mut Apb1enr1W
    {
        let mut w = Apb1enr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1enr1R {
    bits: u32,
}

impl Apb1enr1R {
    # [ doc = "Bit 31 - Low power timer 1 clock enable" ]
    pub fn lptim1en(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - OPAMP interface clock enable" ]
    pub fn opampen(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - DAC1 interface clock enable" ]
    pub fn dac1en(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - Power interface clock enable" ]
    pub fn pwren(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - CAN1 clock enable" ]
    pub fn can1en(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - I2C3 clock enable" ]
    pub fn i2c3en(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - I2C2 clock enable" ]
    pub fn i2c2en(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - I2C1 clock enable" ]
    pub fn i2c1en(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - UART5 clock enable" ]
    pub fn uart5en(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - UART4 clock enable" ]
    pub fn uart4en(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - USART3 clock enable" ]
    pub fn usart3en(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - USART2 clock enable" ]
    pub fn usart2en(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - SPI3 clock enable" ]
    pub fn sp3en(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - SPI2 clock enable" ]
    pub fn spi2en(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Window watchdog clock enable" ]
    pub fn wwdgen(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - LCD clock enable" ]
    pub fn lcden(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - TIM7 timer clock enable" ]
    pub fn tim7en(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - TIM6 timer clock enable" ]
    pub fn tim6en(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Reserved" ]
    pub fn tim5en(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - TIM4 timer clock enable" ]
    pub fn tim4en(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - TIM3 timer clock enable" ]
    pub fn tim3en(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - TIM2 timer clock enable" ]
    pub fn tim2en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1enr1W {
    bits: u32,
}

impl Apb1enr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb1enr1W { bits: 0u32 }
    }
    # [ doc = "Bit 31 - Low power timer 1 clock enable" ]
    pub fn lptim1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - OPAMP interface clock enable" ]
    pub fn opampen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - DAC1 interface clock enable" ]
    pub fn dac1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Power interface clock enable" ]
    pub fn pwren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - CAN1 clock enable" ]
    pub fn can1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - I2C3 clock enable" ]
    pub fn i2c3en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - I2C2 clock enable" ]
    pub fn i2c2en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - I2C1 clock enable" ]
    pub fn i2c1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - UART5 clock enable" ]
    pub fn uart5en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - UART4 clock enable" ]
    pub fn uart4en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - USART3 clock enable" ]
    pub fn usart3en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - USART2 clock enable" ]
    pub fn usart2en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - SPI3 clock enable" ]
    pub fn sp3en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - SPI2 clock enable" ]
    pub fn spi2en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Window watchdog clock enable" ]
    pub fn wwdgen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - LCD clock enable" ]
    pub fn lcden(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - TIM7 timer clock enable" ]
    pub fn tim7en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - TIM6 timer clock enable" ]
    pub fn tim6en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Reserved" ]
    pub fn tim5en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - TIM4 timer clock enable" ]
    pub fn tim4en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - TIM3 timer clock enable" ]
    pub fn tim3en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - TIM2 timer clock enable" ]
    pub fn tim2en(&mut self, value: bool) -> &mut Self {
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
pub struct Apb1enr2 {
    register: ::volatile_register::RW<u32>,
}

impl Apb1enr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb1enr2R, &'w mut Apb1enr2W) -> &'w mut Apb1enr2W
    {
        let bits = self.register.read();
        let r = Apb1enr2R { bits: bits };
        let mut w = Apb1enr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb1enr2R {
        Apb1enr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb1enr2W) -> &mut Apb1enr2W
    {
        let mut w = Apb1enr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1enr2R {
    bits: u32,
}

impl Apb1enr2R {
    # [ doc = "Bit 5 - LPTIM2EN" ]
    pub fn lptim2en(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Single wire protocol clock enable" ]
    pub fn swpmi1en(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Low power UART 1 clock enable" ]
    pub fn lpuart1en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1enr2W {
    bits: u32,
}

impl Apb1enr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb1enr2W { bits: 0u32 }
    }
    # [ doc = "Bit 5 - LPTIM2EN" ]
    pub fn lptim2en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Single wire protocol clock enable" ]
    pub fn swpmi1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Low power UART 1 clock enable" ]
    pub fn lpuart1en(&mut self, value: bool) -> &mut Self {
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
pub struct Apb2enr {
    register: ::volatile_register::RW<u32>,
}

impl Apb2enr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb2enrR, &'w mut Apb2enrW) -> &'w mut Apb2enrW
    {
        let bits = self.register.read();
        let r = Apb2enrR { bits: bits };
        let mut w = Apb2enrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb2enrR {
        Apb2enrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb2enrW) -> &mut Apb2enrW
    {
        let mut w = Apb2enrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb2enrR {
    bits: u32,
}

impl Apb2enrR {
    # [ doc = "Bit 24 - DFSDM timer clock enable" ]
    pub fn dfsdmen(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - SAI2 clock enable" ]
    pub fn sai2en(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - SAI1 clock enable" ]
    pub fn sai1en(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - TIM17 timer clock enable" ]
    pub fn tim17en(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - TIM16 timer clock enable" ]
    pub fn tim16en(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - TIM15 timer clock enable" ]
    pub fn tim15en(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - USART1clock enable" ]
    pub fn usart1en(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - TIM8 timer clock enable" ]
    pub fn tim8en(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - SPI1 clock enable" ]
    pub fn spi1en(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - TIM1 timer clock enable" ]
    pub fn tim1en(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - SDMMC clock enable" ]
    pub fn sdmmcen(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Firewall clock enable" ]
    pub fn firewallen(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - SYSCFG clock enable" ]
    pub fn syscfgen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb2enrW {
    bits: u32,
}

impl Apb2enrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb2enrW { bits: 0u32 }
    }
    # [ doc = "Bit 24 - DFSDM timer clock enable" ]
    pub fn dfsdmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - SAI2 clock enable" ]
    pub fn sai2en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - SAI1 clock enable" ]
    pub fn sai1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - TIM17 timer clock enable" ]
    pub fn tim17en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - TIM16 timer clock enable" ]
    pub fn tim16en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - TIM15 timer clock enable" ]
    pub fn tim15en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - USART1clock enable" ]
    pub fn usart1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - TIM8 timer clock enable" ]
    pub fn tim8en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - SPI1 clock enable" ]
    pub fn spi1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - TIM1 timer clock enable" ]
    pub fn tim1en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - SDMMC clock enable" ]
    pub fn sdmmcen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Firewall clock enable" ]
    pub fn firewallen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - SYSCFG clock enable" ]
    pub fn syscfgen(&mut self, value: bool) -> &mut Self {
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
pub struct Ahb1smenr {
    register: ::volatile_register::RW<u32>,
}

impl Ahb1smenr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ahb1smenrR, &'w mut Ahb1smenrW) -> &'w mut Ahb1smenrW
    {
        let bits = self.register.read();
        let r = Ahb1smenrR { bits: bits };
        let mut w = Ahb1smenrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ahb1smenrR {
        Ahb1smenrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ahb1smenrW) -> &mut Ahb1smenrW
    {
        let mut w = Ahb1smenrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb1smenrR {
    bits: u32,
}

impl Ahb1smenrR {
    # [ doc = "Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes" ]
    pub fn tscsmen(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - CRCSMEN" ]
    pub fn crcsmen(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes" ]
    pub fn sram1smen(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes" ]
    pub fn flashsmen(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - DMA2 clocks enable during Sleep and Stop modes" ]
    pub fn dma2smen(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - DMA1 clocks enable during Sleep and Stop modes" ]
    pub fn dma1smen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb1smenrW {
    bits: u32,
}

impl Ahb1smenrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ahb1smenrW { bits: 70403u32 }
    }
    # [ doc = "Bit 16 - Touch Sensing Controller clocks enable during Sleep and Stop modes" ]
    pub fn tscsmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - CRCSMEN" ]
    pub fn crcsmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - SRAM1 interface clocks enable during Sleep and Stop modes" ]
    pub fn sram1smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Flash memory interface clocks enable during Sleep and Stop modes" ]
    pub fn flashsmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - DMA2 clocks enable during Sleep and Stop modes" ]
    pub fn dma2smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - DMA1 clocks enable during Sleep and Stop modes" ]
    pub fn dma1smen(&mut self, value: bool) -> &mut Self {
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
pub struct Ahb2smenr {
    register: ::volatile_register::RW<u32>,
}

impl Ahb2smenr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ahb2smenrR, &'w mut Ahb2smenrW) -> &'w mut Ahb2smenrW
    {
        let bits = self.register.read();
        let r = Ahb2smenrR { bits: bits };
        let mut w = Ahb2smenrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ahb2smenrR {
        Ahb2smenrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ahb2smenrW) -> &mut Ahb2smenrW
    {
        let mut w = Ahb2smenrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb2smenrR {
    bits: u32,
}

impl Ahb2smenrR {
    # [ doc = "Bit 18 - Random Number Generator clocks enable during Sleep and Stop modes" ]
    pub fn rngsmen(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - AES accelerator clocks enable during Sleep and Stop modes" ]
    pub fn aessmen(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - ADC clocks enable during Sleep and Stop modes" ]
    pub fn adcfssmen(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - OTG full speed clocks enable during Sleep and Stop modes" ]
    pub fn otgfssmen(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - SRAM2 interface clocks enable during Sleep and Stop modes" ]
    pub fn sram2smen(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - IO port H clocks enable during Sleep and Stop modes" ]
    pub fn gpiohsmen(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - IO port G clocks enable during Sleep and Stop modes" ]
    pub fn gpiogsmen(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - IO port F clocks enable during Sleep and Stop modes" ]
    pub fn gpiofsmen(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - IO port E clocks enable during Sleep and Stop modes" ]
    pub fn gpioesmen(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - IO port D clocks enable during Sleep and Stop modes" ]
    pub fn gpiodsmen(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - IO port C clocks enable during Sleep and Stop modes" ]
    pub fn gpiocsmen(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - IO port B clocks enable during Sleep and Stop modes" ]
    pub fn gpiobsmen(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - IO port A clocks enable during Sleep and Stop modes" ]
    pub fn gpioasmen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb2smenrW {
    bits: u32,
}

impl Ahb2smenrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ahb2smenrW { bits: 340735u32 }
    }
    # [ doc = "Bit 18 - Random Number Generator clocks enable during Sleep and Stop modes" ]
    pub fn rngsmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - AES accelerator clocks enable during Sleep and Stop modes" ]
    pub fn aessmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - ADC clocks enable during Sleep and Stop modes" ]
    pub fn adcfssmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - OTG full speed clocks enable during Sleep and Stop modes" ]
    pub fn otgfssmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - SRAM2 interface clocks enable during Sleep and Stop modes" ]
    pub fn sram2smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - IO port H clocks enable during Sleep and Stop modes" ]
    pub fn gpiohsmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - IO port G clocks enable during Sleep and Stop modes" ]
    pub fn gpiogsmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - IO port F clocks enable during Sleep and Stop modes" ]
    pub fn gpiofsmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - IO port E clocks enable during Sleep and Stop modes" ]
    pub fn gpioesmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - IO port D clocks enable during Sleep and Stop modes" ]
    pub fn gpiodsmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - IO port C clocks enable during Sleep and Stop modes" ]
    pub fn gpiocsmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - IO port B clocks enable during Sleep and Stop modes" ]
    pub fn gpiobsmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - IO port A clocks enable during Sleep and Stop modes" ]
    pub fn gpioasmen(&mut self, value: bool) -> &mut Self {
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
pub struct Ahb3smenr {
    register: ::volatile_register::RW<u32>,
}

impl Ahb3smenr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ahb3smenrR, &'w mut Ahb3smenrW) -> &'w mut Ahb3smenrW
    {
        let bits = self.register.read();
        let r = Ahb3smenrR { bits: bits };
        let mut w = Ahb3smenrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ahb3smenrR {
        Ahb3smenrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ahb3smenrW) -> &mut Ahb3smenrW
    {
        let mut w = Ahb3smenrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb3smenrR {
    bits: u32,
}

impl Ahb3smenrR {
    # [ doc = "Bit 8 - QSPISMEN" ]
    pub fn qspismen(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Flexible memory controller clocks enable during Sleep and Stop modes" ]
    pub fn fmcsmen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ahb3smenrW {
    bits: u32,
}

impl Ahb3smenrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ahb3smenrW { bits: 257u32 }
    }
    # [ doc = "Bit 8 - QSPISMEN" ]
    pub fn qspismen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Flexible memory controller clocks enable during Sleep and Stop modes" ]
    pub fn fmcsmen(&mut self, value: bool) -> &mut Self {
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
pub struct Apb1smenr1 {
    register: ::volatile_register::RW<u32>,
}

impl Apb1smenr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb1smenr1R, &'w mut Apb1smenr1W) -> &'w mut Apb1smenr1W
    {
        let bits = self.register.read();
        let r = Apb1smenr1R { bits: bits };
        let mut w = Apb1smenr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb1smenr1R {
        Apb1smenr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb1smenr1W) -> &mut Apb1smenr1W
    {
        let mut w = Apb1smenr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1smenr1R {
    bits: u32,
}

impl Apb1smenr1R {
    # [ doc = "Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes" ]
    pub fn lptim1smen(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - OPAMP interface clocks enable during Sleep and Stop modes" ]
    pub fn opampsmen(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - DAC1 interface clocks enable during Sleep and Stop modes" ]
    pub fn dac1smen(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - Power interface clocks enable during Sleep and Stop modes" ]
    pub fn pwrsmen(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - CAN1 clocks enable during Sleep and Stop modes" ]
    pub fn can1smen(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - I2C3 clocks enable during Sleep and Stop modes" ]
    pub fn i2c3smen(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - I2C2 clocks enable during Sleep and Stop modes" ]
    pub fn i2c2smen(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - I2C1 clocks enable during Sleep and Stop modes" ]
    pub fn i2c1smen(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - UART5 clocks enable during Sleep and Stop modes" ]
    pub fn uart5smen(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - UART4 clocks enable during Sleep and Stop modes" ]
    pub fn uart4smen(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - USART3 clocks enable during Sleep and Stop modes" ]
    pub fn usart3smen(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - USART2 clocks enable during Sleep and Stop modes" ]
    pub fn usart2smen(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - SPI3 clocks enable during Sleep and Stop modes" ]
    pub fn sp3smen(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - SPI2 clocks enable during Sleep and Stop modes" ]
    pub fn spi2smen(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Window watchdog clocks enable during Sleep and Stop modes" ]
    pub fn wwdgsmen(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - LCD clocks enable during Sleep and Stop modes" ]
    pub fn lcdsmen(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim7smen(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim6smen(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Reserved" ]
    pub fn tim5smen(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim4smen(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim3smen(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim2smen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1smenr1W {
    bits: u32,
}

impl Apb1smenr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb1smenr1W { bits: 4076784191u32 }
    }
    # [ doc = "Bit 31 - Low power timer 1 clocks enable during Sleep and Stop modes" ]
    pub fn lptim1smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - OPAMP interface clocks enable during Sleep and Stop modes" ]
    pub fn opampsmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - DAC1 interface clocks enable during Sleep and Stop modes" ]
    pub fn dac1smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Power interface clocks enable during Sleep and Stop modes" ]
    pub fn pwrsmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - CAN1 clocks enable during Sleep and Stop modes" ]
    pub fn can1smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - I2C3 clocks enable during Sleep and Stop modes" ]
    pub fn i2c3smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - I2C2 clocks enable during Sleep and Stop modes" ]
    pub fn i2c2smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - I2C1 clocks enable during Sleep and Stop modes" ]
    pub fn i2c1smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - UART5 clocks enable during Sleep and Stop modes" ]
    pub fn uart5smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - UART4 clocks enable during Sleep and Stop modes" ]
    pub fn uart4smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - USART3 clocks enable during Sleep and Stop modes" ]
    pub fn usart3smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - USART2 clocks enable during Sleep and Stop modes" ]
    pub fn usart2smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - SPI3 clocks enable during Sleep and Stop modes" ]
    pub fn sp3smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - SPI2 clocks enable during Sleep and Stop modes" ]
    pub fn spi2smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Window watchdog clocks enable during Sleep and Stop modes" ]
    pub fn wwdgsmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - LCD clocks enable during Sleep and Stop modes" ]
    pub fn lcdsmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - TIM7 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim7smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - TIM6 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim6smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Reserved" ]
    pub fn tim5smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - TIM4 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim4smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - TIM3 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim3smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - TIM2 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim2smen(&mut self, value: bool) -> &mut Self {
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
pub struct Apb1smenr2 {
    register: ::volatile_register::RW<u32>,
}

impl Apb1smenr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb1smenr2R, &'w mut Apb1smenr2W) -> &'w mut Apb1smenr2W
    {
        let bits = self.register.read();
        let r = Apb1smenr2R { bits: bits };
        let mut w = Apb1smenr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb1smenr2R {
        Apb1smenr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb1smenr2W) -> &mut Apb1smenr2W
    {
        let mut w = Apb1smenr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1smenr2R {
    bits: u32,
}

impl Apb1smenr2R {
    # [ doc = "Bit 5 - LPTIM2SMEN" ]
    pub fn lptim2smen(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Single wire protocol clocks enable during Sleep and Stop modes" ]
    pub fn swpmi1smen(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes" ]
    pub fn lpuart1smen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb1smenr2W {
    bits: u32,
}

impl Apb1smenr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb1smenr2W { bits: 37u32 }
    }
    # [ doc = "Bit 5 - LPTIM2SMEN" ]
    pub fn lptim2smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Single wire protocol clocks enable during Sleep and Stop modes" ]
    pub fn swpmi1smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Low power UART 1 clocks enable during Sleep and Stop modes" ]
    pub fn lpuart1smen(&mut self, value: bool) -> &mut Self {
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
pub struct Apb2smenr {
    register: ::volatile_register::RW<u32>,
}

impl Apb2smenr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Apb2smenrR, &'w mut Apb2smenrW) -> &'w mut Apb2smenrW
    {
        let bits = self.register.read();
        let r = Apb2smenrR { bits: bits };
        let mut w = Apb2smenrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Apb2smenrR {
        Apb2smenrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Apb2smenrW) -> &mut Apb2smenrW
    {
        let mut w = Apb2smenrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb2smenrR {
    bits: u32,
}

impl Apb2smenrR {
    # [ doc = "Bit 24 - DFSDM timer clocks enable during Sleep and Stop modes" ]
    pub fn dfsdmsmen(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - SAI2 clocks enable during Sleep and Stop modes" ]
    pub fn sai2smen(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - SAI1 clocks enable during Sleep and Stop modes" ]
    pub fn sai1smen(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim17smen(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim16smen(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim15smen(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - USART1clocks enable during Sleep and Stop modes" ]
    pub fn usart1smen(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim8smen(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - SPI1 clocks enable during Sleep and Stop modes" ]
    pub fn spi1smen(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim1smen(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - SDMMC clocks enable during Sleep and Stop modes" ]
    pub fn sdmmcsmen(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - SYSCFG clocks enable during Sleep and Stop modes" ]
    pub fn syscfgsmen(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Apb2smenrW {
    bits: u32,
}

impl Apb2smenrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Apb2smenrW { bits: 23559169u32 }
    }
    # [ doc = "Bit 24 - DFSDM timer clocks enable during Sleep and Stop modes" ]
    pub fn dfsdmsmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - SAI2 clocks enable during Sleep and Stop modes" ]
    pub fn sai2smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - SAI1 clocks enable during Sleep and Stop modes" ]
    pub fn sai1smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - TIM17 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim17smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - TIM16 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim16smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - TIM15 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim15smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - USART1clocks enable during Sleep and Stop modes" ]
    pub fn usart1smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - TIM8 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim8smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - SPI1 clocks enable during Sleep and Stop modes" ]
    pub fn spi1smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - TIM1 timer clocks enable during Sleep and Stop modes" ]
    pub fn tim1smen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - SDMMC clocks enable during Sleep and Stop modes" ]
    pub fn sdmmcsmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - SYSCFG clocks enable during Sleep and Stop modes" ]
    pub fn syscfgsmen(&mut self, value: bool) -> &mut Self {
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
pub struct Ccipr {
    register: ::volatile_register::RW<u32>,
}

impl Ccipr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CciprR, &'w mut CciprW) -> &'w mut CciprW
    {
        let bits = self.register.read();
        let r = CciprR { bits: bits };
        let mut w = CciprW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CciprR {
        CciprR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CciprW) -> &mut CciprW
    {
        let mut w = CciprW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CciprR {
    bits: u32,
}

impl CciprR {
    # [ doc = "Bit 31 - DFSDM clock source selection" ]
    pub fn dfsdmsel(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - SWPMI1 clock source selection" ]
    pub fn swpmi1sel(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 28:29 - ADCs clock source selection" ]
    pub fn adcsel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 26:27 - 48 MHz clock source selection" ]
    pub fn clk48sel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 26u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:25 - SAI2 clock source selection" ]
    pub fn sai2sel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:23 - SAI1 clock source selection" ]
    pub fn sai1sel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Low power timer 2 clock source selection" ]
    pub fn lptim2sel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 18:19 - Low power timer 1 clock source selection" ]
    pub fn lptim1sel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:17 - I2C3 clock source selection" ]
    pub fn i2c3sel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 14:15 - I2C2 clock source selection" ]
    pub fn i2c2sel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:13 - I2C1 clock source selection" ]
    pub fn i2c1sel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 10:11 - LPUART1 clock source selection" ]
    pub fn lpuart1sel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:9 - UART5 clock source selection" ]
    pub fn uart5sel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 6:7 - UART4 clock source selection" ]
    pub fn uart4sel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:5 - USART3 clock source selection" ]
    pub fn usart3sel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 2:3 - USART2 clock source selection" ]
    pub fn usart2sel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:1 - USART1 clock source selection" ]
    pub fn usart1sel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CciprW {
    bits: u32,
}

impl CciprW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CciprW { bits: 0u32 }
    }
    # [ doc = "Bit 31 - DFSDM clock source selection" ]
    pub fn dfsdmsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - SWPMI1 clock source selection" ]
    pub fn swpmi1sel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 28:29 - ADCs clock source selection" ]
    pub fn adcsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 26:27 - 48 MHz clock source selection" ]
    pub fn clk48sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 26u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:25 - SAI2 clock source selection" ]
    pub fn sai2sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:23 - SAI1 clock source selection" ]
    pub fn sai1sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Low power timer 2 clock source selection" ]
    pub fn lptim2sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 18:19 - Low power timer 1 clock source selection" ]
    pub fn lptim1sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:17 - I2C3 clock source selection" ]
    pub fn i2c3sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 14:15 - I2C2 clock source selection" ]
    pub fn i2c2sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:13 - I2C1 clock source selection" ]
    pub fn i2c1sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 10:11 - LPUART1 clock source selection" ]
    pub fn lpuart1sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:9 - UART5 clock source selection" ]
    pub fn uart5sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 6:7 - UART4 clock source selection" ]
    pub fn uart4sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:5 - USART3 clock source selection" ]
    pub fn usart3sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 2:3 - USART2 clock source selection" ]
    pub fn usart2sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:1 - USART1 clock source selection" ]
    pub fn usart1sel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bdcr {
    register: ::volatile_register::RW<u32>,
}

impl Bdcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BdcrR, &'w mut BdcrW) -> &'w mut BdcrW
    {
        let bits = self.register.read();
        let r = BdcrR { bits: bits };
        let mut w = BdcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BdcrR {
        BdcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BdcrW) -> &mut BdcrW
    {
        let mut w = BdcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BdcrR {
    bits: u32,
}

impl BdcrR {
    # [ doc = "Bit 25 - Low speed clock output selection" ]
    pub fn lscosel(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Low speed clock output enable" ]
    pub fn lscoen(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Backup domain software reset" ]
    pub fn bdrst(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - RTC clock enable" ]
    pub fn rtcen(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:9 - RTC clock source selection" ]
    pub fn rtcsel(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 6 - LSECSSD" ]
    pub fn lsecssd(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - LSECSSON" ]
    pub fn lsecsson(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 3:4 - SE oscillator drive capability" ]
    pub fn lsedrv(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 2 - LSE oscillator bypass" ]
    pub fn lsebyp(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - LSE oscillator ready" ]
    pub fn lserdy(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - LSE oscillator enable" ]
    pub fn lseon(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BdcrW {
    bits: u32,
}

impl BdcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BdcrW { bits: 0u32 }
    }
    # [ doc = "Bit 25 - Low speed clock output selection" ]
    pub fn lscosel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Low speed clock output enable" ]
    pub fn lscoen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Backup domain software reset" ]
    pub fn bdrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - RTC clock enable" ]
    pub fn rtcen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 8:9 - RTC clock source selection" ]
    pub fn rtcsel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 5 - LSECSSON" ]
    pub fn lsecsson(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 3:4 - SE oscillator drive capability" ]
    pub fn lsedrv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 2 - LSE oscillator bypass" ]
    pub fn lsebyp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - LSE oscillator enable" ]
    pub fn lseon(&mut self, value: bool) -> &mut Self {
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
pub struct Csr {
    register: ::volatile_register::RW<u32>,
}

impl Csr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CsrR, &'w mut CsrW) -> &'w mut CsrW
    {
        let bits = self.register.read();
        let r = CsrR { bits: bits };
        let mut w = CsrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CsrR {
        CsrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CsrW) -> &mut CsrW
    {
        let mut w = CsrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CsrR {
    bits: u32,
}

impl CsrR {
    # [ doc = "Bit 31 - Low-power reset flag" ]
    pub fn lpwrstf(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Window watchdog reset flag" ]
    pub fn wwdgrstf(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - Independent window watchdog reset flag" ]
    pub fn iwdgrstf(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - Software reset flag" ]
    pub fn sftrstf(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 27 - BOR flag" ]
    pub fn borrstf(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Pin reset flag" ]
    pub fn pinrstf(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Option byte loader reset flag" ]
    pub fn oblrstf(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Firewall reset flag" ]
    pub fn firewallrstf(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - Remove reset flag" ]
    pub fn rmvf(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:11 - SI range after Standby mode" ]
    pub fn msisrange(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 1 - LSI oscillator ready" ]
    pub fn lsirdy(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - LSI oscillator enable" ]
    pub fn lsion(&self) -> bool {
        const OFFSET: u8 = 0u8;
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
        CsrW { bits: 201328128u32 }
    }
    # [ doc = "Bit 23 - Remove reset flag" ]
    pub fn rmvf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 8:11 - SI range after Standby mode" ]
    pub fn msisrange(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 0 - LSI oscillator enable" ]
    pub fn lsion(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}
