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
#[doc = "Field `RXENS` reader - Receiver Enable Status"]
pub struct RXENS_R(crate::FieldReader<bool, bool>);
impl RXENS_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXENS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXENS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXENS` reader - Transmitter Enable Status"]
pub struct TXENS_R(crate::FieldReader<bool, bool>);
impl TXENS_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXENS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXENS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXBLOCK` reader - Block Incoming Data"]
pub struct RXBLOCK_R(crate::FieldReader<bool, bool>);
impl RXBLOCK_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXBLOCK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXBLOCK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXC` reader - TX Complete"]
pub struct TXC_R(crate::FieldReader<bool, bool>);
impl TXC_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXC_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXC_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBL` reader - TX Buffer Level"]
pub struct TXBL_R(crate::FieldReader<bool, bool>);
impl TXBL_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXBL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDATAV` reader - RX Data Valid"]
pub struct RXDATAV_R(crate::FieldReader<bool, bool>);
impl RXDATAV_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDATAV_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDATAV_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bit 0 - Receiver Enable Status"]
    #[inline(always)]
    pub fn rxens(&self) -> RXENS_R {
        RXENS_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Transmitter Enable Status"]
    #[inline(always)]
    pub fn txens(&self) -> TXENS_R {
        TXENS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Block Incoming Data"]
    #[inline(always)]
    pub fn rxblock(&self) -> RXBLOCK_R {
        RXBLOCK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - TX Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - TX Buffer Level"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - RX Data Valid"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 5) & 0x01) != 0)
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
#[doc = "`reset()` method sets STATUS to value 0x10"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x10
    }
}
