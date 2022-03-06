#[doc = "Register `MSIZE` reader"]
pub struct R(crate::R<MSIZE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSIZE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSIZE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSIZE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `FLASH` reader - Flash size in kilobytes"]
pub struct FLASH_R(crate::FieldReader<u16, u16>);
impl FLASH_R {
    pub(crate) fn new(bits: u16) -> Self {
        FLASH_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FLASH_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SRAM` reader - SRAM size in kilobytes"]
pub struct SRAM_R(crate::FieldReader<u16, u16>);
impl SRAM_R {
    pub(crate) fn new(bits: u16) -> Self {
        SRAM_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SRAM_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl R {
    #[doc = "Bits 0:15 - Flash size in kilobytes"]
    #[inline(always)]
    pub fn flash(&self) -> FLASH_R {
        FLASH_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - SRAM size in kilobytes"]
    #[inline(always)]
    pub fn sram(&self) -> SRAM_R {
        SRAM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[doc = "Flash and SRAM Memory size in KiloBytes\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [msize](index.html) module"]
pub struct MSIZE_SPEC;
impl crate::RegisterSpec for MSIZE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [msize::R](R) reader structure"]
impl crate::Readable for MSIZE_SPEC {
    type Reader = R;
}
