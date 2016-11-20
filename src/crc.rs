# [ doc = "Cyclic redundancy check calculation unit" ]
# [ repr ( C ) ]
pub struct Crc {
    # [ doc = "0x00 - Data register" ]
    pub dr: Dr,
    # [ doc = "0x04 - Independent data register" ]
    pub idr: Idr,
    # [ doc = "0x08 - Control register" ]
    pub cr: Cr,
    _reserved0: [u8; 4usize],
    # [ doc = "0x10 - Initial CRC value" ]
    pub init: Init,
    # [ doc = "0x14 - polynomial" ]
    pub pol: Pol,
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
    # [ doc = "Bits 0:31 - Data register bits" ]
    pub fn dr(&self) -> u32 {
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
        DrW { bits: 4294967295u32 }
    }
    # [ doc = "Bits 0:31 - Data register bits" ]
    pub fn dr(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Idr {
    register: ::volatile_register::RW<u32>,
}

impl Idr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&IdrR, &'w mut IdrW) -> &'w mut IdrW
    {
        let bits = self.register.read();
        let r = IdrR { bits: bits };
        let mut w = IdrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IdrR {
        IdrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IdrW) -> &mut IdrW
    {
        let mut w = IdrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdrR {
    bits: u32,
}

impl IdrR {
    # [ doc = "Bits 0:7 - General-purpose 8-bit data register bits" ]
    pub fn idr(&self) -> u8 {
        const MASK: u32 = 255;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IdrW {
    bits: u32,
}

impl IdrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IdrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:7 - General-purpose 8-bit data register bits" ]
    pub fn idr(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 255;
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
    # [ doc = "Bit 7 - Reverse output data" ]
    pub fn rev_out(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 5:6 - Reverse input data" ]
    pub fn rev_in(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 5u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 3:4 - Polynomial size" ]
    pub fn polysize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 3u8;
        ((self.bits >> OFFSET) & MASK) as u8
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
    # [ doc = "Bit 7 - Reverse output data" ]
    pub fn rev_out(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 5:6 - Reverse input data" ]
    pub fn rev_in(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 5u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 3:4 - Polynomial size" ]
    pub fn polysize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 3u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 0 - RESET bit" ]
    pub fn reset(&mut self, value: bool) -> &mut Self {
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
pub struct Init {
    register: ::volatile_register::RW<u32>,
}

impl Init {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&InitR, &'w mut InitW) -> &'w mut InitW
    {
        let bits = self.register.read();
        let r = InitR { bits: bits };
        let mut w = InitW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> InitR {
        InitR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut InitW) -> &mut InitW
    {
        let mut w = InitW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct InitR {
    bits: u32,
}

impl InitR {
    # [ doc = "Bits 0:31 - Programmable initial CRC value" ]
    pub fn crc_init(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct InitW {
    bits: u32,
}

impl InitW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        InitW { bits: 4294967295u32 }
    }
    # [ doc = "Bits 0:31 - Programmable initial CRC value" ]
    pub fn crc_init(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Pol {
    register: ::volatile_register::RW<u32>,
}

impl Pol {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&PolR, &'w mut PolW) -> &'w mut PolW
    {
        let bits = self.register.read();
        let r = PolR { bits: bits };
        let mut w = PolW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> PolR {
        PolR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut PolW) -> &mut PolW
    {
        let mut w = PolW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PolR {
    bits: u32,
}

impl PolR {
    # [ doc = "Bits 0:31 - Programmable polynomial" ]
    pub fn polynomialcoefficients(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct PolW {
    bits: u32,
}

impl PolW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        PolW { bits: 79764919u32 }
    }
    # [ doc = "Bits 0:31 - Programmable polynomial" ]
    pub fn polynomialcoefficients(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
