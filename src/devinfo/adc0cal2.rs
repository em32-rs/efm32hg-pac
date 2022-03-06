#[doc = "Register `ADC0CAL2` reader"]
pub struct R(crate::R<ADC0CAL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0CAL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0CAL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0CAL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `2XVDDVSS_OFFSET` reader - Offset for 2XVDDVSS reference"]
pub struct _2XVDDVSS_OFFSET_R(crate::FieldReader<u8, u8>);
impl _2XVDDVSS_OFFSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        _2XVDDVSS_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for _2XVDDVSS_OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TEMP1V25` reader - Temperature reading at 1V25 reference"]
pub struct TEMP1V25_R(crate::FieldReader<u16, u16>);
impl TEMP1V25_R {
    pub(crate) fn new(bits: u16) -> Self {
        TEMP1V25_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TEMP1V25_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:6 - Offset for 2XVDDVSS reference"]
    #[inline(always)]
    pub fn _2xvddvss_offset(&self) -> _2XVDDVSS_OFFSET_R {
        _2XVDDVSS_OFFSET_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 20:31 - Temperature reading at 1V25 reference"]
    #[inline(always)]
    pub fn temp1v25(&self) -> TEMP1V25_R {
        TEMP1V25_R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
#[doc = "ADC0 Calibration register 2\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0cal2](index.html) module"]
pub struct ADC0CAL2_SPEC;
impl crate::RegisterSpec for ADC0CAL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0cal2::R](R) reader structure"]
impl crate::Readable for ADC0CAL2_SPEC {
    type Reader = R;
}
