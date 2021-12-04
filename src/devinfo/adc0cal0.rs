#[doc = "Register `ADC0CAL0` reader"]
pub struct R(crate::R<ADC0CAL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0CAL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0CAL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0CAL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `1V25_OFFSET` reader - Offset for 1V25 reference"]
pub struct _1V25_OFFSET_R(crate::FieldReader<u8, u8>);
impl _1V25_OFFSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        _1V25_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for _1V25_OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `1V25_GAIN` reader - Gain for 1V25 reference"]
pub struct _1V25_GAIN_R(crate::FieldReader<u8, u8>);
impl _1V25_GAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        _1V25_GAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for _1V25_GAIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `2V5_OFFSET` reader - Offset for 2V5 reference"]
pub struct _2V5_OFFSET_R(crate::FieldReader<u8, u8>);
impl _2V5_OFFSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        _2V5_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for _2V5_OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `2V5_GAIN` reader - Gain for 2V5 reference"]
pub struct _2V5_GAIN_R(crate::FieldReader<u8, u8>);
impl _2V5_GAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        _2V5_GAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for _2V5_GAIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:6 - Offset for 1V25 reference"]
    #[inline(always)]
    pub fn _1v25_offset(&self) -> _1V25_OFFSET_R {
        _1V25_OFFSET_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Gain for 1V25 reference"]
    #[inline(always)]
    pub fn _1v25_gain(&self) -> _1V25_GAIN_R {
        _1V25_GAIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Offset for 2V5 reference"]
    #[inline(always)]
    pub fn _2v5_offset(&self) -> _2V5_OFFSET_R {
        _2V5_OFFSET_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Gain for 2V5 reference"]
    #[inline(always)]
    pub fn _2v5_gain(&self) -> _2V5_GAIN_R {
        _2V5_GAIN_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "ADC0 Calibration register 0\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0cal0](index.html) module"]
pub struct ADC0CAL0_SPEC;
impl crate::RegisterSpec for ADC0CAL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0cal0::R](R) reader structure"]
impl crate::Readable for ADC0CAL0_SPEC {
    type Reader = R;
}
