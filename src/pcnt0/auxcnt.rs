#[doc = "Register `AUXCNT` reader"]
pub struct R(crate::R<AUXCNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXCNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXCNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXCNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AUXCNT` writer"]
pub struct W(crate::W<AUXCNT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AUXCNT_SPEC>;
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
impl From<crate::W<AUXCNT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AUXCNT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AUXCNT` reader - Auxiliary Counter Value"]
pub struct AUXCNT_R(crate::FieldReader<u16, u16>);
impl AUXCNT_R {
    pub(crate) fn new(bits: u16) -> Self {
        AUXCNT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for AUXCNT_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `AUXCNT` writer - Auxiliary Counter Value"]
pub struct AUXCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Auxiliary Counter Value"]
    #[inline(always)]
    pub fn auxcnt(&self) -> AUXCNT_R {
        AUXCNT_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Auxiliary Counter Value"]
    #[inline(always)]
    pub fn auxcnt(&mut self) -> AUXCNT_W {
        AUXCNT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Auxiliary Counter Value Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxcnt](index.html) module"]
pub struct AUXCNT_SPEC;
impl crate::RegisterSpec for AUXCNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auxcnt::R](R) reader structure"]
impl crate::Readable for AUXCNT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [auxcnt::W](W) writer structure"]
impl crate::Writable for AUXCNT_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets AUXCNT to value 0"]
impl crate::Resettable for AUXCNT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
