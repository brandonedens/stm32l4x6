# [ doc = "Flexible memory controller" ]
# [ repr ( C ) ]
pub struct Fmc {
    # [ doc = "0x00 - SRAM/NOR-Flash chip-select control register 1" ]
    pub bcr1: Bcr1,
    # [ doc = "0x04 - SRAM/NOR-Flash chip-select timing register 1" ]
    pub btr1: Btr1,
    # [ doc = "0x08 - SRAM/NOR-Flash chip-select control register 2" ]
    pub bcr2: Bcr2,
    # [ doc = "0x0c - SRAM/NOR-Flash chip-select timing register 2" ]
    pub btr2: Btr2,
    # [ doc = "0x10 - SRAM/NOR-Flash chip-select control register 3" ]
    pub bcr3: Bcr3,
    # [ doc = "0x14 - SRAM/NOR-Flash chip-select timing register 3" ]
    pub btr3: Btr3,
    # [ doc = "0x18 - SRAM/NOR-Flash chip-select control register 4" ]
    pub bcr4: Bcr4,
    # [ doc = "0x1c - SRAM/NOR-Flash chip-select timing register 4" ]
    pub btr4: Btr4,
    _reserved0: [u8; 96usize],
    # [ doc = "0x80 - PC Card/NAND Flash control register 3" ]
    pub pcr: Pcr,
    # [ doc = "0x84 - FIFO status and interrupt register 3" ]
    pub sr: Sr,
    # [ doc = "0x88 - Common memory space timing register 3" ]
    pub pmem: Pmem,
    # [ doc = "0x8c - Attribute memory space timing register 3" ]
    pub patt: Patt,
    _reserved1: [u8; 4usize],
    # [ doc = "0x94 - ECC result register 3" ]
    pub eccr: Eccr,
    _reserved2: [u8; 108usize],
    # [ doc = "0x104 - SRAM/NOR-Flash write timing registers 1" ]
    pub bwtr1: Bwtr1,
    _reserved3: [u8; 4usize],
    # [ doc = "0x10c - SRAM/NOR-Flash write timing registers 2" ]
    pub bwtr2: Bwtr2,
    _reserved4: [u8; 4usize],
    # [ doc = "0x114 - SRAM/NOR-Flash write timing registers 3" ]
    pub bwtr3: Bwtr3,
    _reserved5: [u8; 4usize],
    # [ doc = "0x11c - SRAM/NOR-Flash write timing registers 4" ]
    pub bwtr4: Bwtr4,
}

# [ repr ( C ) ]
pub struct Bcr1 {
    register: ::volatile_register::RW<u32>,
}

impl Bcr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Bcr1R, &'w mut Bcr1W) -> &'w mut Bcr1W
    {
        let bits = self.register.read();
        let r = Bcr1R { bits: bits };
        let mut w = Bcr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bcr1R {
        Bcr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bcr1W) -> &mut Bcr1W
    {
        let mut w = Bcr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bcr1R {
    bits: u32,
}

impl Bcr1R {
    # [ doc = "Bit 0 - MBKEN" ]
    pub fn mbken(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - MUXEN" ]
    pub fn muxen(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 2:3 - MTYP" ]
    pub fn mtyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:5 - MWID" ]
    pub fn mwid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 6 - FACCEN" ]
    pub fn faccen(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - BURSTEN" ]
    pub fn bursten(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - WAITPOL" ]
    pub fn waitpol(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - WAITCFG" ]
    pub fn waitcfg(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - WREN" ]
    pub fn wren(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - WAITEN" ]
    pub fn waiten(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - EXTMOD" ]
    pub fn extmod(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - ASYNCWAIT" ]
    pub fn asyncwait(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - CBURSTRW" ]
    pub fn cburstrw(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - CCLKEN" ]
    pub fn cclken(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Write FIFO Disable" ]
    pub fn wfdis(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bcr1W {
    bits: u32,
}

impl Bcr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bcr1W { bits: 12496u32 }
    }
    # [ doc = "Bit 0 - MBKEN" ]
    pub fn mbken(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - MUXEN" ]
    pub fn muxen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 2:3 - MTYP" ]
    pub fn mtyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:5 - MWID" ]
    pub fn mwid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 6 - FACCEN" ]
    pub fn faccen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - BURSTEN" ]
    pub fn bursten(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - WAITPOL" ]
    pub fn waitpol(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - WAITCFG" ]
    pub fn waitcfg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - WREN" ]
    pub fn wren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - WAITEN" ]
    pub fn waiten(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - EXTMOD" ]
    pub fn extmod(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - ASYNCWAIT" ]
    pub fn asyncwait(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - CBURSTRW" ]
    pub fn cburstrw(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - CCLKEN" ]
    pub fn cclken(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Write FIFO Disable" ]
    pub fn wfdis(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Btr1 {
    register: ::volatile_register::RW<u32>,
}

impl Btr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Btr1R, &'w mut Btr1W) -> &'w mut Btr1W
    {
        let bits = self.register.read();
        let r = Btr1R { bits: bits };
        let mut w = Btr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Btr1R {
        Btr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Btr1W) -> &mut Btr1W
    {
        let mut w = Btr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Btr1R {
    bits: u32,
}

impl Btr1R {
    # [ doc = "Bits 28:29 - ACCMOD" ]
    pub fn accmod(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - DATLAT" ]
    pub fn datlat(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - CLKDIV" ]
    pub fn clkdiv(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - BUSTURN" ]
    pub fn busturn(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - DATAST" ]
    pub fn datast(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - ADDHLD" ]
    pub fn addhld(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - ADDSET" ]
    pub fn addset(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Btr1W {
    bits: u32,
}

impl Btr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Btr1W { bits: 4294967295u32 }
    }
    # [ doc = "Bits 28:29 - ACCMOD" ]
    pub fn accmod(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - DATLAT" ]
    pub fn datlat(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - CLKDIV" ]
    pub fn clkdiv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - BUSTURN" ]
    pub fn busturn(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - DATAST" ]
    pub fn datast(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - ADDHLD" ]
    pub fn addhld(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - ADDSET" ]
    pub fn addset(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bcr2 {
    register: ::volatile_register::RW<u32>,
}

impl Bcr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Bcr2R, &'w mut Bcr2W) -> &'w mut Bcr2W
    {
        let bits = self.register.read();
        let r = Bcr2R { bits: bits };
        let mut w = Bcr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bcr2R {
        Bcr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bcr2W) -> &mut Bcr2W
    {
        let mut w = Bcr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bcr2R {
    bits: u32,
}

impl Bcr2R {
    # [ doc = "Bit 19 - CBURSTRW" ]
    pub fn cburstrw(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - ASYNCWAIT" ]
    pub fn asyncwait(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - EXTMOD" ]
    pub fn extmod(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - WAITEN" ]
    pub fn waiten(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - WREN" ]
    pub fn wren(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - WAITCFG" ]
    pub fn waitcfg(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - WRAPMOD" ]
    pub fn wrapmod(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - WAITPOL" ]
    pub fn waitpol(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - BURSTEN" ]
    pub fn bursten(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - FACCEN" ]
    pub fn faccen(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:5 - MWID" ]
    pub fn mwid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 2:3 - MTYP" ]
    pub fn mtyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 1 - MUXEN" ]
    pub fn muxen(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - MBKEN" ]
    pub fn mbken(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bcr2W {
    bits: u32,
}

impl Bcr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bcr2W { bits: 12496u32 }
    }
    # [ doc = "Bit 19 - CBURSTRW" ]
    pub fn cburstrw(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - ASYNCWAIT" ]
    pub fn asyncwait(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - EXTMOD" ]
    pub fn extmod(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - WAITEN" ]
    pub fn waiten(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - WREN" ]
    pub fn wren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - WAITCFG" ]
    pub fn waitcfg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - WRAPMOD" ]
    pub fn wrapmod(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - WAITPOL" ]
    pub fn waitpol(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - BURSTEN" ]
    pub fn bursten(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - FACCEN" ]
    pub fn faccen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:5 - MWID" ]
    pub fn mwid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 2:3 - MTYP" ]
    pub fn mtyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 1 - MUXEN" ]
    pub fn muxen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - MBKEN" ]
    pub fn mbken(&mut self, value: bool) -> &mut Self {
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
pub struct Btr2 {
    register: ::volatile_register::RW<u32>,
}

impl Btr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Btr2R, &'w mut Btr2W) -> &'w mut Btr2W
    {
        let bits = self.register.read();
        let r = Btr2R { bits: bits };
        let mut w = Btr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Btr2R {
        Btr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Btr2W) -> &mut Btr2W
    {
        let mut w = Btr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Btr2R {
    bits: u32,
}

impl Btr2R {
    # [ doc = "Bits 28:29 - ACCMOD" ]
    pub fn accmod(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - DATLAT" ]
    pub fn datlat(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - CLKDIV" ]
    pub fn clkdiv(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - BUSTURN" ]
    pub fn busturn(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - DATAST" ]
    pub fn datast(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - ADDHLD" ]
    pub fn addhld(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - ADDSET" ]
    pub fn addset(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Btr2W {
    bits: u32,
}

impl Btr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Btr2W { bits: 4294967295u32 }
    }
    # [ doc = "Bits 28:29 - ACCMOD" ]
    pub fn accmod(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - DATLAT" ]
    pub fn datlat(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - CLKDIV" ]
    pub fn clkdiv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - BUSTURN" ]
    pub fn busturn(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - DATAST" ]
    pub fn datast(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - ADDHLD" ]
    pub fn addhld(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - ADDSET" ]
    pub fn addset(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bcr3 {
    register: ::volatile_register::RW<u32>,
}

impl Bcr3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Bcr3R, &'w mut Bcr3W) -> &'w mut Bcr3W
    {
        let bits = self.register.read();
        let r = Bcr3R { bits: bits };
        let mut w = Bcr3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bcr3R {
        Bcr3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bcr3W) -> &mut Bcr3W
    {
        let mut w = Bcr3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bcr3R {
    bits: u32,
}

impl Bcr3R {
    # [ doc = "Bit 19 - CBURSTRW" ]
    pub fn cburstrw(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - ASYNCWAIT" ]
    pub fn asyncwait(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - EXTMOD" ]
    pub fn extmod(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - WAITEN" ]
    pub fn waiten(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - WREN" ]
    pub fn wren(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - WAITCFG" ]
    pub fn waitcfg(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - WRAPMOD" ]
    pub fn wrapmod(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - WAITPOL" ]
    pub fn waitpol(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - BURSTEN" ]
    pub fn bursten(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - FACCEN" ]
    pub fn faccen(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:5 - MWID" ]
    pub fn mwid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 2:3 - MTYP" ]
    pub fn mtyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 1 - MUXEN" ]
    pub fn muxen(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - MBKEN" ]
    pub fn mbken(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bcr3W {
    bits: u32,
}

impl Bcr3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bcr3W { bits: 12496u32 }
    }
    # [ doc = "Bit 19 - CBURSTRW" ]
    pub fn cburstrw(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - ASYNCWAIT" ]
    pub fn asyncwait(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - EXTMOD" ]
    pub fn extmod(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - WAITEN" ]
    pub fn waiten(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - WREN" ]
    pub fn wren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - WAITCFG" ]
    pub fn waitcfg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - WRAPMOD" ]
    pub fn wrapmod(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - WAITPOL" ]
    pub fn waitpol(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - BURSTEN" ]
    pub fn bursten(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - FACCEN" ]
    pub fn faccen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:5 - MWID" ]
    pub fn mwid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 2:3 - MTYP" ]
    pub fn mtyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 1 - MUXEN" ]
    pub fn muxen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - MBKEN" ]
    pub fn mbken(&mut self, value: bool) -> &mut Self {
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
pub struct Btr3 {
    register: ::volatile_register::RW<u32>,
}

impl Btr3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Btr3R, &'w mut Btr3W) -> &'w mut Btr3W
    {
        let bits = self.register.read();
        let r = Btr3R { bits: bits };
        let mut w = Btr3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Btr3R {
        Btr3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Btr3W) -> &mut Btr3W
    {
        let mut w = Btr3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Btr3R {
    bits: u32,
}

impl Btr3R {
    # [ doc = "Bits 28:29 - ACCMOD" ]
    pub fn accmod(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - DATLAT" ]
    pub fn datlat(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - CLKDIV" ]
    pub fn clkdiv(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - BUSTURN" ]
    pub fn busturn(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - DATAST" ]
    pub fn datast(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - ADDHLD" ]
    pub fn addhld(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - ADDSET" ]
    pub fn addset(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Btr3W {
    bits: u32,
}

impl Btr3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Btr3W { bits: 4294967295u32 }
    }
    # [ doc = "Bits 28:29 - ACCMOD" ]
    pub fn accmod(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - DATLAT" ]
    pub fn datlat(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - CLKDIV" ]
    pub fn clkdiv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - BUSTURN" ]
    pub fn busturn(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - DATAST" ]
    pub fn datast(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - ADDHLD" ]
    pub fn addhld(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - ADDSET" ]
    pub fn addset(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bcr4 {
    register: ::volatile_register::RW<u32>,
}

impl Bcr4 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Bcr4R, &'w mut Bcr4W) -> &'w mut Bcr4W
    {
        let bits = self.register.read();
        let r = Bcr4R { bits: bits };
        let mut w = Bcr4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bcr4R {
        Bcr4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bcr4W) -> &mut Bcr4W
    {
        let mut w = Bcr4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bcr4R {
    bits: u32,
}

impl Bcr4R {
    # [ doc = "Bit 19 - CBURSTRW" ]
    pub fn cburstrw(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - ASYNCWAIT" ]
    pub fn asyncwait(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - EXTMOD" ]
    pub fn extmod(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - WAITEN" ]
    pub fn waiten(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - WREN" ]
    pub fn wren(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - WAITCFG" ]
    pub fn waitcfg(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - WRAPMOD" ]
    pub fn wrapmod(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - WAITPOL" ]
    pub fn waitpol(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - BURSTEN" ]
    pub fn bursten(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - FACCEN" ]
    pub fn faccen(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:5 - MWID" ]
    pub fn mwid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 2:3 - MTYP" ]
    pub fn mtyp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 1 - MUXEN" ]
    pub fn muxen(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - MBKEN" ]
    pub fn mbken(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bcr4W {
    bits: u32,
}

impl Bcr4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bcr4W { bits: 12496u32 }
    }
    # [ doc = "Bit 19 - CBURSTRW" ]
    pub fn cburstrw(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - ASYNCWAIT" ]
    pub fn asyncwait(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - EXTMOD" ]
    pub fn extmod(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - WAITEN" ]
    pub fn waiten(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - WREN" ]
    pub fn wren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - WAITCFG" ]
    pub fn waitcfg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - WRAPMOD" ]
    pub fn wrapmod(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - WAITPOL" ]
    pub fn waitpol(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - BURSTEN" ]
    pub fn bursten(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - FACCEN" ]
    pub fn faccen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:5 - MWID" ]
    pub fn mwid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 2:3 - MTYP" ]
    pub fn mtyp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 1 - MUXEN" ]
    pub fn muxen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - MBKEN" ]
    pub fn mbken(&mut self, value: bool) -> &mut Self {
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
pub struct Btr4 {
    register: ::volatile_register::RW<u32>,
}

impl Btr4 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Btr4R, &'w mut Btr4W) -> &'w mut Btr4W
    {
        let bits = self.register.read();
        let r = Btr4R { bits: bits };
        let mut w = Btr4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Btr4R {
        Btr4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Btr4W) -> &mut Btr4W
    {
        let mut w = Btr4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Btr4R {
    bits: u32,
}

impl Btr4R {
    # [ doc = "Bits 28:29 - ACCMOD" ]
    pub fn accmod(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - DATLAT" ]
    pub fn datlat(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - CLKDIV" ]
    pub fn clkdiv(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - BUSTURN" ]
    pub fn busturn(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - DATAST" ]
    pub fn datast(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - ADDHLD" ]
    pub fn addhld(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - ADDSET" ]
    pub fn addset(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Btr4W {
    bits: u32,
}

impl Btr4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Btr4W { bits: 4294967295u32 }
    }
    # [ doc = "Bits 28:29 - ACCMOD" ]
    pub fn accmod(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - DATLAT" ]
    pub fn datlat(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - CLKDIV" ]
    pub fn clkdiv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - BUSTURN" ]
    pub fn busturn(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - DATAST" ]
    pub fn datast(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - ADDHLD" ]
    pub fn addhld(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - ADDSET" ]
    pub fn addset(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pcr {
    register: ::volatile_register::RW<u32>,
}

impl Pcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PcrR, &'w mut PcrW) -> &'w mut PcrW
    {
        let bits = self.register.read();
        let r = PcrR { bits: bits };
        let mut w = PcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PcrR {
        PcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PcrW) -> &mut PcrW
    {
        let mut w = PcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PcrR {
    bits: u32,
}

impl PcrR {
    # [ doc = "Bits 17:19 - ECCPS" ]
    pub fn eccps(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 17u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 13:16 - TAR" ]
    pub fn tar(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 13u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 9:12 - TCLR" ]
    pub fn tclr(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 9u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 6 - ECCEN" ]
    pub fn eccen(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 4:5 - PWID" ]
    pub fn pwid(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 3 - PTYP" ]
    pub fn ptyp(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - PBKEN" ]
    pub fn pbken(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - PWAITEN" ]
    pub fn pwaiten(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PcrW {
    bits: u32,
}

impl PcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PcrW { bits: 24u32 }
    }
    # [ doc = "Bits 17:19 - ECCPS" ]
    pub fn eccps(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 17u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 13:16 - TAR" ]
    pub fn tar(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 13u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 9:12 - TCLR" ]
    pub fn tclr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 9u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 6 - ECCEN" ]
    pub fn eccen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 4:5 - PWID" ]
    pub fn pwid(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 3 - PTYP" ]
    pub fn ptyp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - PBKEN" ]
    pub fn pbken(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - PWAITEN" ]
    pub fn pwaiten(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
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
    # [ doc = "Bit 6 - FEMPT" ]
    pub fn fempt(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - IFEN" ]
    pub fn ifen(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - ILEN" ]
    pub fn ilen(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - IREN" ]
    pub fn iren(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - IFS" ]
    pub fn ifs(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - ILS" ]
    pub fn ils(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - IRS" ]
    pub fn irs(&self) -> bool {
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
        SrW { bits: 64u32 }
    }
    # [ doc = "Bit 5 - IFEN" ]
    pub fn ifen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - ILEN" ]
    pub fn ilen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - IREN" ]
    pub fn iren(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - IFS" ]
    pub fn ifs(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - ILS" ]
    pub fn ils(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - IRS" ]
    pub fn irs(&mut self, value: bool) -> &mut Self {
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
pub struct Pmem {
    register: ::volatile_register::RW<u32>,
}

impl Pmem {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PmemR, &'w mut PmemW) -> &'w mut PmemW
    {
        let bits = self.register.read();
        let r = PmemR { bits: bits };
        let mut w = PmemW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PmemR {
        PmemR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PmemW) -> &mut PmemW
    {
        let mut w = PmemW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PmemR {
    bits: u32,
}

impl PmemR {
    # [ doc = "Bits 24:31 - MEMHIZx" ]
    pub fn memhizx(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - MEMHOLDx" ]
    pub fn memholdx(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - MEMWAITx" ]
    pub fn memwaitx(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - MEMSETx" ]
    pub fn memsetx(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PmemW {
    bits: u32,
}

impl PmemW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PmemW { bits: 4244438268u32 }
    }
    # [ doc = "Bits 24:31 - MEMHIZx" ]
    pub fn memhizx(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - MEMHOLDx" ]
    pub fn memholdx(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - MEMWAITx" ]
    pub fn memwaitx(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - MEMSETx" ]
    pub fn memsetx(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Patt {
    register: ::volatile_register::RW<u32>,
}

impl Patt {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PattR, &'w mut PattW) -> &'w mut PattW
    {
        let bits = self.register.read();
        let r = PattR { bits: bits };
        let mut w = PattW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PattR {
        PattR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PattW) -> &mut PattW
    {
        let mut w = PattW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PattR {
    bits: u32,
}

impl PattR {
    # [ doc = "Bits 24:31 - ATTHIZx" ]
    pub fn atthizx(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - ATTHOLDx" ]
    pub fn attholdx(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - ATTWAITx" ]
    pub fn attwaitx(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - ATTSETx" ]
    pub fn attsetx(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PattW {
    bits: u32,
}

impl PattW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PattW { bits: 4244438268u32 }
    }
    # [ doc = "Bits 24:31 - ATTHIZx" ]
    pub fn atthizx(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - ATTHOLDx" ]
    pub fn attholdx(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - ATTWAITx" ]
    pub fn attwaitx(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - ATTSETx" ]
    pub fn attsetx(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Eccr {
    register: ::volatile_register::RO<u32>,
}

impl Eccr {
    pub fn read(&self) -> EccrR {
        EccrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct EccrR {
    bits: u32,
}

impl EccrR {
    # [ doc = "Bits 0:31 - ECCx" ]
    pub fn eccx(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct EccrW {
    bits: u32,
}

impl EccrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        EccrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - ECCx" ]
    pub fn eccx(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bwtr1 {
    register: ::volatile_register::RW<u32>,
}

impl Bwtr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Bwtr1R, &'w mut Bwtr1W) -> &'w mut Bwtr1W
    {
        let bits = self.register.read();
        let r = Bwtr1R { bits: bits };
        let mut w = Bwtr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bwtr1R {
        Bwtr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bwtr1W) -> &mut Bwtr1W
    {
        let mut w = Bwtr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bwtr1R {
    bits: u32,
}

impl Bwtr1R {
    # [ doc = "Bits 28:29 - ACCMOD" ]
    pub fn accmod(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - DATLAT" ]
    pub fn datlat(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - CLKDIV" ]
    pub fn clkdiv(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - DATAST" ]
    pub fn datast(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - ADDHLD" ]
    pub fn addhld(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - ADDSET" ]
    pub fn addset(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bwtr1W {
    bits: u32,
}

impl Bwtr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bwtr1W { bits: 268435455u32 }
    }
    # [ doc = "Bits 28:29 - ACCMOD" ]
    pub fn accmod(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - DATLAT" ]
    pub fn datlat(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - CLKDIV" ]
    pub fn clkdiv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - DATAST" ]
    pub fn datast(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - ADDHLD" ]
    pub fn addhld(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - ADDSET" ]
    pub fn addset(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bwtr2 {
    register: ::volatile_register::RW<u32>,
}

impl Bwtr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Bwtr2R, &'w mut Bwtr2W) -> &'w mut Bwtr2W
    {
        let bits = self.register.read();
        let r = Bwtr2R { bits: bits };
        let mut w = Bwtr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bwtr2R {
        Bwtr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bwtr2W) -> &mut Bwtr2W
    {
        let mut w = Bwtr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bwtr2R {
    bits: u32,
}

impl Bwtr2R {
    # [ doc = "Bits 28:29 - ACCMOD" ]
    pub fn accmod(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - DATLAT" ]
    pub fn datlat(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - CLKDIV" ]
    pub fn clkdiv(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - DATAST" ]
    pub fn datast(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - ADDHLD" ]
    pub fn addhld(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - ADDSET" ]
    pub fn addset(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bwtr2W {
    bits: u32,
}

impl Bwtr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bwtr2W { bits: 268435455u32 }
    }
    # [ doc = "Bits 28:29 - ACCMOD" ]
    pub fn accmod(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - DATLAT" ]
    pub fn datlat(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - CLKDIV" ]
    pub fn clkdiv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - DATAST" ]
    pub fn datast(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - ADDHLD" ]
    pub fn addhld(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - ADDSET" ]
    pub fn addset(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bwtr3 {
    register: ::volatile_register::RW<u32>,
}

impl Bwtr3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Bwtr3R, &'w mut Bwtr3W) -> &'w mut Bwtr3W
    {
        let bits = self.register.read();
        let r = Bwtr3R { bits: bits };
        let mut w = Bwtr3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bwtr3R {
        Bwtr3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bwtr3W) -> &mut Bwtr3W
    {
        let mut w = Bwtr3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bwtr3R {
    bits: u32,
}

impl Bwtr3R {
    # [ doc = "Bits 28:29 - ACCMOD" ]
    pub fn accmod(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - DATLAT" ]
    pub fn datlat(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - CLKDIV" ]
    pub fn clkdiv(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - DATAST" ]
    pub fn datast(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - ADDHLD" ]
    pub fn addhld(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - ADDSET" ]
    pub fn addset(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bwtr3W {
    bits: u32,
}

impl Bwtr3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bwtr3W { bits: 268435455u32 }
    }
    # [ doc = "Bits 28:29 - ACCMOD" ]
    pub fn accmod(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - DATLAT" ]
    pub fn datlat(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - CLKDIV" ]
    pub fn clkdiv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - DATAST" ]
    pub fn datast(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - ADDHLD" ]
    pub fn addhld(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - ADDSET" ]
    pub fn addset(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bwtr4 {
    register: ::volatile_register::RW<u32>,
}

impl Bwtr4 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Bwtr4R, &'w mut Bwtr4W) -> &'w mut Bwtr4W
    {
        let bits = self.register.read();
        let r = Bwtr4R { bits: bits };
        let mut w = Bwtr4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Bwtr4R {
        Bwtr4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Bwtr4W) -> &mut Bwtr4W
    {
        let mut w = Bwtr4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bwtr4R {
    bits: u32,
}

impl Bwtr4R {
    # [ doc = "Bits 28:29 - ACCMOD" ]
    pub fn accmod(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 28u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 24:27 - DATLAT" ]
    pub fn datlat(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - CLKDIV" ]
    pub fn clkdiv(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:15 - DATAST" ]
    pub fn datast(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - ADDHLD" ]
    pub fn addhld(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - ADDSET" ]
    pub fn addset(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Bwtr4W {
    bits: u32,
}

impl Bwtr4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Bwtr4W { bits: 268435455u32 }
    }
    # [ doc = "Bits 28:29 - ACCMOD" ]
    pub fn accmod(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 28u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 24:27 - DATLAT" ]
    pub fn datlat(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - CLKDIV" ]
    pub fn clkdiv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:15 - DATAST" ]
    pub fn datast(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - ADDHLD" ]
    pub fn addhld(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - ADDSET" ]
    pub fn addset(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
