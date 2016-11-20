# [ doc = "Direct memory access controller" ]
# [ repr ( C ) ]
pub struct Dma {
    # [ doc = "0x00 - interrupt status register" ]
    pub isr: Isr,
    # [ doc = "0x04 - interrupt flag clear register" ]
    pub ifcr: Ifcr,
    # [ doc = "0x08 - channel x configuration register" ]
    pub ccr1: Ccr1,
    # [ doc = "0x0c - channel x number of data register" ]
    pub cndtr1: Cndtr1,
    # [ doc = "0x10 - channel x peripheral address register" ]
    pub cpar1: Cpar1,
    # [ doc = "0x14 - channel x memory address register" ]
    pub cmar1: Cmar1,
    _reserved0: [u8; 4usize],
    # [ doc = "0x1c - channel x configuration register" ]
    pub ccr2: Ccr2,
    # [ doc = "0x20 - channel x number of data register" ]
    pub cndtr2: Cndtr2,
    # [ doc = "0x24 - channel x peripheral address register" ]
    pub cpar2: Cpar2,
    # [ doc = "0x28 - channel x memory address register" ]
    pub cmar2: Cmar2,
    _reserved1: [u8; 4usize],
    # [ doc = "0x30 - channel x configuration register" ]
    pub ccr3: Ccr3,
    # [ doc = "0x34 - channel x number of data register" ]
    pub cndtr3: Cndtr3,
    # [ doc = "0x38 - channel x peripheral address register" ]
    pub cpar3: Cpar3,
    # [ doc = "0x3c - channel x memory address register" ]
    pub cmar3: Cmar3,
    _reserved2: [u8; 4usize],
    # [ doc = "0x44 - channel x configuration register" ]
    pub ccr4: Ccr4,
    # [ doc = "0x48 - channel x number of data register" ]
    pub cndtr4: Cndtr4,
    # [ doc = "0x4c - channel x peripheral address register" ]
    pub cpar4: Cpar4,
    # [ doc = "0x50 - channel x memory address register" ]
    pub cmar4: Cmar4,
    _reserved3: [u8; 4usize],
    # [ doc = "0x58 - channel x configuration register" ]
    pub ccr5: Ccr5,
    # [ doc = "0x5c - channel x number of data register" ]
    pub cndtr5: Cndtr5,
    # [ doc = "0x60 - channel x peripheral address register" ]
    pub cpar5: Cpar5,
    # [ doc = "0x64 - channel x memory address register" ]
    pub cmar5: Cmar5,
    _reserved4: [u8; 4usize],
    # [ doc = "0x6c - channel x configuration register" ]
    pub ccr6: Ccr6,
    # [ doc = "0x70 - channel x number of data register" ]
    pub cndtr6: Cndtr6,
    # [ doc = "0x74 - channel x peripheral address register" ]
    pub cpar6: Cpar6,
    # [ doc = "0x78 - channel x memory address register" ]
    pub cmar6: Cmar6,
    _reserved5: [u8; 4usize],
    # [ doc = "0x80 - channel x configuration register" ]
    pub ccr7: Ccr7,
    # [ doc = "0x84 - channel x number of data register" ]
    pub cndtr7: Cndtr7,
    # [ doc = "0x88 - channel x peripheral address register" ]
    pub cpar7: Cpar7,
    # [ doc = "0x8c - channel x memory address register" ]
    pub cmar7: Cmar7,
    _reserved6: [u8; 24usize],
    # [ doc = "0xa8 - channel selection register" ]
    pub cselr: Cselr,
}

# [ repr ( C ) ]
pub struct Isr {
    register: ::volatile_register::RO<u32>,
}

impl Isr {
    pub fn read(&self) -> IsrR {
        IsrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IsrR {
    bits: u32,
}

impl IsrR {
    # [ doc = "Bit 27 - Channel x transfer error flag (x = 1 ..7)" ]
    pub fn teif7(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Channel x half transfer flag (x = 1 ..7)" ]
    pub fn htif7(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Channel x transfer complete flag (x = 1 ..7)" ]
    pub fn tcif7(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Channel x global interrupt flag (x = 1 ..7)" ]
    pub fn gif7(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - Channel x transfer error flag (x = 1 ..7)" ]
    pub fn teif6(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Channel x half transfer flag (x = 1 ..7)" ]
    pub fn htif6(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Channel x transfer complete flag (x = 1 ..7)" ]
    pub fn tcif6(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Channel x global interrupt flag (x = 1 ..7)" ]
    pub fn gif6(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Channel x transfer error flag (x = 1 ..7)" ]
    pub fn teif5(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Channel x half transfer flag (x = 1 ..7)" ]
    pub fn htif5(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Channel x transfer complete flag (x = 1 ..7)" ]
    pub fn tcif5(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Channel x global interrupt flag (x = 1 ..7)" ]
    pub fn gif5(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Channel x transfer error flag (x = 1 ..7)" ]
    pub fn teif4(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Channel x half transfer flag (x = 1 ..7)" ]
    pub fn htif4(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Channel x transfer complete flag (x = 1 ..7)" ]
    pub fn tcif4(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Channel x global interrupt flag (x = 1 ..7)" ]
    pub fn gif4(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Channel x transfer error flag (x = 1 ..7)" ]
    pub fn teif3(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Channel x half transfer flag (x = 1 ..7)" ]
    pub fn htif3(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Channel x transfer complete flag (x = 1 ..7)" ]
    pub fn tcif3(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Channel x global interrupt flag (x = 1 ..7)" ]
    pub fn gif3(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Channel x transfer error flag (x = 1 ..7)" ]
    pub fn teif2(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Channel x half transfer flag (x = 1 ..7)" ]
    pub fn htif2(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Channel x transfer complete flag (x = 1 ..7)" ]
    pub fn tcif2(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Channel x global interrupt flag (x = 1 ..7)" ]
    pub fn gif2(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Channel x transfer error flag (x = 1 ..7)" ]
    pub fn teif1(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Channel x half transfer flag (x = 1 ..7)" ]
    pub fn htif1(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel x transfer complete flag (x = 1 ..7)" ]
    pub fn tcif1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Channel x global interrupt flag (x = 1 ..7)" ]
    pub fn gif1(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IsrW {
    bits: u32,
}

impl IsrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IsrW { bits: 0u32 }
    }
    # [ doc = "Bit 27 - Channel x transfer error flag (x = 1 ..7)" ]
    pub fn teif7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Channel x half transfer flag (x = 1 ..7)" ]
    pub fn htif7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Channel x transfer complete flag (x = 1 ..7)" ]
    pub fn tcif7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Channel x global interrupt flag (x = 1 ..7)" ]
    pub fn gif7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - Channel x transfer error flag (x = 1 ..7)" ]
    pub fn teif6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Channel x half transfer flag (x = 1 ..7)" ]
    pub fn htif6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Channel x transfer complete flag (x = 1 ..7)" ]
    pub fn tcif6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Channel x global interrupt flag (x = 1 ..7)" ]
    pub fn gif6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Channel x transfer error flag (x = 1 ..7)" ]
    pub fn teif5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Channel x half transfer flag (x = 1 ..7)" ]
    pub fn htif5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Channel x transfer complete flag (x = 1 ..7)" ]
    pub fn tcif5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Channel x global interrupt flag (x = 1 ..7)" ]
    pub fn gif5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Channel x transfer error flag (x = 1 ..7)" ]
    pub fn teif4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Channel x half transfer flag (x = 1 ..7)" ]
    pub fn htif4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Channel x transfer complete flag (x = 1 ..7)" ]
    pub fn tcif4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Channel x global interrupt flag (x = 1 ..7)" ]
    pub fn gif4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Channel x transfer error flag (x = 1 ..7)" ]
    pub fn teif3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Channel x half transfer flag (x = 1 ..7)" ]
    pub fn htif3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Channel x transfer complete flag (x = 1 ..7)" ]
    pub fn tcif3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Channel x global interrupt flag (x = 1 ..7)" ]
    pub fn gif3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Channel x transfer error flag (x = 1 ..7)" ]
    pub fn teif2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Channel x half transfer flag (x = 1 ..7)" ]
    pub fn htif2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Channel x transfer complete flag (x = 1 ..7)" ]
    pub fn tcif2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Channel x global interrupt flag (x = 1 ..7)" ]
    pub fn gif2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Channel x transfer error flag (x = 1 ..7)" ]
    pub fn teif1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Channel x half transfer flag (x = 1 ..7)" ]
    pub fn htif1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel x transfer complete flag (x = 1 ..7)" ]
    pub fn tcif1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Channel x global interrupt flag (x = 1 ..7)" ]
    pub fn gif1(&mut self, value: bool) -> &mut Self {
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
pub struct Ifcr {
    register: ::volatile_register::WO<u32>,
}

impl Ifcr {
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut IfcrW) -> &mut IfcrW
    {
        let mut w = IfcrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IfcrR {
    bits: u32,
}

impl IfcrR {
    # [ doc = "Bit 27 - Channel x transfer error clear (x = 1 ..7)" ]
    pub fn cteif7(&self) -> bool {
        const OFFSET: u8 = 27u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 26 - Channel x half transfer clear (x = 1 ..7)" ]
    pub fn chtif7(&self) -> bool {
        const OFFSET: u8 = 26u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 25 - Channel x transfer complete clear (x = 1 ..7)" ]
    pub fn ctcif7(&self) -> bool {
        const OFFSET: u8 = 25u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 24 - Channel x global interrupt clear (x = 1 ..7)" ]
    pub fn cgif7(&self) -> bool {
        const OFFSET: u8 = 24u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 23 - Channel x transfer error clear (x = 1 ..7)" ]
    pub fn cteif6(&self) -> bool {
        const OFFSET: u8 = 23u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 22 - Channel x half transfer clear (x = 1 ..7)" ]
    pub fn chtif6(&self) -> bool {
        const OFFSET: u8 = 22u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 21 - Channel x transfer complete clear (x = 1 ..7)" ]
    pub fn ctcif6(&self) -> bool {
        const OFFSET: u8 = 21u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 20 - Channel x global interrupt clear (x = 1 ..7)" ]
    pub fn cgif6(&self) -> bool {
        const OFFSET: u8 = 20u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 19 - Channel x transfer error clear (x = 1 ..7)" ]
    pub fn cteif5(&self) -> bool {
        const OFFSET: u8 = 19u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 18 - Channel x half transfer clear (x = 1 ..7)" ]
    pub fn chtif5(&self) -> bool {
        const OFFSET: u8 = 18u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 17 - Channel x transfer complete clear (x = 1 ..7)" ]
    pub fn ctcif5(&self) -> bool {
        const OFFSET: u8 = 17u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 16 - Channel x global interrupt clear (x = 1 ..7)" ]
    pub fn cgif5(&self) -> bool {
        const OFFSET: u8 = 16u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 15 - Channel x transfer error clear (x = 1 ..7)" ]
    pub fn cteif4(&self) -> bool {
        const OFFSET: u8 = 15u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 14 - Channel x half transfer clear (x = 1 ..7)" ]
    pub fn chtif4(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 13 - Channel x transfer complete clear (x = 1 ..7)" ]
    pub fn ctcif4(&self) -> bool {
        const OFFSET: u8 = 13u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 12 - Channel x global interrupt clear (x = 1 ..7)" ]
    pub fn cgif4(&self) -> bool {
        const OFFSET: u8 = 12u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 11 - Channel x transfer error clear (x = 1 ..7)" ]
    pub fn cteif3(&self) -> bool {
        const OFFSET: u8 = 11u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Channel x half transfer clear (x = 1 ..7)" ]
    pub fn chtif3(&self) -> bool {
        const OFFSET: u8 = 10u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - Channel x transfer complete clear (x = 1 ..7)" ]
    pub fn ctcif3(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Channel x global interrupt clear (x = 1 ..7)" ]
    pub fn cgif3(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Channel x transfer error clear (x = 1 ..7)" ]
    pub fn cteif2(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Channel x half transfer clear (x = 1 ..7)" ]
    pub fn chtif2(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Channel x transfer complete clear (x = 1 ..7)" ]
    pub fn ctcif2(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Channel x global interrupt clear (x = 1 ..7)" ]
    pub fn cgif2(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Channel x transfer error clear (x = 1 ..7)" ]
    pub fn cteif1(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Channel x half transfer clear (x = 1 ..7)" ]
    pub fn chtif1(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Channel x transfer complete clear (x = 1 ..7)" ]
    pub fn ctcif1(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Channel x global interrupt clear (x = 1 ..7)" ]
    pub fn cgif1(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IfcrW {
    bits: u32,
}

impl IfcrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IfcrW { bits: 0u32 }
    }
    # [ doc = "Bit 27 - Channel x transfer error clear (x = 1 ..7)" ]
    pub fn cteif7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 27u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 26 - Channel x half transfer clear (x = 1 ..7)" ]
    pub fn chtif7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 26u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 25 - Channel x transfer complete clear (x = 1 ..7)" ]
    pub fn ctcif7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 25u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 24 - Channel x global interrupt clear (x = 1 ..7)" ]
    pub fn cgif7(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 24u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 23 - Channel x transfer error clear (x = 1 ..7)" ]
    pub fn cteif6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 23u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 22 - Channel x half transfer clear (x = 1 ..7)" ]
    pub fn chtif6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 22u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 21 - Channel x transfer complete clear (x = 1 ..7)" ]
    pub fn ctcif6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 21u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 20 - Channel x global interrupt clear (x = 1 ..7)" ]
    pub fn cgif6(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 20u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 19 - Channel x transfer error clear (x = 1 ..7)" ]
    pub fn cteif5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 19u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 18 - Channel x half transfer clear (x = 1 ..7)" ]
    pub fn chtif5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 18u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 17 - Channel x transfer complete clear (x = 1 ..7)" ]
    pub fn ctcif5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 17u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 16 - Channel x global interrupt clear (x = 1 ..7)" ]
    pub fn cgif5(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 16u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 15 - Channel x transfer error clear (x = 1 ..7)" ]
    pub fn cteif4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 15u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 14 - Channel x half transfer clear (x = 1 ..7)" ]
    pub fn chtif4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 13 - Channel x transfer complete clear (x = 1 ..7)" ]
    pub fn ctcif4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 13u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 12 - Channel x global interrupt clear (x = 1 ..7)" ]
    pub fn cgif4(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 12u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 11 - Channel x transfer error clear (x = 1 ..7)" ]
    pub fn cteif3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 11u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Channel x half transfer clear (x = 1 ..7)" ]
    pub fn chtif3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 10u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - Channel x transfer complete clear (x = 1 ..7)" ]
    pub fn ctcif3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Channel x global interrupt clear (x = 1 ..7)" ]
    pub fn cgif3(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Channel x transfer error clear (x = 1 ..7)" ]
    pub fn cteif2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Channel x half transfer clear (x = 1 ..7)" ]
    pub fn chtif2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Channel x transfer complete clear (x = 1 ..7)" ]
    pub fn ctcif2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Channel x global interrupt clear (x = 1 ..7)" ]
    pub fn cgif2(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Channel x transfer error clear (x = 1 ..7)" ]
    pub fn cteif1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Channel x half transfer clear (x = 1 ..7)" ]
    pub fn chtif1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Channel x transfer complete clear (x = 1 ..7)" ]
    pub fn ctcif1(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Channel x global interrupt clear (x = 1 ..7)" ]
    pub fn cgif1(&mut self, value: bool) -> &mut Self {
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
pub struct Ccr1 {
    register: ::volatile_register::RW<u32>,
}

impl Ccr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ccr1R, &'w mut Ccr1W) -> &'w mut Ccr1W
    {
        let bits = self.register.read();
        let r = Ccr1R { bits: bits };
        let mut w = Ccr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ccr1R {
        Ccr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ccr1W) -> &mut Ccr1W
    {
        let mut w = Ccr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr1R {
    bits: u32,
}

impl Ccr1R {
    # [ doc = "Bit 14 - Memory to memory mode" ]
    pub fn mem2mem(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:13 - Channel priority level" ]
    pub fn pl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 10:11 - Memory size" ]
    pub fn msize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:9 - Peripheral size" ]
    pub fn psize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Memory increment mode" ]
    pub fn minc(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Peripheral increment mode" ]
    pub fn pinc(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Circular mode" ]
    pub fn circ(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Data transfer direction" ]
    pub fn dir(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Transfer error interrupt enable" ]
    pub fn teie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Half transfer interrupt enable" ]
    pub fn htie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transfer complete interrupt enable" ]
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Channel enable" ]
    pub fn en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr1W {
    bits: u32,
}

impl Ccr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ccr1W { bits: 0u32 }
    }
    # [ doc = "Bit 14 - Memory to memory mode" ]
    pub fn mem2mem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:13 - Channel priority level" ]
    pub fn pl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 10:11 - Memory size" ]
    pub fn msize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:9 - Peripheral size" ]
    pub fn psize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Memory increment mode" ]
    pub fn minc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Peripheral increment mode" ]
    pub fn pinc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Circular mode" ]
    pub fn circ(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Data transfer direction" ]
    pub fn dir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Transfer error interrupt enable" ]
    pub fn teie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Half transfer interrupt enable" ]
    pub fn htie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transfer complete interrupt enable" ]
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Channel enable" ]
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
pub struct Cndtr1 {
    register: ::volatile_register::RW<u32>,
}

impl Cndtr1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cndtr1R, &'w mut Cndtr1W) -> &'w mut Cndtr1W
    {
        let bits = self.register.read();
        let r = Cndtr1R { bits: bits };
        let mut w = Cndtr1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cndtr1R {
        Cndtr1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cndtr1W) -> &mut Cndtr1W
    {
        let mut w = Cndtr1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cndtr1R {
    bits: u32,
}

impl Cndtr1R {
    # [ doc = "Bits 0:15 - Number of data to transfer" ]
    pub fn ndt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cndtr1W {
    bits: u32,
}

impl Cndtr1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cndtr1W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Number of data to transfer" ]
    pub fn ndt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cpar1 {
    register: ::volatile_register::RW<u32>,
}

impl Cpar1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cpar1R, &'w mut Cpar1W) -> &'w mut Cpar1W
    {
        let bits = self.register.read();
        let r = Cpar1R { bits: bits };
        let mut w = Cpar1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cpar1R {
        Cpar1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cpar1W) -> &mut Cpar1W
    {
        let mut w = Cpar1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cpar1R {
    bits: u32,
}

impl Cpar1R {
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cpar1W {
    bits: u32,
}

impl Cpar1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cpar1W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cmar1 {
    register: ::volatile_register::RW<u32>,
}

impl Cmar1 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cmar1R, &'w mut Cmar1W) -> &'w mut Cmar1W
    {
        let bits = self.register.read();
        let r = Cmar1R { bits: bits };
        let mut w = Cmar1W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cmar1R {
        Cmar1R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cmar1W) -> &mut Cmar1W
    {
        let mut w = Cmar1W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cmar1R {
    bits: u32,
}

impl Cmar1R {
    # [ doc = "Bits 0:31 - Memory address" ]
    pub fn ma(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cmar1W {
    bits: u32,
}

impl Cmar1W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cmar1W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory address" ]
    pub fn ma(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ccr2 {
    register: ::volatile_register::RW<u32>,
}

impl Ccr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ccr2R, &'w mut Ccr2W) -> &'w mut Ccr2W
    {
        let bits = self.register.read();
        let r = Ccr2R { bits: bits };
        let mut w = Ccr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ccr2R {
        Ccr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ccr2W) -> &mut Ccr2W
    {
        let mut w = Ccr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr2R {
    bits: u32,
}

impl Ccr2R {
    # [ doc = "Bit 14 - Memory to memory mode" ]
    pub fn mem2mem(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:13 - Channel priority level" ]
    pub fn pl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 10:11 - Memory size" ]
    pub fn msize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:9 - Peripheral size" ]
    pub fn psize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Memory increment mode" ]
    pub fn minc(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Peripheral increment mode" ]
    pub fn pinc(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Circular mode" ]
    pub fn circ(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Data transfer direction" ]
    pub fn dir(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Transfer error interrupt enable" ]
    pub fn teie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Half transfer interrupt enable" ]
    pub fn htie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transfer complete interrupt enable" ]
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Channel enable" ]
    pub fn en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr2W {
    bits: u32,
}

impl Ccr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ccr2W { bits: 0u32 }
    }
    # [ doc = "Bit 14 - Memory to memory mode" ]
    pub fn mem2mem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:13 - Channel priority level" ]
    pub fn pl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 10:11 - Memory size" ]
    pub fn msize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:9 - Peripheral size" ]
    pub fn psize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Memory increment mode" ]
    pub fn minc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Peripheral increment mode" ]
    pub fn pinc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Circular mode" ]
    pub fn circ(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Data transfer direction" ]
    pub fn dir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Transfer error interrupt enable" ]
    pub fn teie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Half transfer interrupt enable" ]
    pub fn htie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transfer complete interrupt enable" ]
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Channel enable" ]
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
pub struct Cndtr2 {
    register: ::volatile_register::RW<u32>,
}

impl Cndtr2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cndtr2R, &'w mut Cndtr2W) -> &'w mut Cndtr2W
    {
        let bits = self.register.read();
        let r = Cndtr2R { bits: bits };
        let mut w = Cndtr2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cndtr2R {
        Cndtr2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cndtr2W) -> &mut Cndtr2W
    {
        let mut w = Cndtr2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cndtr2R {
    bits: u32,
}

impl Cndtr2R {
    # [ doc = "Bits 0:15 - Number of data to transfer" ]
    pub fn ndt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cndtr2W {
    bits: u32,
}

impl Cndtr2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cndtr2W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Number of data to transfer" ]
    pub fn ndt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cpar2 {
    register: ::volatile_register::RW<u32>,
}

impl Cpar2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cpar2R, &'w mut Cpar2W) -> &'w mut Cpar2W
    {
        let bits = self.register.read();
        let r = Cpar2R { bits: bits };
        let mut w = Cpar2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cpar2R {
        Cpar2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cpar2W) -> &mut Cpar2W
    {
        let mut w = Cpar2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cpar2R {
    bits: u32,
}

impl Cpar2R {
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cpar2W {
    bits: u32,
}

impl Cpar2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cpar2W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cmar2 {
    register: ::volatile_register::RW<u32>,
}

impl Cmar2 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cmar2R, &'w mut Cmar2W) -> &'w mut Cmar2W
    {
        let bits = self.register.read();
        let r = Cmar2R { bits: bits };
        let mut w = Cmar2W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cmar2R {
        Cmar2R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cmar2W) -> &mut Cmar2W
    {
        let mut w = Cmar2W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cmar2R {
    bits: u32,
}

impl Cmar2R {
    # [ doc = "Bits 0:31 - Memory address" ]
    pub fn ma(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cmar2W {
    bits: u32,
}

impl Cmar2W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cmar2W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory address" ]
    pub fn ma(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ccr3 {
    register: ::volatile_register::RW<u32>,
}

impl Ccr3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ccr3R, &'w mut Ccr3W) -> &'w mut Ccr3W
    {
        let bits = self.register.read();
        let r = Ccr3R { bits: bits };
        let mut w = Ccr3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ccr3R {
        Ccr3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ccr3W) -> &mut Ccr3W
    {
        let mut w = Ccr3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr3R {
    bits: u32,
}

impl Ccr3R {
    # [ doc = "Bit 14 - Memory to memory mode" ]
    pub fn mem2mem(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:13 - Channel priority level" ]
    pub fn pl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 10:11 - Memory size" ]
    pub fn msize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:9 - Peripheral size" ]
    pub fn psize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Memory increment mode" ]
    pub fn minc(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Peripheral increment mode" ]
    pub fn pinc(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Circular mode" ]
    pub fn circ(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Data transfer direction" ]
    pub fn dir(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Transfer error interrupt enable" ]
    pub fn teie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Half transfer interrupt enable" ]
    pub fn htie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transfer complete interrupt enable" ]
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Channel enable" ]
    pub fn en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr3W {
    bits: u32,
}

impl Ccr3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ccr3W { bits: 0u32 }
    }
    # [ doc = "Bit 14 - Memory to memory mode" ]
    pub fn mem2mem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:13 - Channel priority level" ]
    pub fn pl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 10:11 - Memory size" ]
    pub fn msize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:9 - Peripheral size" ]
    pub fn psize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Memory increment mode" ]
    pub fn minc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Peripheral increment mode" ]
    pub fn pinc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Circular mode" ]
    pub fn circ(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Data transfer direction" ]
    pub fn dir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Transfer error interrupt enable" ]
    pub fn teie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Half transfer interrupt enable" ]
    pub fn htie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transfer complete interrupt enable" ]
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Channel enable" ]
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
pub struct Cndtr3 {
    register: ::volatile_register::RW<u32>,
}

impl Cndtr3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cndtr3R, &'w mut Cndtr3W) -> &'w mut Cndtr3W
    {
        let bits = self.register.read();
        let r = Cndtr3R { bits: bits };
        let mut w = Cndtr3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cndtr3R {
        Cndtr3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cndtr3W) -> &mut Cndtr3W
    {
        let mut w = Cndtr3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cndtr3R {
    bits: u32,
}

impl Cndtr3R {
    # [ doc = "Bits 0:15 - Number of data to transfer" ]
    pub fn ndt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cndtr3W {
    bits: u32,
}

impl Cndtr3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cndtr3W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Number of data to transfer" ]
    pub fn ndt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cpar3 {
    register: ::volatile_register::RW<u32>,
}

impl Cpar3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cpar3R, &'w mut Cpar3W) -> &'w mut Cpar3W
    {
        let bits = self.register.read();
        let r = Cpar3R { bits: bits };
        let mut w = Cpar3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cpar3R {
        Cpar3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cpar3W) -> &mut Cpar3W
    {
        let mut w = Cpar3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cpar3R {
    bits: u32,
}

impl Cpar3R {
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cpar3W {
    bits: u32,
}

impl Cpar3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cpar3W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cmar3 {
    register: ::volatile_register::RW<u32>,
}

impl Cmar3 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cmar3R, &'w mut Cmar3W) -> &'w mut Cmar3W
    {
        let bits = self.register.read();
        let r = Cmar3R { bits: bits };
        let mut w = Cmar3W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cmar3R {
        Cmar3R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cmar3W) -> &mut Cmar3W
    {
        let mut w = Cmar3W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cmar3R {
    bits: u32,
}

impl Cmar3R {
    # [ doc = "Bits 0:31 - Memory address" ]
    pub fn ma(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cmar3W {
    bits: u32,
}

impl Cmar3W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cmar3W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory address" ]
    pub fn ma(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ccr4 {
    register: ::volatile_register::RW<u32>,
}

impl Ccr4 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ccr4R, &'w mut Ccr4W) -> &'w mut Ccr4W
    {
        let bits = self.register.read();
        let r = Ccr4R { bits: bits };
        let mut w = Ccr4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ccr4R {
        Ccr4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ccr4W) -> &mut Ccr4W
    {
        let mut w = Ccr4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr4R {
    bits: u32,
}

impl Ccr4R {
    # [ doc = "Bit 14 - Memory to memory mode" ]
    pub fn mem2mem(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:13 - Channel priority level" ]
    pub fn pl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 10:11 - Memory size" ]
    pub fn msize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:9 - Peripheral size" ]
    pub fn psize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Memory increment mode" ]
    pub fn minc(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Peripheral increment mode" ]
    pub fn pinc(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Circular mode" ]
    pub fn circ(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Data transfer direction" ]
    pub fn dir(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Transfer error interrupt enable" ]
    pub fn teie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Half transfer interrupt enable" ]
    pub fn htie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transfer complete interrupt enable" ]
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Channel enable" ]
    pub fn en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr4W {
    bits: u32,
}

impl Ccr4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ccr4W { bits: 0u32 }
    }
    # [ doc = "Bit 14 - Memory to memory mode" ]
    pub fn mem2mem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:13 - Channel priority level" ]
    pub fn pl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 10:11 - Memory size" ]
    pub fn msize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:9 - Peripheral size" ]
    pub fn psize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Memory increment mode" ]
    pub fn minc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Peripheral increment mode" ]
    pub fn pinc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Circular mode" ]
    pub fn circ(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Data transfer direction" ]
    pub fn dir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Transfer error interrupt enable" ]
    pub fn teie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Half transfer interrupt enable" ]
    pub fn htie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transfer complete interrupt enable" ]
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Channel enable" ]
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
pub struct Cndtr4 {
    register: ::volatile_register::RW<u32>,
}

impl Cndtr4 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cndtr4R, &'w mut Cndtr4W) -> &'w mut Cndtr4W
    {
        let bits = self.register.read();
        let r = Cndtr4R { bits: bits };
        let mut w = Cndtr4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cndtr4R {
        Cndtr4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cndtr4W) -> &mut Cndtr4W
    {
        let mut w = Cndtr4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cndtr4R {
    bits: u32,
}

impl Cndtr4R {
    # [ doc = "Bits 0:15 - Number of data to transfer" ]
    pub fn ndt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cndtr4W {
    bits: u32,
}

impl Cndtr4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cndtr4W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Number of data to transfer" ]
    pub fn ndt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cpar4 {
    register: ::volatile_register::RW<u32>,
}

impl Cpar4 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cpar4R, &'w mut Cpar4W) -> &'w mut Cpar4W
    {
        let bits = self.register.read();
        let r = Cpar4R { bits: bits };
        let mut w = Cpar4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cpar4R {
        Cpar4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cpar4W) -> &mut Cpar4W
    {
        let mut w = Cpar4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cpar4R {
    bits: u32,
}

impl Cpar4R {
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cpar4W {
    bits: u32,
}

impl Cpar4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cpar4W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cmar4 {
    register: ::volatile_register::RW<u32>,
}

impl Cmar4 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cmar4R, &'w mut Cmar4W) -> &'w mut Cmar4W
    {
        let bits = self.register.read();
        let r = Cmar4R { bits: bits };
        let mut w = Cmar4W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cmar4R {
        Cmar4R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cmar4W) -> &mut Cmar4W
    {
        let mut w = Cmar4W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cmar4R {
    bits: u32,
}

impl Cmar4R {
    # [ doc = "Bits 0:31 - Memory address" ]
    pub fn ma(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cmar4W {
    bits: u32,
}

impl Cmar4W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cmar4W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory address" ]
    pub fn ma(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ccr5 {
    register: ::volatile_register::RW<u32>,
}

impl Ccr5 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ccr5R, &'w mut Ccr5W) -> &'w mut Ccr5W
    {
        let bits = self.register.read();
        let r = Ccr5R { bits: bits };
        let mut w = Ccr5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ccr5R {
        Ccr5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ccr5W) -> &mut Ccr5W
    {
        let mut w = Ccr5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr5R {
    bits: u32,
}

impl Ccr5R {
    # [ doc = "Bit 14 - Memory to memory mode" ]
    pub fn mem2mem(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:13 - Channel priority level" ]
    pub fn pl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 10:11 - Memory size" ]
    pub fn msize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:9 - Peripheral size" ]
    pub fn psize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Memory increment mode" ]
    pub fn minc(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Peripheral increment mode" ]
    pub fn pinc(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Circular mode" ]
    pub fn circ(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Data transfer direction" ]
    pub fn dir(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Transfer error interrupt enable" ]
    pub fn teie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Half transfer interrupt enable" ]
    pub fn htie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transfer complete interrupt enable" ]
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Channel enable" ]
    pub fn en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr5W {
    bits: u32,
}

impl Ccr5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ccr5W { bits: 0u32 }
    }
    # [ doc = "Bit 14 - Memory to memory mode" ]
    pub fn mem2mem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:13 - Channel priority level" ]
    pub fn pl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 10:11 - Memory size" ]
    pub fn msize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:9 - Peripheral size" ]
    pub fn psize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Memory increment mode" ]
    pub fn minc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Peripheral increment mode" ]
    pub fn pinc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Circular mode" ]
    pub fn circ(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Data transfer direction" ]
    pub fn dir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Transfer error interrupt enable" ]
    pub fn teie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Half transfer interrupt enable" ]
    pub fn htie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transfer complete interrupt enable" ]
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Channel enable" ]
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
pub struct Cndtr5 {
    register: ::volatile_register::RW<u32>,
}

impl Cndtr5 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cndtr5R, &'w mut Cndtr5W) -> &'w mut Cndtr5W
    {
        let bits = self.register.read();
        let r = Cndtr5R { bits: bits };
        let mut w = Cndtr5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cndtr5R {
        Cndtr5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cndtr5W) -> &mut Cndtr5W
    {
        let mut w = Cndtr5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cndtr5R {
    bits: u32,
}

impl Cndtr5R {
    # [ doc = "Bits 0:15 - Number of data to transfer" ]
    pub fn ndt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cndtr5W {
    bits: u32,
}

impl Cndtr5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cndtr5W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Number of data to transfer" ]
    pub fn ndt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cpar5 {
    register: ::volatile_register::RW<u32>,
}

impl Cpar5 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cpar5R, &'w mut Cpar5W) -> &'w mut Cpar5W
    {
        let bits = self.register.read();
        let r = Cpar5R { bits: bits };
        let mut w = Cpar5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cpar5R {
        Cpar5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cpar5W) -> &mut Cpar5W
    {
        let mut w = Cpar5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cpar5R {
    bits: u32,
}

impl Cpar5R {
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cpar5W {
    bits: u32,
}

impl Cpar5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cpar5W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cmar5 {
    register: ::volatile_register::RW<u32>,
}

impl Cmar5 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cmar5R, &'w mut Cmar5W) -> &'w mut Cmar5W
    {
        let bits = self.register.read();
        let r = Cmar5R { bits: bits };
        let mut w = Cmar5W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cmar5R {
        Cmar5R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cmar5W) -> &mut Cmar5W
    {
        let mut w = Cmar5W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cmar5R {
    bits: u32,
}

impl Cmar5R {
    # [ doc = "Bits 0:31 - Memory address" ]
    pub fn ma(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cmar5W {
    bits: u32,
}

impl Cmar5W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cmar5W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory address" ]
    pub fn ma(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ccr6 {
    register: ::volatile_register::RW<u32>,
}

impl Ccr6 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ccr6R, &'w mut Ccr6W) -> &'w mut Ccr6W
    {
        let bits = self.register.read();
        let r = Ccr6R { bits: bits };
        let mut w = Ccr6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ccr6R {
        Ccr6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ccr6W) -> &mut Ccr6W
    {
        let mut w = Ccr6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr6R {
    bits: u32,
}

impl Ccr6R {
    # [ doc = "Bit 14 - Memory to memory mode" ]
    pub fn mem2mem(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:13 - Channel priority level" ]
    pub fn pl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 10:11 - Memory size" ]
    pub fn msize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:9 - Peripheral size" ]
    pub fn psize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Memory increment mode" ]
    pub fn minc(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Peripheral increment mode" ]
    pub fn pinc(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Circular mode" ]
    pub fn circ(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Data transfer direction" ]
    pub fn dir(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Transfer error interrupt enable" ]
    pub fn teie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Half transfer interrupt enable" ]
    pub fn htie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transfer complete interrupt enable" ]
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Channel enable" ]
    pub fn en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr6W {
    bits: u32,
}

impl Ccr6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ccr6W { bits: 0u32 }
    }
    # [ doc = "Bit 14 - Memory to memory mode" ]
    pub fn mem2mem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:13 - Channel priority level" ]
    pub fn pl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 10:11 - Memory size" ]
    pub fn msize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:9 - Peripheral size" ]
    pub fn psize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Memory increment mode" ]
    pub fn minc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Peripheral increment mode" ]
    pub fn pinc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Circular mode" ]
    pub fn circ(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Data transfer direction" ]
    pub fn dir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Transfer error interrupt enable" ]
    pub fn teie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Half transfer interrupt enable" ]
    pub fn htie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transfer complete interrupt enable" ]
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Channel enable" ]
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
pub struct Cndtr6 {
    register: ::volatile_register::RW<u32>,
}

impl Cndtr6 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cndtr6R, &'w mut Cndtr6W) -> &'w mut Cndtr6W
    {
        let bits = self.register.read();
        let r = Cndtr6R { bits: bits };
        let mut w = Cndtr6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cndtr6R {
        Cndtr6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cndtr6W) -> &mut Cndtr6W
    {
        let mut w = Cndtr6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cndtr6R {
    bits: u32,
}

impl Cndtr6R {
    # [ doc = "Bits 0:15 - Number of data to transfer" ]
    pub fn ndt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cndtr6W {
    bits: u32,
}

impl Cndtr6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cndtr6W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Number of data to transfer" ]
    pub fn ndt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cpar6 {
    register: ::volatile_register::RW<u32>,
}

impl Cpar6 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cpar6R, &'w mut Cpar6W) -> &'w mut Cpar6W
    {
        let bits = self.register.read();
        let r = Cpar6R { bits: bits };
        let mut w = Cpar6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cpar6R {
        Cpar6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cpar6W) -> &mut Cpar6W
    {
        let mut w = Cpar6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cpar6R {
    bits: u32,
}

impl Cpar6R {
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cpar6W {
    bits: u32,
}

impl Cpar6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cpar6W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cmar6 {
    register: ::volatile_register::RW<u32>,
}

impl Cmar6 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cmar6R, &'w mut Cmar6W) -> &'w mut Cmar6W
    {
        let bits = self.register.read();
        let r = Cmar6R { bits: bits };
        let mut w = Cmar6W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cmar6R {
        Cmar6R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cmar6W) -> &mut Cmar6W
    {
        let mut w = Cmar6W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cmar6R {
    bits: u32,
}

impl Cmar6R {
    # [ doc = "Bits 0:31 - Memory address" ]
    pub fn ma(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cmar6W {
    bits: u32,
}

impl Cmar6W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cmar6W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory address" ]
    pub fn ma(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Ccr7 {
    register: ::volatile_register::RW<u32>,
}

impl Ccr7 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Ccr7R, &'w mut Ccr7W) -> &'w mut Ccr7W
    {
        let bits = self.register.read();
        let r = Ccr7R { bits: bits };
        let mut w = Ccr7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Ccr7R {
        Ccr7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Ccr7W) -> &mut Ccr7W
    {
        let mut w = Ccr7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr7R {
    bits: u32,
}

impl Ccr7R {
    # [ doc = "Bit 14 - Memory to memory mode" ]
    pub fn mem2mem(&self) -> bool {
        const OFFSET: u8 = 14u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bits 12:13 - Channel priority level" ]
    pub fn pl(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 10:11 - Memory size" ]
    pub fn msize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 10u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:9 - Peripheral size" ]
    pub fn psize(&self) -> u8 {
        const MASK: u32 = 3;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bit 7 - Memory increment mode" ]
    pub fn minc(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Peripheral increment mode" ]
    pub fn pinc(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Circular mode" ]
    pub fn circ(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Data transfer direction" ]
    pub fn dir(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Transfer error interrupt enable" ]
    pub fn teie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Half transfer interrupt enable" ]
    pub fn htie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transfer complete interrupt enable" ]
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 0 - Channel enable" ]
    pub fn en(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Ccr7W {
    bits: u32,
}

impl Ccr7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Ccr7W { bits: 0u32 }
    }
    # [ doc = "Bit 14 - Memory to memory mode" ]
    pub fn mem2mem(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 14u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bits 12:13 - Channel priority level" ]
    pub fn pl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 10:11 - Memory size" ]
    pub fn msize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 10u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:9 - Peripheral size" ]
    pub fn psize(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 3;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bit 7 - Memory increment mode" ]
    pub fn minc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Peripheral increment mode" ]
    pub fn pinc(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Circular mode" ]
    pub fn circ(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Data transfer direction" ]
    pub fn dir(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Transfer error interrupt enable" ]
    pub fn teie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Half transfer interrupt enable" ]
    pub fn htie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transfer complete interrupt enable" ]
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 0 - Channel enable" ]
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
pub struct Cndtr7 {
    register: ::volatile_register::RW<u32>,
}

impl Cndtr7 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cndtr7R, &'w mut Cndtr7W) -> &'w mut Cndtr7W
    {
        let bits = self.register.read();
        let r = Cndtr7R { bits: bits };
        let mut w = Cndtr7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cndtr7R {
        Cndtr7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cndtr7W) -> &mut Cndtr7W
    {
        let mut w = Cndtr7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cndtr7R {
    bits: u32,
}

impl Cndtr7R {
    # [ doc = "Bits 0:15 - Number of data to transfer" ]
    pub fn ndt(&self) -> u16 {
        const MASK: u32 = 65535;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u16
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cndtr7W {
    bits: u32,
}

impl Cndtr7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cndtr7W { bits: 0u32 }
    }
    # [ doc = "Bits 0:15 - Number of data to transfer" ]
    pub fn ndt(&mut self, value: u16) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u16 = 65535;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cpar7 {
    register: ::volatile_register::RW<u32>,
}

impl Cpar7 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cpar7R, &'w mut Cpar7W) -> &'w mut Cpar7W
    {
        let bits = self.register.read();
        let r = Cpar7R { bits: bits };
        let mut w = Cpar7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cpar7R {
        Cpar7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cpar7W) -> &mut Cpar7W
    {
        let mut w = Cpar7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cpar7R {
    bits: u32,
}

impl Cpar7R {
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cpar7W {
    bits: u32,
}

impl Cpar7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cpar7W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Peripheral address" ]
    pub fn pa(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cmar7 {
    register: ::volatile_register::RW<u32>,
}

impl Cmar7 {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&Cmar7R, &'w mut Cmar7W) -> &'w mut Cmar7W
    {
        let bits = self.register.read();
        let r = Cmar7R { bits: bits };
        let mut w = Cmar7W { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> Cmar7R {
        Cmar7R { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut Cmar7W) -> &mut Cmar7W
    {
        let mut w = Cmar7W::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cmar7R {
    bits: u32,
}

impl Cmar7R {
    # [ doc = "Bits 0:31 - Memory address" ]
    pub fn ma(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct Cmar7W {
    bits: u32,
}

impl Cmar7W {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        Cmar7W { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Memory address" ]
    pub fn ma(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Cselr {
    register: ::volatile_register::RW<u32>,
}

impl Cselr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&CselrR, &'w mut CselrW) -> &'w mut CselrW
    {
        let bits = self.register.read();
        let r = CselrR { bits: bits };
        let mut w = CselrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> CselrR {
        CselrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut CselrW) -> &mut CselrW
    {
        let mut w = CselrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CselrR {
    bits: u32,
}

impl CselrR {
    # [ doc = "Bits 24:27 - DMA channel 7 selection" ]
    pub fn c7s(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 24u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 20:23 - DMA channel 6 selection" ]
    pub fn c6s(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 20u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 16:19 - DMA channel 5 selection" ]
    pub fn c5s(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 16u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 12:15 - DMA channel 4 selection" ]
    pub fn c4s(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 12u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 8:11 - DMA channel 3 selection" ]
    pub fn c3s(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 8u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 4:7 - DMA channel 2 selection" ]
    pub fn c2s(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 4u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
    # [ doc = "Bits 0:3 - DMA channel 1 selection" ]
    pub fn c1s(&self) -> u8 {
        const MASK: u32 = 15;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct CselrW {
    bits: u32,
}

impl CselrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        CselrW { bits: 0u32 }
    }
    # [ doc = "Bits 24:27 - DMA channel 7 selection" ]
    pub fn c7s(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 24u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 20:23 - DMA channel 6 selection" ]
    pub fn c6s(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 20u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 16:19 - DMA channel 5 selection" ]
    pub fn c5s(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 16u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 12:15 - DMA channel 4 selection" ]
    pub fn c4s(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 12u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 8:11 - DMA channel 3 selection" ]
    pub fn c3s(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 8u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 4:7 - DMA channel 2 selection" ]
    pub fn c2s(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 4u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
    # [ doc = "Bits 0:3 - DMA channel 1 selection" ]
    pub fn c1s(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 15;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
