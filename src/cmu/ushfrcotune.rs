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
pub struct FINETUNING_R(crate::FieldReader<u8, u8>);
impl FINETUNING_R {
    pub(crate) fn new(bits: u8) -> Self {
        FINETUNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for FINETUNING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `FINETUNING` writer - Oscillator fine frequency adjust"]
pub struct FINETUNING_W<'a> {
    w: &'a mut W,
}
impl<'a> FINETUNING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | (value as u32 & 0x3f);
        self.w
    }
}
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
    pub fn finetuning(&mut self) -> FINETUNING_W {
        FINETUNING_W { w: self }
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
