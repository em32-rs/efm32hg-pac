#[doc = "Register `GNPTXFSIZ` reader"]
pub struct R(crate::R<GNPTXFSIZ_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<GNPTXFSIZ_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<GNPTXFSIZ_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<GNPTXFSIZ_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `GNPTXFSIZ` writer"]
pub struct W(crate::W<GNPTXFSIZ_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<GNPTXFSIZ_SPEC>;
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
impl From<crate::W<GNPTXFSIZ_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<GNPTXFSIZ_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `NPTXFSTADDR` reader - Non-periodic Transmit RAM Start Address host only"]
pub struct NPTXFSTADDR_R(crate::FieldReader<u16, u16>);
impl NPTXFSTADDR_R {
    pub(crate) fn new(bits: u16) -> Self {
        NPTXFSTADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPTXFSTADDR_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPTXFSTADDR` writer - Non-periodic Transmit RAM Start Address host only"]
pub struct NPTXFSTADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFSTADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | (value as u32 & 0x03ff);
        self.w
    }
}
#[doc = "Field `NPTXFINEPTXF0DEP` reader - Non-periodic TxFIFO Depth host only / IN Endpoint TxFIFO 0 Depth"]
pub struct NPTXFINEPTXF0DEP_R(crate::FieldReader<u16, u16>);
impl NPTXFINEPTXF0DEP_R {
    pub(crate) fn new(bits: u16) -> Self {
        NPTXFINEPTXF0DEP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for NPTXFINEPTXF0DEP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NPTXFINEPTXF0DEP` writer - Non-periodic TxFIFO Depth host only / IN Endpoint TxFIFO 0 Depth"]
pub struct NPTXFINEPTXF0DEP_W<'a> {
    w: &'a mut W,
}
impl<'a> NPTXFINEPTXF0DEP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | ((value as u32 & 0xffff) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - Non-periodic Transmit RAM Start Address host only"]
    #[inline(always)]
    pub fn nptxfstaddr(&self) -> NPTXFSTADDR_R {
        NPTXFSTADDR_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO Depth host only / IN Endpoint TxFIFO 0 Depth"]
    #[inline(always)]
    pub fn nptxfineptxf0dep(&self) -> NPTXFINEPTXF0DEP_R {
        NPTXFINEPTXF0DEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - Non-periodic Transmit RAM Start Address host only"]
    #[inline(always)]
    pub fn nptxfstaddr(&mut self) -> NPTXFSTADDR_W {
        NPTXFSTADDR_W { w: self }
    }
    #[doc = "Bits 16:31 - Non-periodic TxFIFO Depth host only / IN Endpoint TxFIFO 0 Depth"]
    #[inline(always)]
    pub fn nptxfineptxf0dep(&mut self) -> NPTXFINEPTXF0DEP_W {
        NPTXFINEPTXF0DEP_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Non-periodic Transmit FIFO Size Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [gnptxfsiz](index.html) module"]
pub struct GNPTXFSIZ_SPEC;
impl crate::RegisterSpec for GNPTXFSIZ_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [gnptxfsiz::R](R) reader structure"]
impl crate::Readable for GNPTXFSIZ_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [gnptxfsiz::W](W) writer structure"]
impl crate::Writable for GNPTXFSIZ_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets GNPTXFSIZ to value 0x0200_0200"]
impl crate::Resettable for GNPTXFSIZ_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0200_0200
    }
}
