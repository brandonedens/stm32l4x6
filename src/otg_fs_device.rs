# [ doc = "USB on the go full speed" ]
# [ repr ( C ) ]
pub struct OtgFsDevice {
    # [ doc = "0x00 - OTG_FS device configuration register (OTG_FS_DCFG)" ]
    pub fs_dcfg: FsDcfg,
    # [ doc = "0x04 - OTG_FS device control register (OTG_FS_DCTL)" ]
    pub fs_dctl: FsDctl,
    # [ doc = "0x08 - OTG_FS device status register (OTG_FS_DSTS)" ]
    pub fs_dsts: FsDsts,
    _reserved0: [u8; 4usize],
    # [ doc = "0x10 - OTG_FS device IN endpoint common interrupt mask register (OTG_FS_DIEPMSK)" ]
    pub fs_diepmsk: FsDiepmsk,
    # [ doc = "0x14 - OTG_FS device OUT endpoint common interrupt mask register (OTG_FS_DOEPMSK)" ]
    pub fs_doepmsk: FsDoepmsk,
    # [ doc = "0x18 - OTG_FS device all endpoints interrupt register (OTG_FS_DAINT)" ]
    pub fs_daint: FsDaint,
    # [ doc = "0x1c - OTG_FS all endpoints interrupt mask register (OTG_FS_DAINTMSK)" ]
    pub fs_daintmsk: FsDaintmsk,
    _reserved1: [u8; 8usize],
    # [ doc = "0x28 - OTG_FS device VBUS discharge time register" ]
    pub dvbusdis: Dvbusdis,
    # [ doc = "0x2c - OTG_FS device VBUS pulsing time register" ]
    pub dvbuspulse: Dvbuspulse,
    _reserved2: [u8; 4usize],
    # [ doc = "0x34 - OTG_FS device IN endpoint FIFO empty interrupt mask register" ]
    pub diepempmsk: Diepempmsk,
    _reserved3: [u8; 200usize],
    # [ doc = "0x100 - OTG_FS device control IN endpoint 0 control register (OTG_FS_DIEPCTL0)" ]
    pub fs_diepctl0: FsDiepctl0,
    _reserved4: [u8; 28usize],
    # [ doc = "0x120 - OTG device endpoint-1 control register" ]
    pub diepctl1: Diepctl1,
    _reserved5: [u8; 28usize],
    # [ doc = "0x140 - OTG device endpoint-2 control register" ]
    pub diepctl2: Diepctl2,
    _reserved6: [u8; 28usize],
    # [ doc = "0x160 - OTG device endpoint-3 control register" ]
    pub diepctl3: Diepctl3,
    _reserved7: [u8; 412usize],
    # [ doc = "0x300 - device endpoint-0 control register" ]
    pub doepctl0: Doepctl0,
    _reserved8: [u8; 28usize],
    # [ doc = "0x320 - device endpoint-1 control register" ]
    pub doepctl1: Doepctl1,
    _reserved9: [u8; 28usize],
    # [ doc = "0x340 - device endpoint-2 control register" ]
    pub doepctl2: Doepctl2,
    _reserved10: [u8; 28usize],
    # [ doc = "0x360 - device endpoint-3 control register" ]
    pub doepctl3: Doepctl3,
    _reserved11: [u8; 4usize],
    # [ doc = "0x368 - device endpoint-3 interrupt register" ]
    pub doepint3: Doepint3,
    _reserved12: [u8; 4usize],
    # [ doc = "0x370 - device OUT endpoint-3 transfer size register" ]
    pub doeptsiz3: Doeptsiz3,
}

# [ repr ( C ) ]
pub struct FsDcfg {
    register: ::volatile_register::RW<u32>,
}

impl FsDcfg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsDcfgR, &'w mut FsDcfgW) -> &'w mut FsDcfgW
    {
        let bits = self.register.read();
        let r = FsDcfgR { bits: bits };
        let mut w = FsDcfgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsDcfgR {
        FsDcfgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsDcfgW) -> &mut FsDcfgW
    {
        let mut w = FsDcfgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDcfgR {
    bits: u32,
}

impl FsDcfgR {
    # [ doc = "Bits 0:1 - Device speed" ]
    pub fn dspd(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 2 - Non-zero-length status OUT handshake" ]
    pub fn nzlsohsk(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:10 - Device address" ]
    pub fn dad(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 11:12 - Periodic frame interval" ]
    pub fn pfivl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDcfgW {
    bits: u32,
}

impl FsDcfgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsDcfgW { bits: 35651584u32 }
    }
    # [ doc = "Bits 0:1 - Device speed" ]
    pub fn dspd(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 2 - Non-zero-length status OUT handshake" ]
    pub fn nzlsohsk(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:10 - Device address" ]
    pub fn dad(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 11:12 - Periodic frame interval" ]
    pub fn pfivl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsDctl {
    register: ::volatile_register::RW<u32>,
}

impl FsDctl {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsDctlR, &'w mut FsDctlW) -> &'w mut FsDctlW
    {
        let bits = self.register.read();
        let r = FsDctlR { bits: bits };
        let mut w = FsDctlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsDctlR {
        FsDctlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsDctlW) -> &mut FsDctlW
    {
        let mut w = FsDctlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDctlR {
    bits: u32,
}

impl FsDctlR {
    # [ doc = "Bit 0 - Remote wakeup signaling" ]
    pub fn rwusig(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Soft disconnect" ]
    pub fn sdis(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Global IN NAK status" ]
    pub fn ginsts(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Global OUT NAK status" ]
    pub fn gonsts(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:6 - Test control" ]
    pub fn tctl(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Set global IN NAK" ]
    pub fn sginak(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Clear global IN NAK" ]
    pub fn cginak(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Set global OUT NAK" ]
    pub fn sgonak(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Clear global OUT NAK" ]
    pub fn cgonak(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Power-on programming done" ]
    pub fn poprgdne(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDctlW {
    bits: u32,
}

impl FsDctlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsDctlW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Remote wakeup signaling" ]
    pub fn rwusig(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Soft disconnect" ]
    pub fn sdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:6 - Test control" ]
    pub fn tctl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Set global IN NAK" ]
    pub fn sginak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Clear global IN NAK" ]
    pub fn cginak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Set global OUT NAK" ]
    pub fn sgonak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Clear global OUT NAK" ]
    pub fn cgonak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Power-on programming done" ]
    pub fn poprgdne(&mut self, value: bool) -> &mut Self {
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
pub struct FsDsts {
    register: ::volatile_register::RO<u32>,
}

impl FsDsts {
    pub fn read(&self) -> FsDstsR {
        FsDstsR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDstsR {
    bits: u32,
}

impl FsDstsR {
    # [ doc = "Bit 0 - Suspend status" ]
    pub fn suspsts(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 1:2 - Enumerated speed" ]
    pub fn enumspd(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 3 - Erratic error" ]
    pub fn eerr(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:21 - Frame number of the received SOF" ]
    pub fn fnsof(&self) -> u16 {
        const MASK: u32 = 16383;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDstsW {
    bits: u32,
}

impl FsDstsW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsDstsW { bits: 16u32 }
    }
    # [ doc = "Bit 0 - Suspend status" ]
    pub fn suspsts(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 1:2 - Enumerated speed" ]
    pub fn enumspd(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 3 - Erratic error" ]
    pub fn eerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 8:21 - Frame number of the received SOF" ]
    pub fn fnsof(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u16 = 16383;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsDiepmsk {
    register: ::volatile_register::RW<u32>,
}

impl FsDiepmsk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsDiepmskR, &'w mut FsDiepmskW) -> &'w mut FsDiepmskW
    {
        let bits = self.register.read();
        let r = FsDiepmskR { bits: bits };
        let mut w = FsDiepmskW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsDiepmskR {
        FsDiepmskR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsDiepmskW) -> &mut FsDiepmskW
    {
        let mut w = FsDiepmskW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDiepmskR {
    bits: u32,
}

impl FsDiepmskR {
    # [ doc = "Bit 0 - Transfer completed interrupt mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt mask" ]
    pub fn epdm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Timeout condition mask (Non-isochronous endpoints)" ]
    pub fn tom(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO empty mask" ]
    pub fn ittxfemsk(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - IN token received with EP mismatch mask" ]
    pub fn inepnmm(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective mask" ]
    pub fn inepnem(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDiepmskW {
    bits: u32,
}

impl FsDiepmskW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsDiepmskW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt mask" ]
    pub fn epdm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Timeout condition mask (Non-isochronous endpoints)" ]
    pub fn tom(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - IN token received when TxFIFO empty mask" ]
    pub fn ittxfemsk(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - IN token received with EP mismatch mask" ]
    pub fn inepnmm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - IN endpoint NAK effective mask" ]
    pub fn inepnem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct FsDoepmsk {
    register: ::volatile_register::RW<u32>,
}

impl FsDoepmsk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsDoepmskR, &'w mut FsDoepmskW) -> &'w mut FsDoepmskW
    {
        let bits = self.register.read();
        let r = FsDoepmskR { bits: bits };
        let mut w = FsDoepmskW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsDoepmskR {
        FsDoepmskR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsDoepmskW) -> &mut FsDoepmskW
    {
        let mut w = FsDoepmskW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDoepmskR {
    bits: u32,
}

impl FsDoepmskR {
    # [ doc = "Bit 0 - Transfer completed interrupt mask" ]
    pub fn xfrcm(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt mask" ]
    pub fn epdm(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - SETUP phase done mask" ]
    pub fn stupm(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled mask" ]
    pub fn otepdm(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDoepmskW {
    bits: u32,
}

impl FsDoepmskW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsDoepmskW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Transfer completed interrupt mask" ]
    pub fn xfrcm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Endpoint disabled interrupt mask" ]
    pub fn epdm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - SETUP phase done mask" ]
    pub fn stupm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - OUT token received when endpoint disabled mask" ]
    pub fn otepdm(&mut self, value: bool) -> &mut Self {
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
pub struct FsDaint {
    register: ::volatile_register::RO<u32>,
}

impl FsDaint {
    pub fn read(&self) -> FsDaintR {
        FsDaintR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDaintR {
    bits: u32,
}

impl FsDaintR {
    # [ doc = "Bits 0:15 - IN endpoint interrupt bits" ]
    pub fn iepint(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - OUT endpoint interrupt bits" ]
    pub fn oepint(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDaintW {
    bits: u32,
}

impl FsDaintW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsDaintW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - IN endpoint interrupt bits" ]
    pub fn iepint(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - OUT endpoint interrupt bits" ]
    pub fn oepint(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsDaintmsk {
    register: ::volatile_register::RW<u32>,
}

impl FsDaintmsk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsDaintmskR, &'w mut FsDaintmskW) -> &'w mut FsDaintmskW
    {
        let bits = self.register.read();
        let r = FsDaintmskR { bits: bits };
        let mut w = FsDaintmskW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsDaintmskR {
        FsDaintmskR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsDaintmskW) -> &mut FsDaintmskW
    {
        let mut w = FsDaintmskW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDaintmskR {
    bits: u32,
}

impl FsDaintmskR {
    # [ doc = "Bits 0:15 - IN EP interrupt mask bits" ]
    pub fn iepm(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 16:31 - OUT endpoint interrupt bits" ]
    pub fn oepint(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDaintmskW {
    bits: u32,
}

impl FsDaintmskW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsDaintmskW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - IN EP interrupt mask bits" ]
    pub fn iepm(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:31 - OUT endpoint interrupt bits" ]
    pub fn oepint(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dvbusdis {
    register: ::volatile_register::RW<u32>,
}

impl Dvbusdis {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DvbusdisR, &'w mut DvbusdisW) -> &'w mut DvbusdisW
    {
        let bits = self.register.read();
        let r = DvbusdisR { bits: bits };
        let mut w = DvbusdisW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DvbusdisR {
        DvbusdisR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DvbusdisW) -> &mut DvbusdisW
    {
        let mut w = DvbusdisW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DvbusdisR {
    bits: u32,
}

impl DvbusdisR {
    # [ doc = "Bits 0:15 - Device VBUS discharge time" ]
    pub fn vbusdt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DvbusdisW {
    bits: u32,
}

impl DvbusdisW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DvbusdisW { bits: 6103u32 }
    }
    # [ doc = "Bits 0:15 - Device VBUS discharge time" ]
    pub fn vbusdt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dvbuspulse {
    register: ::volatile_register::RW<u32>,
}

impl Dvbuspulse {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DvbuspulseR, &'w mut DvbuspulseW) -> &'w mut DvbuspulseW
    {
        let bits = self.register.read();
        let r = DvbuspulseR { bits: bits };
        let mut w = DvbuspulseW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DvbuspulseR {
        DvbuspulseR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DvbuspulseW) -> &mut DvbuspulseW
    {
        let mut w = DvbuspulseW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DvbuspulseR {
    bits: u32,
}

impl DvbuspulseR {
    # [ doc = "Bits 0:11 - Device VBUS pulsing time" ]
    pub fn dvbusp(&self) -> u16 {
        const MASK: u32 = 4095;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DvbuspulseW {
    bits: u32,
}

impl DvbuspulseW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DvbuspulseW { bits: 1464u32 }
    }
    # [ doc = "Bits 0:11 - Device VBUS pulsing time" ]
    pub fn dvbusp(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 4095;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Diepempmsk {
    register: ::volatile_register::RW<u32>,
}

impl Diepempmsk {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DiepempmskR, &'w mut DiepempmskW) -> &'w mut DiepempmskW
    {
        let bits = self.register.read();
        let r = DiepempmskR { bits: bits };
        let mut w = DiepempmskW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DiepempmskR {
        DiepempmskR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DiepempmskW) -> &mut DiepempmskW
    {
        let mut w = DiepempmskW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DiepempmskR {
    bits: u32,
}

impl DiepempmskR {
    # [ doc = "Bits 0:15 - IN EP Tx FIFO empty interrupt mask bits" ]
    pub fn ineptxfem(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DiepempmskW {
    bits: u32,
}

impl DiepempmskW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DiepempmskW { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - IN EP Tx FIFO empty interrupt mask bits" ]
    pub fn ineptxfem(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct FsDiepctl0 {
    register: ::volatile_register::RW<u32>,
}

impl FsDiepctl0 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FsDiepctl0R, &'w mut FsDiepctl0W) -> &'w mut FsDiepctl0W
    {
        let bits = self.register.read();
        let r = FsDiepctl0R { bits: bits };
        let mut w = FsDiepctl0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FsDiepctl0R {
        FsDiepctl0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FsDiepctl0W) -> &mut FsDiepctl0W
    {
        let mut w = FsDiepctl0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDiepctl0R {
    bits: u32,
}

impl FsDiepctl0R {
    # [ doc = "Bits 0:1 - Maximum packet size" ]
    pub fn mpsiz(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 15 - USB active endpoint" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - NAK status" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - Endpoint type" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 22:25 - TxFIFO number" ]
    pub fn txfnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 30 - Endpoint disable" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - Endpoint enable" ]
    pub fn epena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FsDiepctl0W {
    bits: u32,
}

impl FsDiepctl0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FsDiepctl0W { bits: 0u32 }
    }
    # [ doc = "Bits 0:1 - Maximum packet size" ]
    pub fn mpsiz(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 21 - STALL handshake" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 22:25 - TxFIFO number" ]
    pub fn txfnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 26 - Clear NAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Set NAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Diepctl1 {
    register: ::volatile_register::RW<u32>,
}

impl Diepctl1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Diepctl1R, &'w mut Diepctl1W) -> &'w mut Diepctl1W
    {
        let bits = self.register.read();
        let r = Diepctl1R { bits: bits };
        let mut w = Diepctl1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Diepctl1R {
        Diepctl1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Diepctl1W) -> &mut Diepctl1W
    {
        let mut w = Diepctl1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Diepctl1R {
    bits: u32,
}

impl Diepctl1R {
    # [ doc = "Bit 31 - EPENA" ]
    pub fn epena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - EPDIS" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 22:25 - TXFNUM" ]
    pub fn txfnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 21 - Stall" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - EPTYP" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 17 - NAKSTS" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - EONUM/DPID" ]
    pub fn eonum_dpid(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - USBAEP" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:10 - MPSIZ" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Diepctl1W {
    bits: u32,
}

impl Diepctl1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Diepctl1W { bits: 0u32 }
    }
    # [ doc = "Bit 31 - EPENA" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - EPDIS" ]
    pub fn epdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - SODDFRM/SD1PID" ]
    pub fn soddfrm_sd1pid(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - SD0PID/SEVNFRM" ]
    pub fn sd0pid_sevnfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - SNAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - CNAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 22:25 - TXFNUM" ]
    pub fn txfnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 21 - Stall" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - EPTYP" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - USBAEP" ]
    pub fn usbaep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:10 - MPSIZ" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Diepctl2 {
    register: ::volatile_register::RW<u32>,
}

impl Diepctl2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Diepctl2R, &'w mut Diepctl2W) -> &'w mut Diepctl2W
    {
        let bits = self.register.read();
        let r = Diepctl2R { bits: bits };
        let mut w = Diepctl2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Diepctl2R {
        Diepctl2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Diepctl2W) -> &mut Diepctl2W
    {
        let mut w = Diepctl2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Diepctl2R {
    bits: u32,
}

impl Diepctl2R {
    # [ doc = "Bit 31 - EPENA" ]
    pub fn epena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - EPDIS" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 22:25 - TXFNUM" ]
    pub fn txfnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 21 - Stall" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - EPTYP" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 17 - NAKSTS" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - EONUM/DPID" ]
    pub fn eonum_dpid(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - USBAEP" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:10 - MPSIZ" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Diepctl2W {
    bits: u32,
}

impl Diepctl2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Diepctl2W { bits: 0u32 }
    }
    # [ doc = "Bit 31 - EPENA" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - EPDIS" ]
    pub fn epdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - SODDFRM" ]
    pub fn soddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - SD0PID/SEVNFRM" ]
    pub fn sd0pid_sevnfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - SNAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - CNAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 22:25 - TXFNUM" ]
    pub fn txfnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 21 - Stall" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - EPTYP" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - USBAEP" ]
    pub fn usbaep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:10 - MPSIZ" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Diepctl3 {
    register: ::volatile_register::RW<u32>,
}

impl Diepctl3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Diepctl3R, &'w mut Diepctl3W) -> &'w mut Diepctl3W
    {
        let bits = self.register.read();
        let r = Diepctl3R { bits: bits };
        let mut w = Diepctl3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Diepctl3R {
        Diepctl3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Diepctl3W) -> &mut Diepctl3W
    {
        let mut w = Diepctl3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Diepctl3R {
    bits: u32,
}

impl Diepctl3R {
    # [ doc = "Bit 31 - EPENA" ]
    pub fn epena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - EPDIS" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 22:25 - TXFNUM" ]
    pub fn txfnum(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 22u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 21 - Stall" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - EPTYP" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 17 - NAKSTS" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - EONUM/DPID" ]
    pub fn eonum_dpid(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - USBAEP" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:10 - MPSIZ" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Diepctl3W {
    bits: u32,
}

impl Diepctl3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Diepctl3W { bits: 0u32 }
    }
    # [ doc = "Bit 31 - EPENA" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - EPDIS" ]
    pub fn epdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - SODDFRM" ]
    pub fn soddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - SD0PID/SEVNFRM" ]
    pub fn sd0pid_sevnfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - SNAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - CNAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 22:25 - TXFNUM" ]
    pub fn txfnum(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 22u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 21 - Stall" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - EPTYP" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - USBAEP" ]
    pub fn usbaep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:10 - MPSIZ" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Doepctl0 {
    register: ::volatile_register::RW<u32>,
}

impl Doepctl0 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Doepctl0R, &'w mut Doepctl0W) -> &'w mut Doepctl0W
    {
        let bits = self.register.read();
        let r = Doepctl0R { bits: bits };
        let mut w = Doepctl0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Doepctl0R {
        Doepctl0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Doepctl0W) -> &mut Doepctl0W
    {
        let mut w = Doepctl0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doepctl0R {
    bits: u32,
}

impl Doepctl0R {
    # [ doc = "Bit 30 - EPDIS" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Stall" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - SNPM" ]
    pub fn snpm(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - EPTYP" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 17 - NAKSTS" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - USBAEP" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:1 - MPSIZ" ]
    pub fn mpsiz(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doepctl0W {
    bits: u32,
}

impl Doepctl0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Doepctl0W { bits: 32768u32 }
    }
    # [ doc = "Bit 31 - EPENA" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - SNAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - CNAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Stall" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - SNPM" ]
    pub fn snpm(&mut self, value: bool) -> &mut Self {
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
pub struct Doepctl1 {
    register: ::volatile_register::RW<u32>,
}

impl Doepctl1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Doepctl1R, &'w mut Doepctl1W) -> &'w mut Doepctl1W
    {
        let bits = self.register.read();
        let r = Doepctl1R { bits: bits };
        let mut w = Doepctl1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Doepctl1R {
        Doepctl1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Doepctl1W) -> &mut Doepctl1W
    {
        let mut w = Doepctl1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doepctl1R {
    bits: u32,
}

impl Doepctl1R {
    # [ doc = "Bit 31 - EPENA" ]
    pub fn epena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - EPDIS" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Stall" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - SNPM" ]
    pub fn snpm(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - EPTYP" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 17 - NAKSTS" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - EONUM/DPID" ]
    pub fn eonum_dpid(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - USBAEP" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:10 - MPSIZ" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doepctl1W {
    bits: u32,
}

impl Doepctl1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Doepctl1W { bits: 0u32 }
    }
    # [ doc = "Bit 31 - EPENA" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - EPDIS" ]
    pub fn epdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - SODDFRM" ]
    pub fn soddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - SD0PID/SEVNFRM" ]
    pub fn sd0pid_sevnfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - SNAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - CNAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Stall" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - SNPM" ]
    pub fn snpm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - EPTYP" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - USBAEP" ]
    pub fn usbaep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:10 - MPSIZ" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Doepctl2 {
    register: ::volatile_register::RW<u32>,
}

impl Doepctl2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Doepctl2R, &'w mut Doepctl2W) -> &'w mut Doepctl2W
    {
        let bits = self.register.read();
        let r = Doepctl2R { bits: bits };
        let mut w = Doepctl2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Doepctl2R {
        Doepctl2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Doepctl2W) -> &mut Doepctl2W
    {
        let mut w = Doepctl2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doepctl2R {
    bits: u32,
}

impl Doepctl2R {
    # [ doc = "Bit 31 - EPENA" ]
    pub fn epena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - EPDIS" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Stall" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - SNPM" ]
    pub fn snpm(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - EPTYP" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 17 - NAKSTS" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - EONUM/DPID" ]
    pub fn eonum_dpid(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - USBAEP" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:10 - MPSIZ" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doepctl2W {
    bits: u32,
}

impl Doepctl2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Doepctl2W { bits: 0u32 }
    }
    # [ doc = "Bit 31 - EPENA" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - EPDIS" ]
    pub fn epdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - SODDFRM" ]
    pub fn soddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - SD0PID/SEVNFRM" ]
    pub fn sd0pid_sevnfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - SNAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - CNAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Stall" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - SNPM" ]
    pub fn snpm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - EPTYP" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - USBAEP" ]
    pub fn usbaep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:10 - MPSIZ" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Doepctl3 {
    register: ::volatile_register::RW<u32>,
}

impl Doepctl3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Doepctl3R, &'w mut Doepctl3W) -> &'w mut Doepctl3W
    {
        let bits = self.register.read();
        let r = Doepctl3R { bits: bits };
        let mut w = Doepctl3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Doepctl3R {
        Doepctl3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Doepctl3W) -> &mut Doepctl3W
    {
        let mut w = Doepctl3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doepctl3R {
    bits: u32,
}

impl Doepctl3R {
    # [ doc = "Bit 31 - EPENA" ]
    pub fn epena(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - EPDIS" ]
    pub fn epdis(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Stall" ]
    pub fn stall(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - SNPM" ]
    pub fn snpm(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 18:19 - EPTYP" ]
    pub fn eptyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 18u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 17 - NAKSTS" ]
    pub fn naksts(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - EONUM/DPID" ]
    pub fn eonum_dpid(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - USBAEP" ]
    pub fn usbaep(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:10 - MPSIZ" ]
    pub fn mpsiz(&self) -> u16 {
        const MASK: u32 = 2047;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doepctl3W {
    bits: u32,
}

impl Doepctl3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Doepctl3W { bits: 0u32 }
    }
    # [ doc = "Bit 31 - EPENA" ]
    pub fn epena(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 31u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - EPDIS" ]
    pub fn epdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 29 - SODDFRM" ]
    pub fn soddfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 29u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 28 - SD0PID/SEVNFRM" ]
    pub fn sd0pid_sevnfrm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 28u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - SNAK" ]
    pub fn snak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - CNAK" ]
    pub fn cnak(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Stall" ]
    pub fn stall(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - SNPM" ]
    pub fn snpm(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 18:19 - EPTYP" ]
    pub fn eptyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 18u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 15 - USBAEP" ]
    pub fn usbaep(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:10 - MPSIZ" ]
    pub fn mpsiz(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 2047;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Diepint0 {
    register: ::volatile_register::RW<u32>,
}

impl Diepint0 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Diepint0R, &'w mut Diepint0W) -> &'w mut Diepint0W
    {
        let bits = self.register.read();
        let r = Diepint0R { bits: bits };
        let mut w = Diepint0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Diepint0R {
        Diepint0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Diepint0W) -> &mut Diepint0W
    {
        let mut w = Diepint0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Diepint0R {
    bits: u32,
}

impl Diepint0R {
    # [ doc = "Bit 7 - TXFE" ]
    pub fn txfe(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - INEPNE" ]
    pub fn inepne(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - ITTXFE" ]
    pub fn ittxfe(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - TOC" ]
    pub fn toc(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - EPDISD" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - XFRC" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Diepint0W {
    bits: u32,
}

impl Diepint0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Diepint0W { bits: 128u32 }
    }
    # [ doc = "Bit 6 - INEPNE" ]
    pub fn inepne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - ITTXFE" ]
    pub fn ittxfe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - TOC" ]
    pub fn toc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - EPDISD" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - XFRC" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
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
pub struct Diepint1 {
    register: ::volatile_register::RW<u32>,
}

impl Diepint1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Diepint1R, &'w mut Diepint1W) -> &'w mut Diepint1W
    {
        let bits = self.register.read();
        let r = Diepint1R { bits: bits };
        let mut w = Diepint1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Diepint1R {
        Diepint1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Diepint1W) -> &mut Diepint1W
    {
        let mut w = Diepint1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Diepint1R {
    bits: u32,
}

impl Diepint1R {
    # [ doc = "Bit 7 - TXFE" ]
    pub fn txfe(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - INEPNE" ]
    pub fn inepne(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - ITTXFE" ]
    pub fn ittxfe(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - TOC" ]
    pub fn toc(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - EPDISD" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - XFRC" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Diepint1W {
    bits: u32,
}

impl Diepint1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Diepint1W { bits: 128u32 }
    }
    # [ doc = "Bit 6 - INEPNE" ]
    pub fn inepne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - ITTXFE" ]
    pub fn ittxfe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - TOC" ]
    pub fn toc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - EPDISD" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - XFRC" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
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
pub struct Diepint2 {
    register: ::volatile_register::RW<u32>,
}

impl Diepint2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Diepint2R, &'w mut Diepint2W) -> &'w mut Diepint2W
    {
        let bits = self.register.read();
        let r = Diepint2R { bits: bits };
        let mut w = Diepint2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Diepint2R {
        Diepint2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Diepint2W) -> &mut Diepint2W
    {
        let mut w = Diepint2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Diepint2R {
    bits: u32,
}

impl Diepint2R {
    # [ doc = "Bit 7 - TXFE" ]
    pub fn txfe(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - INEPNE" ]
    pub fn inepne(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - ITTXFE" ]
    pub fn ittxfe(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - TOC" ]
    pub fn toc(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - EPDISD" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - XFRC" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Diepint2W {
    bits: u32,
}

impl Diepint2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Diepint2W { bits: 128u32 }
    }
    # [ doc = "Bit 6 - INEPNE" ]
    pub fn inepne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - ITTXFE" ]
    pub fn ittxfe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - TOC" ]
    pub fn toc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - EPDISD" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - XFRC" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
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
pub struct Diepint3 {
    register: ::volatile_register::RW<u32>,
}

impl Diepint3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Diepint3R, &'w mut Diepint3W) -> &'w mut Diepint3W
    {
        let bits = self.register.read();
        let r = Diepint3R { bits: bits };
        let mut w = Diepint3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Diepint3R {
        Diepint3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Diepint3W) -> &mut Diepint3W
    {
        let mut w = Diepint3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Diepint3R {
    bits: u32,
}

impl Diepint3R {
    # [ doc = "Bit 7 - TXFE" ]
    pub fn txfe(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - INEPNE" ]
    pub fn inepne(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - ITTXFE" ]
    pub fn ittxfe(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - TOC" ]
    pub fn toc(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - EPDISD" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - XFRC" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Diepint3W {
    bits: u32,
}

impl Diepint3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Diepint3W { bits: 128u32 }
    }
    # [ doc = "Bit 6 - INEPNE" ]
    pub fn inepne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - ITTXFE" ]
    pub fn ittxfe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - TOC" ]
    pub fn toc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - EPDISD" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - XFRC" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
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
pub struct Doepint0 {
    register: ::volatile_register::RW<u32>,
}

impl Doepint0 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Doepint0R, &'w mut Doepint0W) -> &'w mut Doepint0W
    {
        let bits = self.register.read();
        let r = Doepint0R { bits: bits };
        let mut w = Doepint0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Doepint0R {
        Doepint0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Doepint0W) -> &mut Doepint0W
    {
        let mut w = Doepint0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doepint0R {
    bits: u32,
}

impl Doepint0R {
    # [ doc = "Bit 6 - B2BSTUP" ]
    pub fn b2bstup(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - OTEPDIS" ]
    pub fn otepdis(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STUP" ]
    pub fn stup(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - EPDISD" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - XFRC" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doepint0W {
    bits: u32,
}

impl Doepint0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Doepint0W { bits: 128u32 }
    }
    # [ doc = "Bit 6 - B2BSTUP" ]
    pub fn b2bstup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - OTEPDIS" ]
    pub fn otepdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STUP" ]
    pub fn stup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - EPDISD" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - XFRC" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
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
pub struct Doepint1 {
    register: ::volatile_register::RW<u32>,
}

impl Doepint1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Doepint1R, &'w mut Doepint1W) -> &'w mut Doepint1W
    {
        let bits = self.register.read();
        let r = Doepint1R { bits: bits };
        let mut w = Doepint1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Doepint1R {
        Doepint1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Doepint1W) -> &mut Doepint1W
    {
        let mut w = Doepint1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doepint1R {
    bits: u32,
}

impl Doepint1R {
    # [ doc = "Bit 6 - B2BSTUP" ]
    pub fn b2bstup(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - OTEPDIS" ]
    pub fn otepdis(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STUP" ]
    pub fn stup(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - EPDISD" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - XFRC" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doepint1W {
    bits: u32,
}

impl Doepint1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Doepint1W { bits: 128u32 }
    }
    # [ doc = "Bit 6 - B2BSTUP" ]
    pub fn b2bstup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - OTEPDIS" ]
    pub fn otepdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STUP" ]
    pub fn stup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - EPDISD" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - XFRC" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
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
pub struct Doepint2 {
    register: ::volatile_register::RW<u32>,
}

impl Doepint2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Doepint2R, &'w mut Doepint2W) -> &'w mut Doepint2W
    {
        let bits = self.register.read();
        let r = Doepint2R { bits: bits };
        let mut w = Doepint2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Doepint2R {
        Doepint2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Doepint2W) -> &mut Doepint2W
    {
        let mut w = Doepint2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doepint2R {
    bits: u32,
}

impl Doepint2R {
    # [ doc = "Bit 6 - B2BSTUP" ]
    pub fn b2bstup(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - OTEPDIS" ]
    pub fn otepdis(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STUP" ]
    pub fn stup(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - EPDISD" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - XFRC" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doepint2W {
    bits: u32,
}

impl Doepint2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Doepint2W { bits: 128u32 }
    }
    # [ doc = "Bit 6 - B2BSTUP" ]
    pub fn b2bstup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - OTEPDIS" ]
    pub fn otepdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STUP" ]
    pub fn stup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - EPDISD" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - XFRC" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
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
pub struct Doepint3 {
    register: ::volatile_register::RW<u32>,
}

impl Doepint3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Doepint3R, &'w mut Doepint3W) -> &'w mut Doepint3W
    {
        let bits = self.register.read();
        let r = Doepint3R { bits: bits };
        let mut w = Doepint3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Doepint3R {
        Doepint3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Doepint3W) -> &mut Doepint3W
    {
        let mut w = Doepint3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doepint3R {
    bits: u32,
}

impl Doepint3R {
    # [ doc = "Bit 6 - B2BSTUP" ]
    pub fn b2bstup(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - OTEPDIS" ]
    pub fn otepdis(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - STUP" ]
    pub fn stup(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - EPDISD" ]
    pub fn epdisd(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - XFRC" ]
    pub fn xfrc(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doepint3W {
    bits: u32,
}

impl Doepint3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Doepint3W { bits: 128u32 }
    }
    # [ doc = "Bit 6 - B2BSTUP" ]
    pub fn b2bstup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - OTEPDIS" ]
    pub fn otepdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - STUP" ]
    pub fn stup(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - EPDISD" ]
    pub fn epdisd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - XFRC" ]
    pub fn xfrc(&mut self, value: bool) -> &mut Self {
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
pub struct Dieptsiz0 {
    register: ::volatile_register::RW<u32>,
}

impl Dieptsiz0 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dieptsiz0R, &'w mut Dieptsiz0W) -> &'w mut Dieptsiz0W
    {
        let bits = self.register.read();
        let r = Dieptsiz0R { bits: bits };
        let mut w = Dieptsiz0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dieptsiz0R {
        Dieptsiz0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dieptsiz0W) -> &mut Dieptsiz0W
    {
        let mut w = Dieptsiz0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dieptsiz0R {
    bits: u32,
}

impl Dieptsiz0R {
    # [ doc = "Bits 19:20 - Packet count" ]
    pub fn pktcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:6 - Transfer size" ]
    pub fn xfrsiz(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dieptsiz0W {
    bits: u32,
}

impl Dieptsiz0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dieptsiz0W { bits: 0u32 }
    }
    # [ doc = "Bits 19:20 - Packet count" ]
    pub fn pktcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 19u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:6 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Doeptsiz0 {
    register: ::volatile_register::RW<u32>,
}

impl Doeptsiz0 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Doeptsiz0R, &'w mut Doeptsiz0W) -> &'w mut Doeptsiz0W
    {
        let bits = self.register.read();
        let r = Doeptsiz0R { bits: bits };
        let mut w = Doeptsiz0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Doeptsiz0R {
        Doeptsiz0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Doeptsiz0W) -> &mut Doeptsiz0W
    {
        let mut w = Doeptsiz0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doeptsiz0R {
    bits: u32,
}

impl Doeptsiz0R {
    # [ doc = "Bits 29:30 - SETUP packet count" ]
    pub fn stupcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 19 - Packet count" ]
    pub fn pktcnt(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:6 - Transfer size" ]
    pub fn xfrsiz(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doeptsiz0W {
    bits: u32,
}

impl Doeptsiz0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Doeptsiz0W { bits: 0u32 }
    }
    # [ doc = "Bits 29:30 - SETUP packet count" ]
    pub fn stupcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 19 - Packet count" ]
    pub fn pktcnt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:6 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dieptsiz1 {
    register: ::volatile_register::RW<u32>,
}

impl Dieptsiz1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dieptsiz1R, &'w mut Dieptsiz1W) -> &'w mut Dieptsiz1W
    {
        let bits = self.register.read();
        let r = Dieptsiz1R { bits: bits };
        let mut w = Dieptsiz1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dieptsiz1R {
        Dieptsiz1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dieptsiz1W) -> &mut Dieptsiz1W
    {
        let mut w = Dieptsiz1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dieptsiz1R {
    bits: u32,
}

impl Dieptsiz1R {
    # [ doc = "Bits 29:30 - Multi count" ]
    pub fn mcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dieptsiz1W {
    bits: u32,
}

impl Dieptsiz1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dieptsiz1W { bits: 0u32 }
    }
    # [ doc = "Bits 29:30 - Multi count" ]
    pub fn mcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
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
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dieptsiz2 {
    register: ::volatile_register::RW<u32>,
}

impl Dieptsiz2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dieptsiz2R, &'w mut Dieptsiz2W) -> &'w mut Dieptsiz2W
    {
        let bits = self.register.read();
        let r = Dieptsiz2R { bits: bits };
        let mut w = Dieptsiz2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dieptsiz2R {
        Dieptsiz2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dieptsiz2W) -> &mut Dieptsiz2W
    {
        let mut w = Dieptsiz2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dieptsiz2R {
    bits: u32,
}

impl Dieptsiz2R {
    # [ doc = "Bits 29:30 - Multi count" ]
    pub fn mcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dieptsiz2W {
    bits: u32,
}

impl Dieptsiz2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dieptsiz2W { bits: 0u32 }
    }
    # [ doc = "Bits 29:30 - Multi count" ]
    pub fn mcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
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
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dieptsiz3 {
    register: ::volatile_register::RW<u32>,
}

impl Dieptsiz3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Dieptsiz3R, &'w mut Dieptsiz3W) -> &'w mut Dieptsiz3W
    {
        let bits = self.register.read();
        let r = Dieptsiz3R { bits: bits };
        let mut w = Dieptsiz3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Dieptsiz3R {
        Dieptsiz3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Dieptsiz3W) -> &mut Dieptsiz3W
    {
        let mut w = Dieptsiz3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dieptsiz3R {
    bits: u32,
}

impl Dieptsiz3R {
    # [ doc = "Bits 29:30 - Multi count" ]
    pub fn mcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dieptsiz3W {
    bits: u32,
}

impl Dieptsiz3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dieptsiz3W { bits: 0u32 }
    }
    # [ doc = "Bits 29:30 - Multi count" ]
    pub fn mcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
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
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dtxfsts0 {
    register: ::volatile_register::RO<u32>,
}

impl Dtxfsts0 {
    pub fn read(&self) -> Dtxfsts0R {
        Dtxfsts0R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dtxfsts0R {
    bits: u32,
}

impl Dtxfsts0R {
    # [ doc = "Bits 0:15 - IN endpoint TxFIFO space available" ]
    pub fn ineptfsav(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dtxfsts0W {
    bits: u32,
}

impl Dtxfsts0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dtxfsts0W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - IN endpoint TxFIFO space available" ]
    pub fn ineptfsav(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dtxfsts1 {
    register: ::volatile_register::RO<u32>,
}

impl Dtxfsts1 {
    pub fn read(&self) -> Dtxfsts1R {
        Dtxfsts1R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dtxfsts1R {
    bits: u32,
}

impl Dtxfsts1R {
    # [ doc = "Bits 0:15 - IN endpoint TxFIFO space available" ]
    pub fn ineptfsav(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dtxfsts1W {
    bits: u32,
}

impl Dtxfsts1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dtxfsts1W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - IN endpoint TxFIFO space available" ]
    pub fn ineptfsav(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dtxfsts2 {
    register: ::volatile_register::RO<u32>,
}

impl Dtxfsts2 {
    pub fn read(&self) -> Dtxfsts2R {
        Dtxfsts2R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dtxfsts2R {
    bits: u32,
}

impl Dtxfsts2R {
    # [ doc = "Bits 0:15 - IN endpoint TxFIFO space available" ]
    pub fn ineptfsav(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dtxfsts2W {
    bits: u32,
}

impl Dtxfsts2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dtxfsts2W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - IN endpoint TxFIFO space available" ]
    pub fn ineptfsav(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dtxfsts3 {
    register: ::volatile_register::RO<u32>,
}

impl Dtxfsts3 {
    pub fn read(&self) -> Dtxfsts3R {
        Dtxfsts3R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dtxfsts3R {
    bits: u32,
}

impl Dtxfsts3R {
    # [ doc = "Bits 0:15 - IN endpoint TxFIFO space available" ]
    pub fn ineptfsav(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Dtxfsts3W {
    bits: u32,
}

impl Dtxfsts3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Dtxfsts3W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - IN endpoint TxFIFO space available" ]
    pub fn ineptfsav(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Doeptsiz1 {
    register: ::volatile_register::RW<u32>,
}

impl Doeptsiz1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Doeptsiz1R, &'w mut Doeptsiz1W) -> &'w mut Doeptsiz1W
    {
        let bits = self.register.read();
        let r = Doeptsiz1R { bits: bits };
        let mut w = Doeptsiz1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Doeptsiz1R {
        Doeptsiz1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Doeptsiz1W) -> &mut Doeptsiz1W
    {
        let mut w = Doeptsiz1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doeptsiz1R {
    bits: u32,
}

impl Doeptsiz1R {
    # [ doc = "Bits 29:30 - Received data PID/SETUP packet count" ]
    pub fn rxdpid_stupcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doeptsiz1W {
    bits: u32,
}

impl Doeptsiz1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Doeptsiz1W { bits: 0u32 }
    }
    # [ doc = "Bits 29:30 - Received data PID/SETUP packet count" ]
    pub fn rxdpid_stupcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
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
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Doeptsiz2 {
    register: ::volatile_register::RW<u32>,
}

impl Doeptsiz2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Doeptsiz2R, &'w mut Doeptsiz2W) -> &'w mut Doeptsiz2W
    {
        let bits = self.register.read();
        let r = Doeptsiz2R { bits: bits };
        let mut w = Doeptsiz2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Doeptsiz2R {
        Doeptsiz2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Doeptsiz2W) -> &mut Doeptsiz2W
    {
        let mut w = Doeptsiz2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doeptsiz2R {
    bits: u32,
}

impl Doeptsiz2R {
    # [ doc = "Bits 29:30 - Received data PID/SETUP packet count" ]
    pub fn rxdpid_stupcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doeptsiz2W {
    bits: u32,
}

impl Doeptsiz2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Doeptsiz2W { bits: 0u32 }
    }
    # [ doc = "Bits 29:30 - Received data PID/SETUP packet count" ]
    pub fn rxdpid_stupcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
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
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Doeptsiz3 {
    register: ::volatile_register::RW<u32>,
}

impl Doeptsiz3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Doeptsiz3R, &'w mut Doeptsiz3W) -> &'w mut Doeptsiz3W
    {
        let bits = self.register.read();
        let r = Doeptsiz3R { bits: bits };
        let mut w = Doeptsiz3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Doeptsiz3R {
        Doeptsiz3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Doeptsiz3W) -> &mut Doeptsiz3W
    {
        let mut w = Doeptsiz3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doeptsiz3R {
    bits: u32,
}

impl Doeptsiz3R {
    # [ doc = "Bits 29:30 - Received data PID/SETUP packet count" ]
    pub fn rxdpid_stupcnt(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 29u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 19:28 - Packet count" ]
    pub fn pktcnt(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 19u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Doeptsiz3W {
    bits: u32,
}

impl Doeptsiz3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Doeptsiz3W { bits: 0u32 }
    }
    # [ doc = "Bits 29:30 - Received data PID/SETUP packet count" ]
    pub fn rxdpid_stupcnt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 29u8;
        const MASK: u8 = 3;
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
    # [ doc = "Bits 0:18 - Transfer size" ]
    pub fn xfrsiz(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 524287;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
