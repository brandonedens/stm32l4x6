# [ doc = "Secure digital input/output interface" ]
# [ repr ( C ) ]
pub struct Sdmmc {
    # [ doc = "0x00 - power control register" ]
    pub power: Power,
    # [ doc = "0x04 - SDI clock control register" ]
    pub clkcr: Clkcr,
    # [ doc = "0x08 - argument register" ]
    pub arg: Arg,
    # [ doc = "0x0c - command register" ]
    pub cmd: Cmd,
    # [ doc = "0x10 - command response register" ]
    pub respcmd: Respcmd,
    # [ doc = "0x14 - response 1..4 register" ]
    pub resp1: Resp1,
    # [ doc = "0x18 - response 1..4 register" ]
    pub resp2: Resp2,
    # [ doc = "0x1c - response 1..4 register" ]
    pub resp3: Resp3,
    # [ doc = "0x20 - response 1..4 register" ]
    pub resp4: Resp4,
    # [ doc = "0x24 - data timer register" ]
    pub dtimer: Dtimer,
    # [ doc = "0x28 - data length register" ]
    pub dlen: Dlen,
    # [ doc = "0x2c - data control register" ]
    pub dctrl: Dctrl,
    # [ doc = "0x30 - data counter register" ]
    pub dcount: Dcount,
    # [ doc = "0x34 - status register" ]
    pub sta: Sta,
    # [ doc = "0x38 - interrupt clear register" ]
    pub icr: Icr,
    # [ doc = "0x3c - mask register" ]
    pub mask: Mask,
    _reserved0: [u8; 8usize],
    # [ doc = "0x48 - FIFO counter register" ]
    pub fifocnt: Fifocnt,
    _reserved1: [u8; 52usize],
    # [ doc = "0x80 - data FIFO register" ]
    pub fifo: Fifo,
}

# [ repr ( C ) ]
pub struct Power {
    register: ::volatile_register::RW<u32>,
}

impl Power {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PowerR, &'w mut PowerW) -> &'w mut PowerW
    {
        let bits = self.register.read();
        let r = PowerR { bits: bits };
        let mut w = PowerW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PowerR {
        PowerR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PowerW) -> &mut PowerW
    {
        let mut w = PowerW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PowerR {
    bits: u32,
}

impl PowerR {
    # [ doc = "Bits 0:1 - PWRCTRL" ]
    pub fn pwrctrl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PowerW {
    bits: u32,
}

impl PowerW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PowerW { bits: 0u32 }
    }
    # [ doc = "Bits 0:1 - PWRCTRL" ]
    pub fn pwrctrl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Clkcr {
    register: ::volatile_register::RW<u32>,
}

impl Clkcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ClkcrR, &'w mut ClkcrW) -> &'w mut ClkcrW
    {
        let bits = self.register.read();
        let r = ClkcrR { bits: bits };
        let mut w = ClkcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ClkcrR {
        ClkcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ClkcrW) -> &mut ClkcrW
    {
        let mut w = ClkcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkcrR {
    bits: u32,
}

impl ClkcrR {
    # [ doc = "Bit 14 - HW Flow Control enable" ]
    pub fn hwfc_en(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - SDIO_CK dephasing selection bit" ]
    pub fn negedge(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 11:12 - Wide bus mode enable bit" ]
    pub fn widbus(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 11u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 10 - Clock divider bypass enable bit" ]
    pub fn bypass(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Power saving configuration bit" ]
    pub fn pwrsav(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Clock enable bit" ]
    pub fn clken(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:7 - Clock divide factor" ]
    pub fn clkdiv(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ClkcrW {
    bits: u32,
}

impl ClkcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ClkcrW { bits: 0u32 }
    }
    # [ doc = "Bit 14 - HW Flow Control enable" ]
    pub fn hwfc_en(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - SDIO_CK dephasing selection bit" ]
    pub fn negedge(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 11:12 - Wide bus mode enable bit" ]
    pub fn widbus(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 11u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 10 - Clock divider bypass enable bit" ]
    pub fn bypass(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Power saving configuration bit" ]
    pub fn pwrsav(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Clock enable bit" ]
    pub fn clken(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:7 - Clock divide factor" ]
    pub fn clkdiv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Arg {
    register: ::volatile_register::RW<u32>,
}

impl Arg {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&ArgR, &'w mut ArgW) -> &'w mut ArgW
    {
        let bits = self.register.read();
        let r = ArgR { bits: bits };
        let mut w = ArgW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> ArgR {
        ArgR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut ArgW) -> &mut ArgW
    {
        let mut w = ArgW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ArgR {
    bits: u32,
}

impl ArgR {
    # [ doc = "Bits 0:31 - Command argument" ]
    pub fn cmdarg(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct ArgW {
    bits: u32,
}

impl ArgW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        ArgW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Command argument" ]
    pub fn cmdarg(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cmd {
    register: ::volatile_register::RW<u32>,
}

impl Cmd {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CmdR, &'w mut CmdW) -> &'w mut CmdW
    {
        let bits = self.register.read();
        let r = CmdR { bits: bits };
        let mut w = CmdW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CmdR {
        CmdR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CmdW) -> &mut CmdW
    {
        let mut w = CmdW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CmdR {
    bits: u32,
}

impl CmdR {
    # [ doc = "Bit 14 - CE-ATA command" ]
    pub fn ce_atacmd(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - not Interrupt Enable" ]
    pub fn n_ien(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Enable CMD completion" ]
    pub fn encmdcompl(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - SD I/O suspend command" ]
    pub fn sdiosuspend(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Command path state machine (CPSM) Enable bit" ]
    pub fn cpsmen(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - CPSM Waits for ends of data transfer (CmdPend internal signal)" ]
    pub fn waitpend(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - CPSM waits for interrupt request" ]
    pub fn waitint(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 6:7 - Wait for response bits" ]
    pub fn waitresp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:5 - Command index" ]
    pub fn cmdindex(&self) -> u8 {
        const MASK: u32 = 63;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CmdW {
    bits: u32,
}

impl CmdW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CmdW { bits: 0u32 }
    }
    # [ doc = "Bit 14 - CE-ATA command" ]
    pub fn ce_atacmd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - not Interrupt Enable" ]
    pub fn n_ien(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Enable CMD completion" ]
    pub fn encmdcompl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - SD I/O suspend command" ]
    pub fn sdiosuspend(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Command path state machine (CPSM) Enable bit" ]
    pub fn cpsmen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - CPSM Waits for ends of data transfer (CmdPend internal signal)" ]
    pub fn waitpend(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - CPSM waits for interrupt request" ]
    pub fn waitint(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 6:7 - Wait for response bits" ]
    pub fn waitresp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:5 - Command index" ]
    pub fn cmdindex(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 63;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Respcmd {
    register: ::volatile_register::RO<u32>,
}

impl Respcmd {
    pub fn read(&self) -> RespcmdR {
        RespcmdR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RespcmdR {
    bits: u32,
}

impl RespcmdR {
    # [ doc = "Bits 0:5 - Response command index" ]
    pub fn respcmd(&self) -> u8 {
        const MASK: u32 = 63;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RespcmdW {
    bits: u32,
}

impl RespcmdW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        RespcmdW { bits: 0u32 }
    }
    # [ doc = "Bits 0:5 - Response command index" ]
    pub fn respcmd(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 63;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Resp1 {
    register: ::volatile_register::RO<u32>,
}

impl Resp1 {
    pub fn read(&self) -> Resp1R {
        Resp1R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Resp1R {
    bits: u32,
}

impl Resp1R {
    # [ doc = "Bits 0:31 - see Table 132" ]
    pub fn cardstatus1(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Resp1W {
    bits: u32,
}

impl Resp1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Resp1W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - see Table 132" ]
    pub fn cardstatus1(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Resp2 {
    register: ::volatile_register::RO<u32>,
}

impl Resp2 {
    pub fn read(&self) -> Resp2R {
        Resp2R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Resp2R {
    bits: u32,
}

impl Resp2R {
    # [ doc = "Bits 0:31 - see Table 132" ]
    pub fn cardstatus2(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Resp2W {
    bits: u32,
}

impl Resp2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Resp2W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - see Table 132" ]
    pub fn cardstatus2(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Resp3 {
    register: ::volatile_register::RO<u32>,
}

impl Resp3 {
    pub fn read(&self) -> Resp3R {
        Resp3R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Resp3R {
    bits: u32,
}

impl Resp3R {
    # [ doc = "Bits 0:31 - see Table 132" ]
    pub fn cardstatus3(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Resp3W {
    bits: u32,
}

impl Resp3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Resp3W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - see Table 132" ]
    pub fn cardstatus3(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Resp4 {
    register: ::volatile_register::RO<u32>,
}

impl Resp4 {
    pub fn read(&self) -> Resp4R {
        Resp4R { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Resp4R {
    bits: u32,
}

impl Resp4R {
    # [ doc = "Bits 0:31 - see Table 132" ]
    pub fn cardstatus4(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Resp4W {
    bits: u32,
}

impl Resp4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Resp4W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - see Table 132" ]
    pub fn cardstatus4(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dtimer {
    register: ::volatile_register::RW<u32>,
}

impl Dtimer {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DtimerR, &'w mut DtimerW) -> &'w mut DtimerW
    {
        let bits = self.register.read();
        let r = DtimerR { bits: bits };
        let mut w = DtimerW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DtimerR {
        DtimerR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DtimerW) -> &mut DtimerW
    {
        let mut w = DtimerW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DtimerR {
    bits: u32,
}

impl DtimerR {
    # [ doc = "Bits 0:31 - Data timeout period" ]
    pub fn datatime(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DtimerW {
    bits: u32,
}

impl DtimerW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DtimerW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Data timeout period" ]
    pub fn datatime(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dlen {
    register: ::volatile_register::RW<u32>,
}

impl Dlen {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DlenR, &'w mut DlenW) -> &'w mut DlenW
    {
        let bits = self.register.read();
        let r = DlenR { bits: bits };
        let mut w = DlenW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DlenR {
        DlenR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DlenW) -> &mut DlenW
    {
        let mut w = DlenW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DlenR {
    bits: u32,
}

impl DlenR {
    # [ doc = "Bits 0:24 - Data length value" ]
    pub fn datalength(&self) -> u32 {
        const MASK: u32 = 33554431;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DlenW {
    bits: u32,
}

impl DlenW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DlenW { bits: 0u32 }
    }
    # [ doc = "Bits 0:24 - Data length value" ]
    pub fn datalength(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 33554431;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Dctrl {
    register: ::volatile_register::RW<u32>,
}

impl Dctrl {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DctrlR, &'w mut DctrlW) -> &'w mut DctrlW
    {
        let bits = self.register.read();
        let r = DctrlR { bits: bits };
        let mut w = DctrlW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DctrlR {
        DctrlR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DctrlW) -> &mut DctrlW
    {
        let mut w = DctrlW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DctrlR {
    bits: u32,
}

impl DctrlR {
    # [ doc = "Bit 11 - SD I/O enable functions" ]
    pub fn sdioen(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Read wait mode" ]
    pub fn rwmod(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Read wait stop" ]
    pub fn rwstop(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Read wait start" ]
    pub fn rwstart(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:7 - Data block size" ]
    pub fn dblocksize(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 3 - DMA enable bit" ]
    pub fn dmaen(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Data transfer mode selection 1: Stream or SDIO multibyte data transfer" ]
    pub fn dtmode(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Data transfer direction selection" ]
    pub fn dtdir(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - DTEN" ]
    pub fn dten(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DctrlW {
    bits: u32,
}

impl DctrlW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DctrlW { bits: 0u32 }
    }
    # [ doc = "Bit 11 - SD I/O enable functions" ]
    pub fn sdioen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Read wait mode" ]
    pub fn rwmod(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Read wait stop" ]
    pub fn rwstop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Read wait start" ]
    pub fn rwstart(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:7 - Data block size" ]
    pub fn dblocksize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 3 - DMA enable bit" ]
    pub fn dmaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Data transfer mode selection 1: Stream or SDIO multibyte data transfer" ]
    pub fn dtmode(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Data transfer direction selection" ]
    pub fn dtdir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - DTEN" ]
    pub fn dten(&mut self, value: bool) -> &mut Self {
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
pub struct Dcount {
    register: ::volatile_register::RO<u32>,
}

impl Dcount {
    pub fn read(&self) -> DcountR {
        DcountR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DcountR {
    bits: u32,
}

impl DcountR {
    # [ doc = "Bits 0:24 - Data count value" ]
    pub fn datacount(&self) -> u32 {
        const MASK: u32 = 33554431;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DcountW {
    bits: u32,
}

impl DcountW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DcountW { bits: 0u32 }
    }
    # [ doc = "Bits 0:24 - Data count value" ]
    pub fn datacount(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 33554431;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Sta {
    register: ::volatile_register::RO<u32>,
}

impl Sta {
    pub fn read(&self) -> StaR {
        StaR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct StaR {
    bits: u32,
}

impl StaR {
    # [ doc = "Bit 23 - CE-ATA command completion signal received for CMD61" ]
    pub fn ceataend(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - SDIO interrupt received" ]
    pub fn sdioit(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Data available in receive FIFO" ]
    pub fn rxdavl(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Data available in transmit FIFO" ]
    pub fn txdavl(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Receive FIFO empty" ]
    pub fn rxfifoe(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Transmit FIFO empty" ]
    pub fn txfifoe(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Receive FIFO full" ]
    pub fn rxfifof(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Transmit FIFO full" ]
    pub fn txfifof(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Receive FIFO half full: there are at least 8 words in the FIFO" ]
    pub fn rxfifohf(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Transmit FIFO half empty: at least 8 words can be written into the FIFO" ]
    pub fn txfifohe(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Data receive in progress" ]
    pub fn rxact(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Data transmit in progress" ]
    pub fn txact(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Command transfer in progress" ]
    pub fn cmdact(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data block sent/received (CRC check passed)" ]
    pub fn dbckend(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Start bit not detected on all data signals in wide bus mode" ]
    pub fn stbiterr(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Data end (data counter, SDIDCOUNT, is zero)" ]
    pub fn dataend(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Command sent (no response required)" ]
    pub fn cmdsent(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Command response received (CRC check passed)" ]
    pub fn cmdrend(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Received FIFO overrun error" ]
    pub fn rxoverr(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Transmit FIFO underrun error" ]
    pub fn txunderr(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Data timeout" ]
    pub fn dtimeout(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Command response timeout" ]
    pub fn ctimeout(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Data block sent/received (CRC check failed)" ]
    pub fn dcrcfail(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Command response received (CRC check failed)" ]
    pub fn ccrcfail(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct StaW {
    bits: u32,
}

impl StaW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        StaW { bits: 0u32 }
    }
    # [ doc = "Bit 23 - CE-ATA command completion signal received for CMD61" ]
    pub fn ceataend(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - SDIO interrupt received" ]
    pub fn sdioit(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Data available in receive FIFO" ]
    pub fn rxdavl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Data available in transmit FIFO" ]
    pub fn txdavl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Receive FIFO empty" ]
    pub fn rxfifoe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Transmit FIFO empty" ]
    pub fn txfifoe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Receive FIFO full" ]
    pub fn rxfifof(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Transmit FIFO full" ]
    pub fn txfifof(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Receive FIFO half full: there are at least 8 words in the FIFO" ]
    pub fn rxfifohf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Transmit FIFO half empty: at least 8 words can be written into the FIFO" ]
    pub fn txfifohe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Data receive in progress" ]
    pub fn rxact(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Data transmit in progress" ]
    pub fn txact(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Command transfer in progress" ]
    pub fn cmdact(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data block sent/received (CRC check passed)" ]
    pub fn dbckend(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Start bit not detected on all data signals in wide bus mode" ]
    pub fn stbiterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Data end (data counter, SDIDCOUNT, is zero)" ]
    pub fn dataend(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Command sent (no response required)" ]
    pub fn cmdsent(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Command response received (CRC check passed)" ]
    pub fn cmdrend(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Received FIFO overrun error" ]
    pub fn rxoverr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Transmit FIFO underrun error" ]
    pub fn txunderr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Data timeout" ]
    pub fn dtimeout(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Command response timeout" ]
    pub fn ctimeout(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Data block sent/received (CRC check failed)" ]
    pub fn dcrcfail(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Command response received (CRC check failed)" ]
    pub fn ccrcfail(&mut self, value: bool) -> &mut Self {
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
    register: ::volatile_register::RW<u32>,
}

impl Icr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&IcrR, &'w mut IcrW) -> &'w mut IcrW
    {
        let bits = self.register.read();
        let r = IcrR { bits: bits };
        let mut w = IcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IcrR {
        IcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
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
    # [ doc = "Bit 23 - CEATAEND flag clear bit" ]
    pub fn ceataendc(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - SDIOIT flag clear bit" ]
    pub fn sdioitc(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - DBCKEND flag clear bit" ]
    pub fn dbckendc(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - STBITERR flag clear bit" ]
    pub fn stbiterrc(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - DATAEND flag clear bit" ]
    pub fn dataendc(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - CMDSENT flag clear bit" ]
    pub fn cmdsentc(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - CMDREND flag clear bit" ]
    pub fn cmdrendc(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - RXOVERR flag clear bit" ]
    pub fn rxoverrc(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - TXUNDERR flag clear bit" ]
    pub fn txunderrc(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - DTIMEOUT flag clear bit" ]
    pub fn dtimeoutc(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - CTIMEOUT flag clear bit" ]
    pub fn ctimeoutc(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - DCRCFAIL flag clear bit" ]
    pub fn dcrcfailc(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - CCRCFAIL flag clear bit" ]
    pub fn ccrcfailc(&self) -> bool {
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
    # [ doc = "Bit 23 - CEATAEND flag clear bit" ]
    pub fn ceataendc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - SDIOIT flag clear bit" ]
    pub fn sdioitc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - DBCKEND flag clear bit" ]
    pub fn dbckendc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - STBITERR flag clear bit" ]
    pub fn stbiterrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - DATAEND flag clear bit" ]
    pub fn dataendc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - CMDSENT flag clear bit" ]
    pub fn cmdsentc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - CMDREND flag clear bit" ]
    pub fn cmdrendc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - RXOVERR flag clear bit" ]
    pub fn rxoverrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - TXUNDERR flag clear bit" ]
    pub fn txunderrc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - DTIMEOUT flag clear bit" ]
    pub fn dtimeoutc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - CTIMEOUT flag clear bit" ]
    pub fn ctimeoutc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - DCRCFAIL flag clear bit" ]
    pub fn dcrcfailc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - CCRCFAIL flag clear bit" ]
    pub fn ccrcfailc(&mut self, value: bool) -> &mut Self {
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
pub struct Mask {
    register: ::volatile_register::RW<u32>,
}

impl Mask {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&MaskR, &'w mut MaskW) -> &'w mut MaskW
    {
        let bits = self.register.read();
        let r = MaskR { bits: bits };
        let mut w = MaskW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> MaskR {
        MaskR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut MaskW) -> &mut MaskW
    {
        let mut w = MaskW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MaskR {
    bits: u32,
}

impl MaskR {
    # [ doc = "Bit 23 - CE-ATA command completion signal received interrupt enable" ]
    pub fn ceataendie(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - SDIO mode interrupt received interrupt enable" ]
    pub fn sdioitie(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Data available in Rx FIFO interrupt enable" ]
    pub fn rxdavlie(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Data available in Tx FIFO interrupt enable" ]
    pub fn txdavlie(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Rx FIFO empty interrupt enable" ]
    pub fn rxfifoeie(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Tx FIFO empty interrupt enable" ]
    pub fn txfifoeie(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Rx FIFO full interrupt enable" ]
    pub fn rxfifofie(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Tx FIFO full interrupt enable" ]
    pub fn txfifofie(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Rx FIFO half full interrupt enable" ]
    pub fn rxfifohfie(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Tx FIFO half empty interrupt enable" ]
    pub fn txfifoheie(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Data receive acting interrupt enable" ]
    pub fn rxactie(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Data transmit acting interrupt enable" ]
    pub fn txactie(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Command acting interrupt enable" ]
    pub fn cmdactie(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data block end interrupt enable" ]
    pub fn dbckendie(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Start bit error interrupt enable" ]
    pub fn stbiterrie(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Data end interrupt enable" ]
    pub fn dataendie(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Command sent interrupt enable" ]
    pub fn cmdsentie(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Command response received interrupt enable" ]
    pub fn cmdrendie(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Rx FIFO overrun error interrupt enable" ]
    pub fn rxoverrie(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Tx FIFO underrun error interrupt enable" ]
    pub fn txunderrie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Data timeout interrupt enable" ]
    pub fn dtimeoutie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Command timeout interrupt enable" ]
    pub fn ctimeoutie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Data CRC fail interrupt enable" ]
    pub fn dcrcfailie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Command CRC fail interrupt enable" ]
    pub fn ccrcfailie(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct MaskW {
    bits: u32,
}

impl MaskW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        MaskW { bits: 0u32 }
    }
    # [ doc = "Bit 23 - CE-ATA command completion signal received interrupt enable" ]
    pub fn ceataendie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - SDIO mode interrupt received interrupt enable" ]
    pub fn sdioitie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Data available in Rx FIFO interrupt enable" ]
    pub fn rxdavlie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Data available in Tx FIFO interrupt enable" ]
    pub fn txdavlie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Rx FIFO empty interrupt enable" ]
    pub fn rxfifoeie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Tx FIFO empty interrupt enable" ]
    pub fn txfifoeie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Rx FIFO full interrupt enable" ]
    pub fn rxfifofie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Tx FIFO full interrupt enable" ]
    pub fn txfifofie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Rx FIFO half full interrupt enable" ]
    pub fn rxfifohfie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Tx FIFO half empty interrupt enable" ]
    pub fn txfifoheie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Data receive acting interrupt enable" ]
    pub fn rxactie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Data transmit acting interrupt enable" ]
    pub fn txactie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Command acting interrupt enable" ]
    pub fn cmdactie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data block end interrupt enable" ]
    pub fn dbckendie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Start bit error interrupt enable" ]
    pub fn stbiterrie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Data end interrupt enable" ]
    pub fn dataendie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Command sent interrupt enable" ]
    pub fn cmdsentie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Command response received interrupt enable" ]
    pub fn cmdrendie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Rx FIFO overrun error interrupt enable" ]
    pub fn rxoverrie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Tx FIFO underrun error interrupt enable" ]
    pub fn txunderrie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Data timeout interrupt enable" ]
    pub fn dtimeoutie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Command timeout interrupt enable" ]
    pub fn ctimeoutie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Data CRC fail interrupt enable" ]
    pub fn dcrcfailie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Command CRC fail interrupt enable" ]
    pub fn ccrcfailie(&mut self, value: bool) -> &mut Self {
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
pub struct Fifocnt {
    register: ::volatile_register::RO<u32>,
}

impl Fifocnt {
    pub fn read(&self) -> FifocntR {
        FifocntR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FifocntR {
    bits: u32,
}

impl FifocntR {
    # [ doc = "Bits 0:23 - Remaining number of words to be written to or read from the FIFO" ]
    pub fn fifocount(&self) -> u32 {
        const MASK: u32 = 16777215;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FifocntW {
    bits: u32,
}

impl FifocntW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FifocntW { bits: 0u32 }
    }
    # [ doc = "Bits 0:23 - Remaining number of words to be written to or read from the FIFO" ]
    pub fn fifocount(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 16777215;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Fifo {
    register: ::volatile_register::RW<u32>,
}

impl Fifo {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&FifoR, &'w mut FifoW) -> &'w mut FifoW
    {
        let bits = self.register.read();
        let r = FifoR { bits: bits };
        let mut w = FifoW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> FifoR {
        FifoR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut FifoW) -> &mut FifoW
    {
        let mut w = FifoW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FifoR {
    bits: u32,
}

impl FifoR {
    # [ doc = "Bits 0:31 - Receive and transmit FIFO data" ]
    pub fn fifodata(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct FifoW {
    bits: u32,
}

impl FifoW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        FifoW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Receive and transmit FIFO data" ]
    pub fn fifodata(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
