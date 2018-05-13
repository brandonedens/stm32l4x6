#[doc = r" Value read from the register"]
pub struct R {
    bits: u32,
}
#[doc = r" Value to write to the register"]
pub struct W {
    bits: u32,
}
impl super::WRP1AR {
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
pub struct WRP1A_STRTR {
    bits: u8,
}
impl WRP1A_STRTR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Value of the field"]
pub struct WRP1A_ENDR {
    bits: u8,
}
impl WRP1A_ENDR {
    #[doc = r" Value of the field as raw bits"]
    #[inline]
    pub fn bits(&self) -> u8 {
        self.bits
    }
}
#[doc = r" Proxy"]
pub struct _WRP1A_STRTW<'a> {
    w: &'a mut W,
}
impl<'a> _WRP1A_STRTW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 0;
        self.w.bits &= !((MASK as u32) << OFFSET);
        self.w.bits |= ((value & MASK) as u32) << OFFSET;
        self.w
    }
}
#[doc = r" Proxy"]
pub struct _WRP1A_ENDW<'a> {
    w: &'a mut W,
}
impl<'a> _WRP1A_ENDW<'a> {
    #[doc = r" Writes raw bits to the field"]
    #[inline]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        const MASK: u8 = 255;
        const OFFSET: u8 = 16;
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
    #[doc = "Bits 0:7 - Bank 1 WRP first area \u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{2020}\u{e2}\u{20ac}\u{2122}\u{c3}\u{192}\u{e2}\u{20ac} \u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{e2}\u{201e}\u{a2}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{c5}\u{a1}\u{c3}\u{192}\u{e2}\u{20ac}\u{161}\u{c3}\u{201a}\u{c2}\u{a2}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{2020}\u{e2}\u{20ac}\u{2122}\u{c3}\u{192}\u{e2}\u{20ac}\u{161}\u{c3}\u{201a}\u{c2}\u{a2}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{201a}\u{c2}\u{a2}\u{c3}\u{192}\u{c2}\u{a2}\u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{c5}\u{a1}\u{c3}\u{201a}\u{c2}\u{ac}\u{c3}\u{192}\u{e2}\u{20ac}\u{a6}\u{c3}\u{201a}\u{c2}\u{a1}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{c5}\u{a1}\u{c3}\u{192}\u{e2}\u{20ac}\u{161}\u{c3}\u{201a}\u{c2}\u{ac}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{2020}\u{e2}\u{20ac}\u{2122}\u{c3}\u{192}\u{c2}\u{a2}\u{c3}\u{a2}\u{e2}\u{20ac}\u{161}\u{c2}\u{ac}\u{c3}\u{201a}\u{c2}\u{a6}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{201a}\u{c2}\u{a2}\u{c3}\u{192}\u{c2}\u{a2}\u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{c5}\u{a1}\u{c3}\u{201a}\u{c2}\u{ac}\u{c3}\u{192}\u{e2}\u{20ac}\u{a6}\u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{c5}\u{201c}A\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{2020}\u{e2}\u{20ac}\u{2122}\u{c3}\u{192}\u{e2}\u{20ac} \u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{e2}\u{201e}\u{a2}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{c5}\u{a1}\u{c3}\u{192}\u{e2}\u{20ac}\u{161}\u{c3}\u{201a}\u{c2}\u{a2}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{2020}\u{e2}\u{20ac}\u{2122}\u{c3}\u{192}\u{e2}\u{20ac}\u{161}\u{c3}\u{201a}\u{c2}\u{a2}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{201a}\u{c2}\u{a2}\u{c3}\u{192}\u{c2}\u{a2}\u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{c5}\u{a1}\u{c3}\u{201a}\u{c2}\u{ac}\u{c3}\u{192}\u{e2}\u{20ac}\u{a6}\u{c3}\u{201a}\u{c2}\u{a1}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{c5}\u{a1}\u{c3}\u{192}\u{e2}\u{20ac}\u{161}\u{c3}\u{201a}\u{c2}\u{ac} start offset"]
    #[inline]
    pub fn wrp1a_strt(&self) -> WRP1A_STRTR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 0;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WRP1A_STRTR { bits }
    }
    #[doc = "Bits 16:23 - Bank 1 WRP first area A end offset"]
    #[inline]
    pub fn wrp1a_end(&self) -> WRP1A_ENDR {
        let bits = {
            const MASK: u8 = 255;
            const OFFSET: u8 = 16;
            ((self.bits >> OFFSET) & MASK as u32) as u8
        };
        WRP1A_ENDR { bits }
    }
}
impl W {
    #[doc = r" Reset value of the register"]
    #[inline]
    pub fn reset_value() -> W {
        W { bits: 4278255360 }
    }
    #[doc = r" Writes raw bits to the register"]
    #[inline]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
    #[doc = "Bits 0:7 - Bank 1 WRP first area \u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{2020}\u{e2}\u{20ac}\u{2122}\u{c3}\u{192}\u{e2}\u{20ac} \u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{e2}\u{201e}\u{a2}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{c5}\u{a1}\u{c3}\u{192}\u{e2}\u{20ac}\u{161}\u{c3}\u{201a}\u{c2}\u{a2}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{2020}\u{e2}\u{20ac}\u{2122}\u{c3}\u{192}\u{e2}\u{20ac}\u{161}\u{c3}\u{201a}\u{c2}\u{a2}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{201a}\u{c2}\u{a2}\u{c3}\u{192}\u{c2}\u{a2}\u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{c5}\u{a1}\u{c3}\u{201a}\u{c2}\u{ac}\u{c3}\u{192}\u{e2}\u{20ac}\u{a6}\u{c3}\u{201a}\u{c2}\u{a1}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{c5}\u{a1}\u{c3}\u{192}\u{e2}\u{20ac}\u{161}\u{c3}\u{201a}\u{c2}\u{ac}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{2020}\u{e2}\u{20ac}\u{2122}\u{c3}\u{192}\u{c2}\u{a2}\u{c3}\u{a2}\u{e2}\u{20ac}\u{161}\u{c2}\u{ac}\u{c3}\u{201a}\u{c2}\u{a6}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{201a}\u{c2}\u{a2}\u{c3}\u{192}\u{c2}\u{a2}\u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{c5}\u{a1}\u{c3}\u{201a}\u{c2}\u{ac}\u{c3}\u{192}\u{e2}\u{20ac}\u{a6}\u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{c5}\u{201c}A\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{2020}\u{e2}\u{20ac}\u{2122}\u{c3}\u{192}\u{e2}\u{20ac} \u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{e2}\u{201e}\u{a2}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{c5}\u{a1}\u{c3}\u{192}\u{e2}\u{20ac}\u{161}\u{c3}\u{201a}\u{c2}\u{a2}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{2020}\u{e2}\u{20ac}\u{2122}\u{c3}\u{192}\u{e2}\u{20ac}\u{161}\u{c3}\u{201a}\u{c2}\u{a2}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{201a}\u{c2}\u{a2}\u{c3}\u{192}\u{c2}\u{a2}\u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{c5}\u{a1}\u{c3}\u{201a}\u{c2}\u{ac}\u{c3}\u{192}\u{e2}\u{20ac}\u{a6}\u{c3}\u{201a}\u{c2}\u{a1}\u{c3}\u{192}\u{c6}\u{2019}\u{c3}\u{a2}\u{e2}\u{201a}\u{ac}\u{c5}\u{a1}\u{c3}\u{192}\u{e2}\u{20ac}\u{161}\u{c3}\u{201a}\u{c2}\u{ac} start offset"]
    #[inline]
    pub fn wrp1a_strt(&mut self) -> _WRP1A_STRTW {
        _WRP1A_STRTW { w: self }
    }
    #[doc = "Bits 16:23 - Bank 1 WRP first area A end offset"]
    #[inline]
    pub fn wrp1a_end(&mut self) -> _WRP1A_ENDW {
        _WRP1A_ENDW { w: self }
    }
}
