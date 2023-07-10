#[doc = "Register `CR1` reader"]
pub struct R(crate::R<CR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR1` writer"]
pub struct W(crate::W<CR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR1_SPEC>;
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
impl From<crate::W<CR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PE` reader - Peripheral enable"]
pub type PE_R = crate::BitReader<PE_A>;
#[doc = "Peripheral enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PE_A {
    #[doc = "0: Peripheral disabled"]
    DISABLED = 0,
    #[doc = "1: Peripheral enabled"]
    ENABLED = 1,
}
impl From<PE_A> for bool {
    #[inline(always)]
    fn from(variant: PE_A) -> Self {
        variant as u8 != 0
    }
}
impl PE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PE_A {
        match self.bits {
            false => PE_A::DISABLED,
            true => PE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PE_A::ENABLED
    }
}
#[doc = "Field `PE` writer - Peripheral enable"]
pub type PE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, PE_A>;
impl<'a, const O: u8> PE_W<'a, O> {
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PE_A::DISABLED)
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PE_A::ENABLED)
    }
}
#[doc = "Field `TXIE` reader - TX Interrupt enable"]
pub type TXIE_R = crate::BitReader<TXIE_A>;
#[doc = "TX Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXIE_A {
    #[doc = "0: Transmit (TXIS) interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Transmit (TXIS) interrupt enabled"]
    ENABLED = 1,
}
impl From<TXIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXIE_A {
        match self.bits {
            false => TXIE_A::DISABLED,
            true => TXIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXIE_A::ENABLED
    }
}
#[doc = "Field `TXIE` writer - TX Interrupt enable"]
pub type TXIE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, TXIE_A>;
impl<'a, const O: u8> TXIE_W<'a, O> {
    #[doc = "Transmit (TXIS) interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXIE_A::DISABLED)
    }
    #[doc = "Transmit (TXIS) interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXIE_A::ENABLED)
    }
}
#[doc = "Field `RXIE` reader - RX Interrupt enable"]
pub type RXIE_R = crate::BitReader<RXIE_A>;
#[doc = "RX Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXIE_A {
    #[doc = "0: Receive (RXNE) interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Receive (RXNE) interrupt enabled"]
    ENABLED = 1,
}
impl From<RXIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXIE_A {
        match self.bits {
            false => RXIE_A::DISABLED,
            true => RXIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXIE_A::ENABLED
    }
}
#[doc = "Field `RXIE` writer - RX Interrupt enable"]
pub type RXIE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, RXIE_A>;
impl<'a, const O: u8> RXIE_W<'a, O> {
    #[doc = "Receive (RXNE) interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXIE_A::DISABLED)
    }
    #[doc = "Receive (RXNE) interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXIE_A::ENABLED)
    }
}
#[doc = "Field `ADDRIE` reader - Address match interrupt enable (slave only)"]
pub type ADDRIE_R = crate::BitReader<ADDRIE_A>;
#[doc = "Address match interrupt enable (slave only)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDRIE_A {
    #[doc = "0: Address match (ADDR) interrupts disabled"]
    DISABLED = 0,
    #[doc = "1: Address match (ADDR) interrupts enabled"]
    ENABLED = 1,
}
impl From<ADDRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ADDRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDRIE_A {
        match self.bits {
            false => ADDRIE_A::DISABLED,
            true => ADDRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADDRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADDRIE_A::ENABLED
    }
}
#[doc = "Field `ADDRIE` writer - Address match interrupt enable (slave only)"]
pub type ADDRIE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, ADDRIE_A>;
impl<'a, const O: u8> ADDRIE_W<'a, O> {
    #[doc = "Address match (ADDR) interrupts disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADDRIE_A::DISABLED)
    }
    #[doc = "Address match (ADDR) interrupts enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADDRIE_A::ENABLED)
    }
}
#[doc = "Field `NACKIE` reader - Not acknowledge received interrupt enable"]
pub type NACKIE_R = crate::BitReader<NACKIE_A>;
#[doc = "Not acknowledge received interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKIE_A {
    #[doc = "0: Not acknowledge (NACKF) received interrupts disabled"]
    DISABLED = 0,
    #[doc = "1: Not acknowledge (NACKF) received interrupts enabled"]
    ENABLED = 1,
}
impl From<NACKIE_A> for bool {
    #[inline(always)]
    fn from(variant: NACKIE_A) -> Self {
        variant as u8 != 0
    }
}
impl NACKIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACKIE_A {
        match self.bits {
            false => NACKIE_A::DISABLED,
            true => NACKIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NACKIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NACKIE_A::ENABLED
    }
}
#[doc = "Field `NACKIE` writer - Not acknowledge received interrupt enable"]
pub type NACKIE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, NACKIE_A>;
impl<'a, const O: u8> NACKIE_W<'a, O> {
    #[doc = "Not acknowledge (NACKF) received interrupts disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NACKIE_A::DISABLED)
    }
    #[doc = "Not acknowledge (NACKF) received interrupts enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NACKIE_A::ENABLED)
    }
}
#[doc = "Field `STOPIE` reader - STOP detection Interrupt enable"]
pub type STOPIE_R = crate::BitReader<STOPIE_A>;
#[doc = "STOP detection Interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPIE_A {
    #[doc = "0: Stop detection (STOPF) interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Stop detection (STOPF) interrupt enabled"]
    ENABLED = 1,
}
impl From<STOPIE_A> for bool {
    #[inline(always)]
    fn from(variant: STOPIE_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPIE_A {
        match self.bits {
            false => STOPIE_A::DISABLED,
            true => STOPIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == STOPIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == STOPIE_A::ENABLED
    }
}
#[doc = "Field `STOPIE` writer - STOP detection Interrupt enable"]
pub type STOPIE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, STOPIE_A>;
impl<'a, const O: u8> STOPIE_W<'a, O> {
    #[doc = "Stop detection (STOPF) interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(STOPIE_A::DISABLED)
    }
    #[doc = "Stop detection (STOPF) interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(STOPIE_A::ENABLED)
    }
}
#[doc = "Field `TCIE` reader - Transfer Complete interrupt enable"]
pub type TCIE_R = crate::BitReader<TCIE_A>;
#[doc = "Transfer Complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE_A {
    #[doc = "0: Transfer Complete interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Transfer Complete interrupt enabled"]
    ENABLED = 1,
}
impl From<TCIE_A> for bool {
    #[inline(always)]
    fn from(variant: TCIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TCIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCIE_A {
        match self.bits {
            false => TCIE_A::DISABLED,
            true => TCIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TCIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TCIE_A::ENABLED
    }
}
#[doc = "Field `TCIE` writer - Transfer Complete interrupt enable"]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, TCIE_A>;
impl<'a, const O: u8> TCIE_W<'a, O> {
    #[doc = "Transfer Complete interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::DISABLED)
    }
    #[doc = "Transfer Complete interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIE_A::ENABLED)
    }
}
#[doc = "Field `ERRIE` reader - Error interrupts enable"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
#[doc = "Error interrupts enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE_A {
    #[doc = "0: Error detection interrupts disabled"]
    DISABLED = 0,
    #[doc = "1: Error detection interrupts enabled"]
    ENABLED = 1,
}
impl From<ERRIE_A> for bool {
    #[inline(always)]
    fn from(variant: ERRIE_A) -> Self {
        variant as u8 != 0
    }
}
impl ERRIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ERRIE_A {
        match self.bits {
            false => ERRIE_A::DISABLED,
            true => ERRIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ERRIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ERRIE_A::ENABLED
    }
}
#[doc = "Field `ERRIE` writer - Error interrupts enable"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, ERRIE_A>;
impl<'a, const O: u8> ERRIE_W<'a, O> {
    #[doc = "Error detection interrupts disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ERRIE_A::DISABLED)
    }
    #[doc = "Error detection interrupts enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ERRIE_A::ENABLED)
    }
}
#[doc = "Field `DNF` reader - Digital noise filter"]
pub type DNF_R = crate::FieldReader<DNF_A>;
#[doc = "Digital noise filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DNF_A {
    #[doc = "0: Digital filter disabled"]
    NO_FILTER = 0,
    #[doc = "1: Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    FILTER1 = 1,
    #[doc = "2: Digital filter enabled and filtering capability up to 2 tI2CCLK"]
    FILTER2 = 2,
    #[doc = "3: Digital filter enabled and filtering capability up to 3 tI2CCLK"]
    FILTER3 = 3,
    #[doc = "4: Digital filter enabled and filtering capability up to 4 tI2CCLK"]
    FILTER4 = 4,
    #[doc = "5: Digital filter enabled and filtering capability up to 5 tI2CCLK"]
    FILTER5 = 5,
    #[doc = "6: Digital filter enabled and filtering capability up to 6 tI2CCLK"]
    FILTER6 = 6,
    #[doc = "7: Digital filter enabled and filtering capability up to 7 tI2CCLK"]
    FILTER7 = 7,
    #[doc = "8: Digital filter enabled and filtering capability up to 8 tI2CCLK"]
    FILTER8 = 8,
    #[doc = "9: Digital filter enabled and filtering capability up to 9 tI2CCLK"]
    FILTER9 = 9,
    #[doc = "10: Digital filter enabled and filtering capability up to 10 tI2CCLK"]
    FILTER10 = 10,
    #[doc = "11: Digital filter enabled and filtering capability up to 11 tI2CCLK"]
    FILTER11 = 11,
    #[doc = "12: Digital filter enabled and filtering capability up to 12 tI2CCLK"]
    FILTER12 = 12,
    #[doc = "13: Digital filter enabled and filtering capability up to 13 tI2CCLK"]
    FILTER13 = 13,
    #[doc = "14: Digital filter enabled and filtering capability up to 14 tI2CCLK"]
    FILTER14 = 14,
    #[doc = "15: Digital filter enabled and filtering capability up to 15 tI2CCLK"]
    FILTER15 = 15,
}
impl From<DNF_A> for u8 {
    #[inline(always)]
    fn from(variant: DNF_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DNF_A {
    type Ux = u8;
}
impl DNF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DNF_A {
        match self.bits {
            0 => DNF_A::NO_FILTER,
            1 => DNF_A::FILTER1,
            2 => DNF_A::FILTER2,
            3 => DNF_A::FILTER3,
            4 => DNF_A::FILTER4,
            5 => DNF_A::FILTER5,
            6 => DNF_A::FILTER6,
            7 => DNF_A::FILTER7,
            8 => DNF_A::FILTER8,
            9 => DNF_A::FILTER9,
            10 => DNF_A::FILTER10,
            11 => DNF_A::FILTER11,
            12 => DNF_A::FILTER12,
            13 => DNF_A::FILTER13,
            14 => DNF_A::FILTER14,
            15 => DNF_A::FILTER15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_FILTER`"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == DNF_A::NO_FILTER
    }
    #[doc = "Checks if the value of the field is `FILTER1`"]
    #[inline(always)]
    pub fn is_filter1(&self) -> bool {
        *self == DNF_A::FILTER1
    }
    #[doc = "Checks if the value of the field is `FILTER2`"]
    #[inline(always)]
    pub fn is_filter2(&self) -> bool {
        *self == DNF_A::FILTER2
    }
    #[doc = "Checks if the value of the field is `FILTER3`"]
    #[inline(always)]
    pub fn is_filter3(&self) -> bool {
        *self == DNF_A::FILTER3
    }
    #[doc = "Checks if the value of the field is `FILTER4`"]
    #[inline(always)]
    pub fn is_filter4(&self) -> bool {
        *self == DNF_A::FILTER4
    }
    #[doc = "Checks if the value of the field is `FILTER5`"]
    #[inline(always)]
    pub fn is_filter5(&self) -> bool {
        *self == DNF_A::FILTER5
    }
    #[doc = "Checks if the value of the field is `FILTER6`"]
    #[inline(always)]
    pub fn is_filter6(&self) -> bool {
        *self == DNF_A::FILTER6
    }
    #[doc = "Checks if the value of the field is `FILTER7`"]
    #[inline(always)]
    pub fn is_filter7(&self) -> bool {
        *self == DNF_A::FILTER7
    }
    #[doc = "Checks if the value of the field is `FILTER8`"]
    #[inline(always)]
    pub fn is_filter8(&self) -> bool {
        *self == DNF_A::FILTER8
    }
    #[doc = "Checks if the value of the field is `FILTER9`"]
    #[inline(always)]
    pub fn is_filter9(&self) -> bool {
        *self == DNF_A::FILTER9
    }
    #[doc = "Checks if the value of the field is `FILTER10`"]
    #[inline(always)]
    pub fn is_filter10(&self) -> bool {
        *self == DNF_A::FILTER10
    }
    #[doc = "Checks if the value of the field is `FILTER11`"]
    #[inline(always)]
    pub fn is_filter11(&self) -> bool {
        *self == DNF_A::FILTER11
    }
    #[doc = "Checks if the value of the field is `FILTER12`"]
    #[inline(always)]
    pub fn is_filter12(&self) -> bool {
        *self == DNF_A::FILTER12
    }
    #[doc = "Checks if the value of the field is `FILTER13`"]
    #[inline(always)]
    pub fn is_filter13(&self) -> bool {
        *self == DNF_A::FILTER13
    }
    #[doc = "Checks if the value of the field is `FILTER14`"]
    #[inline(always)]
    pub fn is_filter14(&self) -> bool {
        *self == DNF_A::FILTER14
    }
    #[doc = "Checks if the value of the field is `FILTER15`"]
    #[inline(always)]
    pub fn is_filter15(&self) -> bool {
        *self == DNF_A::FILTER15
    }
}
#[doc = "Field `DNF` writer - Digital noise filter"]
pub type DNF_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CR1_SPEC, 4, O, DNF_A>;
impl<'a, const O: u8> DNF_W<'a, O> {
    #[doc = "Digital filter disabled"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(DNF_A::NO_FILTER)
    }
    #[doc = "Digital filter enabled and filtering capability up to 1 tI2CCLK"]
    #[inline(always)]
    pub fn filter1(self) -> &'a mut W {
        self.variant(DNF_A::FILTER1)
    }
    #[doc = "Digital filter enabled and filtering capability up to 2 tI2CCLK"]
    #[inline(always)]
    pub fn filter2(self) -> &'a mut W {
        self.variant(DNF_A::FILTER2)
    }
    #[doc = "Digital filter enabled and filtering capability up to 3 tI2CCLK"]
    #[inline(always)]
    pub fn filter3(self) -> &'a mut W {
        self.variant(DNF_A::FILTER3)
    }
    #[doc = "Digital filter enabled and filtering capability up to 4 tI2CCLK"]
    #[inline(always)]
    pub fn filter4(self) -> &'a mut W {
        self.variant(DNF_A::FILTER4)
    }
    #[doc = "Digital filter enabled and filtering capability up to 5 tI2CCLK"]
    #[inline(always)]
    pub fn filter5(self) -> &'a mut W {
        self.variant(DNF_A::FILTER5)
    }
    #[doc = "Digital filter enabled and filtering capability up to 6 tI2CCLK"]
    #[inline(always)]
    pub fn filter6(self) -> &'a mut W {
        self.variant(DNF_A::FILTER6)
    }
    #[doc = "Digital filter enabled and filtering capability up to 7 tI2CCLK"]
    #[inline(always)]
    pub fn filter7(self) -> &'a mut W {
        self.variant(DNF_A::FILTER7)
    }
    #[doc = "Digital filter enabled and filtering capability up to 8 tI2CCLK"]
    #[inline(always)]
    pub fn filter8(self) -> &'a mut W {
        self.variant(DNF_A::FILTER8)
    }
    #[doc = "Digital filter enabled and filtering capability up to 9 tI2CCLK"]
    #[inline(always)]
    pub fn filter9(self) -> &'a mut W {
        self.variant(DNF_A::FILTER9)
    }
    #[doc = "Digital filter enabled and filtering capability up to 10 tI2CCLK"]
    #[inline(always)]
    pub fn filter10(self) -> &'a mut W {
        self.variant(DNF_A::FILTER10)
    }
    #[doc = "Digital filter enabled and filtering capability up to 11 tI2CCLK"]
    #[inline(always)]
    pub fn filter11(self) -> &'a mut W {
        self.variant(DNF_A::FILTER11)
    }
    #[doc = "Digital filter enabled and filtering capability up to 12 tI2CCLK"]
    #[inline(always)]
    pub fn filter12(self) -> &'a mut W {
        self.variant(DNF_A::FILTER12)
    }
    #[doc = "Digital filter enabled and filtering capability up to 13 tI2CCLK"]
    #[inline(always)]
    pub fn filter13(self) -> &'a mut W {
        self.variant(DNF_A::FILTER13)
    }
    #[doc = "Digital filter enabled and filtering capability up to 14 tI2CCLK"]
    #[inline(always)]
    pub fn filter14(self) -> &'a mut W {
        self.variant(DNF_A::FILTER14)
    }
    #[doc = "Digital filter enabled and filtering capability up to 15 tI2CCLK"]
    #[inline(always)]
    pub fn filter15(self) -> &'a mut W {
        self.variant(DNF_A::FILTER15)
    }
}
#[doc = "Field `ANFOFF` reader - Analog noise filter OFF"]
pub type ANFOFF_R = crate::BitReader<ANFOFF_A>;
#[doc = "Analog noise filter OFF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ANFOFF_A {
    #[doc = "0: Analog noise filter enabled"]
    ENABLED = 0,
    #[doc = "1: Analog noise filter disabled"]
    DISABLED = 1,
}
impl From<ANFOFF_A> for bool {
    #[inline(always)]
    fn from(variant: ANFOFF_A) -> Self {
        variant as u8 != 0
    }
}
impl ANFOFF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ANFOFF_A {
        match self.bits {
            false => ANFOFF_A::ENABLED,
            true => ANFOFF_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ANFOFF_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ANFOFF_A::DISABLED
    }
}
#[doc = "Field `ANFOFF` writer - Analog noise filter OFF"]
pub type ANFOFF_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, ANFOFF_A>;
impl<'a, const O: u8> ANFOFF_W<'a, O> {
    #[doc = "Analog noise filter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ANFOFF_A::ENABLED)
    }
    #[doc = "Analog noise filter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ANFOFF_A::DISABLED)
    }
}
#[doc = "Field `SWRST` writer - Software reset"]
pub type SWRST_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O>;
#[doc = "Field `TXDMAEN` reader - DMA transmission requests enable"]
pub type TXDMAEN_R = crate::BitReader<TXDMAEN_A>;
#[doc = "DMA transmission requests enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN_A {
    #[doc = "0: DMA mode disabled for transmission"]
    DISABLED = 0,
    #[doc = "1: DMA mode enabled for transmission"]
    ENABLED = 1,
}
impl From<TXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: TXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXDMAEN_A {
        match self.bits {
            false => TXDMAEN_A::DISABLED,
            true => TXDMAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXDMAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXDMAEN_A::ENABLED
    }
}
#[doc = "Field `TXDMAEN` writer - DMA transmission requests enable"]
pub type TXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, TXDMAEN_A>;
impl<'a, const O: u8> TXDMAEN_W<'a, O> {
    #[doc = "DMA mode disabled for transmission"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::DISABLED)
    }
    #[doc = "DMA mode enabled for transmission"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::ENABLED)
    }
}
#[doc = "Field `RXDMAEN` reader - DMA reception requests enable"]
pub type RXDMAEN_R = crate::BitReader<RXDMAEN_A>;
#[doc = "DMA reception requests enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN_A {
    #[doc = "0: DMA mode disabled for reception"]
    DISABLED = 0,
    #[doc = "1: DMA mode enabled for reception"]
    ENABLED = 1,
}
impl From<RXDMAEN_A> for bool {
    #[inline(always)]
    fn from(variant: RXDMAEN_A) -> Self {
        variant as u8 != 0
    }
}
impl RXDMAEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXDMAEN_A {
        match self.bits {
            false => RXDMAEN_A::DISABLED,
            true => RXDMAEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXDMAEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXDMAEN_A::ENABLED
    }
}
#[doc = "Field `RXDMAEN` writer - DMA reception requests enable"]
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, RXDMAEN_A>;
impl<'a, const O: u8> RXDMAEN_W<'a, O> {
    #[doc = "DMA mode disabled for reception"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::DISABLED)
    }
    #[doc = "DMA mode enabled for reception"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::ENABLED)
    }
}
#[doc = "Field `SBC` reader - Slave byte control"]
pub type SBC_R = crate::BitReader<SBC_A>;
#[doc = "Slave byte control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SBC_A {
    #[doc = "0: Slave byte control disabled"]
    DISABLED = 0,
    #[doc = "1: Slave byte control enabled"]
    ENABLED = 1,
}
impl From<SBC_A> for bool {
    #[inline(always)]
    fn from(variant: SBC_A) -> Self {
        variant as u8 != 0
    }
}
impl SBC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SBC_A {
        match self.bits {
            false => SBC_A::DISABLED,
            true => SBC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SBC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SBC_A::ENABLED
    }
}
#[doc = "Field `SBC` writer - Slave byte control"]
pub type SBC_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, SBC_A>;
impl<'a, const O: u8> SBC_W<'a, O> {
    #[doc = "Slave byte control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SBC_A::DISABLED)
    }
    #[doc = "Slave byte control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SBC_A::ENABLED)
    }
}
#[doc = "Field `NOSTRETCH` reader - Clock stretching disable"]
pub type NOSTRETCH_R = crate::BitReader<NOSTRETCH_A>;
#[doc = "Clock stretching disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NOSTRETCH_A {
    #[doc = "0: Clock stretching enabled"]
    ENABLED = 0,
    #[doc = "1: Clock stretching disabled"]
    DISABLED = 1,
}
impl From<NOSTRETCH_A> for bool {
    #[inline(always)]
    fn from(variant: NOSTRETCH_A) -> Self {
        variant as u8 != 0
    }
}
impl NOSTRETCH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NOSTRETCH_A {
        match self.bits {
            false => NOSTRETCH_A::ENABLED,
            true => NOSTRETCH_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NOSTRETCH_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NOSTRETCH_A::DISABLED
    }
}
#[doc = "Field `NOSTRETCH` writer - Clock stretching disable"]
pub type NOSTRETCH_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, NOSTRETCH_A>;
impl<'a, const O: u8> NOSTRETCH_W<'a, O> {
    #[doc = "Clock stretching enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NOSTRETCH_A::ENABLED)
    }
    #[doc = "Clock stretching disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NOSTRETCH_A::DISABLED)
    }
}
#[doc = "Field `WUPEN` reader - Wakeup from STOP enable"]
pub type WUPEN_R = crate::BitReader<WUPEN_A>;
#[doc = "Wakeup from STOP enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUPEN_A {
    #[doc = "0: Wakeup from Stop mode disabled"]
    DISABLED = 0,
    #[doc = "1: Wakeup from Stop mode enabled"]
    ENABLED = 1,
}
impl From<WUPEN_A> for bool {
    #[inline(always)]
    fn from(variant: WUPEN_A) -> Self {
        variant as u8 != 0
    }
}
impl WUPEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUPEN_A {
        match self.bits {
            false => WUPEN_A::DISABLED,
            true => WUPEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUPEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUPEN_A::ENABLED
    }
}
#[doc = "Field `WUPEN` writer - Wakeup from STOP enable"]
pub type WUPEN_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, WUPEN_A>;
impl<'a, const O: u8> WUPEN_W<'a, O> {
    #[doc = "Wakeup from Stop mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUPEN_A::DISABLED)
    }
    #[doc = "Wakeup from Stop mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUPEN_A::ENABLED)
    }
}
#[doc = "Field `GCEN` reader - General call enable"]
pub type GCEN_R = crate::BitReader<GCEN_A>;
#[doc = "General call enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GCEN_A {
    #[doc = "0: General call disabled. Address 0b00000000 is NACKed"]
    DISABLED = 0,
    #[doc = "1: General call enabled. Address 0b00000000 is ACKed"]
    ENABLED = 1,
}
impl From<GCEN_A> for bool {
    #[inline(always)]
    fn from(variant: GCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl GCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> GCEN_A {
        match self.bits {
            false => GCEN_A::DISABLED,
            true => GCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == GCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == GCEN_A::ENABLED
    }
}
#[doc = "Field `GCEN` writer - General call enable"]
pub type GCEN_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, GCEN_A>;
impl<'a, const O: u8> GCEN_W<'a, O> {
    #[doc = "General call disabled. Address 0b00000000 is NACKed"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(GCEN_A::DISABLED)
    }
    #[doc = "General call enabled. Address 0b00000000 is ACKed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(GCEN_A::ENABLED)
    }
}
#[doc = "Field `SMBHEN` reader - SMBus Host address enable"]
pub type SMBHEN_R = crate::BitReader<SMBHEN_A>;
#[doc = "SMBus Host address enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBHEN_A {
    #[doc = "0: Host address disabled. Address 0b0001000x is NACKed"]
    DISABLED = 0,
    #[doc = "1: Host address enabled. Address 0b0001000x is ACKed"]
    ENABLED = 1,
}
impl From<SMBHEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMBHEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SMBHEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMBHEN_A {
        match self.bits {
            false => SMBHEN_A::DISABLED,
            true => SMBHEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMBHEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMBHEN_A::ENABLED
    }
}
#[doc = "Field `SMBHEN` writer - SMBus Host address enable"]
pub type SMBHEN_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, SMBHEN_A>;
impl<'a, const O: u8> SMBHEN_W<'a, O> {
    #[doc = "Host address disabled. Address 0b0001000x is NACKed"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMBHEN_A::DISABLED)
    }
    #[doc = "Host address enabled. Address 0b0001000x is ACKed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SMBHEN_A::ENABLED)
    }
}
#[doc = "Field `SMBDEN` reader - SMBus Device Default address enable"]
pub type SMBDEN_R = crate::BitReader<SMBDEN_A>;
#[doc = "SMBus Device Default address enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SMBDEN_A {
    #[doc = "0: Device default address disabled. Address 0b1100001x is NACKed"]
    DISABLED = 0,
    #[doc = "1: Device default address enabled. Address 0b1100001x is ACKed"]
    ENABLED = 1,
}
impl From<SMBDEN_A> for bool {
    #[inline(always)]
    fn from(variant: SMBDEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SMBDEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMBDEN_A {
        match self.bits {
            false => SMBDEN_A::DISABLED,
            true => SMBDEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SMBDEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SMBDEN_A::ENABLED
    }
}
#[doc = "Field `SMBDEN` writer - SMBus Device Default address enable"]
pub type SMBDEN_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, SMBDEN_A>;
impl<'a, const O: u8> SMBDEN_W<'a, O> {
    #[doc = "Device default address disabled. Address 0b1100001x is NACKed"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SMBDEN_A::DISABLED)
    }
    #[doc = "Device default address enabled. Address 0b1100001x is ACKed"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SMBDEN_A::ENABLED)
    }
}
#[doc = "Field `ALERTEN` reader - SMBUS alert enable"]
pub type ALERTEN_R = crate::BitReader<ALERTEN_A>;
#[doc = "SMBUS alert enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERTEN_A {
    #[doc = "0: In device mode (SMBHEN=Disabled) Releases SMBA pin high and Alert Response Address Header disabled (0001100x) followed by NACK. In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) not supported"]
    DISABLED = 0,
    #[doc = "1: In device mode (SMBHEN=Disabled) Drives SMBA pin low and Alert Response Address Header enabled (0001100x) followed by ACK.In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) supported"]
    ENABLED = 1,
}
impl From<ALERTEN_A> for bool {
    #[inline(always)]
    fn from(variant: ALERTEN_A) -> Self {
        variant as u8 != 0
    }
}
impl ALERTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALERTEN_A {
        match self.bits {
            false => ALERTEN_A::DISABLED,
            true => ALERTEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ALERTEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ALERTEN_A::ENABLED
    }
}
#[doc = "Field `ALERTEN` writer - SMBUS alert enable"]
pub type ALERTEN_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, ALERTEN_A>;
impl<'a, const O: u8> ALERTEN_W<'a, O> {
    #[doc = "In device mode (SMBHEN=Disabled) Releases SMBA pin high and Alert Response Address Header disabled (0001100x) followed by NACK. In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) not supported"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ALERTEN_A::DISABLED)
    }
    #[doc = "In device mode (SMBHEN=Disabled) Drives SMBA pin low and Alert Response Address Header enabled (0001100x) followed by ACK.In host mode (SMBHEN=Enabled) SMBus Alert pin (SMBA) supported"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ALERTEN_A::ENABLED)
    }
}
#[doc = "Field `PECEN` reader - PEC enable"]
pub type PECEN_R = crate::BitReader<PECEN_A>;
#[doc = "PEC enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECEN_A {
    #[doc = "0: PEC calculation disabled"]
    DISABLED = 0,
    #[doc = "1: PEC calculation enabled"]
    ENABLED = 1,
}
impl From<PECEN_A> for bool {
    #[inline(always)]
    fn from(variant: PECEN_A) -> Self {
        variant as u8 != 0
    }
}
impl PECEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECEN_A {
        match self.bits {
            false => PECEN_A::DISABLED,
            true => PECEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PECEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PECEN_A::ENABLED
    }
}
#[doc = "Field `PECEN` writer - PEC enable"]
pub type PECEN_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, PECEN_A>;
impl<'a, const O: u8> PECEN_W<'a, O> {
    #[doc = "PEC calculation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PECEN_A::DISABLED)
    }
    #[doc = "PEC calculation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PECEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    pub fn pe(&self) -> PE_R {
        PE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - TX Interrupt enable"]
    #[inline(always)]
    pub fn txie(&self) -> TXIE_R {
        TXIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - RX Interrupt enable"]
    #[inline(always)]
    pub fn rxie(&self) -> RXIE_R {
        RXIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address match interrupt enable (slave only)"]
    #[inline(always)]
    pub fn addrie(&self) -> ADDRIE_R {
        ADDRIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Not acknowledge received interrupt enable"]
    #[inline(always)]
    pub fn nackie(&self) -> NACKIE_R {
        NACKIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - STOP detection Interrupt enable"]
    #[inline(always)]
    pub fn stopie(&self) -> STOPIE_R {
        STOPIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Error interrupts enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Digital noise filter"]
    #[inline(always)]
    pub fn dnf(&self) -> DNF_R {
        DNF_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - Analog noise filter OFF"]
    #[inline(always)]
    pub fn anfoff(&self) -> ANFOFF_R {
        ANFOFF_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - DMA transmission requests enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - DMA reception requests enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Slave byte control"]
    #[inline(always)]
    pub fn sbc(&self) -> SBC_R {
        SBC_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Clock stretching disable"]
    #[inline(always)]
    pub fn nostretch(&self) -> NOSTRETCH_R {
        NOSTRETCH_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Wakeup from STOP enable"]
    #[inline(always)]
    pub fn wupen(&self) -> WUPEN_R {
        WUPEN_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - General call enable"]
    #[inline(always)]
    pub fn gcen(&self) -> GCEN_R {
        GCEN_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - SMBus Host address enable"]
    #[inline(always)]
    pub fn smbhen(&self) -> SMBHEN_R {
        SMBHEN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - SMBus Device Default address enable"]
    #[inline(always)]
    pub fn smbden(&self) -> SMBDEN_R {
        SMBDEN_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - SMBUS alert enable"]
    #[inline(always)]
    pub fn alerten(&self) -> ALERTEN_R {
        ALERTEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - PEC enable"]
    #[inline(always)]
    pub fn pecen(&self) -> PECEN_R {
        PECEN_R::new(((self.bits >> 23) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Peripheral enable"]
    #[inline(always)]
    #[must_use]
    pub fn pe(&mut self) -> PE_W<0> {
        PE_W::new(self)
    }
    #[doc = "Bit 1 - TX Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txie(&mut self) -> TXIE_W<1> {
        TXIE_W::new(self)
    }
    #[doc = "Bit 2 - RX Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxie(&mut self) -> RXIE_W<2> {
        RXIE_W::new(self)
    }
    #[doc = "Bit 3 - Address match interrupt enable (slave only)"]
    #[inline(always)]
    #[must_use]
    pub fn addrie(&mut self) -> ADDRIE_W<3> {
        ADDRIE_W::new(self)
    }
    #[doc = "Bit 4 - Not acknowledge received interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn nackie(&mut self) -> NACKIE_W<4> {
        NACKIE_W::new(self)
    }
    #[doc = "Bit 5 - STOP detection Interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn stopie(&mut self) -> STOPIE_W<5> {
        STOPIE_W::new(self)
    }
    #[doc = "Bit 6 - Transfer Complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<6> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 7 - Error interrupts enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<7> {
        ERRIE_W::new(self)
    }
    #[doc = "Bits 8:11 - Digital noise filter"]
    #[inline(always)]
    #[must_use]
    pub fn dnf(&mut self) -> DNF_W<8> {
        DNF_W::new(self)
    }
    #[doc = "Bit 12 - Analog noise filter OFF"]
    #[inline(always)]
    #[must_use]
    pub fn anfoff(&mut self) -> ANFOFF_W<12> {
        ANFOFF_W::new(self)
    }
    #[doc = "Bit 13 - Software reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SWRST_W<13> {
        SWRST_W::new(self)
    }
    #[doc = "Bit 14 - DMA transmission requests enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<14> {
        TXDMAEN_W::new(self)
    }
    #[doc = "Bit 15 - DMA reception requests enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<15> {
        RXDMAEN_W::new(self)
    }
    #[doc = "Bit 16 - Slave byte control"]
    #[inline(always)]
    #[must_use]
    pub fn sbc(&mut self) -> SBC_W<16> {
        SBC_W::new(self)
    }
    #[doc = "Bit 17 - Clock stretching disable"]
    #[inline(always)]
    #[must_use]
    pub fn nostretch(&mut self) -> NOSTRETCH_W<17> {
        NOSTRETCH_W::new(self)
    }
    #[doc = "Bit 18 - Wakeup from STOP enable"]
    #[inline(always)]
    #[must_use]
    pub fn wupen(&mut self) -> WUPEN_W<18> {
        WUPEN_W::new(self)
    }
    #[doc = "Bit 19 - General call enable"]
    #[inline(always)]
    #[must_use]
    pub fn gcen(&mut self) -> GCEN_W<19> {
        GCEN_W::new(self)
    }
    #[doc = "Bit 20 - SMBus Host address enable"]
    #[inline(always)]
    #[must_use]
    pub fn smbhen(&mut self) -> SMBHEN_W<20> {
        SMBHEN_W::new(self)
    }
    #[doc = "Bit 21 - SMBus Device Default address enable"]
    #[inline(always)]
    #[must_use]
    pub fn smbden(&mut self) -> SMBDEN_W<21> {
        SMBDEN_W::new(self)
    }
    #[doc = "Bit 22 - SMBUS alert enable"]
    #[inline(always)]
    #[must_use]
    pub fn alerten(&mut self) -> ALERTEN_W<22> {
        ALERTEN_W::new(self)
    }
    #[doc = "Bit 23 - PEC enable"]
    #[inline(always)]
    #[must_use]
    pub fn pecen(&mut self) -> PECEN_W<23> {
        PECEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr1::R](R) reader structure"]
impl crate::Readable for CR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr1::W](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR1 to value 0"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
