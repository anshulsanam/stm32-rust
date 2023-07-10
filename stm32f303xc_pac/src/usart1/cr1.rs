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
#[doc = "Field `UE` reader - USART enable"]
pub type UE_R = crate::BitReader<UE_A>;
#[doc = "USART enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UE_A {
    #[doc = "0: UART is disabled"]
    DISABLED = 0,
    #[doc = "1: UART is enabled"]
    ENABLED = 1,
}
impl From<UE_A> for bool {
    #[inline(always)]
    fn from(variant: UE_A) -> Self {
        variant as u8 != 0
    }
}
impl UE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UE_A {
        match self.bits {
            false => UE_A::DISABLED,
            true => UE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UE_A::ENABLED
    }
}
#[doc = "Field `UE` writer - USART enable"]
pub type UE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, UE_A>;
impl<'a, const O: u8> UE_W<'a, O> {
    #[doc = "UART is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UE_A::DISABLED)
    }
    #[doc = "UART is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UE_A::ENABLED)
    }
}
#[doc = "Field `UESM` reader - USART enable in Stop mode"]
pub type UESM_R = crate::BitReader<UESM_A>;
#[doc = "USART enable in Stop mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UESM_A {
    #[doc = "0: USART not able to wake up the MCU from Stop mode"]
    DISABLED = 0,
    #[doc = "1: USART able to wake up the MCU from Stop mode"]
    ENABLED = 1,
}
impl From<UESM_A> for bool {
    #[inline(always)]
    fn from(variant: UESM_A) -> Self {
        variant as u8 != 0
    }
}
impl UESM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UESM_A {
        match self.bits {
            false => UESM_A::DISABLED,
            true => UESM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UESM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UESM_A::ENABLED
    }
}
#[doc = "Field `UESM` writer - USART enable in Stop mode"]
pub type UESM_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, UESM_A>;
impl<'a, const O: u8> UESM_W<'a, O> {
    #[doc = "USART not able to wake up the MCU from Stop mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UESM_A::DISABLED)
    }
    #[doc = "USART able to wake up the MCU from Stop mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UESM_A::ENABLED)
    }
}
#[doc = "Field `RE` reader - Receiver enable"]
pub type RE_R = crate::BitReader<RE_A>;
#[doc = "Receiver enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RE_A {
    #[doc = "0: Receiver is disabled"]
    DISABLED = 0,
    #[doc = "1: Receiver is enabled"]
    ENABLED = 1,
}
impl From<RE_A> for bool {
    #[inline(always)]
    fn from(variant: RE_A) -> Self {
        variant as u8 != 0
    }
}
impl RE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RE_A {
        match self.bits {
            false => RE_A::DISABLED,
            true => RE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RE_A::ENABLED
    }
}
#[doc = "Field `RE` writer - Receiver enable"]
pub type RE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, RE_A>;
impl<'a, const O: u8> RE_W<'a, O> {
    #[doc = "Receiver is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RE_A::DISABLED)
    }
    #[doc = "Receiver is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RE_A::ENABLED)
    }
}
#[doc = "Field `TE` reader - Transmitter enable"]
pub type TE_R = crate::BitReader<TE_A>;
#[doc = "Transmitter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TE_A {
    #[doc = "0: Transmitter is disabled"]
    DISABLED = 0,
    #[doc = "1: Transmitter is enabled"]
    ENABLED = 1,
}
impl From<TE_A> for bool {
    #[inline(always)]
    fn from(variant: TE_A) -> Self {
        variant as u8 != 0
    }
}
impl TE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TE_A {
        match self.bits {
            false => TE_A::DISABLED,
            true => TE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TE_A::ENABLED
    }
}
#[doc = "Field `TE` writer - Transmitter enable"]
pub type TE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, TE_A>;
impl<'a, const O: u8> TE_W<'a, O> {
    #[doc = "Transmitter is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TE_A::DISABLED)
    }
    #[doc = "Transmitter is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TE_A::ENABLED)
    }
}
#[doc = "Field `IDLEIE` reader - IDLE interrupt enable"]
pub type IDLEIE_R = crate::BitReader<IDLEIE_A>;
#[doc = "IDLE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IDLEIE_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is generated whenever IDLE=1 in the ISR register"]
    ENABLED = 1,
}
impl From<IDLEIE_A> for bool {
    #[inline(always)]
    fn from(variant: IDLEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl IDLEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IDLEIE_A {
        match self.bits {
            false => IDLEIE_A::DISABLED,
            true => IDLEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IDLEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IDLEIE_A::ENABLED
    }
}
#[doc = "Field `IDLEIE` writer - IDLE interrupt enable"]
pub type IDLEIE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, IDLEIE_A>;
impl<'a, const O: u8> IDLEIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IDLEIE_A::DISABLED)
    }
    #[doc = "Interrupt is generated whenever IDLE=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IDLEIE_A::ENABLED)
    }
}
#[doc = "Field `RXNEIE` reader - RXNE interrupt enable"]
pub type RXNEIE_R = crate::BitReader<RXNEIE_A>;
#[doc = "RXNE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNEIE_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is generated whenever ORE=1 or RXNE=1 in the ISR register"]
    ENABLED = 1,
}
impl From<RXNEIE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXNEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXNEIE_A {
        match self.bits {
            false => RXNEIE_A::DISABLED,
            true => RXNEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RXNEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RXNEIE_A::ENABLED
    }
}
#[doc = "Field `RXNEIE` writer - RXNE interrupt enable"]
pub type RXNEIE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, RXNEIE_A>;
impl<'a, const O: u8> RXNEIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXNEIE_A::DISABLED)
    }
    #[doc = "Interrupt is generated whenever ORE=1 or RXNE=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXNEIE_A::ENABLED)
    }
}
#[doc = "Field `TCIE` reader - Transmission complete interrupt enable"]
pub type TCIE_R = crate::BitReader<TCIE_A>;
#[doc = "Transmission complete interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCIE_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is generated whenever TC=1 in the ISR register"]
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
#[doc = "Field `TCIE` writer - Transmission complete interrupt enable"]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, TCIE_A>;
impl<'a, const O: u8> TCIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TCIE_A::DISABLED)
    }
    #[doc = "Interrupt is generated whenever TC=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TCIE_A::ENABLED)
    }
}
#[doc = "Field `TXEIE` reader - interrupt enable"]
pub type TXEIE_R = crate::BitReader<TXEIE_A>;
#[doc = "interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEIE_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is generated whenever TXE=1 in the ISR register"]
    ENABLED = 1,
}
impl From<TXEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TXEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXEIE_A {
        match self.bits {
            false => TXEIE_A::DISABLED,
            true => TXEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TXEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TXEIE_A::ENABLED
    }
}
#[doc = "Field `TXEIE` writer - interrupt enable"]
pub type TXEIE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, TXEIE_A>;
impl<'a, const O: u8> TXEIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXEIE_A::DISABLED)
    }
    #[doc = "Interrupt is generated whenever TXE=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXEIE_A::ENABLED)
    }
}
#[doc = "Field `PEIE` reader - PE interrupt enable"]
pub type PEIE_R = crate::BitReader<PEIE_A>;
#[doc = "PE interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PEIE_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is generated whenever PE=1 in the ISR register"]
    ENABLED = 1,
}
impl From<PEIE_A> for bool {
    #[inline(always)]
    fn from(variant: PEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl PEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PEIE_A {
        match self.bits {
            false => PEIE_A::DISABLED,
            true => PEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PEIE_A::ENABLED
    }
}
#[doc = "Field `PEIE` writer - PE interrupt enable"]
pub type PEIE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, PEIE_A>;
impl<'a, const O: u8> PEIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PEIE_A::DISABLED)
    }
    #[doc = "Interrupt is generated whenever PE=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PEIE_A::ENABLED)
    }
}
#[doc = "Field `PS` reader - Parity selection"]
pub type PS_R = crate::BitReader<PS_A>;
#[doc = "Parity selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PS_A {
    #[doc = "0: Even parity"]
    EVEN = 0,
    #[doc = "1: Odd parity"]
    ODD = 1,
}
impl From<PS_A> for bool {
    #[inline(always)]
    fn from(variant: PS_A) -> Self {
        variant as u8 != 0
    }
}
impl PS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PS_A {
        match self.bits {
            false => PS_A::EVEN,
            true => PS_A::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == PS_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == PS_A::ODD
    }
}
#[doc = "Field `PS` writer - Parity selection"]
pub type PS_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, PS_A>;
impl<'a, const O: u8> PS_W<'a, O> {
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(PS_A::EVEN)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(PS_A::ODD)
    }
}
#[doc = "Field `PCE` reader - Parity control enable"]
pub type PCE_R = crate::BitReader<PCE_A>;
#[doc = "Parity control enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PCE_A {
    #[doc = "0: Parity control disabled"]
    DISABLED = 0,
    #[doc = "1: Parity control enabled"]
    ENABLED = 1,
}
impl From<PCE_A> for bool {
    #[inline(always)]
    fn from(variant: PCE_A) -> Self {
        variant as u8 != 0
    }
}
impl PCE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PCE_A {
        match self.bits {
            false => PCE_A::DISABLED,
            true => PCE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PCE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PCE_A::ENABLED
    }
}
#[doc = "Field `PCE` writer - Parity control enable"]
pub type PCE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, PCE_A>;
impl<'a, const O: u8> PCE_W<'a, O> {
    #[doc = "Parity control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PCE_A::DISABLED)
    }
    #[doc = "Parity control enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PCE_A::ENABLED)
    }
}
#[doc = "Field `WAKE` reader - Receiver wakeup method"]
pub type WAKE_R = crate::BitReader<WAKE_A>;
#[doc = "Receiver wakeup method\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WAKE_A {
    #[doc = "0: Idle line"]
    IDLE = 0,
    #[doc = "1: Address mask"]
    ADDRESS = 1,
}
impl From<WAKE_A> for bool {
    #[inline(always)]
    fn from(variant: WAKE_A) -> Self {
        variant as u8 != 0
    }
}
impl WAKE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WAKE_A {
        match self.bits {
            false => WAKE_A::IDLE,
            true => WAKE_A::ADDRESS,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE`"]
    #[inline(always)]
    pub fn is_idle(&self) -> bool {
        *self == WAKE_A::IDLE
    }
    #[doc = "Checks if the value of the field is `ADDRESS`"]
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == WAKE_A::ADDRESS
    }
}
#[doc = "Field `WAKE` writer - Receiver wakeup method"]
pub type WAKE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, WAKE_A>;
impl<'a, const O: u8> WAKE_W<'a, O> {
    #[doc = "Idle line"]
    #[inline(always)]
    pub fn idle(self) -> &'a mut W {
        self.variant(WAKE_A::IDLE)
    }
    #[doc = "Address mask"]
    #[inline(always)]
    pub fn address(self) -> &'a mut W {
        self.variant(WAKE_A::ADDRESS)
    }
}
#[doc = "Field `M` reader - Word length"]
pub type M_R = crate::BitReader<M_A>;
#[doc = "Word length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum M_A {
    #[doc = "0: 1 start bit, 8 data bits, n stop bits"]
    BIT8 = 0,
    #[doc = "1: 1 start bit, 9 data bits, n stop bits"]
    BIT9 = 1,
}
impl From<M_A> for bool {
    #[inline(always)]
    fn from(variant: M_A) -> Self {
        variant as u8 != 0
    }
}
impl M_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> M_A {
        match self.bits {
            false => M_A::BIT8,
            true => M_A::BIT9,
        }
    }
    #[doc = "Checks if the value of the field is `BIT8`"]
    #[inline(always)]
    pub fn is_bit8(&self) -> bool {
        *self == M_A::BIT8
    }
    #[doc = "Checks if the value of the field is `BIT9`"]
    #[inline(always)]
    pub fn is_bit9(&self) -> bool {
        *self == M_A::BIT9
    }
}
#[doc = "Field `M` writer - Word length"]
pub type M_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, M_A>;
impl<'a, const O: u8> M_W<'a, O> {
    #[doc = "1 start bit, 8 data bits, n stop bits"]
    #[inline(always)]
    pub fn bit8(self) -> &'a mut W {
        self.variant(M_A::BIT8)
    }
    #[doc = "1 start bit, 9 data bits, n stop bits"]
    #[inline(always)]
    pub fn bit9(self) -> &'a mut W {
        self.variant(M_A::BIT9)
    }
}
#[doc = "Field `MME` reader - Mute mode enable"]
pub type MME_R = crate::BitReader<MME_A>;
#[doc = "Mute mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MME_A {
    #[doc = "0: Receiver in active mode permanently"]
    DISABLED = 0,
    #[doc = "1: Receiver can switch between mute mode and active mode"]
    ENABLED = 1,
}
impl From<MME_A> for bool {
    #[inline(always)]
    fn from(variant: MME_A) -> Self {
        variant as u8 != 0
    }
}
impl MME_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MME_A {
        match self.bits {
            false => MME_A::DISABLED,
            true => MME_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MME_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MME_A::ENABLED
    }
}
#[doc = "Field `MME` writer - Mute mode enable"]
pub type MME_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, MME_A>;
impl<'a, const O: u8> MME_W<'a, O> {
    #[doc = "Receiver in active mode permanently"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MME_A::DISABLED)
    }
    #[doc = "Receiver can switch between mute mode and active mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MME_A::ENABLED)
    }
}
#[doc = "Field `CMIE` reader - Character match interrupt enable"]
pub type CMIE_R = crate::BitReader<CMIE_A>;
#[doc = "Character match interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CMIE_A {
    #[doc = "0: Interrupt is disabled"]
    DISABLED = 0,
    #[doc = "1: Interrupt is generated when the CMF bit is set in the ISR register"]
    ENABLED = 1,
}
impl From<CMIE_A> for bool {
    #[inline(always)]
    fn from(variant: CMIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CMIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CMIE_A {
        match self.bits {
            false => CMIE_A::DISABLED,
            true => CMIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CMIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CMIE_A::ENABLED
    }
}
#[doc = "Field `CMIE` writer - Character match interrupt enable"]
pub type CMIE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, CMIE_A>;
impl<'a, const O: u8> CMIE_W<'a, O> {
    #[doc = "Interrupt is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CMIE_A::DISABLED)
    }
    #[doc = "Interrupt is generated when the CMF bit is set in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CMIE_A::ENABLED)
    }
}
#[doc = "Field `OVER8` reader - Oversampling mode"]
pub type OVER8_R = crate::BitReader<OVER8_A>;
#[doc = "Oversampling mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVER8_A {
    #[doc = "0: Oversampling by 16"]
    OVERSAMPLING16 = 0,
    #[doc = "1: Oversampling by 8"]
    OVERSAMPLING8 = 1,
}
impl From<OVER8_A> for bool {
    #[inline(always)]
    fn from(variant: OVER8_A) -> Self {
        variant as u8 != 0
    }
}
impl OVER8_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVER8_A {
        match self.bits {
            false => OVER8_A::OVERSAMPLING16,
            true => OVER8_A::OVERSAMPLING8,
        }
    }
    #[doc = "Checks if the value of the field is `OVERSAMPLING16`"]
    #[inline(always)]
    pub fn is_oversampling16(&self) -> bool {
        *self == OVER8_A::OVERSAMPLING16
    }
    #[doc = "Checks if the value of the field is `OVERSAMPLING8`"]
    #[inline(always)]
    pub fn is_oversampling8(&self) -> bool {
        *self == OVER8_A::OVERSAMPLING8
    }
}
#[doc = "Field `OVER8` writer - Oversampling mode"]
pub type OVER8_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, OVER8_A>;
impl<'a, const O: u8> OVER8_W<'a, O> {
    #[doc = "Oversampling by 16"]
    #[inline(always)]
    pub fn oversampling16(self) -> &'a mut W {
        self.variant(OVER8_A::OVERSAMPLING16)
    }
    #[doc = "Oversampling by 8"]
    #[inline(always)]
    pub fn oversampling8(self) -> &'a mut W {
        self.variant(OVER8_A::OVERSAMPLING8)
    }
}
#[doc = "Field `DEDT` reader - Driver Enable deassertion time"]
pub type DEDT_R = crate::FieldReader;
#[doc = "Field `DEDT` writer - Driver Enable deassertion time"]
pub type DEDT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CR1_SPEC, 5, O>;
#[doc = "Field `DEAT` reader - Driver Enable assertion time"]
pub type DEAT_R = crate::FieldReader;
#[doc = "Field `DEAT` writer - Driver Enable assertion time"]
pub type DEAT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CR1_SPEC, 5, O>;
#[doc = "Field `RTOIE` reader - Receiver timeout interrupt enable"]
pub type RTOIE_R = crate::BitReader<RTOIE_A>;
#[doc = "Receiver timeout interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTOIE_A {
    #[doc = "0: Interrupt is inhibited"]
    DISABLED = 0,
    #[doc = "1: An USART interrupt is generated when the RTOF bit is set in the ISR register"]
    ENABLED = 1,
}
impl From<RTOIE_A> for bool {
    #[inline(always)]
    fn from(variant: RTOIE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTOIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTOIE_A {
        match self.bits {
            false => RTOIE_A::DISABLED,
            true => RTOIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTOIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTOIE_A::ENABLED
    }
}
#[doc = "Field `RTOIE` writer - Receiver timeout interrupt enable"]
pub type RTOIE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, RTOIE_A>;
impl<'a, const O: u8> RTOIE_W<'a, O> {
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTOIE_A::DISABLED)
    }
    #[doc = "An USART interrupt is generated when the RTOF bit is set in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTOIE_A::ENABLED)
    }
}
#[doc = "Field `EOBIE` reader - End of Block interrupt enable"]
pub type EOBIE_R = crate::BitReader<EOBIE_A>;
#[doc = "End of Block interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EOBIE_A {
    #[doc = "0: Interrupt is inhibited"]
    DISABLED = 0,
    #[doc = "1: A USART interrupt is generated when the EOBF flag is set in the ISR register"]
    ENABLED = 1,
}
impl From<EOBIE_A> for bool {
    #[inline(always)]
    fn from(variant: EOBIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EOBIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EOBIE_A {
        match self.bits {
            false => EOBIE_A::DISABLED,
            true => EOBIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EOBIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EOBIE_A::ENABLED
    }
}
#[doc = "Field `EOBIE` writer - End of Block interrupt enable"]
pub type EOBIE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, EOBIE_A>;
impl<'a, const O: u8> EOBIE_W<'a, O> {
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EOBIE_A::DISABLED)
    }
    #[doc = "A USART interrupt is generated when the EOBF flag is set in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EOBIE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - USART enable"]
    #[inline(always)]
    pub fn ue(&self) -> UE_R {
        UE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - USART enable in Stop mode"]
    #[inline(always)]
    pub fn uesm(&self) -> UESM_R {
        UESM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    pub fn re(&self) -> RE_R {
        RE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    pub fn te(&self) -> TE_R {
        TE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    pub fn idleie(&self) -> IDLEIE_R {
        IDLEIE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - interrupt enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline(always)]
    pub fn peie(&self) -> PEIE_R {
        PEIE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    pub fn ps(&self) -> PS_R {
        PS_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    pub fn pce(&self) -> PCE_R {
        PCE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Receiver wakeup method"]
    #[inline(always)]
    pub fn wake(&self) -> WAKE_R {
        WAKE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    pub fn m(&self) -> M_R {
        M_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Mute mode enable"]
    #[inline(always)]
    pub fn mme(&self) -> MME_R {
        MME_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Character match interrupt enable"]
    #[inline(always)]
    pub fn cmie(&self) -> CMIE_R {
        CMIE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Oversampling mode"]
    #[inline(always)]
    pub fn over8(&self) -> OVER8_R {
        OVER8_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time"]
    #[inline(always)]
    pub fn dedt(&self) -> DEDT_R {
        DEDT_R::new(((self.bits >> 16) & 0x1f) as u8)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time"]
    #[inline(always)]
    pub fn deat(&self) -> DEAT_R {
        DEAT_R::new(((self.bits >> 21) & 0x1f) as u8)
    }
    #[doc = "Bit 26 - Receiver timeout interrupt enable"]
    #[inline(always)]
    pub fn rtoie(&self) -> RTOIE_R {
        RTOIE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - End of Block interrupt enable"]
    #[inline(always)]
    pub fn eobie(&self) -> EOBIE_R {
        EOBIE_R::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - USART enable"]
    #[inline(always)]
    #[must_use]
    pub fn ue(&mut self) -> UE_W<0> {
        UE_W::new(self)
    }
    #[doc = "Bit 1 - USART enable in Stop mode"]
    #[inline(always)]
    #[must_use]
    pub fn uesm(&mut self) -> UESM_W<1> {
        UESM_W::new(self)
    }
    #[doc = "Bit 2 - Receiver enable"]
    #[inline(always)]
    #[must_use]
    pub fn re(&mut self) -> RE_W<2> {
        RE_W::new(self)
    }
    #[doc = "Bit 3 - Transmitter enable"]
    #[inline(always)]
    #[must_use]
    pub fn te(&mut self) -> TE_W<3> {
        TE_W::new(self)
    }
    #[doc = "Bit 4 - IDLE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn idleie(&mut self) -> IDLEIE_W<4> {
        IDLEIE_W::new(self)
    }
    #[doc = "Bit 5 - RXNE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<5> {
        RXNEIE_W::new(self)
    }
    #[doc = "Bit 6 - Transmission complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<6> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 7 - interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TXEIE_W<7> {
        TXEIE_W::new(self)
    }
    #[doc = "Bit 8 - PE interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn peie(&mut self) -> PEIE_W<8> {
        PEIE_W::new(self)
    }
    #[doc = "Bit 9 - Parity selection"]
    #[inline(always)]
    #[must_use]
    pub fn ps(&mut self) -> PS_W<9> {
        PS_W::new(self)
    }
    #[doc = "Bit 10 - Parity control enable"]
    #[inline(always)]
    #[must_use]
    pub fn pce(&mut self) -> PCE_W<10> {
        PCE_W::new(self)
    }
    #[doc = "Bit 11 - Receiver wakeup method"]
    #[inline(always)]
    #[must_use]
    pub fn wake(&mut self) -> WAKE_W<11> {
        WAKE_W::new(self)
    }
    #[doc = "Bit 12 - Word length"]
    #[inline(always)]
    #[must_use]
    pub fn m(&mut self) -> M_W<12> {
        M_W::new(self)
    }
    #[doc = "Bit 13 - Mute mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn mme(&mut self) -> MME_W<13> {
        MME_W::new(self)
    }
    #[doc = "Bit 14 - Character match interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn cmie(&mut self) -> CMIE_W<14> {
        CMIE_W::new(self)
    }
    #[doc = "Bit 15 - Oversampling mode"]
    #[inline(always)]
    #[must_use]
    pub fn over8(&mut self) -> OVER8_W<15> {
        OVER8_W::new(self)
    }
    #[doc = "Bits 16:20 - Driver Enable deassertion time"]
    #[inline(always)]
    #[must_use]
    pub fn dedt(&mut self) -> DEDT_W<16> {
        DEDT_W::new(self)
    }
    #[doc = "Bits 21:25 - Driver Enable assertion time"]
    #[inline(always)]
    #[must_use]
    pub fn deat(&mut self) -> DEAT_W<21> {
        DEAT_W::new(self)
    }
    #[doc = "Bit 26 - Receiver timeout interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtoie(&mut self) -> RTOIE_W<26> {
        RTOIE_W::new(self)
    }
    #[doc = "Bit 27 - End of Block interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eobie(&mut self) -> EOBIE_W<27> {
        EOBIE_W::new(self)
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
