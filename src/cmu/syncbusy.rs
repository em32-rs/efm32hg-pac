#[doc = "Register `SYNCBUSY` reader"]
pub struct R(crate::R<SYNCBUSY_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYNCBUSY_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYNCBUSY_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYNCBUSY_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LFACLKEN0` reader - Low Frequency A Clock Enable 0 Busy"]
pub struct LFACLKEN0_R(crate::FieldReader<bool, bool>);
impl LFACLKEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFACLKEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFACLKEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFAPRESC0` reader - Low Frequency A Prescaler 0 Busy"]
pub struct LFAPRESC0_R(crate::FieldReader<bool, bool>);
impl LFAPRESC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFAPRESC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFAPRESC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFBCLKEN0` reader - Low Frequency B Clock Enable 0 Busy"]
pub struct LFBCLKEN0_R(crate::FieldReader<bool, bool>);
impl LFBCLKEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFBCLKEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFBCLKEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFBPRESC0` reader - Low Frequency B Prescaler 0 Busy"]
pub struct LFBPRESC0_R(crate::FieldReader<bool, bool>);
impl LFBPRESC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFBPRESC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFBPRESC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LFCCLKEN0` reader - Low Frequency C Clock Enable 0 Busy"]
pub struct LFCCLKEN0_R(crate::FieldReader<bool, bool>);
impl LFCCLKEN0_R {
    pub(crate) fn new(bits: bool) -> Self {
        LFCCLKEN0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LFCCLKEN0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Low Frequency A Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfaclken0(&self) -> LFACLKEN0_R {
        LFACLKEN0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 2 - Low Frequency A Prescaler 0 Busy"]
    #[inline(always)]
    pub fn lfapresc0(&self) -> LFAPRESC0_R {
        LFAPRESC0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Low Frequency B Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfbclken0(&self) -> LFBCLKEN0_R {
        LFBCLKEN0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Low Frequency B Prescaler 0 Busy"]
    #[inline(always)]
    pub fn lfbpresc0(&self) -> LFBPRESC0_R {
        LFBPRESC0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Low Frequency C Clock Enable 0 Busy"]
    #[inline(always)]
    pub fn lfcclken0(&self) -> LFCCLKEN0_R {
        LFCCLKEN0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
}
#[doc = "Synchronization Busy Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syncbusy](index.html) module"]
pub struct SYNCBUSY_SPEC;
impl crate::RegisterSpec for SYNCBUSY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syncbusy::R](R) reader structure"]
impl crate::Readable for SYNCBUSY_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SYNCBUSY to value 0"]
impl crate::Resettable for SYNCBUSY_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
