#[doc = "Register `STATUS` reader"]
pub struct R(crate::R<STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `VREGOS` reader - VREGO Sense Output"]
pub struct VREGOS_R(crate::FieldReader<bool, bool>);
impl VREGOS_R {
    pub(crate) fn new(bits: bool) -> Self {
        VREGOS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VREGOS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LEMACTIVE` reader - Low Energy Mode Active"]
pub struct LEMACTIVE_R(crate::FieldReader<bool, bool>);
impl LEMACTIVE_R {
    pub(crate) fn new(bits: bool) -> Self {
        LEMACTIVE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LEMACTIVE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - VREGO Sense Output"]
    #[inline(always)]
    pub fn vregos(&self) -> VREGOS_R {
        VREGOS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low Energy Mode Active"]
    #[inline(always)]
    pub fn lemactive(&self) -> LEMACTIVE_R {
        LEMACTIVE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
}
#[doc = "System Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
