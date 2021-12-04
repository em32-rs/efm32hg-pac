#[doc = "Register `RXDOUBLE` reader"]
pub struct R(crate::R<RXDOUBLE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXDOUBLE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXDOUBLE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXDOUBLE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXDATA0` reader - RX Data 0"]
pub struct RXDATA0_R(crate::FieldReader<u8, u8>);
impl RXDATA0_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXDATA0_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDATA0_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `RXDATA1` reader - RX Data 1"]
pub struct RXDATA1_R(crate::FieldReader<u8, u8>);
impl RXDATA1_R {
    pub(crate) fn new(bits: u8) -> Self {
        RXDATA1_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for RXDATA1_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:7 - RX Data 0"]
    #[inline(always)]
    pub fn rxdata0(&self) -> RXDATA0_R {
        RXDATA0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - RX Data 1"]
    #[inline(always)]
    pub fn rxdata1(&self) -> RXDATA1_R {
        RXDATA1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[doc = "RX FIFO Double Data Register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxdouble](index.html) module"]
pub struct RXDOUBLE_SPEC;
impl crate::RegisterSpec for RXDOUBLE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxdouble::R](R) reader structure"]
impl crate::Readable for RXDOUBLE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXDOUBLE to value 0"]
impl crate::Resettable for RXDOUBLE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
