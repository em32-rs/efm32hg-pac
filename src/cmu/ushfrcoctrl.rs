#[doc = "Register `USHFRCOCTRL` reader"]
pub struct R(crate::R<USHFRCOCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<USHFRCOCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<USHFRCOCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<USHFRCOCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `USHFRCOCTRL` writer"]
pub struct W(crate::W<USHFRCOCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<USHFRCOCTRL_SPEC>;
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
impl From<crate::W<USHFRCOCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<USHFRCOCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TUNING` reader - USHFRCO frequency adjust"]
pub type TUNING_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TUNING` writer - USHFRCO frequency adjust"]
pub type TUNING_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USHFRCOCTRL_SPEC, u8, u8, 7, O>;
#[doc = "Field `DITHEN` reader - USHFRCO dither enable"]
pub type DITHEN_R = crate::BitReader<bool>;
#[doc = "Field `DITHEN` writer - USHFRCO dither enable"]
pub type DITHEN_W<'a, const O: u8> = crate::BitWriter<'a, u32, USHFRCOCTRL_SPEC, bool, O>;
#[doc = "Field `SUSPEND` reader - USHFRCO suspend"]
pub type SUSPEND_R = crate::BitReader<bool>;
#[doc = "Field `SUSPEND` writer - USHFRCO suspend"]
pub type SUSPEND_W<'a, const O: u8> = crate::BitWriter<'a, u32, USHFRCOCTRL_SPEC, bool, O>;
#[doc = "Field `TIMEOUT` reader - USHFRCO Timeout"]
pub type TIMEOUT_R = crate::FieldReader<u8, u8>;
#[doc = "Field `TIMEOUT` writer - USHFRCO Timeout"]
pub type TIMEOUT_W<'a, const O: u8> = crate::FieldWriter<'a, u32, USHFRCOCTRL_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:6 - USHFRCO frequency adjust"]
    #[inline(always)]
    pub fn tuning(&self) -> TUNING_R {
        TUNING_R::new((self.bits & 0x7f) as u8)
    }
    #[doc = "Bit 8 - USHFRCO dither enable"]
    #[inline(always)]
    pub fn dithen(&self) -> DITHEN_R {
        DITHEN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - USHFRCO suspend"]
    #[inline(always)]
    pub fn suspend(&self) -> SUSPEND_R {
        SUSPEND_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:19 - USHFRCO Timeout"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:6 - USHFRCO frequency adjust"]
    #[inline(always)]
    pub fn tuning(&mut self) -> TUNING_W<0> {
        TUNING_W::new(self)
    }
    #[doc = "Bit 8 - USHFRCO dither enable"]
    #[inline(always)]
    pub fn dithen(&mut self) -> DITHEN_W<8> {
        DITHEN_W::new(self)
    }
    #[doc = "Bit 9 - USHFRCO suspend"]
    #[inline(always)]
    pub fn suspend(&mut self) -> SUSPEND_W<9> {
        SUSPEND_W::new(self)
    }
    #[doc = "Bits 12:19 - USHFRCO Timeout"]
    #[inline(always)]
    pub fn timeout(&mut self) -> TIMEOUT_W<12> {
        TIMEOUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "USHFRCO Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ushfrcoctrl](index.html) module"]
pub struct USHFRCOCTRL_SPEC;
impl crate::RegisterSpec for USHFRCOCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ushfrcoctrl::R](R) reader structure"]
impl crate::Readable for USHFRCOCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ushfrcoctrl::W](W) writer structure"]
impl crate::Writable for USHFRCOCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets USHFRCOCTRL to value 0x000f_f040"]
impl crate::Resettable for USHFRCOCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x000f_f040
    }
}
