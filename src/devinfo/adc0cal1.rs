#[doc = "Register `ADC0CAL1` reader"]
pub struct R(crate::R<ADC0CAL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADC0CAL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADC0CAL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADC0CAL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VDD_OFFSET` reader - Offset for VDD reference"]
pub struct VDD_OFFSET_R(crate::FieldReader<u8, u8>);
impl VDD_OFFSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        VDD_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDD_OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDD_GAIN` reader - Gain for VDD reference"]
pub struct VDD_GAIN_R(crate::FieldReader<u8, u8>);
impl VDD_GAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        VDD_GAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDD_GAIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `5VDIFF_OFFSET` reader - Offset for 5VDIFF reference"]
pub struct _5VDIFF_OFFSET_R(crate::FieldReader<u8, u8>);
impl _5VDIFF_OFFSET_R {
    pub(crate) fn new(bits: u8) -> Self {
        _5VDIFF_OFFSET_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for _5VDIFF_OFFSET_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `5VDIFF_GAIN` reader - Gain for 5VDIFF reference"]
pub struct _5VDIFF_GAIN_R(crate::FieldReader<u8, u8>);
impl _5VDIFF_GAIN_R {
    pub(crate) fn new(bits: u8) -> Self {
        _5VDIFF_GAIN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for _5VDIFF_GAIN_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:6 - Offset for VDD reference"]
    #[inline(always)]
    pub fn vdd_offset(&self) -> VDD_OFFSET_R {
        VDD_OFFSET_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bits 8:14 - Gain for VDD reference"]
    #[inline(always)]
    pub fn vdd_gain(&self) -> VDD_GAIN_R {
        VDD_GAIN_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bits 16:22 - Offset for 5VDIFF reference"]
    #[inline(always)]
    pub fn _5vdiff_offset(&self) -> _5VDIFF_OFFSET_R {
        _5VDIFF_OFFSET_R::new(((self.bits >> 16) & 0x7f) as u8)
    }
    #[doc = "Bits 24:30 - Gain for 5VDIFF reference"]
    #[inline(always)]
    pub fn _5vdiff_gain(&self) -> _5VDIFF_GAIN_R {
        _5VDIFF_GAIN_R::new(((self.bits >> 24) & 0x7f) as u8)
    }
}
#[doc = "ADC0 Calibration register 1\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adc0cal1](index.html) module"]
pub struct ADC0CAL1_SPEC;
impl crate::RegisterSpec for ADC0CAL1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [adc0cal1::R](R) reader structure"]
impl crate::Readable for ADC0CAL1_SPEC {
    type Reader = R;
}
