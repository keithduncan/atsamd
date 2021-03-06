#[doc = "Reader of register PRESCBUF"]
pub type R = crate::R<u8, super::PRESCBUF>;
#[doc = "Writer for register PRESCBUF"]
pub type W = crate::W<u8, super::PRESCBUF>;
#[doc = "Register PRESCBUF `reset()`'s with value 0"]
impl crate::ResetValue for super::PRESCBUF {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Prescaler Buffer Value\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PRESCBUF_A {
    #[doc = "0: No division"]
    DIV1,
    #[doc = "1: Divide by 2"]
    DIV2,
    #[doc = "2: Divide by 4"]
    DIV4,
    #[doc = "3: Divide by 8"]
    DIV8,
    #[doc = "4: Divide by 16"]
    DIV16,
    #[doc = "5: Divide by 32"]
    DIV32,
    #[doc = "6: Divide by 64"]
    DIV64,
    #[doc = "7: Divide by 128"]
    DIV128,
    #[doc = "8: Divide by 256"]
    DIV256,
    #[doc = "9: Divide by 512"]
    DIV512,
    #[doc = "10: Divide by 1024"]
    DIV1024,
}
impl From<PRESCBUF_A> for u8 {
    #[inline(always)]
    fn from(variant: PRESCBUF_A) -> Self {
        match variant {
            PRESCBUF_A::DIV1 => 0,
            PRESCBUF_A::DIV2 => 1,
            PRESCBUF_A::DIV4 => 2,
            PRESCBUF_A::DIV8 => 3,
            PRESCBUF_A::DIV16 => 4,
            PRESCBUF_A::DIV32 => 5,
            PRESCBUF_A::DIV64 => 6,
            PRESCBUF_A::DIV128 => 7,
            PRESCBUF_A::DIV256 => 8,
            PRESCBUF_A::DIV512 => 9,
            PRESCBUF_A::DIV1024 => 10,
        }
    }
}
#[doc = "Reader of field `PRESCBUF`"]
pub type PRESCBUF_R = crate::R<u8, PRESCBUF_A>;
impl PRESCBUF_R {
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> crate::Variant<u8, PRESCBUF_A> {
        use crate::Variant::*;
        match self.bits {
            0 => Val(PRESCBUF_A::DIV1),
            1 => Val(PRESCBUF_A::DIV2),
            2 => Val(PRESCBUF_A::DIV4),
            3 => Val(PRESCBUF_A::DIV8),
            4 => Val(PRESCBUF_A::DIV16),
            5 => Val(PRESCBUF_A::DIV32),
            6 => Val(PRESCBUF_A::DIV64),
            7 => Val(PRESCBUF_A::DIV128),
            8 => Val(PRESCBUF_A::DIV256),
            9 => Val(PRESCBUF_A::DIV512),
            10 => Val(PRESCBUF_A::DIV1024),
            i => Res(i),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PRESCBUF_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PRESCBUF_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PRESCBUF_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PRESCBUF_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PRESCBUF_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == PRESCBUF_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == PRESCBUF_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == PRESCBUF_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == PRESCBUF_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == PRESCBUF_A::DIV512
    }
    #[doc = "Checks if the value of the field is `DIV1024`"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == PRESCBUF_A::DIV1024
    }
}
#[doc = "Write proxy for field `PRESCBUF`"]
pub struct PRESCBUF_W<'a> {
    w: &'a mut W,
}
impl<'a> PRESCBUF_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: PRESCBUF_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No division"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV1)
    }
    #[doc = "Divide by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV2)
    }
    #[doc = "Divide by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV4)
    }
    #[doc = "Divide by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV8)
    }
    #[doc = "Divide by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV16)
    }
    #[doc = "Divide by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV32)
    }
    #[doc = "Divide by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV64)
    }
    #[doc = "Divide by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV128)
    }
    #[doc = "Divide by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV256)
    }
    #[doc = "Divide by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV512)
    }
    #[doc = "Divide by 1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut W {
        self.variant(PRESCBUF_A::DIV1024)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u8) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3 - Prescaler Buffer Value"]
    #[inline(always)]
    pub fn prescbuf(&self) -> PRESCBUF_R {
        PRESCBUF_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3 - Prescaler Buffer Value"]
    #[inline(always)]
    pub fn prescbuf(&mut self) -> PRESCBUF_W {
        PRESCBUF_W { w: self }
    }
}
