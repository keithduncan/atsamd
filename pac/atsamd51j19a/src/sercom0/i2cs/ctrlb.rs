#[doc = "Reader of register CTRLB"]
pub type R = crate::R<u32, super::CTRLB>;
#[doc = "Writer for register CTRLB"]
pub type W = crate::W<u32, super::CTRLB>;
#[doc = "Register CTRLB `reset()`'s with value 0"]
impl crate::ResetValue for super::CTRLB {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SMEN`"]
pub type SMEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SMEN`"]
pub struct SMEN_W<'a> {
    w: &'a mut W,
}
impl<'a> SMEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `GCMD`"]
pub type GCMD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GCMD`"]
pub struct GCMD_W<'a> {
    w: &'a mut W,
}
impl<'a> GCMD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `AACKEN`"]
pub type AACKEN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `AACKEN`"]
pub struct AACKEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AACKEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `AMODE`"]
pub type AMODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `AMODE`"]
pub struct AMODE_W<'a> {
    w: &'a mut W,
}
impl<'a> AMODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Write proxy for field `CMD`"]
pub struct CMD_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 16)) | (((value as u32) & 0x03) << 16);
        self.w
    }
}
#[doc = "Reader of field `ACKACT`"]
pub type ACKACT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ACKACT`"]
pub struct ACKACT_W<'a> {
    w: &'a mut W,
}
impl<'a> ACKACT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
impl R {
    #[doc = "Bit 8 - Smart Mode Enable"]
    #[inline(always)]
    pub fn smen(&self) -> SMEN_R {
        SMEN_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - PMBus Group Command"]
    #[inline(always)]
    pub fn gcmd(&self) -> GCMD_R {
        GCMD_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - Automatic Address Acknowledge"]
    #[inline(always)]
    pub fn aacken(&self) -> AACKEN_R {
        AACKEN_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    pub fn amode(&self) -> AMODE_R {
        AMODE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 18 - Acknowledge Action"]
    #[inline(always)]
    pub fn ackact(&self) -> ACKACT_R {
        ACKACT_R::new(((self.bits >> 18) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 8 - Smart Mode Enable"]
    #[inline(always)]
    pub fn smen(&mut self) -> SMEN_W {
        SMEN_W { w: self }
    }
    #[doc = "Bit 9 - PMBus Group Command"]
    #[inline(always)]
    pub fn gcmd(&mut self) -> GCMD_W {
        GCMD_W { w: self }
    }
    #[doc = "Bit 10 - Automatic Address Acknowledge"]
    #[inline(always)]
    pub fn aacken(&mut self) -> AACKEN_W {
        AACKEN_W { w: self }
    }
    #[doc = "Bits 14:15 - Address Mode"]
    #[inline(always)]
    pub fn amode(&mut self) -> AMODE_W {
        AMODE_W { w: self }
    }
    #[doc = "Bits 16:17 - Command"]
    #[inline(always)]
    pub fn cmd(&mut self) -> CMD_W {
        CMD_W { w: self }
    }
    #[doc = "Bit 18 - Acknowledge Action"]
    #[inline(always)]
    pub fn ackact(&mut self) -> ACKACT_W {
        ACKACT_W { w: self }
    }
}
