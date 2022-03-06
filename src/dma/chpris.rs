#[doc = "Register `CHPRIS` writer"]
pub struct W(crate::W<CHPRIS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHPRIS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CHPRIS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHPRIS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0PRIS` writer - Channel 0 High Priority Set"]
pub struct CH0PRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0PRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `CH1PRIS` writer - Channel 1 High Priority Set"]
pub struct CH1PRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1PRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
#[doc = "Field `CH2PRIS` writer - Channel 2 High Priority Set"]
pub struct CH2PRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2PRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `CH3PRIS` writer - Channel 3 High Priority Set"]
pub struct CH3PRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3PRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `CH4PRIS` writer - Channel 4 High Priority Set"]
pub struct CH4PRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4PRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `CH5PRIS` writer - Channel 5 High Priority Set"]
pub struct CH5PRIS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5PRIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - Channel 0 High Priority Set"]
    #[inline(always)]
    pub fn ch0pris(&mut self) -> CH0PRIS_W {
        CH0PRIS_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 High Priority Set"]
    #[inline(always)]
    pub fn ch1pris(&mut self) -> CH1PRIS_W {
        CH1PRIS_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 High Priority Set"]
    #[inline(always)]
    pub fn ch2pris(&mut self) -> CH2PRIS_W {
        CH2PRIS_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 High Priority Set"]
    #[inline(always)]
    pub fn ch3pris(&mut self) -> CH3PRIS_W {
        CH3PRIS_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 High Priority Set"]
    #[inline(always)]
    pub fn ch4pris(&mut self) -> CH4PRIS_W {
        CH4PRIS_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 High Priority Set"]
    #[inline(always)]
    pub fn ch5pris(&mut self) -> CH5PRIS_W {
        CH5PRIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Priority Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chpris](index.html) module"]
pub struct CHPRIS_SPEC;
impl crate::RegisterSpec for CHPRIS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chpris::W](W) writer structure"]
impl crate::Writable for CHPRIS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHPRIS to value 0"]
impl crate::Resettable for CHPRIS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
