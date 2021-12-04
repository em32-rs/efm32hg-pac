#[doc = "Register `CHALTS` writer"]
pub struct W(crate::W<CHALTS_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHALTS_SPEC>;
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
impl From<crate::W<CHALTS_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHALTS_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0ALTS` writer - Channel 0 Alternate Structure Set"]
pub struct CH0ALTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0ALTS_W<'a> {
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
#[doc = "Field `CH1ALTS` writer - Channel 1 Alternate Structure Set"]
pub struct CH1ALTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1ALTS_W<'a> {
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
#[doc = "Field `CH2ALTS` writer - Channel 2 Alternate Structure Set"]
pub struct CH2ALTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2ALTS_W<'a> {
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
#[doc = "Field `CH3ALTS` writer - Channel 3 Alternate Structure Set"]
pub struct CH3ALTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3ALTS_W<'a> {
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
#[doc = "Field `CH4ALTS` writer - Channel 4 Alternate Structure Set"]
pub struct CH4ALTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4ALTS_W<'a> {
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
#[doc = "Field `CH5ALTS` writer - Channel 5 Alternate Structure Set"]
pub struct CH5ALTS_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5ALTS_W<'a> {
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
    #[doc = "Bit 0 - Channel 0 Alternate Structure Set"]
    #[inline(always)]
    pub fn ch0alts(&mut self) -> CH0ALTS_W {
        CH0ALTS_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Alternate Structure Set"]
    #[inline(always)]
    pub fn ch1alts(&mut self) -> CH1ALTS_W {
        CH1ALTS_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Alternate Structure Set"]
    #[inline(always)]
    pub fn ch2alts(&mut self) -> CH2ALTS_W {
        CH2ALTS_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Alternate Structure Set"]
    #[inline(always)]
    pub fn ch3alts(&mut self) -> CH3ALTS_W {
        CH3ALTS_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Alternate Structure Set"]
    #[inline(always)]
    pub fn ch4alts(&mut self) -> CH4ALTS_W {
        CH4ALTS_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Alternate Structure Set"]
    #[inline(always)]
    pub fn ch5alts(&mut self) -> CH5ALTS_W {
        CH5ALTS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Alternate Set Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chalts](index.html) module"]
pub struct CHALTS_SPEC;
impl crate::RegisterSpec for CHALTS_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chalts::W](W) writer structure"]
impl crate::Writable for CHALTS_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHALTS to value 0"]
impl crate::Resettable for CHALTS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
