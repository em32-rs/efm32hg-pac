#[doc = "Register `EXTIPSELL` reader"]
pub struct R(crate::R<EXTIPSELL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<EXTIPSELL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<EXTIPSELL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<EXTIPSELL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `EXTIPSELL` writer"]
pub struct W(crate::W<EXTIPSELL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<EXTIPSELL_SPEC>;
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
impl From<crate::W<EXTIPSELL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<EXTIPSELL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "External Interrupt 0 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPSEL0_A {
    #[doc = "0: Port A pin 0 selected for external interrupt 0"]
    PORTA = 0,
    #[doc = "1: Port B pin 0 selected for external interrupt 0"]
    PORTB = 1,
    #[doc = "2: Port C pin 0 selected for external interrupt 0"]
    PORTC = 2,
    #[doc = "3: Port D pin 0 selected for external interrupt 0"]
    PORTD = 3,
    #[doc = "4: Port E pin 0 selected for external interrupt 0"]
    PORTE = 4,
    #[doc = "5: Port F pin 0 selected for external interrupt 0"]
    PORTF = 5,
}
impl From<EXTIPSEL0_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL0_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPSEL0` reader - External Interrupt 0 Port Select"]
pub struct EXTIPSEL0_R(crate::FieldReader<u8, EXTIPSEL0_A>);
impl EXTIPSEL0_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTIPSEL0_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL0_A> {
        match self.bits {
            0 => Some(EXTIPSEL0_A::PORTA),
            1 => Some(EXTIPSEL0_A::PORTB),
            2 => Some(EXTIPSEL0_A::PORTC),
            3 => Some(EXTIPSEL0_A::PORTD),
            4 => Some(EXTIPSEL0_A::PORTE),
            5 => Some(EXTIPSEL0_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        **self == EXTIPSEL0_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        **self == EXTIPSEL0_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        **self == EXTIPSEL0_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        **self == EXTIPSEL0_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        **self == EXTIPSEL0_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        **self == EXTIPSEL0_A::PORTF
    }
}
impl core::ops::Deref for EXTIPSEL0_R {
    type Target = crate::FieldReader<u8, EXTIPSEL0_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTIPSEL0` writer - External Interrupt 0 Port Select"]
pub struct EXTIPSEL0_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL0_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL0_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTA)
    }
    #[doc = "Port B pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTB)
    }
    #[doc = "Port C pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTC)
    }
    #[doc = "Port D pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTD)
    }
    #[doc = "Port E pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTE)
    }
    #[doc = "Port F pin 0 selected for external interrupt 0"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL0_A::PORTF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | (value as u32 & 0x07);
        self.w
    }
}
#[doc = "External Interrupt 1 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPSEL1_A {
    #[doc = "0: Port A pin 1 selected for external interrupt 1"]
    PORTA = 0,
    #[doc = "1: Port B pin 1 selected for external interrupt 1"]
    PORTB = 1,
    #[doc = "2: Port C pin 1 selected for external interrupt 1"]
    PORTC = 2,
    #[doc = "3: Port D pin 1 selected for external interrupt 1"]
    PORTD = 3,
    #[doc = "4: Port E pin 1 selected for external interrupt 1"]
    PORTE = 4,
    #[doc = "5: Port F pin 1 selected for external interrupt 1"]
    PORTF = 5,
}
impl From<EXTIPSEL1_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL1_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPSEL1` reader - External Interrupt 1 Port Select"]
pub struct EXTIPSEL1_R(crate::FieldReader<u8, EXTIPSEL1_A>);
impl EXTIPSEL1_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTIPSEL1_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL1_A> {
        match self.bits {
            0 => Some(EXTIPSEL1_A::PORTA),
            1 => Some(EXTIPSEL1_A::PORTB),
            2 => Some(EXTIPSEL1_A::PORTC),
            3 => Some(EXTIPSEL1_A::PORTD),
            4 => Some(EXTIPSEL1_A::PORTE),
            5 => Some(EXTIPSEL1_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        **self == EXTIPSEL1_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        **self == EXTIPSEL1_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        **self == EXTIPSEL1_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        **self == EXTIPSEL1_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        **self == EXTIPSEL1_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        **self == EXTIPSEL1_A::PORTF
    }
}
impl core::ops::Deref for EXTIPSEL1_R {
    type Target = crate::FieldReader<u8, EXTIPSEL1_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTIPSEL1` writer - External Interrupt 1 Port Select"]
pub struct EXTIPSEL1_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL1_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL1_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTA)
    }
    #[doc = "Port B pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTB)
    }
    #[doc = "Port C pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTC)
    }
    #[doc = "Port D pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTD)
    }
    #[doc = "Port E pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTE)
    }
    #[doc = "Port F pin 1 selected for external interrupt 1"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL1_A::PORTF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 4)) | ((value as u32 & 0x07) << 4);
        self.w
    }
}
#[doc = "External Interrupt 2 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPSEL2_A {
    #[doc = "0: Port A pin 2 selected for external interrupt 2"]
    PORTA = 0,
    #[doc = "1: Port B pin 2 selected for external interrupt 2"]
    PORTB = 1,
    #[doc = "2: Port C pin 2 selected for external interrupt 2"]
    PORTC = 2,
    #[doc = "3: Port D pin 2 selected for external interrupt 2"]
    PORTD = 3,
    #[doc = "4: Port E pin 2 selected for external interrupt 2"]
    PORTE = 4,
    #[doc = "5: Port F pin 2 selected for external interrupt 2"]
    PORTF = 5,
}
impl From<EXTIPSEL2_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL2_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPSEL2` reader - External Interrupt 2 Port Select"]
pub struct EXTIPSEL2_R(crate::FieldReader<u8, EXTIPSEL2_A>);
impl EXTIPSEL2_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTIPSEL2_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL2_A> {
        match self.bits {
            0 => Some(EXTIPSEL2_A::PORTA),
            1 => Some(EXTIPSEL2_A::PORTB),
            2 => Some(EXTIPSEL2_A::PORTC),
            3 => Some(EXTIPSEL2_A::PORTD),
            4 => Some(EXTIPSEL2_A::PORTE),
            5 => Some(EXTIPSEL2_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        **self == EXTIPSEL2_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        **self == EXTIPSEL2_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        **self == EXTIPSEL2_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        **self == EXTIPSEL2_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        **self == EXTIPSEL2_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        **self == EXTIPSEL2_A::PORTF
    }
}
impl core::ops::Deref for EXTIPSEL2_R {
    type Target = crate::FieldReader<u8, EXTIPSEL2_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTIPSEL2` writer - External Interrupt 2 Port Select"]
pub struct EXTIPSEL2_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL2_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL2_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTA)
    }
    #[doc = "Port B pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTB)
    }
    #[doc = "Port C pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTC)
    }
    #[doc = "Port D pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTD)
    }
    #[doc = "Port E pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTE)
    }
    #[doc = "Port F pin 2 selected for external interrupt 2"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL2_A::PORTF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 8)) | ((value as u32 & 0x07) << 8);
        self.w
    }
}
#[doc = "External Interrupt 3 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPSEL3_A {
    #[doc = "0: Port A pin 3 selected for external interrupt 3"]
    PORTA = 0,
    #[doc = "1: Port B pin 3 selected for external interrupt 3"]
    PORTB = 1,
    #[doc = "2: Port C pin 3 selected for external interrupt 3"]
    PORTC = 2,
    #[doc = "3: Port D pin 3 selected for external interrupt 3"]
    PORTD = 3,
    #[doc = "4: Port E pin 3 selected for external interrupt 3"]
    PORTE = 4,
    #[doc = "5: Port F pin 3 selected for external interrupt 3"]
    PORTF = 5,
}
impl From<EXTIPSEL3_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL3_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPSEL3` reader - External Interrupt 3 Port Select"]
pub struct EXTIPSEL3_R(crate::FieldReader<u8, EXTIPSEL3_A>);
impl EXTIPSEL3_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTIPSEL3_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL3_A> {
        match self.bits {
            0 => Some(EXTIPSEL3_A::PORTA),
            1 => Some(EXTIPSEL3_A::PORTB),
            2 => Some(EXTIPSEL3_A::PORTC),
            3 => Some(EXTIPSEL3_A::PORTD),
            4 => Some(EXTIPSEL3_A::PORTE),
            5 => Some(EXTIPSEL3_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        **self == EXTIPSEL3_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        **self == EXTIPSEL3_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        **self == EXTIPSEL3_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        **self == EXTIPSEL3_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        **self == EXTIPSEL3_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        **self == EXTIPSEL3_A::PORTF
    }
}
impl core::ops::Deref for EXTIPSEL3_R {
    type Target = crate::FieldReader<u8, EXTIPSEL3_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTIPSEL3` writer - External Interrupt 3 Port Select"]
pub struct EXTIPSEL3_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL3_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL3_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTA)
    }
    #[doc = "Port B pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTB)
    }
    #[doc = "Port C pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTC)
    }
    #[doc = "Port D pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTD)
    }
    #[doc = "Port E pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTE)
    }
    #[doc = "Port F pin 3 selected for external interrupt 3"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL3_A::PORTF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 12)) | ((value as u32 & 0x07) << 12);
        self.w
    }
}
#[doc = "External Interrupt 4 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPSEL4_A {
    #[doc = "0: Port A pin 4 selected for external interrupt 4"]
    PORTA = 0,
    #[doc = "1: Port B pin 4 selected for external interrupt 4"]
    PORTB = 1,
    #[doc = "2: Port C pin 4 selected for external interrupt 4"]
    PORTC = 2,
    #[doc = "3: Port D pin 4 selected for external interrupt 4"]
    PORTD = 3,
    #[doc = "4: Port E pin 4 selected for external interrupt 4"]
    PORTE = 4,
    #[doc = "5: Port F pin 4 selected for external interrupt 4"]
    PORTF = 5,
}
impl From<EXTIPSEL4_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL4_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPSEL4` reader - External Interrupt 4 Port Select"]
pub struct EXTIPSEL4_R(crate::FieldReader<u8, EXTIPSEL4_A>);
impl EXTIPSEL4_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTIPSEL4_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL4_A> {
        match self.bits {
            0 => Some(EXTIPSEL4_A::PORTA),
            1 => Some(EXTIPSEL4_A::PORTB),
            2 => Some(EXTIPSEL4_A::PORTC),
            3 => Some(EXTIPSEL4_A::PORTD),
            4 => Some(EXTIPSEL4_A::PORTE),
            5 => Some(EXTIPSEL4_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        **self == EXTIPSEL4_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        **self == EXTIPSEL4_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        **self == EXTIPSEL4_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        **self == EXTIPSEL4_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        **self == EXTIPSEL4_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        **self == EXTIPSEL4_A::PORTF
    }
}
impl core::ops::Deref for EXTIPSEL4_R {
    type Target = crate::FieldReader<u8, EXTIPSEL4_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTIPSEL4` writer - External Interrupt 4 Port Select"]
pub struct EXTIPSEL4_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL4_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL4_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTA)
    }
    #[doc = "Port B pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTB)
    }
    #[doc = "Port C pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTC)
    }
    #[doc = "Port D pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTD)
    }
    #[doc = "Port E pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTE)
    }
    #[doc = "Port F pin 4 selected for external interrupt 4"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL4_A::PORTF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | ((value as u32 & 0x07) << 16);
        self.w
    }
}
#[doc = "External Interrupt 5 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPSEL5_A {
    #[doc = "0: Port A pin 5 selected for external interrupt 5"]
    PORTA = 0,
    #[doc = "1: Port B pin 5 selected for external interrupt 5"]
    PORTB = 1,
    #[doc = "2: Port C pin 5 selected for external interrupt 5"]
    PORTC = 2,
    #[doc = "3: Port D pin 5 selected for external interrupt 5"]
    PORTD = 3,
    #[doc = "4: Port E pin 5 selected for external interrupt 5"]
    PORTE = 4,
    #[doc = "5: Port F pin 5 selected for external interrupt 5"]
    PORTF = 5,
}
impl From<EXTIPSEL5_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL5_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPSEL5` reader - External Interrupt 5 Port Select"]
pub struct EXTIPSEL5_R(crate::FieldReader<u8, EXTIPSEL5_A>);
impl EXTIPSEL5_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTIPSEL5_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL5_A> {
        match self.bits {
            0 => Some(EXTIPSEL5_A::PORTA),
            1 => Some(EXTIPSEL5_A::PORTB),
            2 => Some(EXTIPSEL5_A::PORTC),
            3 => Some(EXTIPSEL5_A::PORTD),
            4 => Some(EXTIPSEL5_A::PORTE),
            5 => Some(EXTIPSEL5_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        **self == EXTIPSEL5_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        **self == EXTIPSEL5_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        **self == EXTIPSEL5_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        **self == EXTIPSEL5_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        **self == EXTIPSEL5_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        **self == EXTIPSEL5_A::PORTF
    }
}
impl core::ops::Deref for EXTIPSEL5_R {
    type Target = crate::FieldReader<u8, EXTIPSEL5_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTIPSEL5` writer - External Interrupt 5 Port Select"]
pub struct EXTIPSEL5_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL5_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL5_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTA)
    }
    #[doc = "Port B pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTB)
    }
    #[doc = "Port C pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTC)
    }
    #[doc = "Port D pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTD)
    }
    #[doc = "Port E pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTE)
    }
    #[doc = "Port F pin 5 selected for external interrupt 5"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL5_A::PORTF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 20)) | ((value as u32 & 0x07) << 20);
        self.w
    }
}
#[doc = "External Interrupt 6 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPSEL6_A {
    #[doc = "0: Port A pin 6 selected for external interrupt 6"]
    PORTA = 0,
    #[doc = "1: Port B pin 6 selected for external interrupt 6"]
    PORTB = 1,
    #[doc = "2: Port C pin 6 selected for external interrupt 6"]
    PORTC = 2,
    #[doc = "3: Port D pin 6 selected for external interrupt 6"]
    PORTD = 3,
    #[doc = "4: Port E pin 6 selected for external interrupt 6"]
    PORTE = 4,
    #[doc = "5: Port F pin 6 selected for external interrupt 6"]
    PORTF = 5,
}
impl From<EXTIPSEL6_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL6_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPSEL6` reader - External Interrupt 6 Port Select"]
pub struct EXTIPSEL6_R(crate::FieldReader<u8, EXTIPSEL6_A>);
impl EXTIPSEL6_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTIPSEL6_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL6_A> {
        match self.bits {
            0 => Some(EXTIPSEL6_A::PORTA),
            1 => Some(EXTIPSEL6_A::PORTB),
            2 => Some(EXTIPSEL6_A::PORTC),
            3 => Some(EXTIPSEL6_A::PORTD),
            4 => Some(EXTIPSEL6_A::PORTE),
            5 => Some(EXTIPSEL6_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        **self == EXTIPSEL6_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        **self == EXTIPSEL6_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        **self == EXTIPSEL6_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        **self == EXTIPSEL6_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        **self == EXTIPSEL6_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        **self == EXTIPSEL6_A::PORTF
    }
}
impl core::ops::Deref for EXTIPSEL6_R {
    type Target = crate::FieldReader<u8, EXTIPSEL6_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTIPSEL6` writer - External Interrupt 6 Port Select"]
pub struct EXTIPSEL6_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL6_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL6_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTA)
    }
    #[doc = "Port B pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTB)
    }
    #[doc = "Port C pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTC)
    }
    #[doc = "Port D pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTD)
    }
    #[doc = "Port E pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTE)
    }
    #[doc = "Port F pin 6 selected for external interrupt 6"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL6_A::PORTF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 24)) | ((value as u32 & 0x07) << 24);
        self.w
    }
}
#[doc = "External Interrupt 7 Port Select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u8)]
pub enum EXTIPSEL7_A {
    #[doc = "0: Port A pin 7 selected for external interrupt 7"]
    PORTA = 0,
    #[doc = "1: Port B pin 7 selected for external interrupt 7"]
    PORTB = 1,
    #[doc = "2: Port C pin 7 selected for external interrupt 7"]
    PORTC = 2,
    #[doc = "3: Port D pin 7 selected for external interrupt 7"]
    PORTD = 3,
    #[doc = "4: Port E pin 7 selected for external interrupt 7"]
    PORTE = 4,
    #[doc = "5: Port F pin 7 selected for external interrupt 7"]
    PORTF = 5,
}
impl From<EXTIPSEL7_A> for u8 {
    #[inline(always)]
    fn from(variant: EXTIPSEL7_A) -> Self {
        variant as _
    }
}
#[doc = "Field `EXTIPSEL7` reader - External Interrupt 7 Port Select"]
pub struct EXTIPSEL7_R(crate::FieldReader<u8, EXTIPSEL7_A>);
impl EXTIPSEL7_R {
    pub(crate) fn new(bits: u8) -> Self {
        EXTIPSEL7_R(crate::FieldReader::new(bits))
    }
    #[doc = r"Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<EXTIPSEL7_A> {
        match self.bits {
            0 => Some(EXTIPSEL7_A::PORTA),
            1 => Some(EXTIPSEL7_A::PORTB),
            2 => Some(EXTIPSEL7_A::PORTC),
            3 => Some(EXTIPSEL7_A::PORTD),
            4 => Some(EXTIPSEL7_A::PORTE),
            5 => Some(EXTIPSEL7_A::PORTF),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PORTA`"]
    #[inline(always)]
    pub fn is_porta(&self) -> bool {
        **self == EXTIPSEL7_A::PORTA
    }
    #[doc = "Checks if the value of the field is `PORTB`"]
    #[inline(always)]
    pub fn is_portb(&self) -> bool {
        **self == EXTIPSEL7_A::PORTB
    }
    #[doc = "Checks if the value of the field is `PORTC`"]
    #[inline(always)]
    pub fn is_portc(&self) -> bool {
        **self == EXTIPSEL7_A::PORTC
    }
    #[doc = "Checks if the value of the field is `PORTD`"]
    #[inline(always)]
    pub fn is_portd(&self) -> bool {
        **self == EXTIPSEL7_A::PORTD
    }
    #[doc = "Checks if the value of the field is `PORTE`"]
    #[inline(always)]
    pub fn is_porte(&self) -> bool {
        **self == EXTIPSEL7_A::PORTE
    }
    #[doc = "Checks if the value of the field is `PORTF`"]
    #[inline(always)]
    pub fn is_portf(&self) -> bool {
        **self == EXTIPSEL7_A::PORTF
    }
}
impl core::ops::Deref for EXTIPSEL7_R {
    type Target = crate::FieldReader<u8, EXTIPSEL7_A>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
#[doc = "Field `EXTIPSEL7` writer - External Interrupt 7 Port Select"]
pub struct EXTIPSEL7_W<'a> {
    w: &'a mut W,
}
impl<'a> EXTIPSEL7_W<'a> {
    #[doc = r"Writes `variant` to the field"]
    #[inline(always)]
    pub fn variant(self, variant: EXTIPSEL7_A) -> &'a mut W {
        unsafe { self.bits(variant.into()) }
    }
    #[doc = "Port A pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn porta(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTA)
    }
    #[doc = "Port B pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn portb(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTB)
    }
    #[doc = "Port C pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn portc(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTC)
    }
    #[doc = "Port D pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn portd(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTD)
    }
    #[doc = "Port E pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn porte(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTE)
    }
    #[doc = "Port F pin 7 selected for external interrupt 7"]
    #[inline(always)]
    pub fn portf(self) -> &'a mut W {
        self.variant(EXTIPSEL7_A::PORTF)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 28)) | ((value as u32 & 0x07) << 28);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - External Interrupt 0 Port Select"]
    #[inline(always)]
    pub fn extipsel0(&self) -> EXTIPSEL0_R {
        EXTIPSEL0_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bits 4:6 - External Interrupt 1 Port Select"]
    #[inline(always)]
    pub fn extipsel1(&self) -> EXTIPSEL1_R {
        EXTIPSEL1_R::new(((self.bits >> 4) & 0x07) as u8)
    }
    #[doc = "Bits 8:10 - External Interrupt 2 Port Select"]
    #[inline(always)]
    pub fn extipsel2(&self) -> EXTIPSEL2_R {
        EXTIPSEL2_R::new(((self.bits >> 8) & 0x07) as u8)
    }
    #[doc = "Bits 12:14 - External Interrupt 3 Port Select"]
    #[inline(always)]
    pub fn extipsel3(&self) -> EXTIPSEL3_R {
        EXTIPSEL3_R::new(((self.bits >> 12) & 0x07) as u8)
    }
    #[doc = "Bits 16:18 - External Interrupt 4 Port Select"]
    #[inline(always)]
    pub fn extipsel4(&self) -> EXTIPSEL4_R {
        EXTIPSEL4_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 20:22 - External Interrupt 5 Port Select"]
    #[inline(always)]
    pub fn extipsel5(&self) -> EXTIPSEL5_R {
        EXTIPSEL5_R::new(((self.bits >> 20) & 0x07) as u8)
    }
    #[doc = "Bits 24:26 - External Interrupt 6 Port Select"]
    #[inline(always)]
    pub fn extipsel6(&self) -> EXTIPSEL6_R {
        EXTIPSEL6_R::new(((self.bits >> 24) & 0x07) as u8)
    }
    #[doc = "Bits 28:30 - External Interrupt 7 Port Select"]
    #[inline(always)]
    pub fn extipsel7(&self) -> EXTIPSEL7_R {
        EXTIPSEL7_R::new(((self.bits >> 28) & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - External Interrupt 0 Port Select"]
    #[inline(always)]
    pub fn extipsel0(&mut self) -> EXTIPSEL0_W {
        EXTIPSEL0_W { w: self }
    }
    #[doc = "Bits 4:6 - External Interrupt 1 Port Select"]
    #[inline(always)]
    pub fn extipsel1(&mut self) -> EXTIPSEL1_W {
        EXTIPSEL1_W { w: self }
    }
    #[doc = "Bits 8:10 - External Interrupt 2 Port Select"]
    #[inline(always)]
    pub fn extipsel2(&mut self) -> EXTIPSEL2_W {
        EXTIPSEL2_W { w: self }
    }
    #[doc = "Bits 12:14 - External Interrupt 3 Port Select"]
    #[inline(always)]
    pub fn extipsel3(&mut self) -> EXTIPSEL3_W {
        EXTIPSEL3_W { w: self }
    }
    #[doc = "Bits 16:18 - External Interrupt 4 Port Select"]
    #[inline(always)]
    pub fn extipsel4(&mut self) -> EXTIPSEL4_W {
        EXTIPSEL4_W { w: self }
    }
    #[doc = "Bits 20:22 - External Interrupt 5 Port Select"]
    #[inline(always)]
    pub fn extipsel5(&mut self) -> EXTIPSEL5_W {
        EXTIPSEL5_W { w: self }
    }
    #[doc = "Bits 24:26 - External Interrupt 6 Port Select"]
    #[inline(always)]
    pub fn extipsel6(&mut self) -> EXTIPSEL6_W {
        EXTIPSEL6_W { w: self }
    }
    #[doc = "Bits 28:30 - External Interrupt 7 Port Select"]
    #[inline(always)]
    pub fn extipsel7(&mut self) -> EXTIPSEL7_W {
        EXTIPSEL7_W { w: self }
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "External Interrupt Port Select Low Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [extipsell](index.html) module"]
pub struct EXTIPSELL_SPEC;
impl crate::RegisterSpec for EXTIPSELL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [extipsell::R](R) reader structure"]
impl crate::Readable for EXTIPSELL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [extipsell::W](W) writer structure"]
impl crate::Writable for EXTIPSELL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets EXTIPSELL to value 0"]
impl crate::Resettable for EXTIPSELL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
