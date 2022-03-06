#[doc = "Register `DUTYCONFIG` reader"]
pub struct R(crate::R<DUTYCONFIG_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DUTYCONFIG_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DUTYCONFIG_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DUTYCONFIG_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DUTYCONFIG` writer"]
pub struct W(crate::W<DUTYCONFIG_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DUTYCONFIG_SPEC>;
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
impl From<crate::W<DUTYCONFIG_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DUTYCONFIG_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUTYCYCLEEN` reader - Duty Cycle Enable."]
pub struct DUTYCYCLEEN_R(crate::FieldReader<bool, bool>);
impl DUTYCYCLEEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        DUTYCYCLEEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for DUTYCYCLEEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DUTYCYCLEEN` writer - Duty Cycle Enable."]
pub struct DUTYCYCLEEN_W<'a> {
    w: &'a mut W,
}
impl<'a> DUTYCYCLEEN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01) | (value as u32 & 0x01);
        self.w
    }
}
#[doc = "Field `EM2DUTYCYCLEDIS` reader - EM2/EM3 Duty Cycle Disable."]
pub struct EM2DUTYCYCLEDIS_R(crate::FieldReader<bool, bool>);
impl EM2DUTYCYCLEDIS_R {
    pub(crate) fn new(bits: bool) -> Self {
        EM2DUTYCYCLEDIS_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for EM2DUTYCYCLEDIS_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EM2DUTYCYCLEDIS` writer - EM2/EM3 Duty Cycle Disable."]
pub struct EM2DUTYCYCLEDIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EM2DUTYCYCLEDIS_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 1)) | ((value as u32 & 0x01) << 1);
        self.w
    }
}
impl R {
    #[doc = "Bit 0 - Duty Cycle Enable."]
    #[inline(always)]
    pub fn dutycycleen(&self) -> DUTYCYCLEEN_R {
        DUTYCYCLEEN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bit 1 - EM2/EM3 Duty Cycle Disable."]
    #[inline(always)]
    pub fn em2dutycycledis(&self) -> EM2DUTYCYCLEDIS_R {
        EM2DUTYCYCLEDIS_R::new(((self.bits >> 1) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Duty Cycle Enable."]
    #[inline(always)]
    pub fn dutycycleen(&mut self) -> DUTYCYCLEEN_W {
        DUTYCYCLEEN_W { w: self }
    }
    #[doc = "Bit 1 - EM2/EM3 Duty Cycle Disable."]
    #[inline(always)]
    pub fn em2dutycycledis(&mut self) -> EM2DUTYCYCLEDIS_W {
        EM2DUTYCYCLEDIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Duty Cycle Configauration Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dutyconfig](index.html) module"]
pub struct DUTYCONFIG_SPEC;
impl crate::RegisterSpec for DUTYCONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dutyconfig::R](R) reader structure"]
impl crate::Readable for DUTYCONFIG_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dutyconfig::W](W) writer structure"]
impl crate::Writable for DUTYCONFIG_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets DUTYCONFIG to value 0"]
impl crate::Resettable for DUTYCONFIG_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
