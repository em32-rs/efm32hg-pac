#[doc = "Register `RXDATAXP` reader"]
pub struct R(crate::R<RXDATAXP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDATAXP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDATAXP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDATAXP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATAP` reader - RX Data Peek"]
pub struct RXDATAP_R(crate::FieldReader<u16, u16>);
impl RXDATAP_R {
    pub(crate) fn new(bits: u16) -> Self {
        RXDATAP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDATAP_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PERRP` reader - Receive Data Parity Error Peek"]
pub struct PERRP_R(crate::FieldReader<bool, bool>);
impl PERRP_R {
    pub(crate) fn new(bits: bool) -> Self {
        PERRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PERRP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FERRP` reader - Receive Data Framing Error Peek"]
pub struct FERRP_R(crate::FieldReader<bool, bool>);
impl FERRP_R {
    pub(crate) fn new(bits: bool) -> Self {
        FERRP_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FERRP_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:8 - RX Data Peek"]
    #[inline(always)]
    pub fn rxdatap(&self) -> RXDATAP_R {
        RXDATAP_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bit 14 - Receive Data Parity Error Peek"]
    #[inline(always)]
    pub fn perrp(&self) -> PERRP_R {
        PERRP_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 15 - Receive Data Framing Error Peek"]
    #[inline(always)]
    pub fn ferrp(&self) -> FERRP_R {
        FERRP_R::new(((self.bits >> 15) & 0x01) != 0)
    }
}
#[doc = "Receive Buffer Data Extended Peek Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdataxp](index.html) module"]
pub struct RXDATAXP_SPEC;
impl crate::RegisterSpec for RXDATAXP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdataxp::R](R) reader structure"]
impl crate::Readable for RXDATAXP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDATAXP to value 0"]
impl crate::Resettable for RXDATAXP_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
