# [ doc = "Firewall" ]
# [ repr ( C ) ]
pub struct Firewall {
    # [ doc = "0x00 - Code segment start address" ]
    pub cssa: Cssa,
    # [ doc = "0x04 - Code segment length" ]
    pub csl: Csl,
    # [ doc = "0x08 - Non-volatile data segment start address" ]
    pub nvdssa: Nvdssa,
    # [ doc = "0x0c - Non-volatile data segment length" ]
    pub nvdsl: Nvdsl,
    # [ doc = "0x10 - Volatile data segment start address" ]
    pub vdssa: Vdssa,
    # [ doc = "0x14 - Volatile data segment length" ]
    pub vdsl: Vdsl,
    _reserved0: [u8; 8usize],
    # [ doc = "0x20 - Configuration register" ]
    pub cr: Cr,
}

# [ repr ( C ) ]
pub struct Cssa {
    register: ::volatile_register::RW<u32>,
}

impl Cssa {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CssaR, &'w mut CssaW) -> &'w mut CssaW
    {
        let bits = self.register.read();
        let r = CssaR { bits: bits };
        let mut w = CssaW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CssaR {
        CssaR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CssaW) -> &mut CssaW
    {
        let mut w = CssaW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CssaR {
    bits: u32,
}

impl CssaR {
    # [ doc = "Bits 8:23 - code segment start address" ]
    pub fn add(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CssaW {
    bits: u32,
}

impl CssaW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CssaW { bits: 0u32 }
    }
    # [ doc = "Bits 8:23 - code segment start address" ]
    pub fn add(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Csl {
    register: ::volatile_register::RW<u32>,
}

impl Csl {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CslR, &'w mut CslW) -> &'w mut CslW
    {
        let bits = self.register.read();
        let r = CslR { bits: bits };
        let mut w = CslW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CslR {
        CslR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CslW) -> &mut CslW
    {
        let mut w = CslW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CslR {
    bits: u32,
}

impl CslR {
    # [ doc = "Bits 8:21 - code segment length" ]
    pub fn leng(&self) -> u16 {
        const MASK: u32 = 16383;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CslW {
    bits: u32,
}

impl CslW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CslW { bits: 0u32 }
    }
    # [ doc = "Bits 8:21 - code segment length" ]
    pub fn leng(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u16 = 16383;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Nvdssa {
    register: ::volatile_register::RW<u32>,
}

impl Nvdssa {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&NvdssaR, &'w mut NvdssaW) -> &'w mut NvdssaW
    {
        let bits = self.register.read();
        let r = NvdssaR { bits: bits };
        let mut w = NvdssaW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> NvdssaR {
        NvdssaR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut NvdssaW) -> &mut NvdssaW
    {
        let mut w = NvdssaW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct NvdssaR {
    bits: u32,
}

impl NvdssaR {
    # [ doc = "Bits 8:23 - Non-volatile data segment start address" ]
    pub fn add(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct NvdssaW {
    bits: u32,
}

impl NvdssaW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        NvdssaW { bits: 0u32 }
    }
    # [ doc = "Bits 8:23 - Non-volatile data segment start address" ]
    pub fn add(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Nvdsl {
    register: ::volatile_register::RW<u32>,
}

impl Nvdsl {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&NvdslR, &'w mut NvdslW) -> &'w mut NvdslW
    {
        let bits = self.register.read();
        let r = NvdslR { bits: bits };
        let mut w = NvdslW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> NvdslR {
        NvdslR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut NvdslW) -> &mut NvdslW
    {
        let mut w = NvdslW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct NvdslR {
    bits: u32,
}

impl NvdslR {
    # [ doc = "Bits 8:21 - Non-volatile data segment length" ]
    pub fn leng(&self) -> u16 {
        const MASK: u32 = 16383;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct NvdslW {
    bits: u32,
}

impl NvdslW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        NvdslW { bits: 0u32 }
    }
    # [ doc = "Bits 8:21 - Non-volatile data segment length" ]
    pub fn leng(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u16 = 16383;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Vdssa {
    register: ::volatile_register::RW<u32>,
}

impl Vdssa {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&VdssaR, &'w mut VdssaW) -> &'w mut VdssaW
    {
        let bits = self.register.read();
        let r = VdssaR { bits: bits };
        let mut w = VdssaW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> VdssaR {
        VdssaR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut VdssaW) -> &mut VdssaW
    {
        let mut w = VdssaW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct VdssaR {
    bits: u32,
}

impl VdssaR {
    # [ doc = "Bits 6:15 - Volatile data segment start address" ]
    pub fn add(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct VdssaW {
    bits: u32,
}

impl VdssaW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        VdssaW { bits: 0u32 }
    }
    # [ doc = "Bits 6:15 - Volatile data segment start address" ]
    pub fn add(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Vdsl {
    register: ::volatile_register::RW<u32>,
}

impl Vdsl {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&VdslR, &'w mut VdslW) -> &'w mut VdslW
    {
        let bits = self.register.read();
        let r = VdslR { bits: bits };
        let mut w = VdslW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> VdslR {
        VdslR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut VdslW) -> &mut VdslW
    {
        let mut w = VdslW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct VdslR {
    bits: u32,
}

impl VdslR {
    # [ doc = "Bits 6:15 - Non-volatile data segment length" ]
    pub fn leng(&self) -> u16 {
        const MASK: u32 = 1023;
        const OFFSET: u8 = 6u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct VdslW {
    bits: u32,
}

impl VdslW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        VdslW { bits: 0u32 }
    }
    # [ doc = "Bits 6:15 - Non-volatile data segment length" ]
    pub fn leng(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 6u8;
        const MASK: u16 = 1023;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
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
    # [ doc = "Bit 2 - Volatile data execution" ]
    pub fn vde(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Volatile data shared" ]
    pub fn vds(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Firewall pre alarm" ]
    pub fn fpa(&self) -> bool {
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
    # [ doc = "Bit 2 - Volatile data execution" ]
    pub fn vde(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Volatile data shared" ]
    pub fn vds(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Firewall pre alarm" ]
    pub fn fpa(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}
