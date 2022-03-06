#[doc = "Register `CC2_CCVB` reader"]
pub struct R(crate::R<CC2_CCVB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CC2_CCVB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CC2_CCVB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CC2_CCVB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CC2_CCVB` writer"]
pub struct W(crate::W<CC2_CCVB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CC2_CCVB_SPEC>;
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
impl From<crate::W<CC2_CCVB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CC2_CCVB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CCVB` reader - CC Channel Value Buffer"]
pub struct CCVB_R(crate::FieldReader<u16, u16>);
impl CCVB_R {
    pub(crate) fn new(bits: u16) -> Self {
        CCVB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CCVB_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CCVB` writer - CC Channel Value Buffer"]
pub struct CCVB_W<'a> {
    w: &'a mut W,
}
impl<'a> CCVB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - CC Channel Value Buffer"]
    #[inline(always)]
    pub fn ccvb(&self) -> CCVB_R {
        CCVB_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - CC Channel Value Buffer"]
    #[inline(always)]
    pub fn ccvb(&mut self) -> CCVB_W {
        CCVB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CC Channel Buffer Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cc2_ccvb](index.html) module"]
pub struct CC2_CCVB_SPEC;
impl crate::RegisterSpec for CC2_CCVB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cc2_ccvb::R](R) reader structure"]
impl crate::Readable for CC2_CCVB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cc2_ccvb::W](W) writer structure"]
impl crate::Writable for CC2_CCVB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CC2_CCVB to value 0"]
impl crate::Resettable for CC2_CCVB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
