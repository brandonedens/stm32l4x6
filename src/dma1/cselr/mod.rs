#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::CSELR {
    #[doc = r" Modifies the contents of the register"]
    #[inline]
    pub fn modify<F>(&self, f: F)
    where
        for<'w> F: FnOnce(&R, &'w mut W) -> &'w mut W,
    {
        let bits = self.register.get();
        let r = R { bits: bits };
        let mut w = W { bits: bits };
        f(&r, &mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Reads the contents of the register"]
    #[inline]
    pub fn read(&self) -> R {
        R {
            bits: self.register.get(),
        }
    }
    #[doc = r" Writes to the register"]
    #[inline]
    pub fn write<F>(&self, f: F)
    where
        F: FnOnce(&mut W) -> &mut W,
    {
        let mut w = W::reset_value();
        f(&mut w);
        self.register.set(w.bits);
    }
    #[doc = r" Writes the reset value to the register"]
    #[inline]
    pub fn reset(&self) {
        self.write(|w| w)
    }
}
#[doc = r" Value of the field"]
pub struct C7SR {
    bits: u8,
}
impl C7SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct C6SR {
    bits: u8,
}
impl C6SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct C5SR {
    bits: u8,
}
impl C5SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct C4SR {
    bits: u8,
}
impl C4SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct C3SR {
    bits: u8,
}
impl C3SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct C2SR {
    bits: u8,
}
impl C2SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct C1SR {
    bits: u8,
}
impl C1SR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _C7SW<'a> {
    w: &'a mut W,
}
impl<'a> _C7SW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 24;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _C6SW<'a> {
    w: &'a mut W,
}
impl<'a> _C6SW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 20;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _C5SW<'a> {
    w: &'a mut W,
}
impl<'a> _C5SW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 16;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _C4SW<'a> {
    w: &'a mut W,
}
impl<'a> _C4SW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 12;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _C3SW<'a> {
    w: &'a mut W,
}
impl<'a> _C3SW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 8;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _C2SW<'a> {
    w: &'a mut W,
}
impl<'a> _C2SW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 4;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _C1SW<'a> {
    w: &'a mut W,
}
impl<'a> _C1SW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 15;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
impl R {
    #[doc = r" Value of the register as raw bits"]
    #[inline]
    pub fn bits(&self) -> u32 {
        self.bits
    }
    #[doc = "Bits 24:27 - DMA channel 7 selection"]
    #[inline]
    pub fn c7s(&self) -> C7SR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 24;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        C7SR { bits }
    }
    #[doc = "Bits 20:23 - DMA channel 6 selection"]
    #[inline]
    pub fn c6s(&self) -> C6SR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 20;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        C6SR { bits }
    }
    #[doc = "Bits 16:19 - DMA channel 5 selection"]
    #[inline]
    pub fn c5s(&self) -> C5SR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        C5SR { bits }
    }
    #[doc = "Bits 12:15 - DMA channel 4 selection"]
    #[inline]
    pub fn c4s(&self) -> C4SR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 12;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        C4SR { bits }
    }
    #[doc = "Bits 8:11 - DMA channel 3 selection"]
    #[inline]
    pub fn c3s(&self) -> C3SR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 8;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        C3SR { bits }
    }
    #[doc = "Bits 4:7 - DMA channel 2 selection"]
    #[inline]
    pub fn c2s(&self) -> C2SR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 4;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        C2SR { bits }
    }
    #[doc = "Bits 0:3 - DMA channel 1 selection"]
    #[inline]
    pub fn c1s(&self) -> C1SR {
        let bits = {
            const MASK: u8 = 15;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        C1SR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 0 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 24:27 - DMA channel 7 selection"]
    #[inline]
    pub fn c7s(&mut self) -> _C7SW {
        _C7SW { w: self }
    }
    #[doc = "Bits 20:23 - DMA channel 6 selection"]
    #[inline]
    pub fn c6s(&mut self) -> _C6SW {
        _C6SW { w: self }
    }
    #[doc = "Bits 16:19 - DMA channel 5 selection"]
    #[inline]
    pub fn c5s(&mut self) -> _C5SW {
        _C5SW { w: self }
    }
    #[doc = "Bits 12:15 - DMA channel 4 selection"]
    #[inline]
    pub fn c4s(&mut self) -> _C4SW {
        _C4SW { w: self }
    }
    #[doc = "Bits 8:11 - DMA channel 3 selection"]
    #[inline]
    pub fn c3s(&mut self) -> _C3SW {
        _C3SW { w: self }
    }
    #[doc = "Bits 4:7 - DMA channel 2 selection"]
    #[inline]
    pub fn c2s(&mut self) -> _C2SW {
        _C2SW { w: self }
    }
    #[doc = "Bits 0:3 - DMA channel 1 selection"]
    #[inline]
    pub fn c1s(&mut self) -> _C1SW {
        _C1SW { w: self }
    }
}
