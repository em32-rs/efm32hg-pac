#[doc = "Register `DAINT` reader"]
pub struct R(crate::R<DAINT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DAINT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DAINT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DAINT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INEPINT0` reader - IN Endpoint 0 Interrupt Bit"]
pub struct INEPINT0_R(crate::FieldReader<bool, bool>);
impl INEPINT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        INEPINT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPINT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEPINT1` reader - IN Endpoint 1 Interrupt Bit"]
pub struct INEPINT1_R(crate::FieldReader<bool, bool>);
impl INEPINT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        INEPINT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPINT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEPINT2` reader - IN Endpoint 2 Interrupt Bit"]
pub struct INEPINT2_R(crate::FieldReader<bool, bool>);
impl INEPINT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        INEPINT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPINT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INEPINT3` reader - IN Endpoint 3 Interrupt Bit"]
pub struct INEPINT3_R(crate::FieldReader<bool, bool>);
impl INEPINT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        INEPINT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INEPINT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTEPINT0` reader - OUT Endpoint 0 Interrupt Bit"]
pub struct OUTEPINT0_R(crate::FieldReader<bool, bool>);
impl OUTEPINT0_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTEPINT0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTEPINT0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTEPINT1` reader - OUT Endpoint 1 Interrupt Bit"]
pub struct OUTEPINT1_R(crate::FieldReader<bool, bool>);
impl OUTEPINT1_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTEPINT1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTEPINT1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTEPINT2` reader - OUT Endpoint 2 Interrupt Bit"]
pub struct OUTEPINT2_R(crate::FieldReader<bool, bool>);
impl OUTEPINT2_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTEPINT2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTEPINT2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `OUTEPINT3` reader - OUT Endpoint 3 Interrupt Bit"]
pub struct OUTEPINT3_R(crate::FieldReader<bool, bool>);
impl OUTEPINT3_R {
    pub(crate) fn new(bits: bool) -> Self {
        OUTEPINT3_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OUTEPINT3_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - IN Endpoint 0 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint0(&self) -> INEPINT0_R {
        INEPINT0_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - IN Endpoint 1 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint1(&self) -> INEPINT1_R {
        INEPINT1_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - IN Endpoint 2 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint2(&self) -> INEPINT2_R {
        INEPINT2_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - IN Endpoint 3 Interrupt Bit"]
    #[inline(always)]
    pub fn inepint3(&self) -> INEPINT3_R {
        INEPINT3_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 16 - OUT Endpoint 0 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint0(&self) -> OUTEPINT0_R {
        OUTEPINT0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 17 - OUT Endpoint 1 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint1(&self) -> OUTEPINT1_R {
        OUTEPINT1_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 18 - OUT Endpoint 2 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint2(&self) -> OUTEPINT2_R {
        OUTEPINT2_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 19 - OUT Endpoint 3 Interrupt Bit"]
    #[inline(always)]
    pub fn outepint3(&self) -> OUTEPINT3_R {
        OUTEPINT3_R::new(((self.bits >> 19) & 0x01) != 0)
    }
}
#[doc = "Device All Endpoints Interrupt Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [daint](index.html) module"]
pub struct DAINT_SPEC;
impl crate::RegisterSpec for DAINT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [daint::R](R) reader structure"]
impl crate::Readable for DAINT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DAINT to value 0"]
impl crate::Resettable for DAINT_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
