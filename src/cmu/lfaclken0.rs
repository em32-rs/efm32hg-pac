#[doc = "Register `LFACLKEN0` reader"]
pub struct R(crate::R<LFACLKEN0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LFACLKEN0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LFACLKEN0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LFACLKEN0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LFACLKEN0` writer"]
pub struct W(crate::W<LFACLKEN0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LFACLKEN0_SPEC>;
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
impl From<crate::W<LFACLKEN0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LFACLKEN0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC` reader - Real-Time Counter Clock Enable"]
pub type RTC_R = crate::BitReader<bool>;
#[doc = "Field `RTC` writer - Real-Time Counter Clock Enable"]
pub type RTC_W<'a, const O: u8> = crate::BitWriter<'a, u32, LFACLKEN0_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Real-Time Counter Clock Enable"]
    #[inline(always)]
    pub fn rtc(&self) -> RTC_R {
        RTC_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Real-Time Counter Clock Enable"]
    #[inline(always)]
    pub fn rtc(&mut self) -> RTC_W<0> {
        RTC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Low Frequency A Clock Enable Register 0 (Async Reg)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lfaclken0](index.html) module"]
pub struct LFACLKEN0_SPEC;
impl crate::RegisterSpec for LFACLKEN0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lfaclken0::R](R) reader structure"]
impl crate::Readable for LFACLKEN0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lfaclken0::W](W) writer structure"]
impl crate::Writable for LFACLKEN0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LFACLKEN0 to value 0"]
impl crate::Resettable for LFACLKEN0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
