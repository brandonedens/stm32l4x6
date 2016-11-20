# [ doc = "QuadSPI interface" ]
# [ repr ( C ) ]
pub struct Quadspi {
    # [ doc = "0x00 - control register" ]
    pub cr: Cr,
    # [ doc = "0x04 - device configuration register" ]
    pub dcr: Dcr,
    # [ doc = "0x08 - status register" ]
    pub sr: Sr,
    # [ doc = "0x0c - flag clear register" ]
    pub fcr: Fcr,
    # [ doc = "0x10 - data length register" ]
    pub dlr: Dlr,
    # [ doc = "0x14 - communication configuration register" ]
    pub ccr: Ccr,
    # [ doc = "0x18 - address register" ]
    pub ar: Ar,
    # [ doc = "0x1c - ABR" ]
    pub abr: Abr,
    # [ doc = "0x20 - data register" ]
    pub dr: Dr,
    # [ doc = "0x24 - polling status mask register" ]
    pub psmkr: Psmkr,
    # [ doc = "0x28 - polling status match register" ]
    pub psmar: Psmar,
    # [ doc = "0x2c - polling interval register" ]
    pub pir: Pir,
    # [ doc = "0x30 - low-power timeout register" ]
    pub lptr: Lptr,
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
    # [ doc = "Bits 24:31 - Clock prescaler" ]
    pub fn prescaler(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 23 - Polling match mode" ]
    pub fn pmm(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Automatic poll mode stop" ]
    pub fn apms(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - TimeOut interrupt enable" ]
    pub fn toie(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Status match interrupt enable" ]
    pub fn smie(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - FIFO threshold interrupt enable" ]
    pub fn ftie(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Transfer complete interrupt enable" ]
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Transfer error interrupt enable" ]
    pub fn teie(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:12 - IFO threshold level" ]
    pub fn fthres(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - FLASH memory selection" ]
    pub fn fsel(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Dual-flash mode" ]
    pub fn dfm(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Sample shift" ]
    pub fn sshift(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Timeout counter enable" ]
    pub fn tcen(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - DMA enable" ]
    pub fn dmaen(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Abort request" ]
    pub fn abort(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Enable" ]
    pub fn en(&self) -> bool {
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
    # [ doc = "Bits 24:31 - Clock prescaler" ]
    pub fn prescaler(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 23 - Polling match mode" ]
    pub fn pmm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Automatic poll mode stop" ]
    pub fn apms(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - TimeOut interrupt enable" ]
    pub fn toie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Status match interrupt enable" ]
    pub fn smie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - FIFO threshold interrupt enable" ]
    pub fn ftie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Transfer complete interrupt enable" ]
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Transfer error interrupt enable" ]
    pub fn teie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 8:12 - IFO threshold level" ]
    pub fn fthres(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - FLASH memory selection" ]
    pub fn fsel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Dual-flash mode" ]
    pub fn dfm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Sample shift" ]
    pub fn sshift(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Timeout counter enable" ]
    pub fn tcen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - DMA enable" ]
    pub fn dmaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Abort request" ]
    pub fn abort(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Enable" ]
    pub fn en(&mut self, value: bool) -> &mut Self {
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
    # [ doc = "Bits 16:20 - FLASH memory size" ]
    pub fn fsize(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:10 - Chip select high time" ]
    pub fn csht(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 0 - Mode 0 / mode 3" ]
    pub fn ckmode(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
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
    # [ doc = "Bits 16:20 - FLASH memory size" ]
    pub fn fsize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:10 - Chip select high time" ]
    pub fn csht(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 0 - Mode 0 / mode 3" ]
    pub fn ckmode(&mut self, value: bool) -> &mut Self {
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
    register: ::volatile_register::RO<u32>,
}

impl Sr {
    pub fn read(&self) -> SrR {
        SrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct SrR {
    bits: u32,
}

impl SrR {
    # [ doc = "Bits 8:14 - FIFO level" ]
    pub fn flevel(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 5 - Busy" ]
    pub fn busy(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Timeout flag" ]
    pub fn tof(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Status match flag" ]
    pub fn smf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - FIFO threshold flag" ]
    pub fn ftf(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transfer complete flag" ]
    pub fn tcf(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Transfer error flag" ]
    pub fn tef(&self) -> bool {
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
    # [ doc = "Bits 8:14 - FIFO level" ]
    pub fn flevel(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 5 - Busy" ]
    pub fn busy(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Timeout flag" ]
    pub fn tof(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Status match flag" ]
    pub fn smf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - FIFO threshold flag" ]
    pub fn ftf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transfer complete flag" ]
    pub fn tcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Transfer error flag" ]
    pub fn tef(&mut self, value: bool) -> &mut Self {
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
pub struct Fcr {
    register: ::volatile_register::RW<u32>,
}

impl Fcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FcrR, &'w mut FcrW) -> &'w mut FcrW
    {
        let bits = self.register.read();
        let r = FcrR { bits: bits };
        let mut w = FcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FcrR {
        FcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FcrW) -> &mut FcrW
    {
        let mut w = FcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FcrR {
    bits: u32,
}

impl FcrR {
    # [ doc = "Bit 4 - Clear timeout flag" ]
    pub fn ctof(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Clear status match flag" ]
    pub fn csmf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Clear transfer complete flag" ]
    pub fn ctcf(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Clear transfer error flag" ]
    pub fn ctef(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FcrW {
    bits: u32,
}

impl FcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FcrW { bits: 0u32 }
    }
    # [ doc = "Bit 4 - Clear timeout flag" ]
    pub fn ctof(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Clear status match flag" ]
    pub fn csmf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Clear transfer complete flag" ]
    pub fn ctcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Clear transfer error flag" ]
    pub fn ctef(&mut self, value: bool) -> &mut Self {
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
pub struct Dlr {
    register: ::volatile_register::RW<u32>,
}

impl Dlr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DlrR, &'w mut DlrW) -> &'w mut DlrW
    {
        let bits = self.register.read();
        let r = DlrR { bits: bits };
        let mut w = DlrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DlrR {
        DlrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DlrW) -> &mut DlrW
    {
        let mut w = DlrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DlrR {
    bits: u32,
}

impl DlrR {
    # [ doc = "Bits 0:31 - Data length" ]
    pub fn dl(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DlrW {
    bits: u32,
}

impl DlrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DlrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Data length" ]
    pub fn dl(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
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
    # [ doc = "Bit 31 - Double data rate mode" ]
    pub fn ddrm(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - DDR hold half cycle" ]
    pub fn dhhc(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - Send instruction only once mode" ]
    pub fn sioo(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 26:27 - Functional mode" ]
    pub fn fmode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 26u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:25 - Data mode" ]
    pub fn dmode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 18:22 - Number of dummy cycles" ]
    pub fn dcyc(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:17 - Alternate bytes size" ]
    pub fn absize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 14:15 - Alternate bytes mode" ]
    pub fn abmode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:13 - Address size" ]
    pub fn adsize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 10:11 - Address mode" ]
    pub fn admode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:9 - Instruction mode" ]
    pub fn imode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - Instruction" ]
    pub fn instruction(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
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
    # [ doc = "Bit 31 - Double data rate mode" ]
    pub fn ddrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - DDR hold half cycle" ]
    pub fn dhhc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Send instruction only once mode" ]
    pub fn sioo(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 26:27 - Functional mode" ]
    pub fn fmode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 26u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:25 - Data mode" ]
    pub fn dmode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 18:22 - Number of dummy cycles" ]
    pub fn dcyc(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:17 - Alternate bytes size" ]
    pub fn absize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 14:15 - Alternate bytes mode" ]
    pub fn abmode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:13 - Address size" ]
    pub fn adsize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 10:11 - Address mode" ]
    pub fn admode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:9 - Instruction mode" ]
    pub fn imode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Instruction" ]
    pub fn instruction(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ar {
    register: ::volatile_register::RW<u32>,
}

impl Ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ArR, &'w mut ArW) -> &'w mut ArW
    {
        let bits = self.register.read();
        let r = ArR { bits: bits };
        let mut w = ArW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ArR {
        ArR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ArW) -> &mut ArW
    {
        let mut w = ArW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ArR {
    bits: u32,
}

impl ArR {
    # [ doc = "Bits 0:31 - Address" ]
    pub fn address(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ArW {
    bits: u32,
}

impl ArW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ArW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Address" ]
    pub fn address(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Abr {
    register: ::volatile_register::RW<u32>,
}

impl Abr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&AbrR, &'w mut AbrW) -> &'w mut AbrW
    {
        let bits = self.register.read();
        let r = AbrR { bits: bits };
        let mut w = AbrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> AbrR {
        AbrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut AbrW) -> &mut AbrW
    {
        let mut w = AbrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AbrR {
    bits: u32,
}

impl AbrR {
    # [ doc = "Bits 0:31 - ALTERNATE" ]
    pub fn alternate(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AbrW {
    bits: u32,
}

impl AbrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        AbrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - ALTERNATE" ]
    pub fn alternate(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dr {
    register: ::volatile_register::RW<u32>,
}

impl Dr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DrR, &'w mut DrW) -> &'w mut DrW
    {
        let bits = self.register.read();
        let r = DrR { bits: bits };
        let mut w = DrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DrR {
        DrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DrW) -> &mut DrW
    {
        let mut w = DrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DrR {
    bits: u32,
}

impl DrR {
    # [ doc = "Bits 0:31 - Data" ]
    pub fn data(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
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
    # [ doc = "Bits 0:31 - Data" ]
    pub fn data(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Psmkr {
    register: ::volatile_register::RW<u32>,
}

impl Psmkr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PsmkrR, &'w mut PsmkrW) -> &'w mut PsmkrW
    {
        let bits = self.register.read();
        let r = PsmkrR { bits: bits };
        let mut w = PsmkrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PsmkrR {
        PsmkrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PsmkrW) -> &mut PsmkrW
    {
        let mut w = PsmkrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PsmkrR {
    bits: u32,
}

impl PsmkrR {
    # [ doc = "Bits 0:31 - Status mask" ]
    pub fn mask(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PsmkrW {
    bits: u32,
}

impl PsmkrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PsmkrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Status mask" ]
    pub fn mask(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Psmar {
    register: ::volatile_register::RW<u32>,
}

impl Psmar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PsmarR, &'w mut PsmarW) -> &'w mut PsmarW
    {
        let bits = self.register.read();
        let r = PsmarR { bits: bits };
        let mut w = PsmarW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PsmarR {
        PsmarR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PsmarW) -> &mut PsmarW
    {
        let mut w = PsmarW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PsmarR {
    bits: u32,
}

impl PsmarR {
    # [ doc = "Bits 0:31 - Status match" ]
    pub fn match_(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PsmarW {
    bits: u32,
}

impl PsmarW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PsmarW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Status match" ]
    pub fn match_(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pir {
    register: ::volatile_register::RW<u32>,
}

impl Pir {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PirR, &'w mut PirW) -> &'w mut PirW
    {
        let bits = self.register.read();
        let r = PirR { bits: bits };
        let mut w = PirW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PirR {
        PirR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PirW) -> &mut PirW
    {
        let mut w = PirW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PirR {
    bits: u32,
}

impl PirR {
    # [ doc = "Bits 0:15 - Polling interval" ]
    pub fn interval(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PirW {
    bits: u32,
}

impl PirW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PirW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Polling interval" ]
    pub fn interval(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Lptr {
    register: ::volatile_register::RW<u32>,
}

impl Lptr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&LptrR, &'w mut LptrW) -> &'w mut LptrW
    {
        let bits = self.register.read();
        let r = LptrR { bits: bits };
        let mut w = LptrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> LptrR {
        LptrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut LptrW) -> &mut LptrW
    {
        let mut w = LptrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct LptrR {
    bits: u32,
}

impl LptrR {
    # [ doc = "Bits 0:15 - Timeout period" ]
    pub fn timeout(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct LptrW {
    bits: u32,
}

impl LptrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        LptrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Timeout period" ]
    pub fn timeout(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
