#[doc = "Register `AUXHFRCOCAL1` reader"]
pub struct R(crate::R<AUXHFRCOCAL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXHFRCOCAL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXHFRCOCAL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXHFRCOCAL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BAND21` reader - 21MHz tuning value for AUXHFRCO"]
pub struct BAND21_R(crate::FieldReader<u8, u8>);
impl BAND21_R {
    pub(crate) fn new(bits: u8) -> Self {
        BAND21_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAND21_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - 21MHz tuning value for AUXHFRCO"]
    #[inline(always)]
    pub fn band21(&self) -> BAND21_R {
        BAND21_R::new((self.bits & 0xff) as u8)
    }
}
#[doc = "AUXHFRCO calibration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxhfrcocal1](index.html) module"]
pub struct AUXHFRCOCAL1_SPEC;
impl crate::RegisterSpec for AUXHFRCOCAL1_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [auxhfrcocal1::R](R) reader structure"]
impl crate::Readable for AUXHFRCOCAL1_SPEC {
    type Reader = R;
}
