#[doc = "Register `XORDATA` reader"]
pub struct R(crate::R<XORDATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<XORDATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<XORDATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<XORDATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `XORDATA` writer"]
pub struct W(crate::W<XORDATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<XORDATA_SPEC>;
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
impl From<crate::W<XORDATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<XORDATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `XORDATA` reader - XOR Data Access"]
pub struct XORDATA_R(crate::FieldReader<u32, u32>);
impl XORDATA_R {
    pub(crate) fn new(bits: u32) -> Self {
        XORDATA_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for XORDATA_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `XORDATA` writer - XOR Data Access"]
pub struct XORDATA_W<'a> {
    w: &'a mut W,
}
impl<'a> XORDATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - XOR Data Access"]
    #[inline(always)]
    pub fn xordata(&self) -> XORDATA_R {
        XORDATA_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - XOR Data Access"]
    #[inline(always)]
    pub fn xordata(&mut self) -> XORDATA_W {
        XORDATA_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "XORDATA Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [xordata](index.html) module"]
pub struct XORDATA_SPEC;
impl crate::RegisterSpec for XORDATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [xordata::R](R) reader structure"]
impl crate::Readable for XORDATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [xordata::W](W) writer structure"]
impl crate::Writable for XORDATA_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets XORDATA to value 0"]
impl crate::Resettable for XORDATA_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
