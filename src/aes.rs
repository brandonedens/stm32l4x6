# [ doc = "Advanced encryption standard hardware accelerator" ]
# [ repr ( C ) ]
pub struct Aes {
    # [ doc = "0x00 - control register" ]
    pub cr: Cr,
    # [ doc = "0x04 - status register" ]
    pub sr: Sr,
    # [ doc = "0x08 - data input register" ]
    pub dinr: Dinr,
    # [ doc = "0x0c - data output register" ]
    pub doutr: Doutr,
    # [ doc = "0x10 - key register 0" ]
    pub keyr0: Keyr0,
    # [ doc = "0x14 - key register 1" ]
    pub keyr1: Keyr1,
    # [ doc = "0x18 - key register 2" ]
    pub keyr2: Keyr2,
    # [ doc = "0x1c - key register 3" ]
    pub keyr3: Keyr3,
    # [ doc = "0x20 - initialization vector register 0" ]
    pub ivr0: Ivr0,
    # [ doc = "0x24 - initialization vector register 1" ]
    pub ivr1: Ivr1,
    # [ doc = "0x28 - initialization vector register 2" ]
    pub ivr2: Ivr2,
    # [ doc = "0x2c - initialization vector register 3" ]
    pub ivr3: Ivr3,
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
    # [ doc = "Bit 12 - Enable DMA management of data output phase" ]
    pub fn dmaouten(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Enable DMA management of data input phase" ]
    pub fn dmainen(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Error interrupt enable" ]
    pub fn errie(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - CCF flag interrupt enable" ]
    pub fn ccfie(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Error clear" ]
    pub fn errc(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Computation Complete Flag Clear" ]
    pub fn ccfc(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 5:6 - AES chaining mode" ]
    pub fn chmod(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 5u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 3:4 - AES operating mode" ]
    pub fn mode(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)" ]
    pub fn datatype(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 1u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 0 - AES enable" ]
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
    # [ doc = "Bit 12 - Enable DMA management of data output phase" ]
    pub fn dmaouten(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Enable DMA management of data input phase" ]
    pub fn dmainen(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Error interrupt enable" ]
    pub fn errie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - CCF flag interrupt enable" ]
    pub fn ccfie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Error clear" ]
    pub fn errc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Computation Complete Flag Clear" ]
    pub fn ccfc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 5:6 - AES chaining mode" ]
    pub fn chmod(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 5u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 3:4 - AES operating mode" ]
    pub fn mode(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 1:2 - Data type selection (for data in and data out to/from the cryptographic block)" ]
    pub fn datatype(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 1u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 0 - AES enable" ]
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
    # [ doc = "Bit 2 - Write error flag" ]
    pub fn wrerr(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Read error flag" ]
    pub fn rderr(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Computation complete flag" ]
    pub fn ccf(&self) -> bool {
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
    # [ doc = "Bit 2 - Write error flag" ]
    pub fn wrerr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Read error flag" ]
    pub fn rderr(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Computation complete flag" ]
    pub fn ccf(&mut self, value: bool) -> &mut Self {
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
pub struct Dinr {
    register: ::volatile_register::RW<u32>,
}

impl Dinr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&DinrR, &'w mut DinrW) -> &'w mut DinrW
    {
        let bits = self.register.read();
        let r = DinrR { bits: bits };
        let mut w = DinrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> DinrR {
        DinrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut DinrW) -> &mut DinrW
    {
        let mut w = DinrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DinrR {
    bits: u32,
}

impl DinrR {
    # [ doc = "Bits 0:31 - Data Input Register" ]
    pub fn aes_dinr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DinrW {
    bits: u32,
}

impl DinrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DinrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Data Input Register" ]
    pub fn aes_dinr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Doutr {
    register: ::volatile_register::RO<u32>,
}

impl Doutr {
    pub fn read(&self) -> DoutrR {
        DoutrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DoutrR {
    bits: u32,
}

impl DoutrR {
    # [ doc = "Bits 0:31 - Data output register" ]
    pub fn aes_doutr(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct DoutrW {
    bits: u32,
}

impl DoutrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        DoutrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Data output register" ]
    pub fn aes_doutr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Keyr0 {
    register: ::volatile_register::RW<u32>,
}

impl Keyr0 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Keyr0R, &'w mut Keyr0W) -> &'w mut Keyr0W
    {
        let bits = self.register.read();
        let r = Keyr0R { bits: bits };
        let mut w = Keyr0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Keyr0R {
        Keyr0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Keyr0W) -> &mut Keyr0W
    {
        let mut w = Keyr0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Keyr0R {
    bits: u32,
}

impl Keyr0R {
    # [ doc = "Bits 0:31 - Data Output Register (LSB key [31:0])" ]
    pub fn aes_keyr0(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Keyr0W {
    bits: u32,
}

impl Keyr0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Keyr0W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Data Output Register (LSB key [31:0])" ]
    pub fn aes_keyr0(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Keyr1 {
    register: ::volatile_register::RW<u32>,
}

impl Keyr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Keyr1R, &'w mut Keyr1W) -> &'w mut Keyr1W
    {
        let bits = self.register.read();
        let r = Keyr1R { bits: bits };
        let mut w = Keyr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Keyr1R {
        Keyr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Keyr1W) -> &mut Keyr1W
    {
        let mut w = Keyr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Keyr1R {
    bits: u32,
}

impl Keyr1R {
    # [ doc = "Bits 0:31 - AES key register (key [63:32])" ]
    pub fn aes_keyr1(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Keyr1W {
    bits: u32,
}

impl Keyr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Keyr1W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - AES key register (key [63:32])" ]
    pub fn aes_keyr1(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Keyr2 {
    register: ::volatile_register::RW<u32>,
}

impl Keyr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Keyr2R, &'w mut Keyr2W) -> &'w mut Keyr2W
    {
        let bits = self.register.read();
        let r = Keyr2R { bits: bits };
        let mut w = Keyr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Keyr2R {
        Keyr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Keyr2W) -> &mut Keyr2W
    {
        let mut w = Keyr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Keyr2R {
    bits: u32,
}

impl Keyr2R {
    # [ doc = "Bits 0:31 - AES key register (key [95:64])" ]
    pub fn aes_keyr2(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Keyr2W {
    bits: u32,
}

impl Keyr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Keyr2W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - AES key register (key [95:64])" ]
    pub fn aes_keyr2(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Keyr3 {
    register: ::volatile_register::RW<u32>,
}

impl Keyr3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Keyr3R, &'w mut Keyr3W) -> &'w mut Keyr3W
    {
        let bits = self.register.read();
        let r = Keyr3R { bits: bits };
        let mut w = Keyr3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Keyr3R {
        Keyr3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Keyr3W) -> &mut Keyr3W
    {
        let mut w = Keyr3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Keyr3R {
    bits: u32,
}

impl Keyr3R {
    # [ doc = "Bits 0:31 - AES key register (MSB key [127:96])" ]
    pub fn aes_keyr3(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Keyr3W {
    bits: u32,
}

impl Keyr3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Keyr3W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - AES key register (MSB key [127:96])" ]
    pub fn aes_keyr3(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ivr0 {
    register: ::volatile_register::RW<u32>,
}

impl Ivr0 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ivr0R, &'w mut Ivr0W) -> &'w mut Ivr0W
    {
        let bits = self.register.read();
        let r = Ivr0R { bits: bits };
        let mut w = Ivr0W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ivr0R {
        Ivr0R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ivr0W) -> &mut Ivr0W
    {
        let mut w = Ivr0W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ivr0R {
    bits: u32,
}

impl Ivr0R {
    # [ doc = "Bits 0:31 - initialization vector register (LSB IVR [31:0])" ]
    pub fn aes_ivr0(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ivr0W {
    bits: u32,
}

impl Ivr0W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ivr0W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - initialization vector register (LSB IVR [31:0])" ]
    pub fn aes_ivr0(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ivr1 {
    register: ::volatile_register::RW<u32>,
}

impl Ivr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ivr1R, &'w mut Ivr1W) -> &'w mut Ivr1W
    {
        let bits = self.register.read();
        let r = Ivr1R { bits: bits };
        let mut w = Ivr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ivr1R {
        Ivr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ivr1W) -> &mut Ivr1W
    {
        let mut w = Ivr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ivr1R {
    bits: u32,
}

impl Ivr1R {
    # [ doc = "Bits 0:31 - Initialization Vector Register (IVR [63:32])" ]
    pub fn aes_ivr1(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ivr1W {
    bits: u32,
}

impl Ivr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ivr1W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Initialization Vector Register (IVR [63:32])" ]
    pub fn aes_ivr1(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ivr2 {
    register: ::volatile_register::RW<u32>,
}

impl Ivr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ivr2R, &'w mut Ivr2W) -> &'w mut Ivr2W
    {
        let bits = self.register.read();
        let r = Ivr2R { bits: bits };
        let mut w = Ivr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ivr2R {
        Ivr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ivr2W) -> &mut Ivr2W
    {
        let mut w = Ivr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ivr2R {
    bits: u32,
}

impl Ivr2R {
    # [ doc = "Bits 0:31 - Initialization Vector Register (IVR [95:64])" ]
    pub fn aes_ivr2(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ivr2W {
    bits: u32,
}

impl Ivr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ivr2W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Initialization Vector Register (IVR [95:64])" ]
    pub fn aes_ivr2(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ivr3 {
    register: ::volatile_register::RW<u32>,
}

impl Ivr3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ivr3R, &'w mut Ivr3W) -> &'w mut Ivr3W
    {
        let bits = self.register.read();
        let r = Ivr3R { bits: bits };
        let mut w = Ivr3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ivr3R {
        Ivr3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ivr3W) -> &mut Ivr3W
    {
        let mut w = Ivr3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ivr3R {
    bits: u32,
}

impl Ivr3R {
    # [ doc = "Bits 0:31 - Initialization Vector Register (MSB IVR [127:96])" ]
    pub fn aes_ivr3(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ivr3W {
    bits: u32,
}

impl Ivr3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ivr3W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Initialization Vector Register (MSB IVR [127:96])" ]
    pub fn aes_ivr3(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
