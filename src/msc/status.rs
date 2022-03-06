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
#[doc = "Field `BUSY` reader - Erase/Write Busy"]
pub struct BUSY_R(crate::FieldReader<bool, bool>);
impl BUSY_R {
    pub(crate) fn new(bits: bool) -> Self {
        BUSY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for BUSY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LOCKED` reader - Access Locked"]
pub struct LOCKED_R(crate::FieldReader<bool, bool>);
impl LOCKED_R {
    pub(crate) fn new(bits: bool) -> Self {
        LOCKED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LOCKED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `INVADDR` reader - Invalid Write Address or Erase Page"]
pub struct INVADDR_R(crate::FieldReader<bool, bool>);
impl INVADDR_R {
    pub(crate) fn new(bits: bool) -> Self {
        INVADDR_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for INVADDR_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WDATAREADY` reader - WDATA Write Ready"]
pub struct WDATAREADY_R(crate::FieldReader<bool, bool>);
impl WDATAREADY_R {
    pub(crate) fn new(bits: bool) -> Self {
        WDATAREADY_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WDATAREADY_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `WORDTIMEOUT` reader - Flash Write Word Timeout"]
pub struct WORDTIMEOUT_R(crate::FieldReader<bool, bool>);
impl WORDTIMEOUT_R {
    pub(crate) fn new(bits: bool) -> Self {
        WORDTIMEOUT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for WORDTIMEOUT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ERASEABORTED` reader - The Current Flash Erase Operation Aborted"]
pub struct ERASEABORTED_R(crate::FieldReader<bool, bool>);
impl ERASEABORTED_R {
    pub(crate) fn new(bits: bool) -> Self {
        ERASEABORTED_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ERASEABORTED_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCRUNNING` reader - Performance Counters Running"]
pub struct PCRUNNING_R(crate::FieldReader<bool, bool>);
impl PCRUNNING_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCRUNNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCRUNNING_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Erase/Write Busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Access Locked"]
    #[inline(always)]
    pub fn locked(&self) -> LOCKED_R {
        LOCKED_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Invalid Write Address or Erase Page"]
    #[inline(always)]
    pub fn invaddr(&self) -> INVADDR_R {
        INVADDR_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - WDATA Write Ready"]
    #[inline(always)]
    pub fn wdataready(&self) -> WDATAREADY_R {
        WDATAREADY_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Flash Write Word Timeout"]
    #[inline(always)]
    pub fn wordtimeout(&self) -> WORDTIMEOUT_R {
        WORDTIMEOUT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - The Current Flash Erase Operation Aborted"]
    #[inline(always)]
    pub fn eraseaborted(&self) -> ERASEABORTED_R {
        ERASEABORTED_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Performance Counters Running"]
    #[inline(always)]
    pub fn pcrunning(&self) -> PCRUNNING_R {
        PCRUNNING_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
#[doc = "Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x08"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x08
    }
}
