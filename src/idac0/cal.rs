#[doc = "Register `CAL` reader"]
pub struct R(crate::R<CAL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CAL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CAL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CAL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CAL` writer"]
pub struct W(crate::W<CAL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CAL_SPEC>;
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
impl From<crate::W<CAL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CAL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUNING` reader - Tune the current to given accuracy"]
pub struct TUNING_R(crate::FieldReader<u8, u8>);
impl TUNING_R {
    pub(crate) fn new(bits: u8) -> Self {
        TUNING_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for TUNING_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `TUNING` writer - Tune the current to given accuracy"]
pub struct TUNING_W<'a> {
    w: &'a mut W,
}
impl<'a> TUNING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x7f) | (value as u32 & 0x7f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:6 - Tune the current to given accuracy"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - Tune the current to given accuracy"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TUNING_W {
        TUNING_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cal](index.html) module"]
pub struct CAL_SPEC;
impl crate::RegisterSpec for CAL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cal::R](R) reader structure"]
impl crate::Readable for CAL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cal::W](W) writer structure"]
impl crate::Writable for CAL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CAL to value 0"]
impl crate::Resettable for CAL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
