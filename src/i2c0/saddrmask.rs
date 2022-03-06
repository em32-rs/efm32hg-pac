#[doc = "Register `SADDRMASK` reader"]
pub struct R(crate::R<SADDRMASK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SADDRMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SADDRMASK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SADDRMASK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SADDRMASK` writer"]
pub struct W(crate::W<SADDRMASK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SADDRMASK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<SADDRMASK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SADDRMASK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MASK` reader - Slave Address Mask"]
pub struct MASK_R(crate::FieldReader<u8, u8>);
impl MASK_R {
    pub(crate) fn new(bits: u8) -> Self {
        MASK_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for MASK_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `MASK` writer - Slave Address Mask"]
pub struct MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> MASK_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 1)) | ((value as u32 & 0x7f) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bits 1:7 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&self) -> MASK_R {
        MASK_R::new(((self.bits >> 1) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 1:7 - Slave Address Mask"]
    #[inline(always)]
    pub fn mask(&mut self) -> MASK_W {
        MASK_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Slave Address Mask Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [saddrmask](index.html) module"]
pub struct SADDRMASK_SPEC;
impl crate::RegisterSpec for SADDRMASK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [saddrmask::R](R) reader structure"]
impl crate::Readable for SADDRMASK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [saddrmask::W](W) writer structure"]
impl crate::Writable for SADDRMASK_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SADDRMASK to value 0"]
impl crate::Resettable for SADDRMASK_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
