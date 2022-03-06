#[doc = "Register `CAL` reader"]
pub struct R(crate::R<CAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CRC` reader - Integrity CRC checksum"]
pub struct CRC_R(crate::FieldReader<u16, u16>);
impl CRC_R {
    pub(crate) fn new(bits: u16) -> Self {
        CRC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CRC_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEMP` reader - Calibration temperature, DegC"]
pub struct TEMP_R(crate::FieldReader<u8, u8>);
impl TEMP_R {
    pub(crate) fn new(bits: u8) -> Self {
        TEMP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEMP_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Integrity CRC checksum"]
    #[inline(always)]
    pub fn crc(&self) -> CRC_R {
        CRC_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:23 - Calibration temperature, DegC"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[doc = "Calibration temperature and checksum\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal](index.html) module"]
pub struct CAL_SPEC;
impl crate::RegisterSpec for CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal::R](R) reader structure"]
impl crate::Readable for CAL_SPEC {
    type Reader = R;
}
