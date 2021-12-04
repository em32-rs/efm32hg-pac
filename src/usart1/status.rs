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
#[doc = "Field `MASTER` reader - SPI Master Mode"]
pub struct MASTER_R(crate::FieldReader<bool, bool>);
impl MASTER_R {
    pub(crate) fn new(bits: bool) -> Self {
        MASTER_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASTER_R {
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
#[doc = "Field `TXTRI` reader - Transmitter Tristated"]
pub struct TXTRI_R(crate::FieldReader<bool, bool>);
impl TXTRI_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXTRI_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXTRI_R {
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
#[doc = "Field `RXFULL` reader - RX FIFO Full"]
pub struct RXFULL_R(crate::FieldReader<bool, bool>);
impl RXFULL_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFULL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFULL_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBDRIGHT` reader - TX Buffer Expects Double Right Data"]
pub struct TXBDRIGHT_R(crate::FieldReader<bool, bool>);
impl TXBDRIGHT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXBDRIGHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBDRIGHT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TXBSRIGHT` reader - TX Buffer Expects Single Right Data"]
pub struct TXBSRIGHT_R(crate::FieldReader<bool, bool>);
impl TXBSRIGHT_R {
    pub(crate) fn new(bits: bool) -> Self {
        TXBSRIGHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TXBSRIGHT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDATAVRIGHT` reader - RX Data Right"]
pub struct RXDATAVRIGHT_R(crate::FieldReader<bool, bool>);
impl RXDATAVRIGHT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXDATAVRIGHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDATAVRIGHT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXFULLRIGHT` reader - RX Full of Right Data"]
pub struct RXFULLRIGHT_R(crate::FieldReader<bool, bool>);
impl RXFULLRIGHT_R {
    pub(crate) fn new(bits: bool) -> Self {
        RXFULLRIGHT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXFULLRIGHT_R {
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
    #[doc = "Bit 2 - SPI Master Mode"]
    #[inline(always)]
    pub fn master(&self) -> MASTER_R {
        MASTER_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Block Incoming Data"]
    #[inline(always)]
    pub fn rxblock(&self) -> RXBLOCK_R {
        RXBLOCK_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Transmitter Tristated"]
    #[inline(always)]
    pub fn txtri(&self) -> TXTRI_R {
        TXTRI_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - TX Complete"]
    #[inline(always)]
    pub fn txc(&self) -> TXC_R {
        TXC_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 6 - TX Buffer Level"]
    #[inline(always)]
    pub fn txbl(&self) -> TXBL_R {
        TXBL_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 7 - RX Data Valid"]
    #[inline(always)]
    pub fn rxdatav(&self) -> RXDATAV_R {
        RXDATAV_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 8 - RX FIFO Full"]
    #[inline(always)]
    pub fn rxfull(&self) -> RXFULL_R {
        RXFULL_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 9 - TX Buffer Expects Double Right Data"]
    #[inline(always)]
    pub fn txbdright(&self) -> TXBDRIGHT_R {
        TXBDRIGHT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 10 - TX Buffer Expects Single Right Data"]
    #[inline(always)]
    pub fn txbsright(&self) -> TXBSRIGHT_R {
        TXBSRIGHT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 11 - RX Data Right"]
    #[inline(always)]
    pub fn rxdatavright(&self) -> RXDATAVRIGHT_R {
        RXDATAVRIGHT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 12 - RX Full of Right Data"]
    #[inline(always)]
    pub fn rxfullright(&self) -> RXFULLRIGHT_R {
        RXFULLRIGHT_R::new(((self.bits >> 12) & 0x01) != 0)
    }
}
#[doc = "USART Status Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [status](index.html) module"]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [status::R](R) reader structure"]
impl crate::Readable for STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATUS to value 0x40"]
impl crate::Resettable for STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x40
    }
}
