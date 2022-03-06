#[doc = "Register `PF_PINLOCKN` reader"]
pub struct R(crate::R<PF_PINLOCKN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PF_PINLOCKN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PF_PINLOCKN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PF_PINLOCKN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PF_PINLOCKN` writer"]
pub struct W(crate::W<PF_PINLOCKN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PF_PINLOCKN_SPEC>;
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
impl From<crate::W<PF_PINLOCKN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PF_PINLOCKN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PINLOCKN` reader - Unlocked Pins"]
pub struct PINLOCKN_R(crate::FieldReader<u16, u16>);
impl PINLOCKN_R {
    pub(crate) fn new(bits: u16) -> Self {
        PINLOCKN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for PINLOCKN_R {
    type Target = crate::FieldReader<u16, u16>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `PINLOCKN` writer - Unlocked Pins"]
pub struct PINLOCKN_W<'a> {
    w: &'a mut W,
}
impl<'a> PINLOCKN_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | (value as u32 & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - Unlocked Pins"]
    #[inline(always)]
    pub fn pinlockn(&self) -> PINLOCKN_R {
        PINLOCKN_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - Unlocked Pins"]
    #[inline(always)]
    pub fn pinlockn(&mut self) -> PINLOCKN_W {
        PINLOCKN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pf_pinlockn](index.html) module"]
pub struct PF_PINLOCKN_SPEC;
impl crate::RegisterSpec for PF_PINLOCKN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pf_pinlockn::R](R) reader structure"]
impl crate::Readable for PF_PINLOCKN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pf_pinlockn::W](W) writer structure"]
impl crate::Writable for PF_PINLOCKN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PF_PINLOCKN to value 0xffff"]
impl crate::Resettable for PF_PINLOCKN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
