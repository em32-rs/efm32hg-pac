#[doc = "Register `DOEP0TSIZ` reader"]
pub struct R(crate::R<DOEP0TSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DOEP0TSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DOEP0TSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DOEP0TSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DOEP0TSIZ` writer"]
pub struct W(crate::W<DOEP0TSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DOEP0TSIZ_SPEC>;
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
impl From<crate::W<DOEP0TSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DOEP0TSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XFERSIZE` reader - Transfer Size"]
pub struct XFERSIZE_R(crate::FieldReader<u8, u8>);
impl XFERSIZE_R {
    pub(crate) fn new(bits: u8) -> Self {
        XFERSIZE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XFERSIZE_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XFERSIZE` writer - Transfer Size"]
pub struct XFERSIZE_W<'a> {
    w: &'a mut W,
}
impl<'a> XFERSIZE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
#[doc = "Field `PKTCNT` reader - Packet Count"]
pub struct PKTCNT_R(crate::FieldReader<bool, bool>);
impl PKTCNT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PKTCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKTCNT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PKTCNT` writer - Packet Count"]
pub struct PKTCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> PKTCNT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 19)) | ((value as u32 & 0x01) << 19);
        self.w
    }
}
#[doc = "Field `SUPCNT` reader - SETUP Packet Count"]
pub struct SUPCNT_R(crate::FieldReader<u8, u8>);
impl SUPCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        SUPCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SUPCNT_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SUPCNT` writer - SETUP Packet Count"]
pub struct SUPCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SUPCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 29)) | ((value as u32 & 0x03) << 29);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 19 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 29:30 - SETUP Packet Count"]
    #[inline(always)]
    pub fn supcnt(&self) -> SUPCNT_R {
        SUPCNT_R::new(((self.bits >> 29) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&mut self) -> XFERSIZE_W {
        XFERSIZE_W { w: self }
    }
    #[doc = "Bit 19 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W {
        PKTCNT_W { w: self }
    }
    #[doc = "Bits 29:30 - SETUP Packet Count"]
    #[inline(always)]
    pub fn supcnt(&mut self) -> SUPCNT_W {
        SUPCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device OUT Endpoint 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [doep0tsiz](index.html) module"]
pub struct DOEP0TSIZ_SPEC;
impl crate::RegisterSpec for DOEP0TSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [doep0tsiz::R](R) reader structure"]
impl crate::Readable for DOEP0TSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [doep0tsiz::W](W) writer structure"]
impl crate::Writable for DOEP0TSIZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DOEP0TSIZ to value 0"]
impl crate::Resettable for DOEP0TSIZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
