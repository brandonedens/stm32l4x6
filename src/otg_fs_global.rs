# [ doc = "USB on the go full speed" ]
# [ repr ( C ) ]
pub struct OtgFsGlobal {
    # [ doc = "0x00 - OTG_FS control and status register (OTG_FS_GOTGCTL)" ]
    pub fs_gotgctl: FsGotgctl,
    # [ doc = "0x04 - OTG_FS interrupt register (OTG_FS_GOTGINT)" ]
    pub fs_gotgint: FsGotgint,
    # [ doc = "0x08 - OTG_FS AHB configuration register (OTG_FS_GAHBCFG)" ]
    pub fs_gahbcfg: FsGahbcfg,
    # [ doc = "0x0c - OTG_FS USB configuration register (OTG_FS_GUSBCFG)" ]
    pub fs_gusbcfg: FsGusbcfg,
    # [ doc = "0x10 - OTG_FS reset register (OTG_FS_GRSTCTL)" ]
    pub fs_grstctl: FsGrstctl,
    # [ doc = "0x14 - OTG_FS core interrupt register (OTG_FS_GINTSTS)" ]
    pub fs_gintsts: FsGintsts,
    # [ doc = "0x18 - OTG_FS interrupt mask register (OTG_FS_GINTMSK)" ]
    pub fs_gintmsk: FsGintmsk,
    # [ doc = "0x1c - OTG_FS Receive status debug read(Device mode)" ]
    pub fs_grxstsr_device: FsGrxstsrDevice,
    _reserved0: [u8; 4usize],
    # [ doc = "0x24 - OTG_FS Receive FIFO size register (OTG_FS_GRXFSIZ)" ]
    pub fs_grxfsiz: FsGrxfsiz,
    # [ doc = "0x28 - OTG_FS non-periodic transmit FIFO size register (Device mode)" ]
    pub fs_gnptxfsiz_device: FsGnptxfsizDevice,
    # [ doc = "0x2c - OTG_FS non-periodic transmit FIFO/queue status register (OTG_FS_GNPTXSTS)" ]
    pub fs_gnptxsts: FsGnptxsts,
    _reserved1: [u8; 8usize],
    # [ doc = "0x38 - OTG_FS general core configuration register (OTG_FS_GCCFG)" ]
    pub fs_gccfg: FsGccfg,
    # [ doc = "0x3c - core ID register" ]
    pub fs_cid: FsCid,
    _reserved2: [u8; 192usize],
    # [ doc = "0x100 - OTG_FS Host periodic transmit FIFO size register (OTG_FS_HPTXFSIZ)" ]
    pub fs_hptxfsiz: FsHptxfsiz,
    # [ doc = "0x104 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF2)" ]
    pub fs_dieptxf1: FsDieptxf1,
    # [ doc = "0x108 - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF3)" ]
    pub fs_dieptxf2: FsDieptxf2,
    # [ doc = "0x10c - OTG_FS device IN endpoint transmit FIFO size register (OTG_FS_DIEPTXF4)" ]
    pub fs_dieptxf3: FsDieptxf3,
}

# [ repr ( C ) ]
pub struct FsGotgctl {
    register: ::volatile_register::RW<u32>,
}

impl FsGotgctl {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsGotgctlR, &'w mut FsGotgctlW) -> &'w mut FsGotgctlW
    {
        let bits = self.register.read();
        let r = FsGotgctlR { bits: bits };
        let mut w = FsGotgctlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsGotgctlR {
        FsGotgctlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsGotgctlW) -> &mut FsGotgctlW
    {
        let mut w = FsGotgctlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGotgctlR {
    bits: u32,
}

impl FsGotgctlR {
    # [ doc = "Bit 0 - Session request success" ]
    pub fn srqscs(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Session request" ]
    pub fn srq(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Host negotiation success" ]
    pub fn hngscs(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - HNP request" ]
    pub fn hnprq(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Host set HNP enable" ]
    pub fn hshnpen(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Device HNP enabled" ]
    pub fn dhnpen(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Connector ID status" ]
    pub fn cidsts(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Long/short debounce time" ]
    pub fn dbct(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - A-session valid" ]
    pub fn asvld(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - B-session valid" ]
    pub fn bsvld(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGotgctlW {
    bits: u32,
}

impl FsGotgctlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsGotgctlW { bits: 2048u32 }
    }
    # [ doc = "Bit 1 - Session request" ]
    pub fn srq(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - HNP request" ]
    pub fn hnprq(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Host set HNP enable" ]
    pub fn hshnpen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Device HNP enabled" ]
    pub fn dhnpen(&mut self, value: bool) -> &mut Self {
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
pub struct FsGotgint {
    register: ::volatile_register::RW<u32>,
}

impl FsGotgint {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsGotgintR, &'w mut FsGotgintW) -> &'w mut FsGotgintW
    {
        let bits = self.register.read();
        let r = FsGotgintR { bits: bits };
        let mut w = FsGotgintW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsGotgintR {
        FsGotgintR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsGotgintW) -> &mut FsGotgintW
    {
        let mut w = FsGotgintW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGotgintR {
    bits: u32,
}

impl FsGotgintR {
    # [ doc = "Bit 2 - Session end detected" ]
    pub fn sedet(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Session request success status change" ]
    pub fn srsschg(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Host negotiation success status change" ]
    pub fn hnsschg(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Host negotiation detected" ]
    pub fn hngdet(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - A-device timeout change" ]
    pub fn adtochg(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Debounce done" ]
    pub fn dbcdne(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGotgintW {
    bits: u32,
}

impl FsGotgintW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsGotgintW { bits: 0u32 }
    }
    # [ doc = "Bit 2 - Session end detected" ]
    pub fn sedet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Session request success status change" ]
    pub fn srsschg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Host negotiation success status change" ]
    pub fn hnsschg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Host negotiation detected" ]
    pub fn hngdet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - A-device timeout change" ]
    pub fn adtochg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Debounce done" ]
    pub fn dbcdne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsGahbcfg {
    register: ::volatile_register::RW<u32>,
}

impl FsGahbcfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsGahbcfgR, &'w mut FsGahbcfgW) -> &'w mut FsGahbcfgW
    {
        let bits = self.register.read();
        let r = FsGahbcfgR { bits: bits };
        let mut w = FsGahbcfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsGahbcfgR {
        FsGahbcfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsGahbcfgW) -> &mut FsGahbcfgW
    {
        let mut w = FsGahbcfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGahbcfgR {
    bits: u32,
}

impl FsGahbcfgR {
    # [ doc = "Bit 0 - Global interrupt mask" ]
    pub fn gint(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - TxFIFO empty level" ]
    pub fn txfelvl(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Periodic TxFIFO empty level" ]
    pub fn ptxfelvl(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGahbcfgW {
    bits: u32,
}

impl FsGahbcfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsGahbcfgW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Global interrupt mask" ]
    pub fn gint(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - TxFIFO empty level" ]
    pub fn txfelvl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Periodic TxFIFO empty level" ]
    pub fn ptxfelvl(&mut self, value: bool) -> &mut Self {
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
pub struct FsGusbcfg {
    register: ::volatile_register::RW<u32>,
}

impl FsGusbcfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsGusbcfgR, &'w mut FsGusbcfgW) -> &'w mut FsGusbcfgW
    {
        let bits = self.register.read();
        let r = FsGusbcfgR { bits: bits };
        let mut w = FsGusbcfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsGusbcfgR {
        FsGusbcfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsGusbcfgW) -> &mut FsGusbcfgW
    {
        let mut w = FsGusbcfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGusbcfgR {
    bits: u32,
}

impl FsGusbcfgR {
    # [ doc = "Bits 0:2 - FS timeout calibration" ]
    pub fn tocal(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 8 - SRP-capable" ]
    pub fn srpcap(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - HNP-capable" ]
    pub fn hnpcap(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 10:13 - USB turnaround time" ]
    pub fn trdt(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Force host mode" ]
    pub fn fhmod(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Force device mode" ]
    pub fn fdmod(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Corrupt Tx packet" ]
    pub fn ctxpkt(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGusbcfgW {
    bits: u32,
}

impl FsGusbcfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsGusbcfgW { bits: 2560u32 }
    }
    # [ doc = "Bits 0:2 - FS timeout calibration" ]
    pub fn tocal(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 6 - Full Speed serial transceiver select" ]
    pub fn physel(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - SRP-capable" ]
    pub fn srpcap(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - HNP-capable" ]
    pub fn hnpcap(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 10:13 - USB turnaround time" ]
    pub fn trdt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Force host mode" ]
    pub fn fhmod(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Force device mode" ]
    pub fn fdmod(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Corrupt Tx packet" ]
    pub fn ctxpkt(&mut self, value: bool) -> &mut Self {
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
pub struct FsGrstctl {
    register: ::volatile_register::RW<u32>,
}

impl FsGrstctl {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsGrstctlR, &'w mut FsGrstctlW) -> &'w mut FsGrstctlW
    {
        let bits = self.register.read();
        let r = FsGrstctlR { bits: bits };
        let mut w = FsGrstctlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsGrstctlR {
        FsGrstctlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsGrstctlW) -> &mut FsGrstctlW
    {
        let mut w = FsGrstctlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGrstctlR {
    bits: u32,
}

impl FsGrstctlR {
    # [ doc = "Bit 0 - Core soft reset" ]
    pub fn csrst(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - HCLK soft reset" ]
    pub fn hsrst(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Host frame counter reset" ]
    pub fn fcrst(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - RxFIFO flush" ]
    pub fn rxfflsh(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - TxFIFO flush" ]
    pub fn txfflsh(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 6:10 - TxFIFO number" ]
    pub fn txfnum(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 31 - AHB master idle" ]
    pub fn ahbidl(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGrstctlW {
    bits: u32,
}

impl FsGrstctlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsGrstctlW { bits: 536870912u32 }
    }
    # [ doc = "Bit 0 - Core soft reset" ]
    pub fn csrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - HCLK soft reset" ]
    pub fn hsrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Host frame counter reset" ]
    pub fn fcrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - RxFIFO flush" ]
    pub fn rxfflsh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - TxFIFO flush" ]
    pub fn txfflsh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 6:10 - TxFIFO number" ]
    pub fn txfnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsGintsts {
    register: ::volatile_register::RW<u32>,
}

impl FsGintsts {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsGintstsR, &'w mut FsGintstsW) -> &'w mut FsGintstsW
    {
        let bits = self.register.read();
        let r = FsGintstsR { bits: bits };
        let mut w = FsGintstsW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsGintstsR {
        FsGintstsR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsGintstsW) -> &mut FsGintstsW
    {
        let mut w = FsGintstsW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGintstsR {
    bits: u32,
}

impl FsGintstsR {
    # [ doc = "Bit 0 - Current mode of operation" ]
    pub fn cmod(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Mode mismatch interrupt" ]
    pub fn mmis(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - OTG interrupt" ]
    pub fn otgint(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Start of frame" ]
    pub fn sof(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - RxFIFO non-empty" ]
    pub fn rxflvl(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Non-periodic TxFIFO empty" ]
    pub fn nptxfe(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Global IN non-periodic NAK effective" ]
    pub fn ginakeff(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Global OUT NAK effective" ]
    pub fn goutnakeff(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Early suspend" ]
    pub fn esusp(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - USB suspend" ]
    pub fn usbsusp(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - USB reset" ]
    pub fn usbrst(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Enumeration done" ]
    pub fn enumdne(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Isochronous OUT packet dropped interrupt" ]
    pub fn isoodrp(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - End of periodic frame interrupt" ]
    pub fn eopf(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - IN endpoint interrupt" ]
    pub fn iepint(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - OUT endpoint interrupt" ]
    pub fn oepint(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Incomplete isochronous IN transfer" ]
    pub fn iisoixfr(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Incomplete periodic transfer(Host mode)/Incomplete isochronous OUT transfer(Device mode)" ]
    pub fn ipxfr_incompisoout(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Host port interrupt" ]
    pub fn hprtint(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Host channels interrupt" ]
    pub fn hcint(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Periodic TxFIFO empty" ]
    pub fn ptxfe(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - Connector ID status change" ]
    pub fn cidschg(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - Disconnect detected interrupt" ]
    pub fn discint(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Session request/new session detected interrupt" ]
    pub fn srqint(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Resume/remote wakeup detected interrupt" ]
    pub fn wkupint(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGintstsW {
    bits: u32,
}

impl FsGintstsW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsGintstsW { bits: 67108896u32 }
    }
    # [ doc = "Bit 1 - Mode mismatch interrupt" ]
    pub fn mmis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Start of frame" ]
    pub fn sof(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Early suspend" ]
    pub fn esusp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - USB suspend" ]
    pub fn usbsusp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - USB reset" ]
    pub fn usbrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Enumeration done" ]
    pub fn enumdne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Isochronous OUT packet dropped interrupt" ]
    pub fn isoodrp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - End of periodic frame interrupt" ]
    pub fn eopf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Incomplete isochronous IN transfer" ]
    pub fn iisoixfr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Incomplete periodic transfer(Host mode)/Incomplete isochronous OUT transfer(Device mode)" ]
    pub fn ipxfr_incompisoout(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Connector ID status change" ]
    pub fn cidschg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Disconnect detected interrupt" ]
    pub fn discint(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Session request/new session detected interrupt" ]
    pub fn srqint(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Resume/remote wakeup detected interrupt" ]
    pub fn wkupint(&mut self, value: bool) -> &mut Self {
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
pub struct FsGintmsk {
    register: ::volatile_register::RW<u32>,
}

impl FsGintmsk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsGintmskR, &'w mut FsGintmskW) -> &'w mut FsGintmskW
    {
        let bits = self.register.read();
        let r = FsGintmskR { bits: bits };
        let mut w = FsGintmskW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsGintmskR {
        FsGintmskR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsGintmskW) -> &mut FsGintmskW
    {
        let mut w = FsGintmskW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGintmskR {
    bits: u32,
}

impl FsGintmskR {
    # [ doc = "Bit 1 - Mode mismatch interrupt mask" ]
    pub fn mmism(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - OTG interrupt mask" ]
    pub fn otgint(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Start of frame mask" ]
    pub fn sofm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Receive FIFO non-empty mask" ]
    pub fn rxflvlm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Non-periodic TxFIFO empty mask" ]
    pub fn nptxfem(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Global non-periodic IN NAK effective mask" ]
    pub fn ginakeffm(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Global OUT NAK effective mask" ]
    pub fn gonakeffm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Early suspend mask" ]
    pub fn esuspm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - USB suspend mask" ]
    pub fn usbsuspm(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - USB reset mask" ]
    pub fn usbrst(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Enumeration done mask" ]
    pub fn enumdnem(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Isochronous OUT packet dropped interrupt mask" ]
    pub fn isoodrpm(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - End of periodic frame interrupt mask" ]
    pub fn eopfm(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Endpoint mismatch interrupt mask" ]
    pub fn epmism(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - IN endpoints interrupt mask" ]
    pub fn iepint(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - OUT endpoints interrupt mask" ]
    pub fn oepint(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Incomplete isochronous IN transfer mask" ]
    pub fn iisoixfrm(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)" ]
    pub fn ipxfrm_iisooxfrm(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Host port interrupt mask" ]
    pub fn prtim(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Host channels interrupt mask" ]
    pub fn hcim(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Periodic TxFIFO empty mask" ]
    pub fn ptxfem(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 28 - Connector ID status change mask" ]
    pub fn cidschgm(&self) -> bool {
        const OFFSET: u8 = 28u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 29 - Disconnect detected interrupt mask" ]
    pub fn discint(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Session request/new session detected interrupt mask" ]
    pub fn srqim(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Resume/remote wakeup detected interrupt mask" ]
    pub fn wuim(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGintmskW {
    bits: u32,
}

impl FsGintmskW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsGintmskW { bits: 0u32 }
    }
    # [ doc = "Bit 1 - Mode mismatch interrupt mask" ]
    pub fn mmism(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - OTG interrupt mask" ]
    pub fn otgint(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Start of frame mask" ]
    pub fn sofm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Receive FIFO non-empty mask" ]
    pub fn rxflvlm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Non-periodic TxFIFO empty mask" ]
    pub fn nptxfem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Global non-periodic IN NAK effective mask" ]
    pub fn ginakeffm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Global OUT NAK effective mask" ]
    pub fn gonakeffm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Early suspend mask" ]
    pub fn esuspm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - USB suspend mask" ]
    pub fn usbsuspm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - USB reset mask" ]
    pub fn usbrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Enumeration done mask" ]
    pub fn enumdnem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Isochronous OUT packet dropped interrupt mask" ]
    pub fn isoodrpm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - End of periodic frame interrupt mask" ]
    pub fn eopfm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Endpoint mismatch interrupt mask" ]
    pub fn epmism(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - IN endpoints interrupt mask" ]
    pub fn iepint(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - OUT endpoints interrupt mask" ]
    pub fn oepint(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Incomplete isochronous IN transfer mask" ]
    pub fn iisoixfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Incomplete periodic transfer mask(Host mode)/Incomplete isochronous OUT transfer mask(Device mode)" ]
    pub fn ipxfrm_iisooxfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Host channels interrupt mask" ]
    pub fn hcim(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Periodic TxFIFO empty mask" ]
    pub fn ptxfem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - Connector ID status change mask" ]
    pub fn cidschgm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - Disconnect detected interrupt mask" ]
    pub fn discint(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Session request/new session detected interrupt mask" ]
    pub fn srqim(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Resume/remote wakeup detected interrupt mask" ]
    pub fn wuim(&mut self, value: bool) -> &mut Self {
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
pub struct FsGrxstsrDevice {
    register: ::volatile_register::RO<u32>,
}

impl FsGrxstsrDevice {
    pub fn read(&self) -> FsGrxstsrDeviceR {
        FsGrxstsrDeviceR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGrxstsrDeviceR {
    bits: u32,
}

impl FsGrxstsrDeviceR {
    # [ doc = "Bits 0:3 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:14 - Byte count" ]
    pub fn bcnt(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 15:16 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 15u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 17:20 - Packet status" ]
    pub fn pktsts(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 17u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 21:24 - Frame number" ]
    pub fn frmnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 21u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGrxstsrDeviceW {
    bits: u32,
}

impl FsGrxstsrDeviceW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsGrxstsrDeviceW { bits: 0u32 }
    }
    # [ doc = "Bits 0:3 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:14 - Byte count" ]
    pub fn bcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 15:16 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 15u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 17:20 - Packet status" ]
    pub fn pktsts(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 17u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 21:24 - Frame number" ]
    pub fn frmnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 21u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsGrxstsrHost {
    register: ::volatile_register::RO<u32>,
}

impl FsGrxstsrHost {
    pub fn read(&self) -> FsGrxstsrHostR {
        FsGrxstsrHostR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGrxstsrHostR {
    bits: u32,
}

impl FsGrxstsrHostR {
    # [ doc = "Bits 0:3 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:14 - Byte count" ]
    pub fn bcnt(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 15:16 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 15u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 17:20 - Packet status" ]
    pub fn pktsts(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 17u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 21:24 - Frame number" ]
    pub fn frmnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 21u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGrxstsrHostW {
    bits: u32,
}

impl FsGrxstsrHostW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsGrxstsrHostW { bits: 0u32 }
    }
    # [ doc = "Bits 0:3 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:14 - Byte count" ]
    pub fn bcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 15:16 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 15u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 17:20 - Packet status" ]
    pub fn pktsts(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 17u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 21:24 - Frame number" ]
    pub fn frmnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 21u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsGrxfsiz {
    register: ::volatile_register::RW<u32>,
}

impl FsGrxfsiz {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsGrxfsizR, &'w mut FsGrxfsizW) -> &'w mut FsGrxfsizW
    {
        let bits = self.register.read();
        let r = FsGrxfsizR { bits: bits };
        let mut w = FsGrxfsizW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsGrxfsizR {
        FsGrxfsizR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsGrxfsizW) -> &mut FsGrxfsizW
    {
        let mut w = FsGrxfsizW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGrxfsizR {
    bits: u32,
}

impl FsGrxfsizR {
    # [ doc = "Bits 0:15 - RxFIFO depth" ]
    pub fn rxfd(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGrxfsizW {
    bits: u32,
}

impl FsGrxfsizW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsGrxfsizW { bits: 512u32 }
    }
    # [ doc = "Bits 0:15 - RxFIFO depth" ]
    pub fn rxfd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsGnptxfsizDevice {
    register: ::volatile_register::RW<u32>,
}

impl FsGnptxfsizDevice {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsGnptxfsizDeviceR, &'w mut FsGnptxfsizDeviceW)
                                -> &'w mut FsGnptxfsizDeviceW
    {
        let bits = self.register.read();
        let r = FsGnptxfsizDeviceR { bits: bits };
        let mut w = FsGnptxfsizDeviceW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsGnptxfsizDeviceR {
        FsGnptxfsizDeviceR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsGnptxfsizDeviceW) -> &mut FsGnptxfsizDeviceW
    {
        let mut w = FsGnptxfsizDeviceW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGnptxfsizDeviceR {
    bits: u32,
}

impl FsGnptxfsizDeviceR {
    # [ doc = "Bits 0:15 - Endpoint 0 transmit RAM start address" ]
    pub fn tx0fsa(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - Endpoint 0 TxFIFO depth" ]
    pub fn tx0fd(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGnptxfsizDeviceW {
    bits: u32,
}

impl FsGnptxfsizDeviceW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsGnptxfsizDeviceW { bits: 512u32 }
    }
    # [ doc = "Bits 0:15 - Endpoint 0 transmit RAM start address" ]
    pub fn tx0fsa(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - Endpoint 0 TxFIFO depth" ]
    pub fn tx0fd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsGnptxfsizHost {
    register: ::volatile_register::RW<u32>,
}

impl FsGnptxfsizHost {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsGnptxfsizHostR, &'w mut FsGnptxfsizHostW)
                                -> &'w mut FsGnptxfsizHostW
    {
        let bits = self.register.read();
        let r = FsGnptxfsizHostR { bits: bits };
        let mut w = FsGnptxfsizHostW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsGnptxfsizHostR {
        FsGnptxfsizHostR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsGnptxfsizHostW) -> &mut FsGnptxfsizHostW
    {
        let mut w = FsGnptxfsizHostW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGnptxfsizHostR {
    bits: u32,
}

impl FsGnptxfsizHostR {
    # [ doc = "Bits 0:15 - Non-periodic transmit RAM start address" ]
    pub fn nptxfsa(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - Non-periodic TxFIFO depth" ]
    pub fn nptxfd(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGnptxfsizHostW {
    bits: u32,
}

impl FsGnptxfsizHostW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsGnptxfsizHostW { bits: 512u32 }
    }
    # [ doc = "Bits 0:15 - Non-periodic transmit RAM start address" ]
    pub fn nptxfsa(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - Non-periodic TxFIFO depth" ]
    pub fn nptxfd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsGnptxsts {
    register: ::volatile_register::RO<u32>,
}

impl FsGnptxsts {
    pub fn read(&self) -> FsGnptxstsR {
        FsGnptxstsR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGnptxstsR {
    bits: u32,
}

impl FsGnptxstsR {
    # [ doc = "Bits 0:15 - Non-periodic TxFIFO space available" ]
    pub fn nptxfsav(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:23 - Non-periodic transmit request queue space available" ]
    pub fn nptqxsav(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:30 - Top of the non-periodic transmit request queue" ]
    pub fn nptxqtop(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGnptxstsW {
    bits: u32,
}

impl FsGnptxstsW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsGnptxstsW { bits: 524800u32 }
    }
    # [ doc = "Bits 0:15 - Non-periodic TxFIFO space available" ]
    pub fn nptxfsav(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - Non-periodic transmit request queue space available" ]
    pub fn nptqxsav(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:30 - Top of the non-periodic transmit request queue" ]
    pub fn nptxqtop(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsGccfg {
    register: ::volatile_register::RW<u32>,
}

impl FsGccfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsGccfgR, &'w mut FsGccfgW) -> &'w mut FsGccfgW
    {
        let bits = self.register.read();
        let r = FsGccfgR { bits: bits };
        let mut w = FsGccfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsGccfgR {
        FsGccfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsGccfgW) -> &mut FsGccfgW
    {
        let mut w = FsGccfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGccfgR {
    bits: u32,
}

impl FsGccfgR {
    # [ doc = "Bit 16 - Power down" ]
    pub fn pwrdwn(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Enable the VBUS sensing device" ]
    pub fn vbusasen(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Enable the VBUS sensing device" ]
    pub fn vbusbsen(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - SOF output enable" ]
    pub fn sofouten(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsGccfgW {
    bits: u32,
}

impl FsGccfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsGccfgW { bits: 0u32 }
    }
    # [ doc = "Bit 16 - Power down" ]
    pub fn pwrdwn(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Enable the VBUS sensing device" ]
    pub fn vbusasen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Enable the VBUS sensing device" ]
    pub fn vbusbsen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - SOF output enable" ]
    pub fn sofouten(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsCid {
    register: ::volatile_register::RW<u32>,
}

impl FsCid {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsCidR, &'w mut FsCidW) -> &'w mut FsCidW
    {
        let bits = self.register.read();
        let r = FsCidR { bits: bits };
        let mut w = FsCidW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsCidR {
        FsCidR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsCidW) -> &mut FsCidW
    {
        let mut w = FsCidW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsCidR {
    bits: u32,
}

impl FsCidR {
    # [ doc = "Bits 0:31 - Product ID field" ]
    pub fn product_id(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsCidW {
    bits: u32,
}

impl FsCidW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsCidW { bits: 4096u32 }
    }
    # [ doc = "Bits 0:31 - Product ID field" ]
    pub fn product_id(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsHptxfsiz {
    register: ::volatile_register::RW<u32>,
}

impl FsHptxfsiz {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHptxfsizR, &'w mut FsHptxfsizW) -> &'w mut FsHptxfsizW
    {
        let bits = self.register.read();
        let r = FsHptxfsizR { bits: bits };
        let mut w = FsHptxfsizW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHptxfsizR {
        FsHptxfsizR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHptxfsizW) -> &mut FsHptxfsizW
    {
        let mut w = FsHptxfsizW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHptxfsizR {
    bits: u32,
}

impl FsHptxfsizR {
    # [ doc = "Bits 0:15 - Host periodic TxFIFO start address" ]
    pub fn ptxsa(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - Host periodic TxFIFO depth" ]
    pub fn ptxfsiz(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHptxfsizW {
    bits: u32,
}

impl FsHptxfsizW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHptxfsizW { bits: 33555968u32 }
    }
    # [ doc = "Bits 0:15 - Host periodic TxFIFO start address" ]
    pub fn ptxsa(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - Host periodic TxFIFO depth" ]
    pub fn ptxfsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsDieptxf1 {
    register: ::volatile_register::RW<u32>,
}

impl FsDieptxf1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsDieptxf1R, &'w mut FsDieptxf1W) -> &'w mut FsDieptxf1W
    {
        let bits = self.register.read();
        let r = FsDieptxf1R { bits: bits };
        let mut w = FsDieptxf1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsDieptxf1R {
        FsDieptxf1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsDieptxf1W) -> &mut FsDieptxf1W
    {
        let mut w = FsDieptxf1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDieptxf1R {
    bits: u32,
}

impl FsDieptxf1R {
    # [ doc = "Bits 0:15 - IN endpoint FIFO2 transmit RAM start address" ]
    pub fn ineptxsa(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDieptxf1W {
    bits: u32,
}

impl FsDieptxf1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsDieptxf1W { bits: 33555456u32 }
    }
    # [ doc = "Bits 0:15 - IN endpoint FIFO2 transmit RAM start address" ]
    pub fn ineptxsa(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsDieptxf2 {
    register: ::volatile_register::RW<u32>,
}

impl FsDieptxf2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsDieptxf2R, &'w mut FsDieptxf2W) -> &'w mut FsDieptxf2W
    {
        let bits = self.register.read();
        let r = FsDieptxf2R { bits: bits };
        let mut w = FsDieptxf2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsDieptxf2R {
        FsDieptxf2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsDieptxf2W) -> &mut FsDieptxf2W
    {
        let mut w = FsDieptxf2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDieptxf2R {
    bits: u32,
}

impl FsDieptxf2R {
    # [ doc = "Bits 0:15 - IN endpoint FIFO3 transmit RAM start address" ]
    pub fn ineptxsa(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDieptxf2W {
    bits: u32,
}

impl FsDieptxf2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsDieptxf2W { bits: 33555456u32 }
    }
    # [ doc = "Bits 0:15 - IN endpoint FIFO3 transmit RAM start address" ]
    pub fn ineptxsa(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsDieptxf3 {
    register: ::volatile_register::RW<u32>,
}

impl FsDieptxf3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsDieptxf3R, &'w mut FsDieptxf3W) -> &'w mut FsDieptxf3W
    {
        let bits = self.register.read();
        let r = FsDieptxf3R { bits: bits };
        let mut w = FsDieptxf3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsDieptxf3R {
        FsDieptxf3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsDieptxf3W) -> &mut FsDieptxf3W
    {
        let mut w = FsDieptxf3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDieptxf3R {
    bits: u32,
}

impl FsDieptxf3R {
    # [ doc = "Bits 0:15 - IN endpoint FIFO4 transmit RAM start address" ]
    pub fn ineptxsa(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDieptxf3W {
    bits: u32,
}

impl FsDieptxf3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsDieptxf3W { bits: 33555456u32 }
    }
    # [ doc = "Bits 0:15 - IN endpoint FIFO4 transmit RAM start address" ]
    pub fn ineptxsa(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - IN endpoint TxFIFO depth" ]
    pub fn ineptxfd(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
