# [ doc = "Serial audio interface" ]
# [ repr ( C ) ]
pub struct Sai {
    _reserved0: [u8; 36usize],
    # [ doc = "0x24 - BConfiguration register 1" ]
    pub bcr1: Bcr1,
    # [ doc = "0x28 - BConfiguration register 2" ]
    pub bcr2: Bcr2,
    # [ doc = "0x2c - BFRCR" ]
    pub bfrcr: Bfrcr,
    # [ doc = "0x30 - BSlot register" ]
    pub bslotr: Bslotr,
    # [ doc = "0x34 - BInterrupt mask register2" ]
    pub bim: Bim,
    # [ doc = "0x38 - BStatus register" ]
    pub bsr: Bsr,
    # [ doc = "0x3c - BClear flag register" ]
    pub bclrfr: Bclrfr,
    # [ doc = "0x40 - BData register" ]
    pub bdr: Bdr,
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
    # [ doc = "Bits 20:23 - Master clock divider" ]
    pub fn mcjdiv(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 19 - No divider" ]
    pub fn nodiv(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - DMA enable" ]
    pub fn dmaen(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Audio block B enable" ]
    pub fn saiben(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Output drive" ]
    pub fn out_dri(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Mono mode" ]
    pub fn mono(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 10:11 - Synchronization enable" ]
    pub fn syncen(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 9 - Clock strobing edge" ]
    pub fn ckstr(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Least significant bit first" ]
    pub fn lsbfirst(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 5:7 - Data size" ]
    pub fn ds(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 5u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 2:3 - Protocol configuration" ]
    pub fn prtcfg(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:1 - Audio block mode" ]
    pub fn mode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
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
        Bcr1W { bits: 64u32 }
    }
    # [ doc = "Bits 20:23 - Master clock divider" ]
    pub fn mcjdiv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 19 - No divider" ]
    pub fn nodiv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - DMA enable" ]
    pub fn dmaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Audio block B enable" ]
    pub fn saiben(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Output drive" ]
    pub fn out_dri(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Mono mode" ]
    pub fn mono(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 10:11 - Synchronization enable" ]
    pub fn syncen(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 9 - Clock strobing edge" ]
    pub fn ckstr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Least significant bit first" ]
    pub fn lsbfirst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 5:7 - Data size" ]
    pub fn ds(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 5u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 2:3 - Protocol configuration" ]
    pub fn prtcfg(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:1 - Audio block mode" ]
    pub fn mode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
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
    # [ doc = "Bits 14:15 - Companding mode" ]
    pub fn comp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 13 - Complement bit" ]
    pub fn cpl(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 7:12 - Mute counter" ]
    pub fn mutecn(&self) -> u8 {
        const MASK: u32 = 63;
        const OFFSET: u8 = 7u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 6 - Mute value" ]
    pub fn muteval(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Mute" ]
    pub fn mute(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Tristate management on data line" ]
    pub fn tris(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - FIFO flush" ]
    pub fn fflus(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:2 - FIFO threshold" ]
    pub fn fth(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
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
        Bcr2W { bits: 0u32 }
    }
    # [ doc = "Bits 14:15 - Companding mode" ]
    pub fn comp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 13 - Complement bit" ]
    pub fn cpl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 7:12 - Mute counter" ]
    pub fn mutecn(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 7u8;
        const MASK: u8 = 63;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 6 - Mute value" ]
    pub fn muteval(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Mute" ]
    pub fn mute(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Tristate management on data line" ]
    pub fn tris(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - FIFO flush" ]
    pub fn fflus(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:2 - FIFO threshold" ]
    pub fn fth(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bfrcr {
    register: ::volatile_register::RW<u32>,
}

impl Bfrcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BfrcrR, &'w mut BfrcrW) -> &'w mut BfrcrW
    {
        let bits = self.register.read();
        let r = BfrcrR { bits: bits };
        let mut w = BfrcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BfrcrR {
        BfrcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BfrcrW) -> &mut BfrcrW
    {
        let mut w = BfrcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BfrcrR {
    bits: u32,
}

impl BfrcrR {
    # [ doc = "Bit 18 - Frame synchronization offset" ]
    pub fn fsoff(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Frame synchronization polarity" ]
    pub fn fspol(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Frame synchronization definition" ]
    pub fn fsdef(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:14 - Frame synchronization active level length" ]
    pub fn fsall(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - Frame length" ]
    pub fn frl(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BfrcrW {
    bits: u32,
}

impl BfrcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BfrcrW { bits: 7u32 }
    }
    # [ doc = "Bit 18 - Frame synchronization offset" ]
    pub fn fsoff(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Frame synchronization polarity" ]
    pub fn fspol(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Frame synchronization definition" ]
    pub fn fsdef(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 8:14 - Frame synchronization active level length" ]
    pub fn fsall(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Frame length" ]
    pub fn frl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bslotr {
    register: ::volatile_register::RW<u32>,
}

impl Bslotr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BslotrR, &'w mut BslotrW) -> &'w mut BslotrW
    {
        let bits = self.register.read();
        let r = BslotrR { bits: bits };
        let mut w = BslotrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BslotrR {
        BslotrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BslotrW) -> &mut BslotrW
    {
        let mut w = BslotrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BslotrR {
    bits: u32,
}

impl BslotrR {
    # [ doc = "Bits 16:31 - Slot enable" ]
    pub fn sloten(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 8:11 - Number of slots in an audio frame" ]
    pub fn nbslot(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 6:7 - Slot size" ]
    pub fn slotsz(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:4 - First bit offset" ]
    pub fn fboff(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BslotrW {
    bits: u32,
}

impl BslotrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BslotrW { bits: 0u32 }
    }
    # [ doc = "Bits 16:31 - Slot enable" ]
    pub fn sloten(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - Number of slots in an audio frame" ]
    pub fn nbslot(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 6:7 - Slot size" ]
    pub fn slotsz(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:4 - First bit offset" ]
    pub fn fboff(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Bim {
    register: ::volatile_register::RW<u32>,
}

impl Bim {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BimR, &'w mut BimW) -> &'w mut BimW
    {
        let bits = self.register.read();
        let r = BimR { bits: bits };
        let mut w = BimW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BimR {
        BimR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BimW) -> &mut BimW
    {
        let mut w = BimW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BimR {
    bits: u32,
}

impl BimR {
    # [ doc = "Bit 6 - Late frame synchronization detection interrupt enable" ]
    pub fn lfsdetie(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Anticipated frame synchronization detection interrupt enable" ]
    pub fn afsdetie(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Codec not ready interrupt enable" ]
    pub fn cnrdyie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - FIFO request interrupt enable" ]
    pub fn freqie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wrong clock configuration interrupt enable" ]
    pub fn wckcfg(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Mute detection interrupt enable" ]
    pub fn mutedet(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Overrun/underrun interrupt enable" ]
    pub fn ovrudrie(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BimW {
    bits: u32,
}

impl BimW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BimW { bits: 0u32 }
    }
    # [ doc = "Bit 6 - Late frame synchronization detection interrupt enable" ]
    pub fn lfsdetie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Anticipated frame synchronization detection interrupt enable" ]
    pub fn afsdetie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Codec not ready interrupt enable" ]
    pub fn cnrdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - FIFO request interrupt enable" ]
    pub fn freqie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wrong clock configuration interrupt enable" ]
    pub fn wckcfg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Mute detection interrupt enable" ]
    pub fn mutedet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Overrun/underrun interrupt enable" ]
    pub fn ovrudrie(&mut self, value: bool) -> &mut Self {
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
pub struct Bsr {
    register: ::volatile_register::RO<u32>,
}

impl Bsr {
    pub fn read(&self) -> BsrR {
        BsrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BsrR {
    bits: u32,
}

impl BsrR {
    # [ doc = "Bits 16:18 - FIFO level threshold" ]
    pub fn flvl(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 6 - Late frame synchronization detection" ]
    pub fn lfsdet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Anticipated frame synchronization detection" ]
    pub fn afsdet(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Codec not ready" ]
    pub fn cnrdy(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - FIFO request" ]
    pub fn freq(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wrong clock configuration flag" ]
    pub fn wckcfg(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Mute detection" ]
    pub fn mutedet(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Overrun / underrun" ]
    pub fn ovrudr(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BsrW {
    bits: u32,
}

impl BsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BsrW { bits: 0u32 }
    }
    # [ doc = "Bits 16:18 - FIFO level threshold" ]
    pub fn flvl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 6 - Late frame synchronization detection" ]
    pub fn lfsdet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Anticipated frame synchronization detection" ]
    pub fn afsdet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Codec not ready" ]
    pub fn cnrdy(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - FIFO request" ]
    pub fn freq(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wrong clock configuration flag" ]
    pub fn wckcfg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Mute detection" ]
    pub fn mutedet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Overrun / underrun" ]
    pub fn ovrudr(&mut self, value: bool) -> &mut Self {
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
pub struct Bclrfr {
    register: ::volatile_register::WO<u32>,
}

impl Bclrfr {
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut BclrfrW) -> &mut BclrfrW
    {
        let mut w = BclrfrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BclrfrR {
    bits: u32,
}

impl BclrfrR {
    # [ doc = "Bit 6 - Clear late frame synchronization detection flag" ]
    pub fn lfsdet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Clear anticipated frame synchronization detection flag" ]
    pub fn cafsdet(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Clear codec not ready flag" ]
    pub fn cnrdy(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Clear wrong clock configuration flag" ]
    pub fn wckcfg(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Mute detection flag" ]
    pub fn mutedet(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Clear overrun / underrun" ]
    pub fn ovrudr(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BclrfrW {
    bits: u32,
}

impl BclrfrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BclrfrW { bits: 0u32 }
    }
    # [ doc = "Bit 6 - Clear late frame synchronization detection flag" ]
    pub fn lfsdet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Clear anticipated frame synchronization detection flag" ]
    pub fn cafsdet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Clear codec not ready flag" ]
    pub fn cnrdy(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Clear wrong clock configuration flag" ]
    pub fn wckcfg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Mute detection flag" ]
    pub fn mutedet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Clear overrun / underrun" ]
    pub fn ovrudr(&mut self, value: bool) -> &mut Self {
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
pub struct Bdr {
    register: ::volatile_register::RW<u32>,
}

impl Bdr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BdrR, &'w mut BdrW) -> &'w mut BdrW
    {
        let bits = self.register.read();
        let r = BdrR { bits: bits };
        let mut w = BdrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BdrR {
        BdrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BdrW) -> &mut BdrW
    {
        let mut w = BdrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BdrR {
    bits: u32,
}

impl BdrR {
    # [ doc = "Bits 0:31 - Data" ]
    pub fn data(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BdrW {
    bits: u32,
}

impl BdrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BdrW { bits: 0u32 }
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
pub struct Acr1 {
    register: ::volatile_register::RW<u32>,
}

impl Acr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Acr1R, &'w mut Acr1W) -> &'w mut Acr1W
    {
        let bits = self.register.read();
        let r = Acr1R { bits: bits };
        let mut w = Acr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Acr1R {
        Acr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Acr1W) -> &mut Acr1W
    {
        let mut w = Acr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Acr1R {
    bits: u32,
}

impl Acr1R {
    # [ doc = "Bits 20:23 - Master clock divider" ]
    pub fn mcjdiv(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 19 - No divider" ]
    pub fn nodiv(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - DMA enable" ]
    pub fn dmaen(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Audio block A enable" ]
    pub fn saiaen(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Output drive" ]
    pub fn out_dri(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Mono mode" ]
    pub fn mono(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 10:11 - Synchronization enable" ]
    pub fn syncen(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 9 - Clock strobing edge" ]
    pub fn ckstr(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Least significant bit first" ]
    pub fn lsbfirst(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 5:7 - Data size" ]
    pub fn ds(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 5u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 2:3 - Protocol configuration" ]
    pub fn prtcfg(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 2u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:1 - Audio block mode" ]
    pub fn mode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Acr1W {
    bits: u32,
}

impl Acr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Acr1W { bits: 64u32 }
    }
    # [ doc = "Bits 20:23 - Master clock divider" ]
    pub fn mcjdiv(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 19 - No divider" ]
    pub fn nodiv(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - DMA enable" ]
    pub fn dmaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Audio block A enable" ]
    pub fn saiaen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Output drive" ]
    pub fn out_dri(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Mono mode" ]
    pub fn mono(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 10:11 - Synchronization enable" ]
    pub fn syncen(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 9 - Clock strobing edge" ]
    pub fn ckstr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Least significant bit first" ]
    pub fn lsbfirst(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 5:7 - Data size" ]
    pub fn ds(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 5u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 2:3 - Protocol configuration" ]
    pub fn prtcfg(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 2u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:1 - Audio block mode" ]
    pub fn mode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Acr2 {
    register: ::volatile_register::RW<u32>,
}

impl Acr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Acr2R, &'w mut Acr2W) -> &'w mut Acr2W
    {
        let bits = self.register.read();
        let r = Acr2R { bits: bits };
        let mut w = Acr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Acr2R {
        Acr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Acr2W) -> &mut Acr2W
    {
        let mut w = Acr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Acr2R {
    bits: u32,
}

impl Acr2R {
    # [ doc = "Bits 14:15 - Companding mode" ]
    pub fn comp(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 14u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 13 - Complement bit" ]
    pub fn cpl(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 7:12 - Mute counter" ]
    pub fn mutecn(&self) -> u8 {
        const MASK: u32 = 63;
        const OFFSET: u8 = 7u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 6 - Mute value" ]
    pub fn muteval(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Mute" ]
    pub fn mute(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Tristate management on data line" ]
    pub fn tris(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - FIFO flush" ]
    pub fn fflus(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 0:2 - FIFO threshold" ]
    pub fn fth(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Acr2W {
    bits: u32,
}

impl Acr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Acr2W { bits: 0u32 }
    }
    # [ doc = "Bits 14:15 - Companding mode" ]
    pub fn comp(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 14u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 13 - Complement bit" ]
    pub fn cpl(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 7:12 - Mute counter" ]
    pub fn mutecn(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 7u8;
        const MASK: u8 = 63;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 6 - Mute value" ]
    pub fn muteval(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Mute" ]
    pub fn mute(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Tristate management on data line" ]
    pub fn tris(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - FIFO flush" ]
    pub fn fflus(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 0:2 - FIFO threshold" ]
    pub fn fth(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Afrcr {
    register: ::volatile_register::RW<u32>,
}

impl Afrcr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&AfrcrR, &'w mut AfrcrW) -> &'w mut AfrcrW
    {
        let bits = self.register.read();
        let r = AfrcrR { bits: bits };
        let mut w = AfrcrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> AfrcrR {
        AfrcrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut AfrcrW) -> &mut AfrcrW
    {
        let mut w = AfrcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AfrcrR {
    bits: u32,
}

impl AfrcrR {
    # [ doc = "Bit 18 - Frame synchronization offset" ]
    pub fn fsoff(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Frame synchronization polarity" ]
    pub fn fspol(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Frame synchronization definition" ]
    pub fn fsdef(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 8:14 - Frame synchronization active level length" ]
    pub fn fsall(&self) -> u8 {
        const MASK: u32 = 127;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:7 - Frame length" ]
    pub fn frl(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AfrcrW {
    bits: u32,
}

impl AfrcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        AfrcrW { bits: 7u32 }
    }
    # [ doc = "Bit 18 - Frame synchronization offset" ]
    pub fn fsoff(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Frame synchronization polarity" ]
    pub fn fspol(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Frame synchronization definition" ]
    pub fn fsdef(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 8:14 - Frame synchronization active level length" ]
    pub fn fsall(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 127;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:7 - Frame length" ]
    pub fn frl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Aslotr {
    register: ::volatile_register::RW<u32>,
}

impl Aslotr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&AslotrR, &'w mut AslotrW) -> &'w mut AslotrW
    {
        let bits = self.register.read();
        let r = AslotrR { bits: bits };
        let mut w = AslotrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> AslotrR {
        AslotrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut AslotrW) -> &mut AslotrW
    {
        let mut w = AslotrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AslotrR {
    bits: u32,
}

impl AslotrR {
    # [ doc = "Bits 16:31 - Slot enable" ]
    pub fn sloten(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
    # [ doc = "Bits 8:11 - Number of slots in an audio frame" ]
    pub fn nbslot(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 6:7 - Slot size" ]
    pub fn slotsz(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:4 - First bit offset" ]
    pub fn fboff(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AslotrW {
    bits: u32,
}

impl AslotrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        AslotrW { bits: 0u32 }
    }
    # [ doc = "Bits 16:31 - Slot enable" ]
    pub fn sloten(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - Number of slots in an audio frame" ]
    pub fn nbslot(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 6:7 - Slot size" ]
    pub fn slotsz(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:4 - First bit offset" ]
    pub fn fboff(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Aim {
    register: ::volatile_register::RW<u32>,
}

impl Aim {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&AimR, &'w mut AimW) -> &'w mut AimW
    {
        let bits = self.register.read();
        let r = AimR { bits: bits };
        let mut w = AimW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> AimR {
        AimR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut AimW) -> &mut AimW
    {
        let mut w = AimW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AimR {
    bits: u32,
}

impl AimR {
    # [ doc = "Bit 6 - Late frame synchronization detection interrupt enable" ]
    pub fn lfsdet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Anticipated frame synchronization detection interrupt enable" ]
    pub fn afsdetie(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Codec not ready interrupt enable" ]
    pub fn cnrdyie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - FIFO request interrupt enable" ]
    pub fn freqie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wrong clock configuration interrupt enable" ]
    pub fn wckcfg(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Mute detection interrupt enable" ]
    pub fn mutedet(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Overrun/underrun interrupt enable" ]
    pub fn ovrudrie(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AimW {
    bits: u32,
}

impl AimW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        AimW { bits: 0u32 }
    }
    # [ doc = "Bit 6 - Late frame synchronization detection interrupt enable" ]
    pub fn lfsdet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Anticipated frame synchronization detection interrupt enable" ]
    pub fn afsdetie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Codec not ready interrupt enable" ]
    pub fn cnrdyie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - FIFO request interrupt enable" ]
    pub fn freqie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wrong clock configuration interrupt enable" ]
    pub fn wckcfg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Mute detection interrupt enable" ]
    pub fn mutedet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Overrun/underrun interrupt enable" ]
    pub fn ovrudrie(&mut self, value: bool) -> &mut Self {
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
pub struct Asr {
    register: ::volatile_register::RW<u32>,
}

impl Asr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&AsrR, &'w mut AsrW) -> &'w mut AsrW
    {
        let bits = self.register.read();
        let r = AsrR { bits: bits };
        let mut w = AsrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> AsrR {
        AsrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut AsrW) -> &mut AsrW
    {
        let mut w = AsrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AsrR {
    bits: u32,
}

impl AsrR {
    # [ doc = "Bits 16:18 - FIFO level threshold" ]
    pub fn flvl(&self) -> u8 {
        const MASK: u32 = 7;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 6 - Late frame synchronization detection" ]
    pub fn lfsdet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Anticipated frame synchronization detection" ]
    pub fn afsdet(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Codec not ready" ]
    pub fn cnrdy(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - FIFO request" ]
    pub fn freq(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Wrong clock configuration flag. This bit is read only" ]
    pub fn wckcfg(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Mute detection" ]
    pub fn mutedet(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Overrun / underrun" ]
    pub fn ovrudr(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AsrW {
    bits: u32,
}

impl AsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        AsrW { bits: 0u32 }
    }
    # [ doc = "Bits 16:18 - FIFO level threshold" ]
    pub fn flvl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 7;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 6 - Late frame synchronization detection" ]
    pub fn lfsdet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Anticipated frame synchronization detection" ]
    pub fn afsdet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Codec not ready" ]
    pub fn cnrdy(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - FIFO request" ]
    pub fn freq(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Wrong clock configuration flag. This bit is read only" ]
    pub fn wckcfg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Mute detection" ]
    pub fn mutedet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Overrun / underrun" ]
    pub fn ovrudr(&mut self, value: bool) -> &mut Self {
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
pub struct Aclrfr {
    register: ::volatile_register::RW<u32>,
}

impl Aclrfr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&AclrfrR, &'w mut AclrfrW) -> &'w mut AclrfrW
    {
        let bits = self.register.read();
        let r = AclrfrR { bits: bits };
        let mut w = AclrfrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> AclrfrR {
        AclrfrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut AclrfrW) -> &mut AclrfrW
    {
        let mut w = AclrfrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AclrfrR {
    bits: u32,
}

impl AclrfrR {
    # [ doc = "Bit 6 - Clear late frame synchronization detection flag" ]
    pub fn lfsdet(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Clear anticipated frame synchronization detection flag" ]
    pub fn cafsdet(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Clear codec not ready flag" ]
    pub fn cnrdy(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Clear wrong clock configuration flag" ]
    pub fn wckcfg(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Mute detection flag" ]
    pub fn mutedet(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Clear overrun / underrun" ]
    pub fn ovrudr(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AclrfrW {
    bits: u32,
}

impl AclrfrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        AclrfrW { bits: 0u32 }
    }
    # [ doc = "Bit 6 - Clear late frame synchronization detection flag" ]
    pub fn lfsdet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Clear anticipated frame synchronization detection flag" ]
    pub fn cafsdet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Clear codec not ready flag" ]
    pub fn cnrdy(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Clear wrong clock configuration flag" ]
    pub fn wckcfg(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Mute detection flag" ]
    pub fn mutedet(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Clear overrun / underrun" ]
    pub fn ovrudr(&mut self, value: bool) -> &mut Self {
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
pub struct Adr {
    register: ::volatile_register::RW<u32>,
}

impl Adr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&AdrR, &'w mut AdrW) -> &'w mut AdrW
    {
        let bits = self.register.read();
        let r = AdrR { bits: bits };
        let mut w = AdrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> AdrR {
        AdrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut AdrW) -> &mut AdrW
    {
        let mut w = AdrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AdrR {
    bits: u32,
}

impl AdrR {
    # [ doc = "Bits 0:31 - Data" ]
    pub fn data(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct AdrW {
    bits: u32,
}

impl AdrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        AdrW { bits: 0u32 }
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
