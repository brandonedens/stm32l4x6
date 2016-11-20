# [ doc = "Single Wire Protocol Master Interface" ]
# [ repr ( C ) ]
pub struct Swpmi {
    # [ doc = "0x00 - SWPMI Configuration/Control register" ]
    pub cr: Cr,
    # [ doc = "0x04 - SWPMI Bitrate register" ]
    pub brr: Brr,
    _reserved0: [u8; 4usize],
    # [ doc = "0x0c - SWPMI Interrupt and Status register" ]
    pub isr: Isr,
    # [ doc = "0x10 - SWPMI Interrupt Flag Clear register" ]
    pub icr: Icr,
    # [ doc = "0x14 - SWPMI Interrupt Enable register" ]
    pub ier: Ier,
    # [ doc = "0x18 - SWPMI Receive Frame Length register" ]
    pub rfl: Rfl,
    # [ doc = "0x1c - SWPMI Transmit data register" ]
    pub tdr: Tdr,
    # [ doc = "0x20 - SWPMI Receive data register" ]
    pub rdr: Rdr,
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
    # [ doc = "Bit 0 - Reception DMA enable" ]
    pub fn rxdma(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transmission DMA enable" ]
    pub fn txdma(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Reception buffering mode" ]
    pub fn rxmode(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Transmission buffering mode" ]
    pub fn txmode(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Loopback mode enable" ]
    pub fn lpbk(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Single wire protocol master interface enable" ]
    pub fn swpme(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - Single wire protocol master interface deactivate" ]
    pub fn deact(&self) -> bool {
        const OFFSET: u8 = 10u8;
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
    # [ doc = "Bit 0 - Reception DMA enable" ]
    pub fn rxdma(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transmission DMA enable" ]
    pub fn txdma(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Reception buffering mode" ]
    pub fn rxmode(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Transmission buffering mode" ]
    pub fn txmode(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Loopback mode enable" ]
    pub fn lpbk(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Single wire protocol master interface enable" ]
    pub fn swpme(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - Single wire protocol master interface deactivate" ]
    pub fn deact(&mut self, value: bool) -> &mut Self {
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
pub struct Brr {
    register: ::volatile_register::RW<u32>,
}

impl Brr {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&BrrR, &'w mut BrrW) -> &'w mut BrrW
    {
        let bits = self.register.read();
        let r = BrrR { bits: bits };
        let mut w = BrrW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> BrrR {
        BrrR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut BrrW) -> &mut BrrW
    {
        let mut w = BrrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BrrR {
    bits: u32,
}

impl BrrR {
    # [ doc = "Bits 0:5 - Bitrate prescaler" ]
    pub fn br(&self) -> u8 {
        const MASK: u32 = 63;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct BrrW {
    bits: u32,
}

impl BrrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        BrrW { bits: 1u32 }
    }
    # [ doc = "Bits 0:5 - Bitrate prescaler" ]
    pub fn br(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 63;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
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
    # [ doc = "Bit 0 - Receive buffer full flag" ]
    pub fn rxbff(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transmit buffer empty flag" ]
    pub fn txbef(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Receive CRC error flag" ]
    pub fn rxberf(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Receive overrun error flag" ]
    pub fn rxovrf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Transmit underrun error flag" ]
    pub fn txunrf(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Receive data register not empty" ]
    pub fn rxne(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Transmit data register empty" ]
    pub fn txe(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transfer complete flag" ]
    pub fn tcf(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Slave resume flag" ]
    pub fn srf(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 9 - SUSPEND flag" ]
    pub fn susp(&self) -> bool {
        const OFFSET: u8 = 9u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 10 - DEACTIVATED flag" ]
    pub fn deactf(&self) -> bool {
        const OFFSET: u8 = 10u8;
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
        IsrW { bits: 706u32 }
    }
    # [ doc = "Bit 0 - Receive buffer full flag" ]
    pub fn rxbff(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transmit buffer empty flag" ]
    pub fn txbef(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Receive CRC error flag" ]
    pub fn rxberf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Receive overrun error flag" ]
    pub fn rxovrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Transmit underrun error flag" ]
    pub fn txunrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Receive data register not empty" ]
    pub fn rxne(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Transmit data register empty" ]
    pub fn txe(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transfer complete flag" ]
    pub fn tcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Slave resume flag" ]
    pub fn srf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 9 - SUSPEND flag" ]
    pub fn susp(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 9u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 10 - DEACTIVATED flag" ]
    pub fn deactf(&mut self, value: bool) -> &mut Self {
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
pub struct Icr {
    register: ::volatile_register::WO<u32>,
}

impl Icr {
    pub fn write<F>(&self, f: F)
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
    # [ doc = "Bit 0 - Clear receive buffer full flag" ]
    pub fn crxbff(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Clear transmit buffer empty flag" ]
    pub fn ctxbef(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Clear receive CRC error flag" ]
    pub fn crxberf(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Clear receive overrun error flag" ]
    pub fn crxovrf(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Clear transmit underrun error flag" ]
    pub fn ctxunrf(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Clear transfer complete flag" ]
    pub fn ctcf(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Clear slave resume flag" ]
    pub fn csrf(&self) -> bool {
        const OFFSET: u8 = 8u8;
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
    # [ doc = "Bit 0 - Clear receive buffer full flag" ]
    pub fn crxbff(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Clear transmit buffer empty flag" ]
    pub fn ctxbef(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Clear receive CRC error flag" ]
    pub fn crxberf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Clear receive overrun error flag" ]
    pub fn crxovrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Clear transmit underrun error flag" ]
    pub fn ctxunrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Clear transfer complete flag" ]
    pub fn ctcf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Clear slave resume flag" ]
    pub fn csrf(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Ier {
    register: ::volatile_register::RW<u32>,
}

impl Ier {
    pub fn modify<F>(&mut self, f: F)
        where for<'w> F: FnOnce(&IerR, &'w mut IerW) -> &'w mut IerW
    {
        let bits = self.register.read();
        let r = IerR { bits: bits };
        let mut w = IerW { bits: bits };
        f(&r, &mut w);
        self.register.write(w.bits);
    }
    pub fn read(&self) -> IerR {
        IerR { bits: self.register.read() }
    }
    pub fn write<F>(&mut self, f: F)
        where F: FnOnce(&mut IerW) -> &mut IerW
    {
        let mut w = IerW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IerR {
    bits: u32,
}

impl IerR {
    # [ doc = "Bit 0 - Receive buffer full interrupt enable" ]
    pub fn rxbfie(&self) -> bool {
        const OFFSET: u8 = 0u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 1 - Transmit buffer empty interrupt enable" ]
    pub fn txbeie(&self) -> bool {
        const OFFSET: u8 = 1u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 2 - Receive CRC error interrupt enable" ]
    pub fn rxberie(&self) -> bool {
        const OFFSET: u8 = 2u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 3 - Receive overrun error interrupt enable" ]
    pub fn rxovrie(&self) -> bool {
        const OFFSET: u8 = 3u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 4 - Transmit underrun error interrupt enable" ]
    pub fn txunrie(&self) -> bool {
        const OFFSET: u8 = 4u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 5 - Receive interrupt enable" ]
    pub fn rie(&self) -> bool {
        const OFFSET: u8 = 5u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 6 - Transmit interrupt enable" ]
    pub fn tie(&self) -> bool {
        const OFFSET: u8 = 6u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 7 - Transmit complete interrupt enable" ]
    pub fn tcie(&self) -> bool {
        const OFFSET: u8 = 7u8;
        self.bits & (1 << OFFSET) != 0
    }
    # [ doc = "Bit 8 - Slave resume interrupt enable" ]
    pub fn srie(&self) -> bool {
        const OFFSET: u8 = 8u8;
        self.bits & (1 << OFFSET) != 0
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct IerW {
    bits: u32,
}

impl IerW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        IerW { bits: 0u32 }
    }
    # [ doc = "Bit 0 - Receive buffer full interrupt enable" ]
    pub fn rxbfie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 0u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 1 - Transmit buffer empty interrupt enable" ]
    pub fn txbeie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 1u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 2 - Receive CRC error interrupt enable" ]
    pub fn rxberie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 2u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 3 - Receive overrun error interrupt enable" ]
    pub fn rxovrie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 3u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 4 - Transmit underrun error interrupt enable" ]
    pub fn txunrie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 4u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 5 - Receive interrupt enable" ]
    pub fn rie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 5u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 6 - Transmit interrupt enable" ]
    pub fn tie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 6u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 7 - Transmit complete interrupt enable" ]
    pub fn tcie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 7u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
    # [ doc = "Bit 8 - Slave resume interrupt enable" ]
    pub fn srie(&mut self, value: bool) -> &mut Self {
        const OFFSET: u8 = 8u8;
        if value {
            self.bits |= 1 << OFFSET;
        } else {
            self.bits &= !(1 << OFFSET);
        }
        self
    }
}

# [ repr ( C ) ]
pub struct Rfl {
    register: ::volatile_register::RO<u32>,
}

impl Rfl {
    pub fn read(&self) -> RflR {
        RflR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RflR {
    bits: u32,
}

impl RflR {
    # [ doc = "Bits 0:4 - Receive frame length" ]
    pub fn rfl(&self) -> u8 {
        const MASK: u32 = 31;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u8
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RflW {
    bits: u32,
}

impl RflW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        RflW { bits: 0u32 }
    }
    # [ doc = "Bits 0:4 - Receive frame length" ]
    pub fn rfl(&mut self, value: u8) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u8 = 31;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Tdr {
    register: ::volatile_register::WO<u32>,
}

impl Tdr {
    pub fn write<F>(&self, f: F)
        where F: FnOnce(&mut TdrW) -> &mut TdrW
    {
        let mut w = TdrW::reset_value();
        f(&mut w);
        self.register.write(w.bits);
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct TdrR {
    bits: u32,
}

impl TdrR {
    # [ doc = "Bits 0:31 - Transmit data" ]
    pub fn td(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct TdrW {
    bits: u32,
}

impl TdrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        TdrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - Transmit data" ]
    pub fn td(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}

# [ repr ( C ) ]
pub struct Rdr {
    register: ::volatile_register::RO<u32>,
}

impl Rdr {
    pub fn read(&self) -> RdrR {
        RdrR { bits: self.register.read() }
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RdrR {
    bits: u32,
}

impl RdrR {
    # [ doc = "Bits 0:31 - received data" ]
    pub fn rd(&self) -> u32 {
        const MASK: u32 = 4294967295;
        const OFFSET: u8 = 0u8;
        ((self.bits >> OFFSET) & MASK) as u32
    }
}

# [ derive ( Clone , Copy ) ]
# [ repr ( C ) ]
pub struct RdrW {
    bits: u32,
}

impl RdrW {
    # [ doc = r" Reset value" ]
    pub fn reset_value() -> Self {
        RdrW { bits: 0u32 }
    }
    # [ doc = "Bits 0:31 - received data" ]
    pub fn rd(&mut self, value: u32) -> &mut Self {
        const OFFSET: u8 = 0u8;
        const MASK: u32 = 4294967295;
        self.bits &= !((MASK as u32) << OFFSET);
        self.bits |= ((value & MASK) as u32) << OFFSET;
        self
    }
}
