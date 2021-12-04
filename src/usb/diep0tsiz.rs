#[doc = "Register `DIEP0TSIZ` reader"]
pub struct R(crate::R<DIEP0TSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DIEP0TSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DIEP0TSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DIEP0TSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DIEP0TSIZ` writer"]
pub struct W(crate::W<DIEP0TSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DIEP0TSIZ_SPEC>;
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
impl From<crate::W<DIEP0TSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DIEP0TSIZ_SPEC>) -> Self {
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
pub struct PKTCNT_R(crate::FieldReader<u8, u8>);
impl PKTCNT_R {
    pub(crate) fn new(bits: u8) -> Self {
        PKTCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PKTCNT_R {
    type Target = crate::FieldReader<u8, u8>;
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
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 19)) | ((value as u32 & 0x03) << 19);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&self) -> XFERSIZE_R {
        XFERSIZE_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&self) -> PKTCNT_R {
        PKTCNT_R::new(((self.bits >> 19) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Transfer Size"]
    #[inline(always)]
    pub fn xfersize(&mut self) -> XFERSIZE_W {
        XFERSIZE_W { w: self }
    }
    #[doc = "Bits 19:20 - Packet Count"]
    #[inline(always)]
    pub fn pktcnt(&mut self) -> PKTCNT_W {
        PKTCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Device IN Endpoint 0 Transfer Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [diep0tsiz](index.html) module"]
pub struct DIEP0TSIZ_SPEC;
impl crate::RegisterSpec for DIEP0TSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [diep0tsiz::R](R) reader structure"]
impl crate::Readable for DIEP0TSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [diep0tsiz::W](W) writer structure"]
impl crate::Writable for DIEP0TSIZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DIEP0TSIZ to value 0"]
impl crate::Resettable for DIEP0TSIZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
