#[doc = "Register `USHFRCOTUNE` reader"]
pub struct R(crate::R<USHFRCOTUNE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USHFRCOTUNE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USHFRCOTUNE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USHFRCOTUNE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USHFRCOTUNE` writer"]
pub struct W(crate::W<USHFRCOTUNE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USHFRCOTUNE_SPEC>;
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
impl From<crate::W<USHFRCOTUNE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USHFRCOTUNE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FINETUNING` reader - Oscillator fine frequency adjust"]
pub type FINETUNING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `FINETUNING` writer - Oscillator fine frequency adjust"]
pub type FINETUNING_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, USHFRCOTUNE_SPEC, u8, u8, 6, O>;
impl R {
    #[doc = "Bits 0:5 - Oscillator fine frequency adjust"]
    #[inline(always)]
    pub fn finetuning(&self) -> FINETUNING_R {
        FINETUNING_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:5 - Oscillator fine frequency adjust"]
    #[inline(always)]
    pub fn finetuning(&mut self) -> FINETUNING_W<0> {
        FINETUNING_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USHFRCO Frequency Tune\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ushfrcotune](index.html) module"]
pub struct USHFRCOTUNE_SPEC;
impl crate::RegisterSpec for USHFRCOTUNE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ushfrcotune::R](R) reader structure"]
impl crate::Readable for USHFRCOTUNE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ushfrcotune::W](W) writer structure"]
impl crate::Writable for USHFRCOTUNE_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USHFRCOTUNE to value 0x20"]
impl crate::Resettable for USHFRCOTUNE_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x20
    }
}
