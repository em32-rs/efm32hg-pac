#[doc = "Register `USHFRCOCAL0` reader"]
pub struct R(crate::R<USHFRCOCAL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USHFRCOCAL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USHFRCOCAL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USHFRCOCAL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BAND24_TUNING` reader - 24 MHz TUNING value for USFRCO"]
pub struct BAND24_TUNING_R(crate::FieldReader<u8, u8>);
impl BAND24_TUNING_R {
    pub(crate) fn new(bits: u8) -> Self {
        BAND24_TUNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAND24_TUNING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAND24_FINETUNING` reader - 24 MHz FINETUNING value for USFRCO"]
pub struct BAND24_FINETUNING_R(crate::FieldReader<u8, u8>);
impl BAND24_FINETUNING_R {
    pub(crate) fn new(bits: u8) -> Self {
        BAND24_FINETUNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAND24_FINETUNING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAND48_TUNING` reader - 24 MHz TUNING value for USFRCO"]
pub struct BAND48_TUNING_R(crate::FieldReader<u8, u8>);
impl BAND48_TUNING_R {
    pub(crate) fn new(bits: u8) -> Self {
        BAND48_TUNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAND48_TUNING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAND48_FINETUNING` reader - 24 MHz FINETUNING value for USFRCO"]
pub struct BAND48_FINETUNING_R(crate::FieldReader<u8, u8>);
impl BAND48_FINETUNING_R {
    pub(crate) fn new(bits: u8) -> Self {
        BAND48_FINETUNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAND48_FINETUNING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:6 - 24 MHz TUNING value for USFRCO"]
    #[inline(always)]
    pub fn band24_tuning(&self) -> BAND24_TUNING_R {
        BAND24_TUNING_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:13 - 24 MHz FINETUNING value for USFRCO"]
    #[inline(always)]
    pub fn band24_finetuning(&self) -> BAND24_FINETUNING_R {
        BAND24_FINETUNING_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bits 16:22 - 24 MHz TUNING value for USFRCO"]
    #[inline(always)]
    pub fn band48_tuning(&self) -> BAND48_TUNING_R {
        BAND48_TUNING_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:29 - 24 MHz FINETUNING value for USFRCO"]
    #[inline(always)]
    pub fn band48_finetuning(&self) -> BAND48_FINETUNING_R {
        BAND48_FINETUNING_R::new(((self.bits >> 24) & 0x3f) as u8)
    }
}
#[doc = "USHFRCO calibration register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ushfrcocal0](index.html) module"]
pub struct USHFRCOCAL0_SPEC;
impl crate::RegisterSpec for USHFRCOCAL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ushfrcocal0::R](R) reader structure"]
impl crate::Readable for USHFRCOCAL0_SPEC {
    type Reader = R;
}
