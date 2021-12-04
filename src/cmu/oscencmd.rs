#[doc = "Register `OSCENCMD` writer"]
pub struct W(crate::W<OSCENCMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<OSCENCMD_SPEC>;
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
impl From<crate::W<OSCENCMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<OSCENCMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HFRCOEN` writer - HFRCO Enable"]
pub struct HFRCOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HFRCOEN_W<'a> {
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
#[doc = "Field `HFRCODIS` writer - HFRCO Disable"]
pub struct HFRCODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HFRCODIS_W<'a> {
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
#[doc = "Field `HFXOEN` writer - HFXO Enable"]
pub struct HFXOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | ((value as u32 & 0x01) << 2);
        self.w
    }
}
#[doc = "Field `HFXODIS` writer - HFXO Disable"]
pub struct HFXODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> HFXODIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | ((value as u32 & 0x01) << 3);
        self.w
    }
}
#[doc = "Field `AUXHFRCOEN` writer - AUXHFRCO Enable"]
pub struct AUXHFRCOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXHFRCOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | ((value as u32 & 0x01) << 4);
        self.w
    }
}
#[doc = "Field `AUXHFRCODIS` writer - AUXHFRCO Disable"]
pub struct AUXHFRCODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> AUXHFRCODIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | ((value as u32 & 0x01) << 5);
        self.w
    }
}
#[doc = "Field `LFRCOEN` writer - LFRCO Enable"]
pub struct LFRCOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LFRCOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | ((value as u32 & 0x01) << 6);
        self.w
    }
}
#[doc = "Field `LFRCODIS` writer - LFRCO Disable"]
pub struct LFRCODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LFRCODIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | ((value as u32 & 0x01) << 7);
        self.w
    }
}
#[doc = "Field `LFXOEN` writer - LFXO Enable"]
pub struct LFXOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | ((value as u32 & 0x01) << 8);
        self.w
    }
}
#[doc = "Field `LFXODIS` writer - LFXO Disable"]
pub struct LFXODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> LFXODIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | ((value as u32 & 0x01) << 9);
        self.w
    }
}
#[doc = "Field `USHFRCOEN` writer - USHFRCO Enable"]
pub struct USHFRCOEN_W<'a> {
    w: &'a mut W,
}
impl<'a> USHFRCOEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | ((value as u32 & 0x01) << 10);
        self.w
    }
}
#[doc = "Field `USHFRCODIS` writer - USHFRCO Disable"]
pub struct USHFRCODIS_W<'a> {
    w: &'a mut W,
}
impl<'a> USHFRCODIS_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | ((value as u32 & 0x01) << 11);
        self.w
    }
}
impl W {
    #[doc = "Bit 0 - HFRCO Enable"]
    #[inline(always)]
    pub fn hfrcoen(&mut self) -> HFRCOEN_W {
        HFRCOEN_W { w: self }
    }
    #[doc = "Bit 1 - HFRCO Disable"]
    #[inline(always)]
    pub fn hfrcodis(&mut self) -> HFRCODIS_W {
        HFRCODIS_W { w: self }
    }
    #[doc = "Bit 2 - HFXO Enable"]
    #[inline(always)]
    pub fn hfxoen(&mut self) -> HFXOEN_W {
        HFXOEN_W { w: self }
    }
    #[doc = "Bit 3 - HFXO Disable"]
    #[inline(always)]
    pub fn hfxodis(&mut self) -> HFXODIS_W {
        HFXODIS_W { w: self }
    }
    #[doc = "Bit 4 - AUXHFRCO Enable"]
    #[inline(always)]
    pub fn auxhfrcoen(&mut self) -> AUXHFRCOEN_W {
        AUXHFRCOEN_W { w: self }
    }
    #[doc = "Bit 5 - AUXHFRCO Disable"]
    #[inline(always)]
    pub fn auxhfrcodis(&mut self) -> AUXHFRCODIS_W {
        AUXHFRCODIS_W { w: self }
    }
    #[doc = "Bit 6 - LFRCO Enable"]
    #[inline(always)]
    pub fn lfrcoen(&mut self) -> LFRCOEN_W {
        LFRCOEN_W { w: self }
    }
    #[doc = "Bit 7 - LFRCO Disable"]
    #[inline(always)]
    pub fn lfrcodis(&mut self) -> LFRCODIS_W {
        LFRCODIS_W { w: self }
    }
    #[doc = "Bit 8 - LFXO Enable"]
    #[inline(always)]
    pub fn lfxoen(&mut self) -> LFXOEN_W {
        LFXOEN_W { w: self }
    }
    #[doc = "Bit 9 - LFXO Disable"]
    #[inline(always)]
    pub fn lfxodis(&mut self) -> LFXODIS_W {
        LFXODIS_W { w: self }
    }
    #[doc = "Bit 10 - USHFRCO Enable"]
    #[inline(always)]
    pub fn ushfrcoen(&mut self) -> USHFRCOEN_W {
        USHFRCOEN_W { w: self }
    }
    #[doc = "Bit 11 - USHFRCO Disable"]
    #[inline(always)]
    pub fn ushfrcodis(&mut self) -> USHFRCODIS_W {
        USHFRCODIS_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Oscillator Enable/Disable Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [oscencmd](index.html) module"]
pub struct OSCENCMD_SPEC;
impl crate::RegisterSpec for OSCENCMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [oscencmd::W](W) writer structure"]
impl crate::Writable for OSCENCMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets OSCENCMD to value 0"]
impl crate::Resettable for OSCENCMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
