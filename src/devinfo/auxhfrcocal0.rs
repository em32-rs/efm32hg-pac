#[doc = "Register `AUXHFRCOCAL0` reader"]
pub struct R(crate::R<AUXHFRCOCAL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AUXHFRCOCAL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AUXHFRCOCAL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AUXHFRCOCAL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BAND1` reader - 1MHz tuning value for AUXHFRCO"]
pub struct BAND1_R(crate::FieldReader<u8, u8>);
impl BAND1_R {
    pub(crate) fn new(bits: u8) -> Self {
        BAND1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAND1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAND7` reader - 7MHz tuning value for AUXHFRCO"]
pub struct BAND7_R(crate::FieldReader<u8, u8>);
impl BAND7_R {
    pub(crate) fn new(bits: u8) -> Self {
        BAND7_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAND7_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAND11` reader - 11MHz tuning value for AUXHFRCO"]
pub struct BAND11_R(crate::FieldReader<u8, u8>);
impl BAND11_R {
    pub(crate) fn new(bits: u8) -> Self {
        BAND11_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAND11_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `BAND14` reader - 14MHz tuning value for AUXHFRCO"]
pub struct BAND14_R(crate::FieldReader<u8, u8>);
impl BAND14_R {
    pub(crate) fn new(bits: u8) -> Self {
        BAND14_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BAND14_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - 1MHz tuning value for AUXHFRCO"]
    #[inline(always)]
    pub fn band1(&self) -> BAND1_R {
        BAND1_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - 7MHz tuning value for AUXHFRCO"]
    #[inline(always)]
    pub fn band7(&self) -> BAND7_R {
        BAND7_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - 11MHz tuning value for AUXHFRCO"]
    #[inline(always)]
    pub fn band11(&self) -> BAND11_R {
        BAND11_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - 14MHz tuning value for AUXHFRCO"]
    #[inline(always)]
    pub fn band14(&self) -> BAND14_R {
        BAND14_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[doc = "AUXHFRCO calibration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [auxhfrcocal0](index.html) module"]
pub struct AUXHFRCOCAL0_SPEC;
impl crate::RegisterSpec for AUXHFRCOCAL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [auxhfrcocal0::R](R) reader structure"]
impl crate::Readable for AUXHFRCOCAL0_SPEC {
    type Reader = R;
}
