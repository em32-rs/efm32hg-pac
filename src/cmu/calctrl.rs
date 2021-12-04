#[doc = "Register `CALCTRL` reader"]
pub struct R(crate::R<CALCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CALCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CALCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CALCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CALCTRL` writer"]
pub struct W(crate::W<CALCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CALCTRL_SPEC>;
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
impl From<crate::W<CALCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CALCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Calibration Up-counter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum UPSEL_A {
    #[doc = "0: Select HFXO as up-counter."]
    HFXO = 0,
    #[doc = "1: Select LFXO as up-counter."]
    LFXO = 1,
    #[doc = "2: Select HFRCO as up-counter."]
    HFRCO = 2,
    #[doc = "3: Select LFRCO as up-counter."]
    LFRCO = 3,
    #[doc = "4: Select AUXHFRCO as up-counter."]
    AUXHFRCO = 4,
    #[doc = "5: Select USHFRCO as up-counter."]
    USHFRCO = 5,
}
impl From<UPSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: UPSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `UPSEL` reader - Calibration Up-counter Select"]
pub struct UPSEL_R(crate::FieldReader<u8, UPSEL_A>);
impl UPSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        UPSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<UPSEL_A> {
        match self.bits {
            0 => Some(UPSEL_A::HFXO),
            1 => Some(UPSEL_A::LFXO),
            2 => Some(UPSEL_A::HFRCO),
            3 => Some(UPSEL_A::LFRCO),
            4 => Some(UPSEL_A::AUXHFRCO),
            5 => Some(UPSEL_A::USHFRCO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        **self == UPSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        **self == UPSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        **self == UPSEL_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        **self == UPSEL_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        **self == UPSEL_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        **self == UPSEL_A::USHFRCO
    }
}
impl core::ops::Deref for UPSEL_R {
    type Target = crate::FieldReader<u8, UPSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `UPSEL` writer - Calibration Up-counter Select"]
pub struct UPSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> UPSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: UPSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select HFXO as up-counter."]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(UPSEL_A::HFXO)
    }
    #[doc = "Select LFXO as up-counter."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(UPSEL_A::LFXO)
    }
    #[doc = "Select HFRCO as up-counter."]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::HFRCO)
    }
    #[doc = "Select LFRCO as up-counter."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::LFRCO)
    }
    #[doc = "Select AUXHFRCO as up-counter."]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::AUXHFRCO)
    }
    #[doc = "Select USHFRCO as up-counter."]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(UPSEL_A::USHFRCO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Calibration Down-counter Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum DOWNSEL_A {
    #[doc = "0: Select HFCLK for down-counter."]
    HFCLK = 0,
    #[doc = "1: Select HFXO for down-counter."]
    HFXO = 1,
    #[doc = "2: Select LFXO for down-counter."]
    LFXO = 2,
    #[doc = "3: Select HFRCO for down-counter."]
    HFRCO = 3,
    #[doc = "4: Select LFRCO for down-counter."]
    LFRCO = 4,
    #[doc = "5: Select AUXHFRCO for down-counter."]
    AUXHFRCO = 5,
    #[doc = "6: Select USHFRCO for down-counter."]
    USHFRCO = 6,
}
impl From<DOWNSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DOWNSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `DOWNSEL` reader - Calibration Down-counter Select"]
pub struct DOWNSEL_R(crate::FieldReader<u8, DOWNSEL_A>);
impl DOWNSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        DOWNSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DOWNSEL_A> {
        match self.bits {
            0 => Some(DOWNSEL_A::HFCLK),
            1 => Some(DOWNSEL_A::HFXO),
            2 => Some(DOWNSEL_A::LFXO),
            3 => Some(DOWNSEL_A::HFRCO),
            4 => Some(DOWNSEL_A::LFRCO),
            5 => Some(DOWNSEL_A::AUXHFRCO),
            6 => Some(DOWNSEL_A::USHFRCO),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HFCLK`"]
    #[inline(always)]
    pub fn is_hfclk(&self) -> bool {
        **self == DOWNSEL_A::HFCLK
    }
    #[doc = "Checks if the value of the field is `HFXO`"]
    #[inline(always)]
    pub fn is_hfxo(&self) -> bool {
        **self == DOWNSEL_A::HFXO
    }
    #[doc = "Checks if the value of the field is `LFXO`"]
    #[inline(always)]
    pub fn is_lfxo(&self) -> bool {
        **self == DOWNSEL_A::LFXO
    }
    #[doc = "Checks if the value of the field is `HFRCO`"]
    #[inline(always)]
    pub fn is_hfrco(&self) -> bool {
        **self == DOWNSEL_A::HFRCO
    }
    #[doc = "Checks if the value of the field is `LFRCO`"]
    #[inline(always)]
    pub fn is_lfrco(&self) -> bool {
        **self == DOWNSEL_A::LFRCO
    }
    #[doc = "Checks if the value of the field is `AUXHFRCO`"]
    #[inline(always)]
    pub fn is_auxhfrco(&self) -> bool {
        **self == DOWNSEL_A::AUXHFRCO
    }
    #[doc = "Checks if the value of the field is `USHFRCO`"]
    #[inline(always)]
    pub fn is_ushfrco(&self) -> bool {
        **self == DOWNSEL_A::USHFRCO
    }
}
impl core::ops::Deref for DOWNSEL_R {
    type Target = crate::FieldReader<u8, DOWNSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `DOWNSEL` writer - Calibration Down-counter Select"]
pub struct DOWNSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> DOWNSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: DOWNSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Select HFCLK for down-counter."]
    #[inline(always)]
    pub fn hfclk(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HFCLK)
    }
    #[doc = "Select HFXO for down-counter."]
    #[inline(always)]
    pub fn hfxo(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HFXO)
    }
    #[doc = "Select LFXO for down-counter."]
    #[inline(always)]
    pub fn lfxo(self) -> &'a mut W {
        self.variant(DOWNSEL_A::LFXO)
    }
    #[doc = "Select HFRCO for down-counter."]
    #[inline(always)]
    pub fn hfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::HFRCO)
    }
    #[doc = "Select LFRCO for down-counter."]
    #[inline(always)]
    pub fn lfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::LFRCO)
    }
    #[doc = "Select AUXHFRCO for down-counter."]
    #[inline(always)]
    pub fn auxhfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::AUXHFRCO)
    }
    #[doc = "Select USHFRCO for down-counter."]
    #[inline(always)]
    pub fn ushfrco(self) -> &'a mut W {
        self.variant(DOWNSEL_A::USHFRCO)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | ((value as u32 & 0x07) << 3);
        self.w
    }
}
#[doc = "Field `CONT` reader - Continuous Calibration"]
pub struct CONT_R(crate::FieldReader<bool, bool>);
impl CONT_R {
    pub(crate) fn new(bits: bool) -> Self {
        CONT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CONT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CONT` writer - Continuous Calibration"]
pub struct CONT_W<'a> {
    w: &'a mut W,
}
impl<'a> CONT_W<'a> {
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
impl R {
    #[doc = "Bits 0:2 - Calibration Up-counter Select"]
    #[inline(always)]
    pub fn upsel(&self) -> UPSEL_R {
        UPSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 3:5 - Calibration Down-counter Select"]
    #[inline(always)]
    pub fn downsel(&self) -> DOWNSEL_R {
        DOWNSEL_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bit 6 - Continuous Calibration"]
    #[inline(always)]
    pub fn cont(&self) -> CONT_R {
        CONT_R::new(((self.bits >> 6) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Calibration Up-counter Select"]
    #[inline(always)]
    pub fn upsel(&mut self) -> UPSEL_W {
        UPSEL_W { w: self }
    }
    #[doc = "Bits 3:5 - Calibration Down-counter Select"]
    #[inline(always)]
    pub fn downsel(&mut self) -> DOWNSEL_W {
        DOWNSEL_W { w: self }
    }
    #[doc = "Bit 6 - Continuous Calibration"]
    #[inline(always)]
    pub fn cont(&mut self) -> CONT_W {
        CONT_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Calibration Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [calctrl](index.html) module"]
pub struct CALCTRL_SPEC;
impl crate::RegisterSpec for CALCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [calctrl::R](R) reader structure"]
impl crate::Readable for CALCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [calctrl::W](W) writer structure"]
impl crate::Writable for CALCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CALCTRL to value 0"]
impl crate::Resettable for CALCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
