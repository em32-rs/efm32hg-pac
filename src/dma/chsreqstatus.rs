#[doc = "Register `CHSREQSTATUS` reader"]
pub struct R(crate::R<CHSREQSTATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CHSREQSTATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CHSREQSTATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CHSREQSTATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH0SREQSTATUS` reader - Channel 0 Single Request Status"]
pub struct CH0SREQSTATUS_R(crate::FieldReader<bool, bool>);
impl CH0SREQSTATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0SREQSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0SREQSTATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1SREQSTATUS` reader - Channel 1 Single Request Status"]
pub struct CH1SREQSTATUS_R(crate::FieldReader<bool, bool>);
impl CH1SREQSTATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1SREQSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1SREQSTATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2SREQSTATUS` reader - Channel 2 Single Request Status"]
pub struct CH2SREQSTATUS_R(crate::FieldReader<bool, bool>);
impl CH2SREQSTATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2SREQSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2SREQSTATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3SREQSTATUS` reader - Channel 3 Single Request Status"]
pub struct CH3SREQSTATUS_R(crate::FieldReader<bool, bool>);
impl CH3SREQSTATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3SREQSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3SREQSTATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4SREQSTATUS` reader - Channel 4 Single Request Status"]
pub struct CH4SREQSTATUS_R(crate::FieldReader<bool, bool>);
impl CH4SREQSTATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH4SREQSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH4SREQSTATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5SREQSTATUS` reader - Channel 5 Single Request Status"]
pub struct CH5SREQSTATUS_R(crate::FieldReader<bool, bool>);
impl CH5SREQSTATUS_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH5SREQSTATUS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH5SREQSTATUS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Channel 0 Single Request Status"]
    #[inline(always)]
    pub fn ch0sreqstatus(&self) -> CH0SREQSTATUS_R {
        CH0SREQSTATUS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Channel 1 Single Request Status"]
    #[inline(always)]
    pub fn ch1sreqstatus(&self) -> CH1SREQSTATUS_R {
        CH1SREQSTATUS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Channel 2 Single Request Status"]
    #[inline(always)]
    pub fn ch2sreqstatus(&self) -> CH2SREQSTATUS_R {
        CH2SREQSTATUS_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Channel 3 Single Request Status"]
    #[inline(always)]
    pub fn ch3sreqstatus(&self) -> CH3SREQSTATUS_R {
        CH3SREQSTATUS_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Channel 4 Single Request Status"]
    #[inline(always)]
    pub fn ch4sreqstatus(&self) -> CH4SREQSTATUS_R {
        CH4SREQSTATUS_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Channel 5 Single Request Status"]
    #[inline(always)]
    pub fn ch5sreqstatus(&self) -> CH5SREQSTATUS_R {
        CH5SREQSTATUS_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
#[doc = "Channel Single Request Status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [chsreqstatus](index.html) module"]
pub struct CHSREQSTATUS_SPEC;
impl crate::RegisterSpec for CHSREQSTATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [chsreqstatus::R](R) reader structure"]
impl crate::Readable for CHSREQSTATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CHSREQSTATUS to value 0"]
impl crate::Resettable for CHSREQSTATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
