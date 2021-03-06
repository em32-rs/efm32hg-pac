#[doc = "Reader of register IEN"]
pub type R = crate::R<u32, super::IEN>;
#[doc = "Writer for register IEN"]
pub type W = crate::W<u32, super::IEN>;
#[doc = "Register IEN `reset()`'s with value 0"]
impl crate::ResetValue for super::IEN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `VREGOSH`"]
pub type VREGOSH_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREGOSH`"]
pub struct VREGOSH_W<'a> {
    w: &'a mut W,
}
impl<'a> VREGOSH_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
#[doc = "Reader of field `VREGOSL`"]
pub type VREGOSL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `VREGOSL`"]
pub struct VREGOSL_W<'a> {
    w: &'a mut W,
}
impl<'a> VREGOSL_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - VREGO Sense High Interrupt Enable"]
    #[inline(always)]
    pub fn vregosh(&self) -> VREGOSH_R {
        VREGOSH_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - VREGO Sense Low Interrupt Enable"]
    #[inline(always)]
    pub fn vregosl(&self) -> VREGOSL_R {
        VREGOSL_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - VREGO Sense High Interrupt Enable"]
    #[inline(always)]
    pub fn vregosh(&mut self) -> VREGOSH_W {
        VREGOSH_W { w: self }
    }
    #[doc = "Bit 1 - VREGO Sense Low Interrupt Enable"]
    #[inline(always)]
    pub fn vregosl(&mut self) -> VREGOSL_W {
        VREGOSL_W { w: self }
    }
}
