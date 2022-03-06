#[doc = "Register `IF` reader"]
pub struct R(crate::R<IF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `SINGLE` reader - Single Conversion Complete Interrupt Flag"]
pub struct SINGLE_R(crate::FieldReader<bool, bool>);
impl SINGLE_R {
    pub(crate) fn new(bits: bool) -> Self {
        SINGLE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGLE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCAN` reader - Scan Conversion Complete Interrupt Flag"]
pub struct SCAN_R(crate::FieldReader<bool, bool>);
impl SCAN_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCAN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCAN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SINGLEOF` reader - Single Result Overflow Interrupt Flag"]
pub struct SINGLEOF_R(crate::FieldReader<bool, bool>);
impl SINGLEOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SINGLEOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SINGLEOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SCANOF` reader - Scan Result Overflow Interrupt Flag"]
pub struct SCANOF_R(crate::FieldReader<bool, bool>);
impl SCANOF_R {
    pub(crate) fn new(bits: bool) -> Self {
        SCANOF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SCANOF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Single Conversion Complete Interrupt Flag"]
    #[inline(always)]
    pub fn single(&self) -> SINGLE_R {
        SINGLE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Scan Conversion Complete Interrupt Flag"]
    #[inline(always)]
    pub fn scan(&self) -> SCAN_R {
        SCAN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 8 - Single Result Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn singleof(&self) -> SINGLEOF_R {
        SINGLEOF_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - Scan Result Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn scanof(&self) -> SCANOF_R {
        SCANOF_R::new(((self.bits >> 9) & 0x01) != 0)
    }
}
#[doc = "Interrupt Flag Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [if_](index.html) module"]
pub struct IF_SPEC;
impl crate::RegisterSpec for IF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [if_::R](R) reader structure"]
impl crate::Readable for IF_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets IF to value 0"]
impl crate::Resettable for IF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
