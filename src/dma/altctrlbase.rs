#[doc = "Register `ALTCTRLBASE` reader"]
pub struct R(crate::R<ALTCTRLBASE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ALTCTRLBASE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ALTCTRLBASE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ALTCTRLBASE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ALTCTRLBASE` reader - Channel Alternate Control Data Base Pointer"]
pub struct ALTCTRLBASE_R(crate::FieldReader<u32, u32>);
impl ALTCTRLBASE_R {
    pub(crate) fn new(bits: u32) -> Self {
        ALTCTRLBASE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ALTCTRLBASE_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:31 - Channel Alternate Control Data Base Pointer"]
    #[inline(always)]
    pub fn altctrlbase(&self) -> ALTCTRLBASE_R {
        ALTCTRLBASE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
#[doc = "Channel Alternate Control Data Base Pointer Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [altctrlbase](index.html) module"]
pub struct ALTCTRLBASE_SPEC;
impl crate::RegisterSpec for ALTCTRLBASE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [altctrlbase::R](R) reader structure"]
impl crate::Readable for ALTCTRLBASE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets ALTCTRLBASE to value 0x80"]
impl crate::Resettable for ALTCTRLBASE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
