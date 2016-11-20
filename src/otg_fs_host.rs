# [ doc = "USB on the go full speed" ]
# [ repr ( C ) ]
pub struct OtgFsHost {
    # [ doc = "0x00 - OTG_FS host configuration register (OTG_FS_HCFG)" ]
    pub fs_hcfg: FsHcfg,
    # [ doc = "0x04 - OTG_FS Host frame interval register" ]
    pub hfir: Hfir,
    # [ doc = "0x08 - OTG_FS host frame number/frame time remaining register (OTG_FS_HFNUM)" ]
    pub fs_hfnum: FsHfnum,
    _reserved0: [u8; 4usize],
    # [ doc = "0x10 - OTG_FS_Host periodic transmit FIFO/queue status register (OTG_FS_HPTXSTS)" ]
    pub fs_hptxsts: FsHptxsts,
    # [ doc = "0x14 - OTG_FS Host all channels interrupt register" ]
    pub haint: Haint,
    # [ doc = "0x18 - OTG_FS host all channels interrupt mask register" ]
    pub haintmsk: Haintmsk,
    _reserved1: [u8; 36usize],
    # [ doc = "0x40 - OTG_FS host port control and status register (OTG_FS_HPRT)" ]
    pub fs_hprt: FsHprt,
    _reserved2: [u8; 188usize],
    # [ doc = "0x100 - OTG_FS host channel-0 characteristics register (OTG_FS_HCCHAR0)" ]
    pub fs_hcchar0: FsHcchar0,
    _reserved3: [u8; 28usize],
    # [ doc = "0x120 - OTG_FS host channel-1 characteristics register (OTG_FS_HCCHAR1)" ]
    pub fs_hcchar1: FsHcchar1,
    _reserved4: [u8; 28usize],
    # [ doc = "0x140 - OTG_FS host channel-2 characteristics register (OTG_FS_HCCHAR2)" ]
    pub fs_hcchar2: FsHcchar2,
    _reserved5: [u8; 28usize],
    # [ doc = "0x160 - OTG_FS host channel-3 characteristics register (OTG_FS_HCCHAR3)" ]
    pub fs_hcchar3: FsHcchar3,
    _reserved6: [u8; 28usize],
    # [ doc = "0x180 - OTG_FS host channel-4 characteristics register (OTG_FS_HCCHAR4)" ]
    pub fs_hcchar4: FsHcchar4,
    _reserved7: [u8; 28usize],
    # [ doc = "0x1a0 - OTG_FS host channel-5 characteristics register (OTG_FS_HCCHAR5)" ]
    pub fs_hcchar5: FsHcchar5,
    _reserved8: [u8; 28usize],
    # [ doc = "0x1c0 - OTG_FS host channel-6 characteristics register (OTG_FS_HCCHAR6)" ]
    pub fs_hcchar6: FsHcchar6,
    _reserved9: [u8; 28usize],
    # [ doc = "0x1e0 - OTG_FS host channel-7 characteristics register (OTG_FS_HCCHAR7)" ]
    pub fs_hcchar7: FsHcchar7,
    _reserved10: [u8; 4usize],
    # [ doc = "0x1e8 - OTG_FS host channel-7 interrupt register (OTG_FS_HCINT7)" ]
    pub fs_hcint7: FsHcint7,
    # [ doc = "0x1ec - OTG_FS host channel-7 mask register (OTG_FS_HCINTMSK7)" ]
    pub fs_hcintmsk7: FsHcintmsk7,
    # [ doc = "0x1f0 - OTG_FS host channel-7 transfer size register" ]
    pub fs_hctsiz7: FsHctsiz7,
}

# [ repr ( C ) ]
pub struct FsHcfg {
    register: ::volatile_register::RW<u32>,
}

impl FsHcfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcfgR, &'w mut FsHcfgW) -> &'w mut FsHcfgW
    {
        let bits = self.register.read();
        let r = FsHcfgR { bits: bits };
        let mut w = FsHcfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcfgR {
        FsHcfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcfgW) -> &mut FsHcfgW
    {
        let mut w = FsHcfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcfgR {
    bits: u32,
}

impl FsHcfgR {
    # [ doc = "Bits 0:1 - FS/LS PHY clock select" ]
    pub fn fslspcs(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 2 - FS- and LS-only support" ]
    pub fn fslss(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcfgW {
    bits: u32,
}

impl FsHcfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcfgW { bits: 0u32 }
    }
    # [ doc = "Bits 0:1 - FS/LS PHY clock select" ]
    pub fn fslspcs(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Hfir {
    register: ::volatile_register::RW<u32>,
}

impl Hfir {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&HfirR, &'w mut HfirW) -> &'w mut HfirW
    {
        let bits = self.register.read();
        let r = HfirR { bits: bits };
        let mut w = HfirW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> HfirR {
        HfirR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut HfirW) -> &mut HfirW
    {
        let mut w = HfirW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct HfirR {
    bits: u32,
}

impl HfirR {
    # [ doc = "Bits 0:15 - Frame interval" ]
    pub fn frivl(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct HfirW {
    bits: u32,
}

impl HfirW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        HfirW { bits: 60000u32 }
    }
    # [ doc = "Bits 0:15 - Frame interval" ]
    pub fn frivl(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsHfnum {
    register: ::volatile_register::RO<u32>,
}

impl FsHfnum {
    pub fn read(&self) -> FsHfnumR {
        FsHfnumR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHfnumR {
    bits: u32,
}

impl FsHfnumR {
    # [ doc = "Bits 0:15 - Frame number" ]
    pub fn frnum(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - Frame time remaining" ]
    pub fn ftrem(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHfnumW {
    bits: u32,
}

impl FsHfnumW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHfnumW { bits: 16383u32 }
    }
    # [ doc = "Bits 0:15 - Frame number" ]
    pub fn frnum(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - Frame time remaining" ]
    pub fn ftrem(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsHptxsts {
    register: ::volatile_register::RW<u32>,
}

impl FsHptxsts {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHptxstsR, &'w mut FsHptxstsW) -> &'w mut FsHptxstsW
    {
        let bits = self.register.read();
        let r = FsHptxstsR { bits: bits };
        let mut w = FsHptxstsW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHptxstsR {
        FsHptxstsR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHptxstsW) -> &mut FsHptxstsW
    {
        let mut w = FsHptxstsW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHptxstsR {
    bits: u32,
}

impl FsHptxstsR {
    # [ doc = "Bits 0:15 - Periodic transmit data FIFO space available" ]
    pub fn ptxfsavl(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:23 - Periodic transmit request queue space available" ]
    pub fn ptxqsav(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:31 - Top of the periodic transmit request queue" ]
    pub fn ptxqtop(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHptxstsW {
    bits: u32,
}

impl FsHptxstsW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHptxstsW { bits: 524544u32 }
    }
    # [ doc = "Bits 0:15 - Periodic transmit data FIFO space available" ]
    pub fn ptxfsavl(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Haint {
    register: ::volatile_register::RO<u32>,
}

impl Haint {
    pub fn read(&self) -> HaintR {
        HaintR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct HaintR {
    bits: u32,
}

impl HaintR {
    # [ doc = "Bits 0:15 - Channel interrupts" ]
    pub fn haint(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct HaintW {
    bits: u32,
}

impl HaintW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        HaintW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Channel interrupts" ]
    pub fn haint(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Haintmsk {
    register: ::volatile_register::RW<u32>,
}

impl Haintmsk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&HaintmskR, &'w mut HaintmskW) -> &'w mut HaintmskW
    {
        let bits = self.register.read();
        let r = HaintmskR { bits: bits };
        let mut w = HaintmskW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> HaintmskR {
        HaintmskR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut HaintmskW) -> &mut HaintmskW
    {
        let mut w = HaintmskW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct HaintmskR {
    bits: u32,
}

impl HaintmskR {
    # [ doc = "Bits 0:15 - Channel interrupt mask" ]
    pub fn haintm(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct HaintmskW {
    bits: u32,
}

impl HaintmskW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        HaintmskW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Channel interrupt mask" ]
    pub fn haintm(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsHprt {
    register: ::volatile_register::RW<u32>,
}

impl FsHprt {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHprtR, &'w mut FsHprtW) -> &'w mut FsHprtW
    {
        let bits = self.register.read();
        let r = FsHprtR { bits: bits };
        let mut w = FsHprtW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHprtR {
        FsHprtR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHprtW) -> &mut FsHprtW
    {
        let mut w = FsHprtW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHprtR {
    bits: u32,
}

impl FsHprtR {
    # [ doc = "Bit 0 - Port connect status" ]
    pub fn pcsts(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Port connect detected" ]
    pub fn pcdet(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Port enable" ]
    pub fn pena(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Port enable/disable change" ]
    pub fn penchng(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Port overcurrent active" ]
    pub fn poca(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Port overcurrent change" ]
    pub fn pocchng(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Port resume" ]
    pub fn pres(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Port suspend" ]
    pub fn psusp(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Port reset" ]
    pub fn prst(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 10:11 - Port line status" ]
    pub fn plsts(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 12 - Port power" ]
    pub fn ppwr(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 13:16 - Port test control" ]
    pub fn ptctl(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 17:18 - Port speed" ]
    pub fn pspd(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 17u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHprtW {
    bits: u32,
}

impl FsHprtW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHprtW { bits: 0u32 }
    }
    # [ doc = "Bit 1 - Port connect detected" ]
    pub fn pcdet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Port enable" ]
    pub fn pena(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Port enable/disable change" ]
    pub fn penchng(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Port overcurrent change" ]
    pub fn pocchng(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Port resume" ]
    pub fn pres(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Port suspend" ]
    pub fn psusp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Port reset" ]
    pub fn prst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Port power" ]
    pub fn ppwr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 13:16 - Port test control" ]
    pub fn ptctl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsHcchar0 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcchar0 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcchar0R, &'w mut FsHcchar0W) -> &'w mut FsHcchar0W
    {
        let bits = self.register.read();
        let r = FsHcchar0R { bits: bits };
        let mut w = FsHcchar0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcchar0R {
        FsHcchar0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcchar0W) -> &mut FsHcchar0W
    {
        let mut w = FsHcchar0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcchar0R {
    bits: u32,
}

impl FsHcchar0R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multicount" ]
    pub fn mcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcchar0W {
    bits: u32,
}

impl FsHcchar0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcchar0W { bits: 0u32 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multicount" ]
    pub fn mcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct FsHcchar1 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcchar1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcchar1R, &'w mut FsHcchar1W) -> &'w mut FsHcchar1W
    {
        let bits = self.register.read();
        let r = FsHcchar1R { bits: bits };
        let mut w = FsHcchar1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcchar1R {
        FsHcchar1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcchar1W) -> &mut FsHcchar1W
    {
        let mut w = FsHcchar1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcchar1R {
    bits: u32,
}

impl FsHcchar1R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multicount" ]
    pub fn mcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcchar1W {
    bits: u32,
}

impl FsHcchar1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcchar1W { bits: 0u32 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multicount" ]
    pub fn mcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct FsHcchar2 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcchar2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcchar2R, &'w mut FsHcchar2W) -> &'w mut FsHcchar2W
    {
        let bits = self.register.read();
        let r = FsHcchar2R { bits: bits };
        let mut w = FsHcchar2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcchar2R {
        FsHcchar2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcchar2W) -> &mut FsHcchar2W
    {
        let mut w = FsHcchar2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcchar2R {
    bits: u32,
}

impl FsHcchar2R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multicount" ]
    pub fn mcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcchar2W {
    bits: u32,
}

impl FsHcchar2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcchar2W { bits: 0u32 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multicount" ]
    pub fn mcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct FsHcchar3 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcchar3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcchar3R, &'w mut FsHcchar3W) -> &'w mut FsHcchar3W
    {
        let bits = self.register.read();
        let r = FsHcchar3R { bits: bits };
        let mut w = FsHcchar3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcchar3R {
        FsHcchar3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcchar3W) -> &mut FsHcchar3W
    {
        let mut w = FsHcchar3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcchar3R {
    bits: u32,
}

impl FsHcchar3R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multicount" ]
    pub fn mcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcchar3W {
    bits: u32,
}

impl FsHcchar3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcchar3W { bits: 0u32 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multicount" ]
    pub fn mcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct FsHcchar4 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcchar4 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcchar4R, &'w mut FsHcchar4W) -> &'w mut FsHcchar4W
    {
        let bits = self.register.read();
        let r = FsHcchar4R { bits: bits };
        let mut w = FsHcchar4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcchar4R {
        FsHcchar4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcchar4W) -> &mut FsHcchar4W
    {
        let mut w = FsHcchar4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcchar4R {
    bits: u32,
}

impl FsHcchar4R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multicount" ]
    pub fn mcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcchar4W {
    bits: u32,
}

impl FsHcchar4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcchar4W { bits: 0u32 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multicount" ]
    pub fn mcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct FsHcchar5 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcchar5 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcchar5R, &'w mut FsHcchar5W) -> &'w mut FsHcchar5W
    {
        let bits = self.register.read();
        let r = FsHcchar5R { bits: bits };
        let mut w = FsHcchar5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcchar5R {
        FsHcchar5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcchar5W) -> &mut FsHcchar5W
    {
        let mut w = FsHcchar5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcchar5R {
    bits: u32,
}

impl FsHcchar5R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multicount" ]
    pub fn mcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcchar5W {
    bits: u32,
}

impl FsHcchar5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcchar5W { bits: 0u32 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multicount" ]
    pub fn mcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct FsHcchar6 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcchar6 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcchar6R, &'w mut FsHcchar6W) -> &'w mut FsHcchar6W
    {
        let bits = self.register.read();
        let r = FsHcchar6R { bits: bits };
        let mut w = FsHcchar6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcchar6R {
        FsHcchar6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcchar6W) -> &mut FsHcchar6W
    {
        let mut w = FsHcchar6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcchar6R {
    bits: u32,
}

impl FsHcchar6R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multicount" ]
    pub fn mcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcchar6W {
    bits: u32,
}

impl FsHcchar6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcchar6W { bits: 0u32 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multicount" ]
    pub fn mcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct FsHcchar7 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcchar7 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcchar7R, &'w mut FsHcchar7W) -> &'w mut FsHcchar7W
    {
        let bits = self.register.read();
        let r = FsHcchar7R { bits: bits };
        let mut w = FsHcchar7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcchar7R {
        FsHcchar7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcchar7W) -> &mut FsHcchar7W
    {
        let mut w = FsHcchar7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcchar7R {
    bits: u32,
}

impl FsHcchar7R {
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:21 - Multicount" ]
    pub fn mcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&self) -> bool {
        const OFFSET: u8 = 29u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcchar7W {
    bits: u32,
}

impl FsHcchar7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcchar7W { bits: 0u32 }
    }
    # [ doc = "Bits 0:10 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:14 - Endpoint number" ]
    pub fn epnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - Endpoint direction" ]
    pub fn epdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Low-speed device" ]
    pub fn lsdev(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:21 - Multicount" ]
    pub fn mcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 22:28 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 29 - Odd frame" ]
    pub fn oddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Channel disable" ]
    pub fn chdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - Channel enable" ]
    pub fn chena(&mut self, value: bool) -> &mut Self {
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
pub struct FsHcint0 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcint0 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcint0R, &'w mut FsHcint0W) -> &'w mut FsHcint0W
    {
        let bits = self.register.read();
        let r = FsHcint0R { bits: bits };
        let mut w = FsHcint0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcint0R {
        FsHcint0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcint0W) -> &mut FsHcint0W
    {
        let mut w = FsHcint0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcint0R {
    bits: u32,
}

impl FsHcint0R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcint0W {
    bits: u32,
}

impl FsHcint0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcint0W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsHcint1 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcint1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcint1R, &'w mut FsHcint1W) -> &'w mut FsHcint1W
    {
        let bits = self.register.read();
        let r = FsHcint1R { bits: bits };
        let mut w = FsHcint1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcint1R {
        FsHcint1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcint1W) -> &mut FsHcint1W
    {
        let mut w = FsHcint1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcint1R {
    bits: u32,
}

impl FsHcint1R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcint1W {
    bits: u32,
}

impl FsHcint1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcint1W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsHcint2 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcint2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcint2R, &'w mut FsHcint2W) -> &'w mut FsHcint2W
    {
        let bits = self.register.read();
        let r = FsHcint2R { bits: bits };
        let mut w = FsHcint2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcint2R {
        FsHcint2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcint2W) -> &mut FsHcint2W
    {
        let mut w = FsHcint2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcint2R {
    bits: u32,
}

impl FsHcint2R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcint2W {
    bits: u32,
}

impl FsHcint2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcint2W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsHcint3 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcint3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcint3R, &'w mut FsHcint3W) -> &'w mut FsHcint3W
    {
        let bits = self.register.read();
        let r = FsHcint3R { bits: bits };
        let mut w = FsHcint3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcint3R {
        FsHcint3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcint3W) -> &mut FsHcint3W
    {
        let mut w = FsHcint3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcint3R {
    bits: u32,
}

impl FsHcint3R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcint3W {
    bits: u32,
}

impl FsHcint3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcint3W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsHcint4 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcint4 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcint4R, &'w mut FsHcint4W) -> &'w mut FsHcint4W
    {
        let bits = self.register.read();
        let r = FsHcint4R { bits: bits };
        let mut w = FsHcint4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcint4R {
        FsHcint4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcint4W) -> &mut FsHcint4W
    {
        let mut w = FsHcint4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcint4R {
    bits: u32,
}

impl FsHcint4R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcint4W {
    bits: u32,
}

impl FsHcint4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcint4W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsHcint5 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcint5 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcint5R, &'w mut FsHcint5W) -> &'w mut FsHcint5W
    {
        let bits = self.register.read();
        let r = FsHcint5R { bits: bits };
        let mut w = FsHcint5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcint5R {
        FsHcint5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcint5W) -> &mut FsHcint5W
    {
        let mut w = FsHcint5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcint5R {
    bits: u32,
}

impl FsHcint5R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcint5W {
    bits: u32,
}

impl FsHcint5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcint5W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsHcint6 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcint6 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcint6R, &'w mut FsHcint6W) -> &'w mut FsHcint6W
    {
        let bits = self.register.read();
        let r = FsHcint6R { bits: bits };
        let mut w = FsHcint6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcint6R {
        FsHcint6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcint6W) -> &mut FsHcint6W
    {
        let mut w = FsHcint6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcint6R {
    bits: u32,
}

impl FsHcint6R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcint6W {
    bits: u32,
}

impl FsHcint6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcint6W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsHcint7 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcint7 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcint7R, &'w mut FsHcint7W) -> &'w mut FsHcint7W
    {
        let bits = self.register.read();
        let r = FsHcint7R { bits: bits };
        let mut w = FsHcint7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcint7R {
        FsHcint7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcint7W) -> &mut FsHcint7W
    {
        let mut w = FsHcint7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcint7R {
    bits: u32,
}

impl FsHcint7R {
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcint7W {
    bits: u32,
}

impl FsHcint7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcint7W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Transfer completed" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted" ]
    pub fn chh(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt" ]
    pub fn nak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt" ]
    pub fn ack(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error" ]
    pub fn txerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error" ]
    pub fn bberr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun" ]
    pub fn frmor(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error" ]
    pub fn dterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsHcintmsk0 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcintmsk0 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcintmsk0R, &'w mut FsHcintmsk0W) -> &'w mut FsHcintmsk0W
    {
        let bits = self.register.read();
        let r = FsHcintmsk0R { bits: bits };
        let mut w = FsHcintmsk0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcintmsk0R {
        FsHcintmsk0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcintmsk0W) -> &mut FsHcintmsk0W
    {
        let mut w = FsHcintmsk0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcintmsk0R {
    bits: u32,
}

impl FsHcintmsk0R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcintmsk0W {
    bits: u32,
}

impl FsHcintmsk0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcintmsk0W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsHcintmsk1 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcintmsk1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcintmsk1R, &'w mut FsHcintmsk1W) -> &'w mut FsHcintmsk1W
    {
        let bits = self.register.read();
        let r = FsHcintmsk1R { bits: bits };
        let mut w = FsHcintmsk1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcintmsk1R {
        FsHcintmsk1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcintmsk1W) -> &mut FsHcintmsk1W
    {
        let mut w = FsHcintmsk1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcintmsk1R {
    bits: u32,
}

impl FsHcintmsk1R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcintmsk1W {
    bits: u32,
}

impl FsHcintmsk1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcintmsk1W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsHcintmsk2 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcintmsk2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcintmsk2R, &'w mut FsHcintmsk2W) -> &'w mut FsHcintmsk2W
    {
        let bits = self.register.read();
        let r = FsHcintmsk2R { bits: bits };
        let mut w = FsHcintmsk2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcintmsk2R {
        FsHcintmsk2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcintmsk2W) -> &mut FsHcintmsk2W
    {
        let mut w = FsHcintmsk2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcintmsk2R {
    bits: u32,
}

impl FsHcintmsk2R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcintmsk2W {
    bits: u32,
}

impl FsHcintmsk2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcintmsk2W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsHcintmsk3 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcintmsk3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcintmsk3R, &'w mut FsHcintmsk3W) -> &'w mut FsHcintmsk3W
    {
        let bits = self.register.read();
        let r = FsHcintmsk3R { bits: bits };
        let mut w = FsHcintmsk3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcintmsk3R {
        FsHcintmsk3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcintmsk3W) -> &mut FsHcintmsk3W
    {
        let mut w = FsHcintmsk3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcintmsk3R {
    bits: u32,
}

impl FsHcintmsk3R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcintmsk3W {
    bits: u32,
}

impl FsHcintmsk3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcintmsk3W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsHcintmsk4 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcintmsk4 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcintmsk4R, &'w mut FsHcintmsk4W) -> &'w mut FsHcintmsk4W
    {
        let bits = self.register.read();
        let r = FsHcintmsk4R { bits: bits };
        let mut w = FsHcintmsk4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcintmsk4R {
        FsHcintmsk4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcintmsk4W) -> &mut FsHcintmsk4W
    {
        let mut w = FsHcintmsk4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcintmsk4R {
    bits: u32,
}

impl FsHcintmsk4R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcintmsk4W {
    bits: u32,
}

impl FsHcintmsk4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcintmsk4W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsHcintmsk5 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcintmsk5 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcintmsk5R, &'w mut FsHcintmsk5W) -> &'w mut FsHcintmsk5W
    {
        let bits = self.register.read();
        let r = FsHcintmsk5R { bits: bits };
        let mut w = FsHcintmsk5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcintmsk5R {
        FsHcintmsk5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcintmsk5W) -> &mut FsHcintmsk5W
    {
        let mut w = FsHcintmsk5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcintmsk5R {
    bits: u32,
}

impl FsHcintmsk5R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcintmsk5W {
    bits: u32,
}

impl FsHcintmsk5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcintmsk5W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsHcintmsk6 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcintmsk6 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcintmsk6R, &'w mut FsHcintmsk6W) -> &'w mut FsHcintmsk6W
    {
        let bits = self.register.read();
        let r = FsHcintmsk6R { bits: bits };
        let mut w = FsHcintmsk6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcintmsk6R {
        FsHcintmsk6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcintmsk6W) -> &mut FsHcintmsk6W
    {
        let mut w = FsHcintmsk6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcintmsk6R {
    bits: u32,
}

impl FsHcintmsk6R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcintmsk6W {
    bits: u32,
}

impl FsHcintmsk6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcintmsk6W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsHcintmsk7 {
    register: ::volatile_register::RW<u32>,
}

impl FsHcintmsk7 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHcintmsk7R, &'w mut FsHcintmsk7W) -> &'w mut FsHcintmsk7W
    {
        let bits = self.register.read();
        let r = FsHcintmsk7R { bits: bits };
        let mut w = FsHcintmsk7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHcintmsk7R {
        FsHcintmsk7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHcintmsk7W) -> &mut FsHcintmsk7W
    {
        let mut w = FsHcintmsk7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcintmsk7R {
    bits: u32,
}

impl FsHcintmsk7R {
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHcintmsk7W {
    bits: u32,
}

impl FsHcintmsk7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHcintmsk7W { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Transfer completed mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel halted mask" ]
    pub fn chhm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STALL response received interrupt mask" ]
    pub fn stallm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - NAK response received interrupt mask" ]
    pub fn nakm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - ACK response received/transmitted interrupt mask" ]
    pub fn ackm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - response received interrupt mask" ]
    pub fn nyet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transaction error mask" ]
    pub fn txerrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Babble error mask" ]
    pub fn bberrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Frame overrun mask" ]
    pub fn frmorm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data toggle error mask" ]
    pub fn dterrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsHctsiz0 {
    register: ::volatile_register::RW<u32>,
}

impl FsHctsiz0 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHctsiz0R, &'w mut FsHctsiz0W) -> &'w mut FsHctsiz0W
    {
        let bits = self.register.read();
        let r = FsHctsiz0R { bits: bits };
        let mut w = FsHctsiz0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHctsiz0R {
        FsHctsiz0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHctsiz0W) -> &mut FsHctsiz0W
    {
        let mut w = FsHctsiz0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHctsiz0R {
    bits: u32,
}

impl FsHctsiz0R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHctsiz0W {
    bits: u32,
}

impl FsHctsiz0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHctsiz0W { bits: 0u32 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsHctsiz1 {
    register: ::volatile_register::RW<u32>,
}

impl FsHctsiz1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHctsiz1R, &'w mut FsHctsiz1W) -> &'w mut FsHctsiz1W
    {
        let bits = self.register.read();
        let r = FsHctsiz1R { bits: bits };
        let mut w = FsHctsiz1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHctsiz1R {
        FsHctsiz1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHctsiz1W) -> &mut FsHctsiz1W
    {
        let mut w = FsHctsiz1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHctsiz1R {
    bits: u32,
}

impl FsHctsiz1R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHctsiz1W {
    bits: u32,
}

impl FsHctsiz1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHctsiz1W { bits: 0u32 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsHctsiz2 {
    register: ::volatile_register::RW<u32>,
}

impl FsHctsiz2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHctsiz2R, &'w mut FsHctsiz2W) -> &'w mut FsHctsiz2W
    {
        let bits = self.register.read();
        let r = FsHctsiz2R { bits: bits };
        let mut w = FsHctsiz2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHctsiz2R {
        FsHctsiz2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHctsiz2W) -> &mut FsHctsiz2W
    {
        let mut w = FsHctsiz2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHctsiz2R {
    bits: u32,
}

impl FsHctsiz2R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHctsiz2W {
    bits: u32,
}

impl FsHctsiz2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHctsiz2W { bits: 0u32 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsHctsiz3 {
    register: ::volatile_register::RW<u32>,
}

impl FsHctsiz3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHctsiz3R, &'w mut FsHctsiz3W) -> &'w mut FsHctsiz3W
    {
        let bits = self.register.read();
        let r = FsHctsiz3R { bits: bits };
        let mut w = FsHctsiz3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHctsiz3R {
        FsHctsiz3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHctsiz3W) -> &mut FsHctsiz3W
    {
        let mut w = FsHctsiz3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHctsiz3R {
    bits: u32,
}

impl FsHctsiz3R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHctsiz3W {
    bits: u32,
}

impl FsHctsiz3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHctsiz3W { bits: 0u32 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsHctsiz4 {
    register: ::volatile_register::RW<u32>,
}

impl FsHctsiz4 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHctsiz4R, &'w mut FsHctsiz4W) -> &'w mut FsHctsiz4W
    {
        let bits = self.register.read();
        let r = FsHctsiz4R { bits: bits };
        let mut w = FsHctsiz4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHctsiz4R {
        FsHctsiz4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHctsiz4W) -> &mut FsHctsiz4W
    {
        let mut w = FsHctsiz4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHctsiz4R {
    bits: u32,
}

impl FsHctsiz4R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHctsiz4W {
    bits: u32,
}

impl FsHctsiz4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHctsiz4W { bits: 0u32 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsHctsiz5 {
    register: ::volatile_register::RW<u32>,
}

impl FsHctsiz5 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHctsiz5R, &'w mut FsHctsiz5W) -> &'w mut FsHctsiz5W
    {
        let bits = self.register.read();
        let r = FsHctsiz5R { bits: bits };
        let mut w = FsHctsiz5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHctsiz5R {
        FsHctsiz5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHctsiz5W) -> &mut FsHctsiz5W
    {
        let mut w = FsHctsiz5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHctsiz5R {
    bits: u32,
}

impl FsHctsiz5R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHctsiz5W {
    bits: u32,
}

impl FsHctsiz5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHctsiz5W { bits: 0u32 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsHctsiz6 {
    register: ::volatile_register::RW<u32>,
}

impl FsHctsiz6 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHctsiz6R, &'w mut FsHctsiz6W) -> &'w mut FsHctsiz6W
    {
        let bits = self.register.read();
        let r = FsHctsiz6R { bits: bits };
        let mut w = FsHctsiz6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHctsiz6R {
        FsHctsiz6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHctsiz6W) -> &mut FsHctsiz6W
    {
        let mut w = FsHctsiz6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHctsiz6R {
    bits: u32,
}

impl FsHctsiz6R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHctsiz6W {
    bits: u32,
}

impl FsHctsiz6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHctsiz6W { bits: 0u32 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsHctsiz7 {
    register: ::volatile_register::RW<u32>,
}

impl FsHctsiz7 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsHctsiz7R, &'w mut FsHctsiz7W) -> &'w mut FsHctsiz7W
    {
        let bits = self.register.read();
        let r = FsHctsiz7R { bits: bits };
        let mut w = FsHctsiz7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsHctsiz7R {
        FsHctsiz7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsHctsiz7W) -> &mut FsHctsiz7W
    {
        let mut w = FsHctsiz7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHctsiz7R {
    bits: u32,
}

impl FsHctsiz7R {
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsHctsiz7W {
    bits: u32,
}

impl FsHctsiz7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsHctsiz7W { bits: 0u32 }
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 29:30 - Data PID" ]
    pub fn dpid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
