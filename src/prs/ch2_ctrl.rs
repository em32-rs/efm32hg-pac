#[doc = "Register `CH2_CTRL` reader"]
pub struct R(crate::R<CH2_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH2_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH2_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH2_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH2_CTRL` writer"]
pub struct W(crate::W<CH2_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH2_CTRL_SPEC>;
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
impl From<crate::W<CH2_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH2_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SIGSEL` reader - Signal Select"]
pub struct SIGSEL_R(crate::FieldReader<u8, u8>);
impl SIGSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SIGSEL_R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for SIGSEL_R {
    type Target = crate::FieldReader<u8, u8>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SIGSEL` writer - Signal Select"]
pub struct SIGSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SIGSEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "Source Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum SOURCESEL_A {
    #[doc = "0: No source selected"]
    NONE = 0,
    #[doc = "1: Voltage Comparator"]
    VCMP = 1,
    #[doc = "2: Analog Comparator 0"]
    ACMP0 = 2,
    #[doc = "8: Analog to Digital Converter 0"]
    ADC0 = 8,
    #[doc = "16: Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    USART0 = 16,
    #[doc = "17: Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    USART1 = 17,
    #[doc = "28: Timer 0"]
    TIMER0 = 28,
    #[doc = "29: Timer 1"]
    TIMER1 = 29,
    #[doc = "30: Timer 2"]
    TIMER2 = 30,
    #[doc = "36: Universal Serial Bus Interface"]
    USB = 36,
    #[doc = "40: Real-Time Counter"]
    RTC = 40,
    #[doc = "48: General purpose Input/Output"]
    GPIOL = 48,
    #[doc = "49: General purpose Input/Output"]
    GPIOH = 49,
    #[doc = "54: Pulse Counter 0"]
    PCNT0 = 54,
}
impl From<SOURCESEL_A> for u8 {
    #[inline(always)]
    fn from(variant: SOURCESEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `SOURCESEL` reader - Source Select"]
pub struct SOURCESEL_R(crate::FieldReader<u8, SOURCESEL_A>);
impl SOURCESEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        SOURCESEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SOURCESEL_A> {
        match self.bits {
            0 => Some(SOURCESEL_A::NONE),
            1 => Some(SOURCESEL_A::VCMP),
            2 => Some(SOURCESEL_A::ACMP0),
            8 => Some(SOURCESEL_A::ADC0),
            16 => Some(SOURCESEL_A::USART0),
            17 => Some(SOURCESEL_A::USART1),
            28 => Some(SOURCESEL_A::TIMER0),
            29 => Some(SOURCESEL_A::TIMER1),
            30 => Some(SOURCESEL_A::TIMER2),
            36 => Some(SOURCESEL_A::USB),
            40 => Some(SOURCESEL_A::RTC),
            48 => Some(SOURCESEL_A::GPIOL),
            49 => Some(SOURCESEL_A::GPIOH),
            54 => Some(SOURCESEL_A::PCNT0),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NONE`"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        **self == SOURCESEL_A::NONE
    }
    #[doc = "Checks if the value of the field is `VCMP`"]
    #[inline(always)]
    pub fn is_vcmp(&self) -> bool {
        **self == SOURCESEL_A::VCMP
    }
    #[doc = "Checks if the value of the field is `ACMP0`"]
    #[inline(always)]
    pub fn is_acmp0(&self) -> bool {
        **self == SOURCESEL_A::ACMP0
    }
    #[doc = "Checks if the value of the field is `ADC0`"]
    #[inline(always)]
    pub fn is_adc0(&self) -> bool {
        **self == SOURCESEL_A::ADC0
    }
    #[doc = "Checks if the value of the field is `USART0`"]
    #[inline(always)]
    pub fn is_usart0(&self) -> bool {
        **self == SOURCESEL_A::USART0
    }
    #[doc = "Checks if the value of the field is `USART1`"]
    #[inline(always)]
    pub fn is_usart1(&self) -> bool {
        **self == SOURCESEL_A::USART1
    }
    #[doc = "Checks if the value of the field is `TIMER0`"]
    #[inline(always)]
    pub fn is_timer0(&self) -> bool {
        **self == SOURCESEL_A::TIMER0
    }
    #[doc = "Checks if the value of the field is `TIMER1`"]
    #[inline(always)]
    pub fn is_timer1(&self) -> bool {
        **self == SOURCESEL_A::TIMER1
    }
    #[doc = "Checks if the value of the field is `TIMER2`"]
    #[inline(always)]
    pub fn is_timer2(&self) -> bool {
        **self == SOURCESEL_A::TIMER2
    }
    #[doc = "Checks if the value of the field is `USB`"]
    #[inline(always)]
    pub fn is_usb(&self) -> bool {
        **self == SOURCESEL_A::USB
    }
    #[doc = "Checks if the value of the field is `RTC`"]
    #[inline(always)]
    pub fn is_rtc(&self) -> bool {
        **self == SOURCESEL_A::RTC
    }
    #[doc = "Checks if the value of the field is `GPIOL`"]
    #[inline(always)]
    pub fn is_gpiol(&self) -> bool {
        **self == SOURCESEL_A::GPIOL
    }
    #[doc = "Checks if the value of the field is `GPIOH`"]
    #[inline(always)]
    pub fn is_gpioh(&self) -> bool {
        **self == SOURCESEL_A::GPIOH
    }
    #[doc = "Checks if the value of the field is `PCNT0`"]
    #[inline(always)]
    pub fn is_pcnt0(&self) -> bool {
        **self == SOURCESEL_A::PCNT0
    }
}
impl core::ops::Deref for SOURCESEL_R {
    type Target = crate::FieldReader<u8, SOURCESEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `SOURCESEL` writer - Source Select"]
pub struct SOURCESEL_W<'a> {
    w: &'a mut W,
}
impl<'a> SOURCESEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: SOURCESEL_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "No source selected"]
    #[inline(always)]
    pub fn none(self) -> &'a mut W {
        self.variant(SOURCESEL_A::NONE)
    }
    #[doc = "Voltage Comparator"]
    #[inline(always)]
    pub fn vcmp(self) -> &'a mut W {
        self.variant(SOURCESEL_A::VCMP)
    }
    #[doc = "Analog Comparator 0"]
    #[inline(always)]
    pub fn acmp0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::ACMP0)
    }
    #[doc = "Analog to Digital Converter 0"]
    #[inline(always)]
    pub fn adc0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::ADC0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 0"]
    #[inline(always)]
    pub fn usart0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART0)
    }
    #[doc = "Universal Synchronous/Asynchronous Receiver/Transmitter 1"]
    #[inline(always)]
    pub fn usart1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USART1)
    }
    #[doc = "Timer 0"]
    #[inline(always)]
    pub fn timer0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER0)
    }
    #[doc = "Timer 1"]
    #[inline(always)]
    pub fn timer1(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER1)
    }
    #[doc = "Timer 2"]
    #[inline(always)]
    pub fn timer2(self) -> &'a mut W {
        self.variant(SOURCESEL_A::TIMER2)
    }
    #[doc = "Universal Serial Bus Interface"]
    #[inline(always)]
    pub fn usb(self) -> &'a mut W {
        self.variant(SOURCESEL_A::USB)
    }
    #[doc = "Real-Time Counter"]
    #[inline(always)]
    pub fn rtc(self) -> &'a mut W {
        self.variant(SOURCESEL_A::RTC)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpiol(self) -> &'a mut W {
        self.variant(SOURCESEL_A::GPIOL)
    }
    #[doc = "General purpose Input/Output"]
    #[inline(always)]
    pub fn gpioh(self) -> &'a mut W {
        self.variant(SOURCESEL_A::GPIOH)
    }
    #[doc = "Pulse Counter 0"]
    #[inline(always)]
    pub fn pcnt0(self) -> &'a mut W {
        self.variant(SOURCESEL_A::PCNT0)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 16)) | ((value as u32 & 0x3f) << 16);
        self.w
    }
}
#[doc = "Edge Detect Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EDSEL_A {
    #[doc = "0: Signal is left as it is"]
    OFF = 0,
    #[doc = "1: A one HFPERCLK cycle pulse is generated for every positive edge of the incoming signal"]
    POSEDGE = 1,
    #[doc = "2: A one HFPERCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    NEGEDGE = 2,
    #[doc = "3: A one HFPERCLK clock cycle pulse is generated for every edge of the incoming signal"]
    BOTHEDGES = 3,
}
impl From<EDSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: EDSEL_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EDSEL` reader - Edge Detect Select"]
pub struct EDSEL_R(crate::FieldReader<u8, EDSEL_A>);
impl EDSEL_R {
    pub(crate) fn new(bits: u8) -> Self {
        EDSEL_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EDSEL_A {
        match self.bits {
            0 => EDSEL_A::OFF,
            1 => EDSEL_A::POSEDGE,
            2 => EDSEL_A::NEGEDGE,
            3 => EDSEL_A::BOTHEDGES,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `OFF`"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        **self == EDSEL_A::OFF
    }
    #[doc = "Checks if the value of the field is `POSEDGE`"]
    #[inline(always)]
    pub fn is_posedge(&self) -> bool {
        **self == EDSEL_A::POSEDGE
    }
    #[doc = "Checks if the value of the field is `NEGEDGE`"]
    #[inline(always)]
    pub fn is_negedge(&self) -> bool {
        **self == EDSEL_A::NEGEDGE
    }
    #[doc = "Checks if the value of the field is `BOTHEDGES`"]
    #[inline(always)]
    pub fn is_bothedges(&self) -> bool {
        **self == EDSEL_A::BOTHEDGES
    }
}
impl core::ops::Deref for EDSEL_R {
    type Target = crate::FieldReader<u8, EDSEL_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EDSEL` writer - Edge Detect Select"]
pub struct EDSEL_W<'a> {
    w: &'a mut W,
}
impl<'a> EDSEL_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EDSEL_A) -> &'a mut W {
        self.bits(variant.into())
    }
    #[doc = "Signal is left as it is"]
    #[inline(always)]
    pub fn off(self) -> &'a mut W {
        self.variant(EDSEL_A::OFF)
    }
    #[doc = "A one HFPERCLK cycle pulse is generated for every positive edge of the incoming signal"]
    #[inline(always)]
    pub fn posedge(self) -> &'a mut W {
        self.variant(EDSEL_A::POSEDGE)
    }
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every negative edge of the incoming signal"]
    #[inline(always)]
    pub fn negedge(self) -> &'a mut W {
        self.variant(EDSEL_A::NEGEDGE)
    }
    #[doc = "A one HFPERCLK clock cycle pulse is generated for every edge of the incoming signal"]
    #[inline(always)]
    pub fn bothedges(self) -> &'a mut W {
        self.variant(EDSEL_A::BOTHEDGES)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 24)) | ((value as u32 & 0x03) << 24);
        self.w
    }
}
#[doc = "Field `ASYNC_` reader - Asynchronous reflex"]
pub struct ASYNC__R(crate::FieldReader<bool, bool>);
impl ASYNC__R {
    pub(crate) fn new(bits: bool) -> Self {
        ASYNC__R(crate::FieldReader::new(bits))
    }
}
impl core::ops::Deref for ASYNC__R {
    type Target = crate::FieldReader<bool, bool>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `ASYNC_` writer - Asynchronous reflex"]
pub struct ASYNC__W<'a> {
    w: &'a mut W,
}
impl<'a> ASYNC__W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | ((value as u32 & 0x01) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&self) -> SIGSEL_R {
        SIGSEL_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&self) -> SOURCESEL_R {
        SOURCESEL_R::new(((self.bits >> 16) & 0x3f) as u8)
    }
    #[doc = "Bits 24:25 - Edge Detect Select"]
    #[inline(always)]
    pub fn edsel(&self) -> EDSEL_R {
        EDSEL_R::new(((self.bits >> 24) & 0x03) as u8)
    }
    #[doc = "Bit 28 - Asynchronous reflex"]
    #[inline(always)]
    pub fn async_(&self) -> ASYNC__R {
        ASYNC__R::new(((self.bits >> 28) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - Signal Select"]
    #[inline(always)]
    pub fn sigsel(&mut self) -> SIGSEL_W {
        SIGSEL_W { w: self }
    }
    #[doc = "Bits 16:21 - Source Select"]
    #[inline(always)]
    pub fn sourcesel(&mut self) -> SOURCESEL_W {
        SOURCESEL_W { w: self }
    }
    #[doc = "Bits 24:25 - Edge Detect Select"]
    #[inline(always)]
    pub fn edsel(&mut self) -> EDSEL_W {
        EDSEL_W { w: self }
    }
    #[doc = "Bit 28 - Asynchronous reflex"]
    #[inline(always)]
    pub fn async_(&mut self) -> ASYNC__W {
        ASYNC__W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Channel Control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch2_ctrl](index.html) module"]
pub struct CH2_CTRL_SPEC;
impl crate::RegisterSpec for CH2_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch2_ctrl::R](R) reader structure"]
impl crate::Readable for CH2_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch2_ctrl::W](W) writer structure"]
impl crate::Writable for CH2_CTRL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CH2_CTRL to value 0"]
impl crate::Resettable for CH2_CTRL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
