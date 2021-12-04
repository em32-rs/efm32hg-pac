#[doc = "Register `IRCTRL` reader"]
pub struct R(crate::R<IRCTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<IRCTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<IRCTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<IRCTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `IRCTRL` writer"]
pub struct W(crate::W<IRCTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<IRCTRL_SPEC>;
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
impl From<crate::W<IRCTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<IRCTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `IREN` reader - Enable IrDA Module"]
pub struct IREN_R(crate::FieldReader<bool, bool>);
impl IREN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IREN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IREN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IREN` writer - Enable IrDA Module"]
pub struct IREN_W<'a> {
    w: &'a mut W,
}
impl<'a> IREN_W<'a> {
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
#[doc = "IrDA TX Pulse Width\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IRPW_A {
    #[doc = "0: IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    ONE = 0,
    #[doc = "1: IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    TWO = 1,
    #[doc = "2: IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    THREE = 2,
    #[doc = "3: IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    FOUR = 3,
}
impl From<IRPW_A> for u8 {
    #[inline(always)]
    fn from(variant: IRPW_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IRPW` reader - IrDA TX Pulse Width"]
pub struct IRPW_R(crate::FieldReader<u8, IRPW_A>);
impl IRPW_R {
    pub(crate) fn new(bits: u8) -> Self {
        IRPW_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRPW_A {
        match self.bits {
            0 => IRPW_A::ONE,
            1 => IRPW_A::TWO,
            2 => IRPW_A::THREE,
            3 => IRPW_A::FOUR,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ONE`"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        **self == IRPW_A::ONE
    }
    #[doc = "Checks if the value of the field is `TWO`"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        **self == IRPW_A::TWO
    }
    #[doc = "Checks if the value of the field is `THREE`"]
    #[inline(always)]
    pub fn is_three(&self) -> bool {
        **self == IRPW_A::THREE
    }
    #[doc = "Checks if the value of the field is `FOUR`"]
    #[inline(always)]
    pub fn is_four(&self) -> bool {
        **self == IRPW_A::FOUR
    }
}
impl core::ops::Deref for IRPW_R {
    type Target = crate::FieldReader<u8, IRPW_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRPW` writer - IrDA TX Pulse Width"]
pub struct IRPW_W<'a> {
    w: &'a mut W,
}
impl<'a> IRPW_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRPW_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "IrDA pulse width is 1/16 for OVS=0 and 1/8 for OVS=1"]
    #[inline(always)]
    pub fn one(self) -> &'a mut W {
        self.variant(IRPW_A::ONE)
    }
    #[doc = "IrDA pulse width is 2/16 for OVS=0 and 2/8 for OVS=1"]
    #[inline(always)]
    pub fn two(self) -> &'a mut W {
        self.variant(IRPW_A::TWO)
    }
    #[doc = "IrDA pulse width is 3/16 for OVS=0 and 3/8 for OVS=1"]
    #[inline(always)]
    pub fn three(self) -> &'a mut W {
        self.variant(IRPW_A::THREE)
    }
    #[doc = "IrDA pulse width is 4/16 for OVS=0 and 4/8 for OVS=1"]
    #[inline(always)]
    pub fn four(self) -> &'a mut W {
        self.variant(IRPW_A::FOUR)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | ((value as u32 & 0x03) << 1);
        self.w
    }
}
#[doc = "Field `IRFILT` reader - IrDA RX Filter"]
pub struct IRFILT_R(crate::FieldReader<bool, bool>);
impl IRFILT_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRFILT_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRFILT_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRFILT` writer - IrDA RX Filter"]
pub struct IRFILT_W<'a> {
    w: &'a mut W,
}
impl<'a> IRFILT_W<'a> {
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
#[doc = "IrDA PRS Channel Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum IRPRSSEL_A {
    #[doc = "0: PRS Channel 0 selected"]
    PRSCH0 = 0,
    #[doc = "1: PRS Channel 1 selected"]
    PRSCH1 = 1,
    #[doc = "2: PRS Channel 2 selected"]
    PRSCH2 = 2,
    #[doc = "3: PRS Channel 3 selected"]
    PRSCH3 = 3,
    #[doc = "4: PRS Channel 4 selected"]
    PRSCH4 = 4,
    #[doc = "5: PRS Channel 5 selected"]
    PRSCH5 = 5,
}
impl From<IRPRSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: IRPRSSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `IRPRSSEL` reader - IrDA PRS Channel Select"]
pub struct IRPRSSEL_R(crate::FieldReader<u8, IRPRSSEL_A>);
impl IRPRSSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        IRPRSSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<IRPRSSEL_A> {
        match self.bits {
            0 => Some(IRPRSSEL_A::PRSCH0),
            1 => Some(IRPRSSEL_A::PRSCH1),
            2 => Some(IRPRSSEL_A::PRSCH2),
            3 => Some(IRPRSSEL_A::PRSCH3),
            4 => Some(IRPRSSEL_A::PRSCH4),
            5 => Some(IRPRSSEL_A::PRSCH5),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PRSCH0`"]
    #[inline(always)]
    pub fn is_prsch0(&self) -> bool {
        **self == IRPRSSEL_A::PRSCH0
    }
    #[doc = "Checks if the value of the field is `PRSCH1`"]
    #[inline(always)]
    pub fn is_prsch1(&self) -> bool {
        **self == IRPRSSEL_A::PRSCH1
    }
    #[doc = "Checks if the value of the field is `PRSCH2`"]
    #[inline(always)]
    pub fn is_prsch2(&self) -> bool {
        **self == IRPRSSEL_A::PRSCH2
    }
    #[doc = "Checks if the value of the field is `PRSCH3`"]
    #[inline(always)]
    pub fn is_prsch3(&self) -> bool {
        **self == IRPRSSEL_A::PRSCH3
    }
    #[doc = "Checks if the value of the field is `PRSCH4`"]
    #[inline(always)]
    pub fn is_prsch4(&self) -> bool {
        **self == IRPRSSEL_A::PRSCH4
    }
    #[doc = "Checks if the value of the field is `PRSCH5`"]
    #[inline(always)]
    pub fn is_prsch5(&self) -> bool {
        **self == IRPRSSEL_A::PRSCH5
    }
}
impl core::ops::Deref for IRPRSSEL_R {
    type Target = crate::FieldReader<u8, IRPRSSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRPRSSEL` writer - IrDA PRS Channel Select"]
pub struct IRPRSSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> IRPRSSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: IRPRSSEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "PRS Channel 0 selected"]
    #[inline(always)]
    pub fn prsch0(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH0)
    }
    #[doc = "PRS Channel 1 selected"]
    #[inline(always)]
    pub fn prsch1(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH1)
    }
    #[doc = "PRS Channel 2 selected"]
    #[inline(always)]
    pub fn prsch2(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH2)
    }
    #[doc = "PRS Channel 3 selected"]
    #[inline(always)]
    pub fn prsch3(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH3)
    }
    #[doc = "PRS Channel 4 selected"]
    #[inline(always)]
    pub fn prsch4(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH4)
    }
    #[doc = "PRS Channel 5 selected"]
    #[inline(always)]
    pub fn prsch5(self) -> &'a mut W {
        self.variant(IRPRSSEL_A::PRSCH5)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "Field `IRPRSEN` reader - IrDA PRS Channel Enable"]
pub struct IRPRSEN_R(crate::FieldReader<bool, bool>);
impl IRPRSEN_R {
    pub(crate) fn new(bits: bool) -> Self {
        IRPRSEN_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for IRPRSEN_R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `IRPRSEN` writer - IrDA PRS Channel Enable"]
pub struct IRPRSEN_W<'a> {
    w: &'a mut W,
}
impl<'a> IRPRSEN_W<'a> {
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
impl R {
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new((self.bits & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline(always)]
    pub fn irpw(&self) -> IRPW_R {
        IRPW_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline(always)]
    pub fn irfilt(&self) -> IRFILT_R {
        IRFILT_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 4:6 - IrDA PRS Channel Select"]
    #[inline(always)]
    pub fn irprssel(&self) -> IRPRSSEL_R {
        IRPRSSEL_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bit 7 - IrDA PRS Channel Enable"]
    #[inline(always)]
    pub fn irprsen(&self) -> IRPRSEN_R {
        IRPRSEN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable IrDA Module"]
    #[inline(always)]
    pub fn iren(&mut self) -> IREN_W {
        IREN_W { w: self }
    }
    #[doc = "Bits 1:2 - IrDA TX Pulse Width"]
    #[inline(always)]
    pub fn irpw(&mut self) -> IRPW_W {
        IRPW_W { w: self }
    }
    #[doc = "Bit 3 - IrDA RX Filter"]
    #[inline(always)]
    pub fn irfilt(&mut self) -> IRFILT_W {
        IRFILT_W { w: self }
    }
    #[doc = "Bits 4:6 - IrDA PRS Channel Select"]
    #[inline(always)]
    pub fn irprssel(&mut self) -> IRPRSSEL_W {
        IRPRSSEL_W { w: self }
    }
    #[doc = "Bit 7 - IrDA PRS Channel Enable"]
    #[inline(always)]
    pub fn irprsen(&mut self) -> IRPRSEN_W {
        IRPRSEN_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "IrDA Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [irctrl](index.html) module"]
pub struct IRCTRL_SPEC;
impl crate::RegisterSpec for IRCTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [irctrl::R](R) reader structure"]
impl crate::Readable for IRCTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [irctrl::W](W) writer structure"]
impl crate::Writable for IRCTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets IRCTRL to value 0"]
impl crate::Resettable for IRCTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
