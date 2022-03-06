#[doc = "Register `INPUTSEL` reader"]
pub struct R(crate::R<INPUTSEL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INPUTSEL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INPUTSEL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INPUTSEL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INPUTSEL` writer"]
pub struct W(crate::W<INPUTSEL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INPUTSEL_SPEC>;
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
impl From<crate::W<INPUTSEL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INPUTSEL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Positive Input Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum POSSEL_A {
    #[doc = "0: Channel 0 as positive input."]
    CH0 = 0,
    #[doc = "1: Channel 1 as positive input."]
    CH1 = 1,
    #[doc = "2: Channel 2 as positive input."]
    CH2 = 2,
    #[doc = "3: Channel 3 as positive input."]
    CH3 = 3,
    #[doc = "4: Channel 4 as positive input."]
    CH4 = 4,
    #[doc = "5: Channel 5 as positive input."]
    CH5 = 5,
    #[doc = "6: Channel 6 as positive input."]
    CH6 = 6,
    #[doc = "7: Channel 7 as positive input."]
    CH7 = 7,
}
impl From<POSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: POSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `POSSEL` reader - Positive Input Select"]
pub struct POSSEL_R(crate::FieldReader<u8, POSSEL_A>);
impl POSSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        POSSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POSSEL_A {
        match self.bits {
            0 => POSSEL_A::CH0,
            1 => POSSEL_A::CH1,
            2 => POSSEL_A::CH2,
            3 => POSSEL_A::CH3,
            4 => POSSEL_A::CH4,
            5 => POSSEL_A::CH5,
            6 => POSSEL_A::CH6,
            7 => POSSEL_A::CH7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        **self == POSSEL_A::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        **self == POSSEL_A::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        **self == POSSEL_A::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        **self == POSSEL_A::CH3
    }
    #[doc = "Checks if the value of the field is `CH4`"]
    #[inline(always)]
    pub fn is_ch4(&self) -> bool {
        **self == POSSEL_A::CH4
    }
    #[doc = "Checks if the value of the field is `CH5`"]
    #[inline(always)]
    pub fn is_ch5(&self) -> bool {
        **self == POSSEL_A::CH5
    }
    #[doc = "Checks if the value of the field is `CH6`"]
    #[inline(always)]
    pub fn is_ch6(&self) -> bool {
        **self == POSSEL_A::CH6
    }
    #[doc = "Checks if the value of the field is `CH7`"]
    #[inline(always)]
    pub fn is_ch7(&self) -> bool {
        **self == POSSEL_A::CH7
    }
}
impl core::ops::Deref for POSSEL_R {
    type Target = crate::FieldReader<u8, POSSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `POSSEL` writer - Positive Input Select"]
pub struct POSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> POSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: POSSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Channel 0 as positive input."]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(POSSEL_A::CH0)
    }
    #[doc = "Channel 1 as positive input."]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(POSSEL_A::CH1)
    }
    #[doc = "Channel 2 as positive input."]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(POSSEL_A::CH2)
    }
    #[doc = "Channel 3 as positive input."]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(POSSEL_A::CH3)
    }
    #[doc = "Channel 4 as positive input."]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut W {
        self.variant(POSSEL_A::CH4)
    }
    #[doc = "Channel 5 as positive input."]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut W {
        self.variant(POSSEL_A::CH5)
    }
    #[doc = "Channel 6 as positive input."]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut W {
        self.variant(POSSEL_A::CH6)
    }
    #[doc = "Channel 7 as positive input."]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut W {
        self.variant(POSSEL_A::CH7)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Negative Input Select\n\nValue on reset: 8"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum NEGSEL_A {
    #[doc = "0: Channel 0 as negative input."]
    CH0 = 0,
    #[doc = "1: Channel 1 as negative input."]
    CH1 = 1,
    #[doc = "2: Channel 2 as negative input."]
    CH2 = 2,
    #[doc = "3: Channel 3 as negative input."]
    CH3 = 3,
    #[doc = "4: Channel 4 as negative input."]
    CH4 = 4,
    #[doc = "5: Channel 5 as negative input."]
    CH5 = 5,
    #[doc = "6: Channel 6 as negative input."]
    CH6 = 6,
    #[doc = "7: Channel 7 as negative input."]
    CH7 = 7,
    #[doc = "8: 1.25 V as negative input."]
    _1V25 = 8,
    #[doc = "9: 2.5 V as negative input."]
    _2V5 = 9,
    #[doc = "10: Scaled VDD as negative input."]
    VDD = 10,
    #[doc = "11: Capacitive sense mode."]
    CAPSENSE = 11,
}
impl From<NEGSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: NEGSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `NEGSEL` reader - Negative Input Select"]
pub struct NEGSEL_R(crate::FieldReader<u8, NEGSEL_A>);
impl NEGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        NEGSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<NEGSEL_A> {
        match self.bits {
            0 => Some(NEGSEL_A::CH0),
            1 => Some(NEGSEL_A::CH1),
            2 => Some(NEGSEL_A::CH2),
            3 => Some(NEGSEL_A::CH3),
            4 => Some(NEGSEL_A::CH4),
            5 => Some(NEGSEL_A::CH5),
            6 => Some(NEGSEL_A::CH6),
            7 => Some(NEGSEL_A::CH7),
            8 => Some(NEGSEL_A::_1V25),
            9 => Some(NEGSEL_A::_2V5),
            10 => Some(NEGSEL_A::VDD),
            11 => Some(NEGSEL_A::CAPSENSE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `CH0`"]
    #[inline(always)]
    pub fn is_ch0(&self) -> bool {
        **self == NEGSEL_A::CH0
    }
    #[doc = "Checks if the value of the field is `CH1`"]
    #[inline(always)]
    pub fn is_ch1(&self) -> bool {
        **self == NEGSEL_A::CH1
    }
    #[doc = "Checks if the value of the field is `CH2`"]
    #[inline(always)]
    pub fn is_ch2(&self) -> bool {
        **self == NEGSEL_A::CH2
    }
    #[doc = "Checks if the value of the field is `CH3`"]
    #[inline(always)]
    pub fn is_ch3(&self) -> bool {
        **self == NEGSEL_A::CH3
    }
    #[doc = "Checks if the value of the field is `CH4`"]
    #[inline(always)]
    pub fn is_ch4(&self) -> bool {
        **self == NEGSEL_A::CH4
    }
    #[doc = "Checks if the value of the field is `CH5`"]
    #[inline(always)]
    pub fn is_ch5(&self) -> bool {
        **self == NEGSEL_A::CH5
    }
    #[doc = "Checks if the value of the field is `CH6`"]
    #[inline(always)]
    pub fn is_ch6(&self) -> bool {
        **self == NEGSEL_A::CH6
    }
    #[doc = "Checks if the value of the field is `CH7`"]
    #[inline(always)]
    pub fn is_ch7(&self) -> bool {
        **self == NEGSEL_A::CH7
    }
    #[doc = "Checks if the value of the field is `_1V25`"]
    #[inline(always)]
    pub fn is_1v25(&self) -> bool {
        **self == NEGSEL_A::_1V25
    }
    #[doc = "Checks if the value of the field is `_2V5`"]
    #[inline(always)]
    pub fn is_2v5(&self) -> bool {
        **self == NEGSEL_A::_2V5
    }
    #[doc = "Checks if the value of the field is `VDD`"]
    #[inline(always)]
    pub fn is_vdd(&self) -> bool {
        **self == NEGSEL_A::VDD
    }
    #[doc = "Checks if the value of the field is `CAPSENSE`"]
    #[inline(always)]
    pub fn is_capsense(&self) -> bool {
        **self == NEGSEL_A::CAPSENSE
    }
}
impl core::ops::Deref for NEGSEL_R {
    type Target = crate::FieldReader<u8, NEGSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `NEGSEL` writer - Negative Input Select"]
pub struct NEGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> NEGSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: NEGSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Channel 0 as negative input."]
    #[inline(always)]
    pub fn ch0(self) -> &'a mut W {
        self.variant(NEGSEL_A::CH0)
    }
    #[doc = "Channel 1 as negative input."]
    #[inline(always)]
    pub fn ch1(self) -> &'a mut W {
        self.variant(NEGSEL_A::CH1)
    }
    #[doc = "Channel 2 as negative input."]
    #[inline(always)]
    pub fn ch2(self) -> &'a mut W {
        self.variant(NEGSEL_A::CH2)
    }
    #[doc = "Channel 3 as negative input."]
    #[inline(always)]
    pub fn ch3(self) -> &'a mut W {
        self.variant(NEGSEL_A::CH3)
    }
    #[doc = "Channel 4 as negative input."]
    #[inline(always)]
    pub fn ch4(self) -> &'a mut W {
        self.variant(NEGSEL_A::CH4)
    }
    #[doc = "Channel 5 as negative input."]
    #[inline(always)]
    pub fn ch5(self) -> &'a mut W {
        self.variant(NEGSEL_A::CH5)
    }
    #[doc = "Channel 6 as negative input."]
    #[inline(always)]
    pub fn ch6(self) -> &'a mut W {
        self.variant(NEGSEL_A::CH6)
    }
    #[doc = "Channel 7 as negative input."]
    #[inline(always)]
    pub fn ch7(self) -> &'a mut W {
        self.variant(NEGSEL_A::CH7)
    }
    #[doc = "1.25 V as negative input."]
    #[inline(always)]
    pub fn _1v25(self) -> &'a mut W {
        self.variant(NEGSEL_A::_1V25)
    }
    #[doc = "2.5 V as negative input."]
    #[inline(always)]
    pub fn _2v5(self) -> &'a mut W {
        self.variant(NEGSEL_A::_2V5)
    }
    #[doc = "Scaled VDD as negative input."]
    #[inline(always)]
    pub fn vdd(self) -> &'a mut W {
        self.variant(NEGSEL_A::VDD)
    }
    #[doc = "Capacitive sense mode."]
    #[inline(always)]
    pub fn capsense(self) -> &'a mut W {
        self.variant(NEGSEL_A::CAPSENSE)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 4)) | ((value as u32 & 0x0f) << 4);
        self.w
    }
}
#[doc = "Field `VDDLEVEL` reader - VDD Reference Level"]
pub struct VDDLEVEL_R(crate::FieldReader<u8, u8>);
impl VDDLEVEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        VDDLEVEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for VDDLEVEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `VDDLEVEL` writer - VDD Reference Level"]
pub struct VDDLEVEL_W<'a> {
    w: &'a mut W,
}
impl<'a> VDDLEVEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 8)) | ((value as u32 & 0x3f) << 8);
        self.w
    }
}
#[doc = "Field `LPREF` reader - Low Power Reference Mode"]
pub struct LPREF_R(crate::FieldReader<bool, bool>);
impl LPREF_R {
    pub(crate) fn new(bits: bool) -> Self {
        LPREF_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for LPREF_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `LPREF` writer - Low Power Reference Mode"]
pub struct LPREF_W<'a> {
    w: &'a mut W,
}
impl<'a> LPREF_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | ((value as u32 & 0x01) << 16);
        self.w
    }
}
#[doc = "Field `CSRESEN` reader - Capacitive Sense Mode Internal Resistor Enable"]
pub struct CSRESEN_R(crate::FieldReader<bool, bool>);
impl CSRESEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        CSRESEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for CSRESEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSRESEN` writer - Capacitive Sense Mode Internal Resistor Enable"]
pub struct CSRESEN_W<'a> {
    w: &'a mut W,
}
impl<'a> CSRESEN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 24)) | ((value as u32 & 0x01) << 24);
        self.w
    }
}
#[doc = "Capacitive Sense Mode Internal Resistor Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum CSRESSEL_A {
    #[doc = "0: Internal capacitive sense resistor value 0."]
    RES0 = 0,
    #[doc = "1: Internal capacitive sense resistor value 1."]
    RES1 = 1,
    #[doc = "2: Internal capacitive sense resistor value 2."]
    RES2 = 2,
    #[doc = "3: Internal capacitive sense resistor value 3."]
    RES3 = 3,
}
impl From<CSRESSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: CSRESSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `CSRESSEL` reader - Capacitive Sense Mode Internal Resistor Select"]
pub struct CSRESSEL_R(crate::FieldReader<u8, CSRESSEL_A>);
impl CSRESSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        CSRESSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CSRESSEL_A {
        match self.bits {
            0 => CSRESSEL_A::RES0,
            1 => CSRESSEL_A::RES1,
            2 => CSRESSEL_A::RES2,
            3 => CSRESSEL_A::RES3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RES0`"]
    #[inline(always)]
    pub fn is_res0(&self) -> bool {
        **self == CSRESSEL_A::RES0
    }
    #[doc = "Checks if the value of the field is `RES1`"]
    #[inline(always)]
    pub fn is_res1(&self) -> bool {
        **self == CSRESSEL_A::RES1
    }
    #[doc = "Checks if the value of the field is `RES2`"]
    #[inline(always)]
    pub fn is_res2(&self) -> bool {
        **self == CSRESSEL_A::RES2
    }
    #[doc = "Checks if the value of the field is `RES3`"]
    #[inline(always)]
    pub fn is_res3(&self) -> bool {
        **self == CSRESSEL_A::RES3
    }
}
impl core::ops::Deref for CSRESSEL_R {
    type Target = crate::FieldReader<u8, CSRESSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `CSRESSEL` writer - Capacitive Sense Mode Internal Resistor Select"]
pub struct CSRESSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> CSRESSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: CSRESSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Internal capacitive sense resistor value 0."]
    #[inline(always)]
    pub fn res0(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES0)
    }
    #[doc = "Internal capacitive sense resistor value 1."]
    #[inline(always)]
    pub fn res1(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES1)
    }
    #[doc = "Internal capacitive sense resistor value 2."]
    #[inline(always)]
    pub fn res2(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES2)
    }
    #[doc = "Internal capacitive sense resistor value 3."]
    #[inline(always)]
    pub fn res3(self) -> &'a mut W {
        self.variant(CSRESSEL_A::RES3)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 28)) | ((value as u32 & 0x03) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Positive Input Select"]
    #[inline(always)]
    pub fn possel(&self) -> POSSEL_R {
        POSSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:7 - Negative Input Select"]
    #[inline(always)]
    pub fn negsel(&self) -> NEGSEL_R {
        NEGSEL_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:13 - VDD Reference Level"]
    #[inline(always)]
    pub fn vddlevel(&self) -> VDDLEVEL_R {
        VDDLEVEL_R::new(((self.bits >> 8) & 0x3f) as u8)
    }
    #[doc = "Bit 16 - Low Power Reference Mode"]
    #[inline(always)]
    pub fn lpref(&self) -> LPREF_R {
        LPREF_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 24 - Capacitive Sense Mode Internal Resistor Enable"]
    #[inline(always)]
    pub fn csresen(&self) -> CSRESEN_R {
        CSRESEN_R::new(((self.bits >> 24) & 0x01) != 0)
    }
    #[doc = "Bits 28:29 - Capacitive Sense Mode Internal Resistor Select"]
    #[inline(always)]
    pub fn csressel(&self) -> CSRESSEL_R {
        CSRESSEL_R::new(((self.bits >> 28) & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Positive Input Select"]
    #[inline(always)]
    pub fn possel(&mut self) -> POSSEL_W {
        POSSEL_W { w: self }
    }
    #[doc = "Bits 4:7 - Negative Input Select"]
    #[inline(always)]
    pub fn negsel(&mut self) -> NEGSEL_W {
        NEGSEL_W { w: self }
    }
    #[doc = "Bits 8:13 - VDD Reference Level"]
    #[inline(always)]
    pub fn vddlevel(&mut self) -> VDDLEVEL_W {
        VDDLEVEL_W { w: self }
    }
    #[doc = "Bit 16 - Low Power Reference Mode"]
    #[inline(always)]
    pub fn lpref(&mut self) -> LPREF_W {
        LPREF_W { w: self }
    }
    #[doc = "Bit 24 - Capacitive Sense Mode Internal Resistor Enable"]
    #[inline(always)]
    pub fn csresen(&mut self) -> CSRESEN_W {
        CSRESEN_W { w: self }
    }
    #[doc = "Bits 28:29 - Capacitive Sense Mode Internal Resistor Select"]
    #[inline(always)]
    pub fn csressel(&mut self) -> CSRESSEL_W {
        CSRESSEL_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Input Selection Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [inputsel](index.html) module"]
pub struct INPUTSEL_SPEC;
impl crate::RegisterSpec for INPUTSEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [inputsel::R](R) reader structure"]
impl crate::Readable for INPUTSEL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [inputsel::W](W) writer structure"]
impl crate::Writable for INPUTSEL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets INPUTSEL to value 0x0001_0080"]
impl crate::Resettable for INPUTSEL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0001_0080
    }
}
