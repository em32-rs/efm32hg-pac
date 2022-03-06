#[doc = "Register `CHREQMASKC` writer"]
pub struct W(crate::W<CHREQMASKC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CHREQMASKC_SPEC>;
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
impl From<crate::W<CHREQMASKC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CHREQMASKC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH0REQMASKC` writer - Channel 0 Request Mask Clear"]
pub struct CH0REQMASKC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0REQMASKC_W<'a> {
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
#[doc = "Field `CH1REQMASKC` writer - Channel 1 Request Mask Clear"]
pub struct CH1REQMASKC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH1REQMASKC_W<'a> {
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
#[doc = "Field `CH2REQMASKC` writer - Channel 2 Request Mask Clear"]
pub struct CH2REQMASKC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH2REQMASKC_W<'a> {
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
#[doc = "Field `CH3REQMASKC` writer - Channel 3 Request Mask Clear"]
pub struct CH3REQMASKC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH3REQMASKC_W<'a> {
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
#[doc = "Field `CH4REQMASKC` writer - Channel 4 Request Mask Clear"]
pub struct CH4REQMASKC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH4REQMASKC_W<'a> {
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
#[doc = "Field `CH5REQMASKC` writer - Channel 5 Request Mask Clear"]
pub struct CH5REQMASKC_W<'a> {
    w: &'a mut W,
}
impl<'a> CH5REQMASKC_W<'a> {
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
    #[doc = "Bit 0 - Channel 0 Request Mask Clear"]
    #[inline(always)]
    pub fn ch0reqmaskc(&mut self) -> CH0REQMASKC_W {
        CH0REQMASKC_W { w: self }
    }
    #[doc = "Bit 1 - Channel 1 Request Mask Clear"]
    #[inline(always)]
    pub fn ch1reqmaskc(&mut self) -> CH1REQMASKC_W {
        CH1REQMASKC_W { w: self }
    }
    #[doc = "Bit 2 - Channel 2 Request Mask Clear"]
    #[inline(always)]
    pub fn ch2reqmaskc(&mut self) -> CH2REQMASKC_W {
        CH2REQMASKC_W { w: self }
    }
    #[doc = "Bit 3 - Channel 3 Request Mask Clear"]
    #[inline(always)]
    pub fn ch3reqmaskc(&mut self) -> CH3REQMASKC_W {
        CH3REQMASKC_W { w: self }
    }
    #[doc = "Bit 4 - Channel 4 Request Mask Clear"]
    #[inline(always)]
    pub fn ch4reqmaskc(&mut self) -> CH4REQMASKC_W {
        CH4REQMASKC_W { w: self }
    }
    #[doc = "Bit 5 - Channel 5 Request Mask Clear"]
    #[inline(always)]
    pub fn ch5reqmaskc(&mut self) -> CH5REQMASKC_W {
        CH5REQMASKC_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Request Mask Clear Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chreqmaskc](index.html) module"]
pub struct CHREQMASKC_SPEC;
impl crate::RegisterSpec for CHREQMASKC_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [chreqmaskc::W](W) writer structure"]
impl crate::Writable for CHREQMASKC_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CHREQMASKC to value 0"]
impl crate::Resettable for CHREQMASKC_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
