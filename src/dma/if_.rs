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
#[doc = "Field `CH0DONE` reader - DMA Channel 0 Complete Interrupt Flag"]
pub struct CH0DONE_R(crate::FieldReader<bool, bool>);
impl CH0DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH0DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH0DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH1DONE` reader - DMA Channel 1 Complete Interrupt Flag"]
pub struct CH1DONE_R(crate::FieldReader<bool, bool>);
impl CH1DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH1DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH1DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH2DONE` reader - DMA Channel 2 Complete Interrupt Flag"]
pub struct CH2DONE_R(crate::FieldReader<bool, bool>);
impl CH2DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH2DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH2DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH3DONE` reader - DMA Channel 3 Complete Interrupt Flag"]
pub struct CH3DONE_R(crate::FieldReader<bool, bool>);
impl CH3DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH3DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH3DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH4DONE` reader - DMA Channel 4 Complete Interrupt Flag"]
pub struct CH4DONE_R(crate::FieldReader<bool, bool>);
impl CH4DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH4DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH4DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CH5DONE` reader - DMA Channel 5 Complete Interrupt Flag"]
pub struct CH5DONE_R(crate::FieldReader<bool, bool>);
impl CH5DONE_R {
    pub(crate) fn new(bits: bool) -> Self {
        CH5DONE_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CH5DONE_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERR` reader - DMA Error Interrupt Flag"]
pub struct ERR_R(crate::FieldReader<bool, bool>);
impl ERR_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - DMA Channel 0 Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch0done(&self) -> CH0DONE_R {
        CH0DONE_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - DMA Channel 1 Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch1done(&self) -> CH1DONE_R {
        CH1DONE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - DMA Channel 2 Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch2done(&self) -> CH2DONE_R {
        CH2DONE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - DMA Channel 3 Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch3done(&self) -> CH3DONE_R {
        CH3DONE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - DMA Channel 4 Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch4done(&self) -> CH4DONE_R {
        CH4DONE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - DMA Channel 5 Complete Interrupt Flag"]
    #[inline(always)]
    pub fn ch5done(&self) -> CH5DONE_R {
        CH5DONE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 31 - DMA Error Interrupt Flag"]
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 31) & 0x01) != 0)
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
