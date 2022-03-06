#[doc = "Register `CACHEHITS` reader"]
pub struct R(crate::R<CACHEHITS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CACHEHITS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CACHEHITS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CACHEHITS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CACHEHITS` reader - Cache hits since last performance counter start command."]
pub struct CACHEHITS_R(crate::FieldReader<u32, u32>);
impl CACHEHITS_R {
    pub(crate) fn new(bits: u32) -> Self {
        CACHEHITS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CACHEHITS_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:19 - Cache hits since last performance counter start command."]
    #[inline(always)]
    pub fn cachehits(&self) -> CACHEHITS_R {
        CACHEHITS_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
#[doc = "Cache Hits Performance Counter\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cachehits](index.html) module"]
pub struct CACHEHITS_SPEC;
impl crate::RegisterSpec for CACHEHITS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cachehits::R](R) reader structure"]
impl crate::Readable for CACHEHITS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CACHEHITS to value 0"]
impl crate::Resettable for CACHEHITS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
