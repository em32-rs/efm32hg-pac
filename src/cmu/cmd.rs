#[doc = "Register `CMD` writer"]
pub struct W(crate::W<CMD_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMD_SPEC>;
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
impl From<crate::W<CMD_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMD_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "HFCLK Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum HFCLKSEL_AW {
    #[doc = "1: Select HFRCO as HFCLK."]
    HFRCO = 1,
    #[doc = "2: Select HFXO as HFCLK."]
    HFXO = 2,
    #[doc = "3: Select LFRCO as HFCLK."]
    LFRCO = 3,
    #[doc = "4: Select LFXO as HFCLK."]
    LFXO = 4,
    #[doc = "5: Select USHFRCO divided by two as HFCLK."]
    USHFRCODIV2 = 5,
}
impl From<HFCLKSEL_AW> for u8 {
    #[inline(always)]
    fn from(variant: HFCLKSEL_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `HFCLKSEL` writer - HFCLK Select"]
pub struct HFCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HFCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: HFCLKSEL_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select HFRCO as HFCLK."]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(HFCLKSEL_AW::HFRCO)
    }
    #[doc = "Select HFXO as HFCLK."]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(HFCLKSEL_AW::HFXO)
    }
    #[doc = "Select LFRCO as HFCLK."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(HFCLKSEL_AW::LFRCO)
    }
    #[doc = "Select LFXO as HFCLK."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(HFCLKSEL_AW::LFXO)
    }
    #[doc = "Select USHFRCO divided by two as HFCLK."]
    #[inline(always)]
    pub fn ushfrcodiv2(self) -> &'a mut W {
        self.variant(HFCLKSEL_AW::USHFRCODIV2)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Field `CALSTART` writer - Calibration Start"]
pub struct CALSTART_W<'a> {
    w: &'a mut W,
}
impl<'a> CALSTART_W<'a> {
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
#[doc = "Field `CALSTOP` writer - Calibration Stop"]
pub struct CALSTOP_W<'a> {
    w: &'a mut W,
}
impl<'a> CALSTOP_W<'a> {
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
#[doc = "USB Core Clock Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum USBCCLKSEL_AW {
    #[doc = "2: Select LFXO as HFCORECLKUSBC."]
    LFXO = 2,
    #[doc = "3: Select LFRCO as HFCORECLKUSBC."]
    LFRCO = 3,
    #[doc = "4: Select USHFRCO as HFCORECLKUSBC."]
    USHFRCO = 4,
}
impl From<USBCCLKSEL_AW> for u8 {
    #[inline(always)]
    fn from(variant: USBCCLKSEL_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `USBCCLKSEL` writer - USB Core Clock Select"]
pub struct USBCCLKSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> USBCCLKSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: USBCCLKSEL_AW) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select LFXO as HFCORECLKUSBC."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(USBCCLKSEL_AW::LFXO)
    }
    #[doc = "Select LFRCO as HFCORECLKUSBC."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(USBCCLKSEL_AW::LFRCO)
    }
    #[doc = "Select USHFRCO as HFCORECLKUSBC."]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(USBCCLKSEL_AW::USHFRCO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 5)) | ((value as u32 & 0x07) << 5);
        self.w
    }
}
impl W {
    #[doc = "Bits 0:2 - HFCLK Select"]
    #[inline(always)]
    pub fn hfclksel(&mut self) -> HFCLKSEL_W {
        HFCLKSEL_W { w: self }
    }
    #[doc = "Bit 3 - Calibration Start"]
    #[inline(always)]
    pub fn calstart(&mut self) -> CALSTART_W {
        CALSTART_W { w: self }
    }
    #[doc = "Bit 4 - Calibration Stop"]
    #[inline(always)]
    pub fn calstop(&mut self) -> CALSTOP_W {
        CALSTOP_W { w: self }
    }
    #[doc = "Bits 5:7 - USB Core Clock Select"]
    #[inline(always)]
    pub fn usbcclksel(&mut self) -> USBCCLKSEL_W {
        USBCCLKSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Command Register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd](index.html) module"]
pub struct CMD_SPEC;
impl crate::RegisterSpec for CMD_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [cmd::W](W) writer structure"]
impl crate::Writable for CMD_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CMD to value 0"]
impl crate::Resettable for CMD_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
