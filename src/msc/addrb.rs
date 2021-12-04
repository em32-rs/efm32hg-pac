#[doc = "Register `ADDRB` reader"]
pub struct R(crate::R<ADDRB_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADDRB_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADDRB_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADDRB_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADDRB` writer"]
pub struct W(crate::W<ADDRB_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADDRB_SPEC>;
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
impl From<crate::W<ADDRB_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADDRB_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADDRB` reader - Page Erase or Write Address Buffer"]
pub struct ADDRB_R(crate::FieldReader<u32, u32>);
impl ADDRB_R {
    pub(crate) fn new(bits: u32) -> Self {
        ADDRB_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ADDRB_R {
    type Target = crate::FieldReader<u32, u32>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ADDRB` writer - Page Erase or Write Address Buffer"]
pub struct ADDRB_W<'a> {
    w: &'a mut W,
}
impl<'a> ADDRB_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | (value as u32 & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - Page Erase or Write Address Buffer"]
    #[inline(always)]
    pub fn addrb(&self) -> ADDRB_R {
        ADDRB_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - Page Erase or Write Address Buffer"]
    #[inline(always)]
    pub fn addrb(&mut self) -> ADDRB_W {
        ADDRB_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Page Erase/Write Address Buffer\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [addrb](index.html) module"]
pub struct ADDRB_SPEC;
impl crate::RegisterSpec for ADDRB_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [addrb::R](R) reader structure"]
impl crate::Readable for ADDRB_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [addrb::W](W) writer structure"]
impl crate::Writable for ADDRB_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADDRB to value 0"]
impl crate::Resettable for ADDRB_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
