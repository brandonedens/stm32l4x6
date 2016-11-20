# [ doc = "Flash" ]
# [ repr ( C ) ]
pub struct Flash {
    # [ doc = "0x00 - Access control register" ]
    pub acr: Acr,
    # [ doc = "0x04 - Power down key register" ]
    pub pdkeyr: Pdkeyr,
    # [ doc = "0x08 - Flash key register" ]
    pub keyr: Keyr,
    # [ doc = "0x0c - Option byte key register" ]
    pub optkeyr: Optkeyr,
    # [ doc = "0x10 - Status register" ]
    pub sr: Sr,
    # [ doc = "0x14 - Flash control register" ]
    pub cr: Cr,
    # [ doc = "0x18 - Flash ECC register" ]
    pub eccr: Eccr,
    _reserved0: [u8; 4usize],
    # [ doc = "0x20 - Flash option register" ]
    pub optr: Optr,
    # [ doc = "0x24 - Flash Bank 1 PCROP Start address register" ]
    pub pcrop1sr: Pcrop1sr,
    # [ doc = "0x28 - Flash Bank 1 PCROP End address register" ]
    pub pcrop1er: Pcrop1er,
    # [ doc = "0x2c - Flash Bank 1 WRP area A address register" ]
    pub wrp1ar: Wrp1ar,
    # [ doc = "0x30 - Flash Bank 1 WRP area B address register" ]
    pub wrp1br: Wrp1br,
    _reserved1: [u8; 16usize],
    # [ doc = "0x44 - Flash Bank 2 PCROP Start address register" ]
    pub pcrop2sr: Pcrop2sr,
    # [ doc = "0x48 - Flash Bank 2 PCROP End address register" ]
    pub pcrop2er: Pcrop2er,
    # [ doc = "0x4c - Flash Bank 2 WRP area A address register" ]
    pub wrp2ar: Wrp2ar,
    # [ doc = "0x50 - Flash Bank 2 WRP area B address register" ]
    pub wrp2br: Wrp2br,
}

# [ repr ( C ) ]
pub struct Acr {
    register: ::volatile_register::RW<u32>,
}

impl Acr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&AcrR, &'w mut AcrW) -> &'w mut AcrW
    {
        let bits = self.register.read();
        let r = AcrR { bits: bits };
        let mut w = AcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> AcrR {
        AcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut AcrW) -> &mut AcrW
    {
        let mut w = AcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AcrR {
    bits: u32,
}

impl AcrR {
    # [ doc = "Bits 0:2 - Latency" ]
    pub fn latency(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 8 - Prefetch enable" ]
    pub fn prften(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Instruction cache enable" ]
    pub fn icen(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Data cache enable" ]
    pub fn dcen(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Instruction cache reset" ]
    pub fn icrst(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Data cache reset" ]
    pub fn dcrst(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Flash Power-down mode during Low-power run mode" ]
    pub fn run_pd(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Flash Power-down mode during Low-power sleep mode" ]
    pub fn sleep_pd(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AcrW {
    bits: u32,
}

impl AcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        AcrW { bits: 1536u32 }
    }
    # [ doc = "Bits 0:2 - Latency" ]
    pub fn latency(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 8 - Prefetch enable" ]
    pub fn prften(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Instruction cache enable" ]
    pub fn icen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Data cache enable" ]
    pub fn dcen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Instruction cache reset" ]
    pub fn icrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Data cache reset" ]
    pub fn dcrst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Flash Power-down mode during Low-power run mode" ]
    pub fn run_pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Flash Power-down mode during Low-power sleep mode" ]
    pub fn sleep_pd(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Pdkeyr {
    register: ::volatile_register::WO<u32>,
}

impl Pdkeyr {
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut PdkeyrW) -> &mut PdkeyrW
    {
        let mut w = PdkeyrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdkeyrR {
    bits: u32,
}

impl PdkeyrR {
    # [ doc = "Bits 0:31 - RUN_PD in FLASH_ACR key" ]
    pub fn pdkeyr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PdkeyrW {
    bits: u32,
}

impl PdkeyrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PdkeyrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - RUN_PD in FLASH_ACR key" ]
    pub fn pdkeyr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Keyr {
    register: ::volatile_register::WO<u32>,
}

impl Keyr {
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut KeyrW) -> &mut KeyrW
    {
        let mut w = KeyrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct KeyrR {
    bits: u32,
}

impl KeyrR {
    # [ doc = "Bits 0:31 - KEYR" ]
    pub fn keyr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct KeyrW {
    bits: u32,
}

impl KeyrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        KeyrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - KEYR" ]
    pub fn keyr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Optkeyr {
    register: ::volatile_register::WO<u32>,
}

impl Optkeyr {
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut OptkeyrW) -> &mut OptkeyrW
    {
        let mut w = OptkeyrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OptkeyrR {
    bits: u32,
}

impl OptkeyrR {
    # [ doc = "Bits 0:31 - Option byte key" ]
    pub fn optkeyr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OptkeyrW {
    bits: u32,
}

impl OptkeyrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OptkeyrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Option byte key" ]
    pub fn optkeyr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
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
    # [ doc = "Bit 0 - End of operation" ]
    pub fn eop(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Operation error" ]
    pub fn operr(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Programming error" ]
    pub fn progerr(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Write protected error" ]
    pub fn wrperr(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Programming alignment error" ]
    pub fn pgaerr(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Size error" ]
    pub fn sizerr(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Programming sequence error" ]
    pub fn pgserr(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Fast programming data miss error" ]
    pub fn miserr(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Fast programming error" ]
    pub fn fasterr(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - PCROP read error" ]
    pub fn rderr(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Option validity error" ]
    pub fn optverr(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Busy" ]
    pub fn bsy(&self) -> bool {
        const OFFSET: u8 = 16u8;
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
    # [ doc = "Bit 0 - End of operation" ]
    pub fn eop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Operation error" ]
    pub fn operr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Programming error" ]
    pub fn progerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Write protected error" ]
    pub fn wrperr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Programming alignment error" ]
    pub fn pgaerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Size error" ]
    pub fn sizerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Programming sequence error" ]
    pub fn pgserr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Fast programming data miss error" ]
    pub fn miserr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Fast programming error" ]
    pub fn fasterr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - PCROP read error" ]
    pub fn rderr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Option validity error" ]
    pub fn optverr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
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
    # [ doc = "Bit 0 - Programming" ]
    pub fn pg(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Page erase" ]
    pub fn per(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Bank 1 Mass erase" ]
    pub fn mer1(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 3:10 - Page number" ]
    pub fn pnb(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 11 - Bank erase" ]
    pub fn bker(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Bank 2 Mass erase" ]
    pub fn mer2(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Start" ]
    pub fn start(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Options modification start" ]
    pub fn optstrt(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Fast programming" ]
    pub fn fstpg(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - End of operation interrupt enable" ]
    pub fn eopie(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Error interrupt enable" ]
    pub fn errie(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - PCROP read error interrupt enable" ]
    pub fn rderrie(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 27 - Force the option byte loading" ]
    pub fn obl_launch(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - Options Lock" ]
    pub fn optlock(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - FLASH_CR Lock" ]
    pub fn lock(&self) -> bool {
        const OFFSET: u8 = 31u8;
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
        CrW { bits: 3221225472u32 }
    }
    # [ doc = "Bit 0 - Programming" ]
    pub fn pg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Page erase" ]
    pub fn per(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Bank 1 Mass erase" ]
    pub fn mer1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 3:10 - Page number" ]
    pub fn pnb(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 11 - Bank erase" ]
    pub fn bker(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Bank 2 Mass erase" ]
    pub fn mer2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Start" ]
    pub fn start(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Options modification start" ]
    pub fn optstrt(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Fast programming" ]
    pub fn fstpg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - End of operation interrupt enable" ]
    pub fn eopie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Error interrupt enable" ]
    pub fn errie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - PCROP read error interrupt enable" ]
    pub fn rderrie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 27 - Force the option byte loading" ]
    pub fn obl_launch(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - Options Lock" ]
    pub fn optlock(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - FLASH_CR Lock" ]
    pub fn lock(&mut self, value: bool) -> &mut Self {
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
pub struct Eccr {
    register: ::volatile_register::RW<u32>,
}

impl Eccr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&EccrR, &'w mut EccrW) -> &'w mut EccrW
    {
        let bits = self.register.read();
        let r = EccrR { bits: bits };
        let mut w = EccrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> EccrR {
        EccrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut EccrW) -> &mut EccrW
    {
        let mut w = EccrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct EccrR {
    bits: u32,
}

impl EccrR {
    # [ doc = "Bits 0:18 - ECC fail address" ]
    pub fn addr_ecc(&self) -> u32 {
        const MASK: u32 = 524287;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
    # [ doc = "Bit 19 - ECC fail bank" ]
    pub fn bk_ecc(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - System Flash ECC fail" ]
    pub fn sysf_ecc(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - ECC correction interrupt enable" ]
    pub fn eccie(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 30 - ECC correction" ]
    pub fn eccc(&self) -> bool {
        const OFFSET: u8 = 30u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 31 - ECC detection" ]
    pub fn eccd(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
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
    # [ doc = "Bit 24 - ECC correction interrupt enable" ]
    pub fn eccie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 30 - ECC correction" ]
    pub fn eccc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 30u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 31 - ECC detection" ]
    pub fn eccd(&mut self, value: bool) -> &mut Self {
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
pub struct Optr {
    register: ::volatile_register::RW<u32>,
}

impl Optr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&OptrR, &'w mut OptrW) -> &'w mut OptrW
    {
        let bits = self.register.read();
        let r = OptrR { bits: bits };
        let mut w = OptrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> OptrR {
        OptrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut OptrW) -> &mut OptrW
    {
        let mut w = OptrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OptrR {
    bits: u32,
}

impl OptrR {
    # [ doc = "Bits 0:7 - Read protection level" ]
    pub fn rdp(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:10 - BOR reset Level" ]
    pub fn bor_lev(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 12 - nRST_STOP" ]
    pub fn n_rst_stop(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - nRST_STDBY" ]
    pub fn n_rst_stdby(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Independent watchdog selection" ]
    pub fn idwg_sw(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Independent watchdog counter freeze in Stop mode" ]
    pub fn iwdg_stop(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Independent watchdog counter freeze in Standby mode" ]
    pub fn iwdg_stdby(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Window watchdog selection" ]
    pub fn wwdg_sw(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Dual-bank boot" ]
    pub fn bfb2(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Dual-Bank on 512 KB or 256 KB Flash memory devices" ]
    pub fn dualbank(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - Boot configuration" ]
    pub fn n_boot1(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - SRAM2 parity check enable" ]
    pub fn sram2_pe(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - SRAM2 Erase when system reset" ]
    pub fn sram2_rst(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct OptrW {
    bits: u32,
}

impl OptrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        OptrW { bits: 4026531840u32 }
    }
    # [ doc = "Bits 0:7 - Read protection level" ]
    pub fn rdp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:10 - BOR reset Level" ]
    pub fn bor_lev(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 12 - nRST_STOP" ]
    pub fn n_rst_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - nRST_STDBY" ]
    pub fn n_rst_stdby(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Independent watchdog selection" ]
    pub fn idwg_sw(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Independent watchdog counter freeze in Stop mode" ]
    pub fn iwdg_stop(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Independent watchdog counter freeze in Standby mode" ]
    pub fn iwdg_stdby(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Window watchdog selection" ]
    pub fn wwdg_sw(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Dual-bank boot" ]
    pub fn bfb2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Dual-Bank on 512 KB or 256 KB Flash memory devices" ]
    pub fn dualbank(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - Boot configuration" ]
    pub fn n_boot1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - SRAM2 parity check enable" ]
    pub fn sram2_pe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - SRAM2 Erase when system reset" ]
    pub fn sram2_rst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Pcrop1sr {
    register: ::volatile_register::RW<u32>,
}

impl Pcrop1sr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Pcrop1srR, &'w mut Pcrop1srW) -> &'w mut Pcrop1srW
    {
        let bits = self.register.read();
        let r = Pcrop1srR { bits: bits };
        let mut w = Pcrop1srW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Pcrop1srR {
        Pcrop1srR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Pcrop1srW) -> &mut Pcrop1srW
    {
        let mut w = Pcrop1srW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pcrop1srR {
    bits: u32,
}

impl Pcrop1srR {
    # [ doc = "Bits 0:15 - Bank 1 PCROP area start offset" ]
    pub fn pcrop1_strt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pcrop1srW {
    bits: u32,
}

impl Pcrop1srW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Pcrop1srW { bits: 4294901760u32 }
    }
    # [ doc = "Bits 0:15 - Bank 1 PCROP area start offset" ]
    pub fn pcrop1_strt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pcrop1er {
    register: ::volatile_register::RW<u32>,
}

impl Pcrop1er {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Pcrop1erR, &'w mut Pcrop1erW) -> &'w mut Pcrop1erW
    {
        let bits = self.register.read();
        let r = Pcrop1erR { bits: bits };
        let mut w = Pcrop1erW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Pcrop1erR {
        Pcrop1erR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Pcrop1erW) -> &mut Pcrop1erW
    {
        let mut w = Pcrop1erW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pcrop1erR {
    bits: u32,
}

impl Pcrop1erR {
    # [ doc = "Bits 0:15 - Bank 1 PCROP area end offset" ]
    pub fn pcrop1_end(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bit 31 - PCROP area preserved when RDP level decreased" ]
    pub fn pcrop_rdp(&self) -> bool {
        const OFFSET: u8 = 31u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pcrop1erW {
    bits: u32,
}

impl Pcrop1erW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Pcrop1erW { bits: 268369920u32 }
    }
    # [ doc = "Bits 0:15 - Bank 1 PCROP area end offset" ]
    pub fn pcrop1_end(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 31 - PCROP area preserved when RDP level decreased" ]
    pub fn pcrop_rdp(&mut self, value: bool) -> &mut Self {
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
pub struct Wrp1ar {
    register: ::volatile_register::RW<u32>,
}

impl Wrp1ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Wrp1arR, &'w mut Wrp1arW) -> &'w mut Wrp1arW
    {
        let bits = self.register.read();
        let r = Wrp1arR { bits: bits };
        let mut w = Wrp1arW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Wrp1arR {
        Wrp1arR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Wrp1arW) -> &mut Wrp1arW
    {
        let mut w = Wrp1arW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Wrp1arR {
    bits: u32,
}

impl Wrp1arR {
    # [ doc = "Bits 0:7 - Bank 1 WRP first area ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã¢â‚¬Å“AÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ start offset" ]
    pub fn wrp1a_strt(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - Bank 1 WRP first area A end offset" ]
    pub fn wrp1a_end(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Wrp1arW {
    bits: u32,
}

impl Wrp1arW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Wrp1arW { bits: 4278255360u32 }
    }
    # [ doc = "Bits 0:7 - Bank 1 WRP first area ÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ÃƒÆ’Ã†â€™ÃƒÂ¢Ã¢â€šÂ¬Ã‚Â¦ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã¢â‚¬Å“AÃƒÆ’Ã†â€™Ãƒâ€ Ã¢â‚¬â„¢ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã†â€™Ãƒâ€šÃ‚Â¢ÃƒÆ’Ã‚Â¢ÃƒÂ¢Ã¢â‚¬Å¡Ã‚Â¬Ãƒâ€¦Ã‚Â¡ÃƒÆ’Ã¢â‚¬Å¡Ãƒâ€šÃ‚Â¬ start offset" ]
    pub fn wrp1a_strt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - Bank 1 WRP first area A end offset" ]
    pub fn wrp1a_end(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Wrp1br {
    register: ::volatile_register::RW<u32>,
}

impl Wrp1br {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Wrp1brR, &'w mut Wrp1brW) -> &'w mut Wrp1brW
    {
        let bits = self.register.read();
        let r = Wrp1brR { bits: bits };
        let mut w = Wrp1brW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Wrp1brR {
        Wrp1brR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Wrp1brW) -> &mut Wrp1brW
    {
        let mut w = Wrp1brW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Wrp1brR {
    bits: u32,
}

impl Wrp1brR {
    # [ doc = "Bits 16:23 - Bank 1 WRP second area B end offset" ]
    pub fn wrp1b_strt(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - Bank 1 WRP second area B start offset" ]
    pub fn wrp1b_end(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Wrp1brW {
    bits: u32,
}

impl Wrp1brW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Wrp1brW { bits: 4278255360u32 }
    }
    # [ doc = "Bits 16:23 - Bank 1 WRP second area B end offset" ]
    pub fn wrp1b_strt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Bank 1 WRP second area B start offset" ]
    pub fn wrp1b_end(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pcrop2sr {
    register: ::volatile_register::RW<u32>,
}

impl Pcrop2sr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Pcrop2srR, &'w mut Pcrop2srW) -> &'w mut Pcrop2srW
    {
        let bits = self.register.read();
        let r = Pcrop2srR { bits: bits };
        let mut w = Pcrop2srW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Pcrop2srR {
        Pcrop2srR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Pcrop2srW) -> &mut Pcrop2srW
    {
        let mut w = Pcrop2srW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pcrop2srR {
    bits: u32,
}

impl Pcrop2srR {
    # [ doc = "Bits 0:15 - Bank 2 PCROP area start offset" ]
    pub fn pcrop2_strt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pcrop2srW {
    bits: u32,
}

impl Pcrop2srW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Pcrop2srW { bits: 4294901760u32 }
    }
    # [ doc = "Bits 0:15 - Bank 2 PCROP area start offset" ]
    pub fn pcrop2_strt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pcrop2er {
    register: ::volatile_register::RW<u32>,
}

impl Pcrop2er {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Pcrop2erR, &'w mut Pcrop2erW) -> &'w mut Pcrop2erW
    {
        let bits = self.register.read();
        let r = Pcrop2erR { bits: bits };
        let mut w = Pcrop2erW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Pcrop2erR {
        Pcrop2erR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Pcrop2erW) -> &mut Pcrop2erW
    {
        let mut w = Pcrop2erW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pcrop2erR {
    bits: u32,
}

impl Pcrop2erR {
    # [ doc = "Bits 0:15 - Bank 2 PCROP area end offset" ]
    pub fn pcrop2_end(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Pcrop2erW {
    bits: u32,
}

impl Pcrop2erW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Pcrop2erW { bits: 4294901760u32 }
    }
    # [ doc = "Bits 0:15 - Bank 2 PCROP area end offset" ]
    pub fn pcrop2_end(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Wrp2ar {
    register: ::volatile_register::RW<u32>,
}

impl Wrp2ar {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Wrp2arR, &'w mut Wrp2arW) -> &'w mut Wrp2arW
    {
        let bits = self.register.read();
        let r = Wrp2arR { bits: bits };
        let mut w = Wrp2arW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Wrp2arR {
        Wrp2arR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Wrp2arW) -> &mut Wrp2arW
    {
        let mut w = Wrp2arW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Wrp2arR {
    bits: u32,
}

impl Wrp2arR {
    # [ doc = "Bits 0:7 - Bank 2 WRP first area A start offset" ]
    pub fn wrp2a_strt(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - Bank 2 WRP first area A end offset" ]
    pub fn wrp2a_end(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Wrp2arW {
    bits: u32,
}

impl Wrp2arW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Wrp2arW { bits: 4278255360u32 }
    }
    # [ doc = "Bits 0:7 - Bank 2 WRP first area A start offset" ]
    pub fn wrp2a_strt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - Bank 2 WRP first area A end offset" ]
    pub fn wrp2a_end(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Wrp2br {
    register: ::volatile_register::RW<u32>,
}

impl Wrp2br {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Wrp2brR, &'w mut Wrp2brW) -> &'w mut Wrp2brW
    {
        let bits = self.register.read();
        let r = Wrp2brR { bits: bits };
        let mut w = Wrp2brW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Wrp2brR {
        Wrp2brR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Wrp2brW) -> &mut Wrp2brW
    {
        let mut w = Wrp2brW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Wrp2brR {
    bits: u32,
}

impl Wrp2brR {
    # [ doc = "Bits 0:7 - Bank 2 WRP second area B start offset" ]
    pub fn wrp2b_strt(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:23 - Bank 2 WRP second area B end offset" ]
    pub fn wrp2b_end(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Wrp2brW {
    bits: u32,
}

impl Wrp2brW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Wrp2brW { bits: 4278255360u32 }
    }
    # [ doc = "Bits 0:7 - Bank 2 WRP second area B start offset" ]
    pub fn wrp2b_strt(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:23 - Bank 2 WRP second area B end offset" ]
    pub fn wrp2b_end(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
