#[doc = "Reader of register DBGCTRL"]
pub type R = crate::R<u8, super::DBGCTRL>;
#[doc = "Writer for register DBGCTRL"]
pub type W = crate::W<u8, super::DBGCTRL>;
#[doc = "Register DBGCTRL `reset()`'s with value 0"]
impl crate::ResetValue for super::DBGCTRL {
    type Type = u8;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DBGRUN`"]
pub type DBGRUN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DBGRUN`"]
pub struct DBGRUN_W<'a> {
    w: &'a mut W,
}
impl<'a> DBGRUN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | ((value as u8) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Debug Run Mode"]
    #[inline(always)]
    pub fn dbgrun(&self) -> DBGRUN_R {
        DBGRUN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Debug Run Mode"]
    #[inline(always)]
    pub fn dbgrun(&mut self) -> DBGRUN_W {
        DBGRUN_W { w: self }
    }
}
