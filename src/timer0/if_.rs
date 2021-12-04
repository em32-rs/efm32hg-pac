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
#[doc = "Field `OF` reader - Overflow Interrupt Flag"]
pub struct OF_R(crate::FieldReader<bool, bool>);
impl OF_R {
    pub(crate) fn new(bits: bool) -> Self {
        OF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for OF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UF` reader - Underflow Interrupt Flag"]
pub struct UF_R(crate::FieldReader<bool, bool>);
impl UF_R {
    pub(crate) fn new(bits: bool) -> Self {
        UF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for UF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC0` reader - CC Channel 0 Interrupt Flag"]
pub struct CC0_R(crate::FieldReader<bool, bool>);
impl CC0_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC1` reader - CC Channel 1 Interrupt Flag"]
pub struct CC1_R(crate::FieldReader<bool, bool>);
impl CC1_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CC2` reader - CC Channel 2 Interrupt Flag"]
pub struct CC2_R(crate::FieldReader<bool, bool>);
impl CC2_R {
    pub(crate) fn new(bits: bool) -> Self {
        CC2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CC2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICBOF0` reader - CC Channel 0 Input Capture Buffer Overflow Interrupt Flag"]
pub struct ICBOF0_R(crate::FieldReader<bool, bool>);
impl ICBOF0_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICBOF0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICBOF0_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICBOF1` reader - CC Channel 1 Input Capture Buffer Overflow Interrupt Flag"]
pub struct ICBOF1_R(crate::FieldReader<bool, bool>);
impl ICBOF1_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICBOF1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICBOF1_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ICBOF2` reader - CC Channel 2 Input Capture Buffer Overflow Interrupt Flag"]
pub struct ICBOF2_R(crate::FieldReader<bool, bool>);
impl ICBOF2_R {
    pub(crate) fn new(bits: bool) -> Self {
        ICBOF2_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ICBOF2_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn of(&self) -> OF_R {
        OF_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Underflow Interrupt Flag"]
    #[inline(always)]
    pub fn uf(&self) -> UF_R {
        UF_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CC Channel 0 Interrupt Flag"]
    #[inline(always)]
    pub fn cc0(&self) -> CC0_R {
        CC0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CC Channel 1 Interrupt Flag"]
    #[inline(always)]
    pub fn cc1(&self) -> CC1_R {
        CC1_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - CC Channel 2 Interrupt Flag"]
    #[inline(always)]
    pub fn cc2(&self) -> CC2_R {
        CC2_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 8 - CC Channel 0 Input Capture Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn icbof0(&self) -> ICBOF0_R {
        ICBOF0_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - CC Channel 1 Input Capture Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn icbof1(&self) -> ICBOF1_R {
        ICBOF1_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - CC Channel 2 Input Capture Buffer Overflow Interrupt Flag"]
    #[inline(always)]
    pub fn icbof2(&self) -> ICBOF2_R {
        ICBOF2_R::new(((self.bits >> 10) & 0x01) != 0)
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
