#[doc = "Register `PC_PINLOCKN` reader"]
pub struct R(crate::R<PC_PINLOCKN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PC_PINLOCKN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PC_PINLOCKN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PC_PINLOCKN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PC_PINLOCKN` writer"]
pub struct W(crate::W<PC_PINLOCKN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PC_PINLOCKN_SPEC>;
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
impl From<crate::W<PC_PINLOCKN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PC_PINLOCKN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PINLOCKN` reader - Unlocked Pins"]
pub type PINLOCKN_R = crate::FieldReader<u16, u16>;
#[doc = "Field `PINLOCKN` writer - Unlocked Pins"]
pub type PINLOCKN_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, PC_PINLOCKN_SPEC, u16, u16, 16, O>;
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
    pub fn pinlockn(&mut self) -> PINLOCKN_W<0> {
        PINLOCKN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Port Unlocked Pins Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pc_pinlockn](index.html) module"]
pub struct PC_PINLOCKN_SPEC;
impl crate::RegisterSpec for PC_PINLOCKN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pc_pinlockn::R](R) reader structure"]
impl crate::Readable for PC_PINLOCKN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pc_pinlockn::W](W) writer structure"]
impl crate::Writable for PC_PINLOCKN_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PC_PINLOCKN to value 0xffff"]
impl crate::Resettable for PC_PINLOCKN_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0xffff
    }
}
