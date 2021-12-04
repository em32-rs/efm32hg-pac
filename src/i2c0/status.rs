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
#[doc = "Field `PSTART` reader - Pending START"]
pub struct PSTART_R(crate::FieldReader<bool, bool>);
impl PSTART_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSTART_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSTART_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PSTOP` reader - Pending STOP"]
pub struct PSTOP_R(crate::FieldReader<bool, bool>);
impl PSTOP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PSTOP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PSTOP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PACK` reader - Pending ACK"]
pub struct PACK_R(crate::FieldReader<bool, bool>);
impl PACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PNACK` reader - Pending NACK"]
pub struct PNACK_R(crate::FieldReader<bool, bool>);
impl PNACK_R {
    pub(crate) fn new(bits: bool) -> Self {
        PNACK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PNACK_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PCONT` reader - Pending continue"]
pub struct PCONT_R(crate::FieldReader<bool, bool>);
impl PCONT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PCONT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PCONT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PABORT` reader - Pending abort"]
pub struct PABORT_R(crate::FieldReader<bool, bool>);
impl PABORT_R {
    pub(crate) fn new(bits: bool) -> Self {
        PABORT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PABORT_R {
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
    #[doc = "Bit 0 - Pending START"]
    #[inline(always)]
    pub fn pstart(&self) -> PSTART_R {
        PSTART_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - Pending STOP"]
    #[inline(always)]
    pub fn pstop(&self) -> PSTOP_R {
        PSTOP_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Pending ACK"]
    #[inline(always)]
    pub fn pack(&self) -> PACK_R {
        PACK_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Pending NACK"]
    #[inline(always)]
    pub fn pnack(&self) -> PNACK_R {
        PNACK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Pending continue"]
    #[inline(always)]
    pub fn pcont(&self) -> PCONT_R {
        PCONT_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Pending abort"]
    #[inline(always)]
    pub fn pabort(&self) -> PABORT_R {
        PABORT_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TX Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - TX Buffer Level"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RX Data Valid"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 8) & 0x01) != 0)
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
#[doc = "`reset()` method sets STATUS to value 0x80"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x80
    }
}
