#[doc = "Register `CR3` reader"]
pub struct R(crate::R<CR3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR3` writer"]
pub struct W(crate::W<CR3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR3_SPEC>;
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
impl From<crate::W<CR3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EIE` reader - Error interrupt enable"]
pub type EIE_R = crate::BitReader<EIE_A>;
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EIE_A {
    #[doc = "0: Interrupt is inhibited"]
    DISABLED = 0,
    #[doc = "1: An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register"]
    ENABLED = 1,
}
impl From<EIE_A> for bool {
    #[inline(always)]
    fn from(variant: EIE_A) -> Self {
        variant as u8 != 0
    }
}
impl EIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EIE_A {
        match self.bits {
            false => EIE_A::DISABLED,
            true => EIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EIE_A::ENABLED
    }
}
#[doc = "Field `EIE` writer - Error interrupt enable"]
pub type EIE_W<'a, const O: u8> = crate::BitWriter<'a, CR3_SPEC, O, EIE_A>;
impl<'a, const O: u8> EIE_W<'a, O> {
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EIE_A::DISABLED)
    }
    #[doc = "An interrupt is generated when FE=1 or ORE=1 or NF=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EIE_A::ENABLED)
    }
}
#[doc = "Field `IREN` reader - IrDA mode enable"]
pub type IREN_R = crate::BitReader<IREN_A>;
#[doc = "IrDA mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IREN_A {
    #[doc = "0: IrDA disabled"]
    DISABLED = 0,
    #[doc = "1: IrDA enabled"]
    ENABLED = 1,
}
impl From<IREN_A> for bool {
    #[inline(always)]
    fn from(variant: IREN_A) -> Self {
        variant as u8 != 0
    }
}
impl IREN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IREN_A {
        match self.bits {
            false => IREN_A::DISABLED,
            true => IREN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == IREN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == IREN_A::ENABLED
    }
}
#[doc = "Field `IREN` writer - IrDA mode enable"]
pub type IREN_W<'a, const O: u8> = crate::BitWriter<'a, CR3_SPEC, O, IREN_A>;
impl<'a, const O: u8> IREN_W<'a, O> {
    #[doc = "IrDA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(IREN_A::DISABLED)
    }
    #[doc = "IrDA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(IREN_A::ENABLED)
    }
}
#[doc = "Field `IRLP` reader - IrDA low-power"]
pub type IRLP_R = crate::BitReader<IRLP_A>;
#[doc = "IrDA low-power\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IRLP_A {
    #[doc = "0: Normal mode"]
    NORMAL = 0,
    #[doc = "1: Low-power mode"]
    LOW_POWER = 1,
}
impl From<IRLP_A> for bool {
    #[inline(always)]
    fn from(variant: IRLP_A) -> Self {
        variant as u8 != 0
    }
}
impl IRLP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IRLP_A {
        match self.bits {
            false => IRLP_A::NORMAL,
            true => IRLP_A::LOW_POWER,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == IRLP_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `LOW_POWER`"]
    #[inline(always)]
    pub fn is_low_power(&self) -> bool {
        *self == IRLP_A::LOW_POWER
    }
}
#[doc = "Field `IRLP` writer - IrDA low-power"]
pub type IRLP_W<'a, const O: u8> = crate::BitWriter<'a, CR3_SPEC, O, IRLP_A>;
impl<'a, const O: u8> IRLP_W<'a, O> {
    #[doc = "Normal mode"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(IRLP_A::NORMAL)
    }
    #[doc = "Low-power mode"]
    #[inline(always)]
    pub fn low_power(self) -> &'a mut W {
        self.variant(IRLP_A::LOW_POWER)
    }
}
#[doc = "Field `HDSEL` reader - Half-duplex selection"]
pub type HDSEL_R = crate::BitReader<HDSEL_A>;
#[doc = "Half-duplex selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HDSEL_A {
    #[doc = "0: Half duplex mode is not selected"]
    NOT_SELECTED = 0,
    #[doc = "1: Half duplex mode is selected"]
    SELECTED = 1,
}
impl From<HDSEL_A> for bool {
    #[inline(always)]
    fn from(variant: HDSEL_A) -> Self {
        variant as u8 != 0
    }
}
impl HDSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HDSEL_A {
        match self.bits {
            false => HDSEL_A::NOT_SELECTED,
            true => HDSEL_A::SELECTED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_SELECTED`"]
    #[inline(always)]
    pub fn is_not_selected(&self) -> bool {
        *self == HDSEL_A::NOT_SELECTED
    }
    #[doc = "Checks if the value of the field is `SELECTED`"]
    #[inline(always)]
    pub fn is_selected(&self) -> bool {
        *self == HDSEL_A::SELECTED
    }
}
#[doc = "Field `HDSEL` writer - Half-duplex selection"]
pub type HDSEL_W<'a, const O: u8> = crate::BitWriter<'a, CR3_SPEC, O, HDSEL_A>;
impl<'a, const O: u8> HDSEL_W<'a, O> {
    #[doc = "Half duplex mode is not selected"]
    #[inline(always)]
    pub fn not_selected(self) -> &'a mut W {
        self.variant(HDSEL_A::NOT_SELECTED)
    }
    #[doc = "Half duplex mode is selected"]
    #[inline(always)]
    pub fn selected(self) -> &'a mut W {
        self.variant(HDSEL_A::SELECTED)
    }
}
#[doc = "Field `NACK` reader - Smartcard NACK enable"]
pub type NACK_R = crate::BitReader<NACK_A>;
#[doc = "Smartcard NACK enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACK_A {
    #[doc = "0: NACK transmission in case of parity error is disabled"]
    DISABLED = 0,
    #[doc = "1: NACK transmission during parity error is enabled"]
    ENABLED = 1,
}
impl From<NACK_A> for bool {
    #[inline(always)]
    fn from(variant: NACK_A) -> Self {
        variant as u8 != 0
    }
}
impl NACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACK_A {
        match self.bits {
            false => NACK_A::DISABLED,
            true => NACK_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == NACK_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == NACK_A::ENABLED
    }
}
#[doc = "Field `NACK` writer - Smartcard NACK enable"]
pub type NACK_W<'a, const O: u8> = crate::BitWriter<'a, CR3_SPEC, O, NACK_A>;
impl<'a, const O: u8> NACK_W<'a, O> {
    #[doc = "NACK transmission in case of parity error is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(NACK_A::DISABLED)
    }
    #[doc = "NACK transmission during parity error is enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(NACK_A::ENABLED)
    }
}
#[doc = "Field `SCEN` reader - Smartcard mode enable"]
pub type SCEN_R = crate::BitReader<SCEN_A>;
#[doc = "Smartcard mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SCEN_A {
    #[doc = "0: Smartcard Mode disabled"]
    DISABLED = 0,
    #[doc = "1: Smartcard Mode enabled"]
    ENABLED = 1,
}
impl From<SCEN_A> for bool {
    #[inline(always)]
    fn from(variant: SCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl SCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SCEN_A {
        match self.bits {
            false => SCEN_A::DISABLED,
            true => SCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SCEN_A::ENABLED
    }
}
#[doc = "Field `SCEN` writer - Smartcard mode enable"]
pub type SCEN_W<'a, const O: u8> = crate::BitWriter<'a, CR3_SPEC, O, SCEN_A>;
impl<'a, const O: u8> SCEN_W<'a, O> {
    #[doc = "Smartcard Mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SCEN_A::DISABLED)
    }
    #[doc = "Smartcard Mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SCEN_A::ENABLED)
    }
}
#[doc = "Field `DMAR` reader - DMA enable receiver"]
pub type DMAR_R = crate::BitReader<DMAR_A>;
#[doc = "DMA enable receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAR_A {
    #[doc = "0: DMA mode is disabled for reception"]
    DISABLED = 0,
    #[doc = "1: DMA mode is enabled for reception"]
    ENABLED = 1,
}
impl From<DMAR_A> for bool {
    #[inline(always)]
    fn from(variant: DMAR_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAR_A {
        match self.bits {
            false => DMAR_A::DISABLED,
            true => DMAR_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAR_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAR_A::ENABLED
    }
}
#[doc = "Field `DMAR` writer - DMA enable receiver"]
pub type DMAR_W<'a, const O: u8> = crate::BitWriter<'a, CR3_SPEC, O, DMAR_A>;
impl<'a, const O: u8> DMAR_W<'a, O> {
    #[doc = "DMA mode is disabled for reception"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAR_A::DISABLED)
    }
    #[doc = "DMA mode is enabled for reception"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAR_A::ENABLED)
    }
}
#[doc = "Field `DMAT` reader - DMA enable transmitter"]
pub type DMAT_R = crate::BitReader<DMAT_A>;
#[doc = "DMA enable transmitter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMAT_A {
    #[doc = "0: DMA mode is disabled for transmission"]
    DISABLED = 0,
    #[doc = "1: DMA mode is enabled for transmission"]
    ENABLED = 1,
}
impl From<DMAT_A> for bool {
    #[inline(always)]
    fn from(variant: DMAT_A) -> Self {
        variant as u8 != 0
    }
}
impl DMAT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMAT_A {
        match self.bits {
            false => DMAT_A::DISABLED,
            true => DMAT_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DMAT_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DMAT_A::ENABLED
    }
}
#[doc = "Field `DMAT` writer - DMA enable transmitter"]
pub type DMAT_W<'a, const O: u8> = crate::BitWriter<'a, CR3_SPEC, O, DMAT_A>;
impl<'a, const O: u8> DMAT_W<'a, O> {
    #[doc = "DMA mode is disabled for transmission"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DMAT_A::DISABLED)
    }
    #[doc = "DMA mode is enabled for transmission"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DMAT_A::ENABLED)
    }
}
#[doc = "Field `RTSE` reader - RTS enable"]
pub type RTSE_R = crate::BitReader<RTSE_A>;
#[doc = "RTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RTSE_A {
    #[doc = "0: RTS hardware flow control disabled"]
    DISABLED = 0,
    #[doc = "1: RTS output enabled, data is only requested when there is space in the receive buffer"]
    ENABLED = 1,
}
impl From<RTSE_A> for bool {
    #[inline(always)]
    fn from(variant: RTSE_A) -> Self {
        variant as u8 != 0
    }
}
impl RTSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTSE_A {
        match self.bits {
            false => RTSE_A::DISABLED,
            true => RTSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == RTSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == RTSE_A::ENABLED
    }
}
#[doc = "Field `RTSE` writer - RTS enable"]
pub type RTSE_W<'a, const O: u8> = crate::BitWriter<'a, CR3_SPEC, O, RTSE_A>;
impl<'a, const O: u8> RTSE_W<'a, O> {
    #[doc = "RTS hardware flow control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RTSE_A::DISABLED)
    }
    #[doc = "RTS output enabled, data is only requested when there is space in the receive buffer"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RTSE_A::ENABLED)
    }
}
#[doc = "Field `CTSE` reader - CTS enable"]
pub type CTSE_R = crate::BitReader<CTSE_A>;
#[doc = "CTS enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSE_A {
    #[doc = "0: CTS hardware flow control disabled"]
    DISABLED = 0,
    #[doc = "1: CTS mode enabled, data is only transmitted when the CTS input is asserted"]
    ENABLED = 1,
}
impl From<CTSE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSE_A {
        match self.bits {
            false => CTSE_A::DISABLED,
            true => CTSE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSE_A::ENABLED
    }
}
#[doc = "Field `CTSE` writer - CTS enable"]
pub type CTSE_W<'a, const O: u8> = crate::BitWriter<'a, CR3_SPEC, O, CTSE_A>;
impl<'a, const O: u8> CTSE_W<'a, O> {
    #[doc = "CTS hardware flow control disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSE_A::DISABLED)
    }
    #[doc = "CTS mode enabled, data is only transmitted when the CTS input is asserted"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSE_A::ENABLED)
    }
}
#[doc = "Field `CTSIE` reader - CTS interrupt enable"]
pub type CTSIE_R = crate::BitReader<CTSIE_A>;
#[doc = "CTS interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CTSIE_A {
    #[doc = "0: Interrupt is inhibited"]
    DISABLED = 0,
    #[doc = "1: An interrupt is generated whenever CTSIF=1 in the ISR register"]
    ENABLED = 1,
}
impl From<CTSIE_A> for bool {
    #[inline(always)]
    fn from(variant: CTSIE_A) -> Self {
        variant as u8 != 0
    }
}
impl CTSIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CTSIE_A {
        match self.bits {
            false => CTSIE_A::DISABLED,
            true => CTSIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CTSIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CTSIE_A::ENABLED
    }
}
#[doc = "Field `CTSIE` writer - CTS interrupt enable"]
pub type CTSIE_W<'a, const O: u8> = crate::BitWriter<'a, CR3_SPEC, O, CTSIE_A>;
impl<'a, const O: u8> CTSIE_W<'a, O> {
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CTSIE_A::DISABLED)
    }
    #[doc = "An interrupt is generated whenever CTSIF=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CTSIE_A::ENABLED)
    }
}
#[doc = "Field `ONEBIT` reader - One sample bit method enable"]
pub type ONEBIT_R = crate::BitReader<ONEBIT_A>;
#[doc = "One sample bit method enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ONEBIT_A {
    #[doc = "0: Three sample bit method"]
    SAMPLE3 = 0,
    #[doc = "1: One sample bit method"]
    SAMPLE1 = 1,
}
impl From<ONEBIT_A> for bool {
    #[inline(always)]
    fn from(variant: ONEBIT_A) -> Self {
        variant as u8 != 0
    }
}
impl ONEBIT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ONEBIT_A {
        match self.bits {
            false => ONEBIT_A::SAMPLE3,
            true => ONEBIT_A::SAMPLE1,
        }
    }
    #[doc = "Checks if the value of the field is `SAMPLE3`"]
    #[inline(always)]
    pub fn is_sample3(&self) -> bool {
        *self == ONEBIT_A::SAMPLE3
    }
    #[doc = "Checks if the value of the field is `SAMPLE1`"]
    #[inline(always)]
    pub fn is_sample1(&self) -> bool {
        *self == ONEBIT_A::SAMPLE1
    }
}
#[doc = "Field `ONEBIT` writer - One sample bit method enable"]
pub type ONEBIT_W<'a, const O: u8> = crate::BitWriter<'a, CR3_SPEC, O, ONEBIT_A>;
impl<'a, const O: u8> ONEBIT_W<'a, O> {
    #[doc = "Three sample bit method"]
    #[inline(always)]
    pub fn sample3(self) -> &'a mut W {
        self.variant(ONEBIT_A::SAMPLE3)
    }
    #[doc = "One sample bit method"]
    #[inline(always)]
    pub fn sample1(self) -> &'a mut W {
        self.variant(ONEBIT_A::SAMPLE1)
    }
}
#[doc = "Field `OVRDIS` reader - Overrun Disable"]
pub type OVRDIS_R = crate::BitReader<OVRDIS_A>;
#[doc = "Overrun Disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRDIS_A {
    #[doc = "0: Overrun Error Flag, ORE, is set when received data is not read before receiving new data"]
    ENABLED = 0,
    #[doc = "1: Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register"]
    DISABLED = 1,
}
impl From<OVRDIS_A> for bool {
    #[inline(always)]
    fn from(variant: OVRDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl OVRDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRDIS_A {
        match self.bits {
            false => OVRDIS_A::ENABLED,
            true => OVRDIS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OVRDIS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OVRDIS_A::DISABLED
    }
}
#[doc = "Field `OVRDIS` writer - Overrun Disable"]
pub type OVRDIS_W<'a, const O: u8> = crate::BitWriter<'a, CR3_SPEC, O, OVRDIS_A>;
impl<'a, const O: u8> OVRDIS_W<'a, O> {
    #[doc = "Overrun Error Flag, ORE, is set when received data is not read before receiving new data"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OVRDIS_A::ENABLED)
    }
    #[doc = "Overrun functionality is disabled. If new data is received while the RXNE flag is still set the ORE flag is not set and the new received data overwrites the previous content of the RDR register"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OVRDIS_A::DISABLED)
    }
}
#[doc = "Field `DDRE` reader - DMA Disable on Reception Error"]
pub type DDRE_R = crate::BitReader<DDRE_A>;
#[doc = "DMA Disable on Reception Error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DDRE_A {
    #[doc = "0: DMA is not disabled in case of reception error"]
    NOT_DISABLED = 0,
    #[doc = "1: DMA is disabled following a reception error"]
    DISABLED = 1,
}
impl From<DDRE_A> for bool {
    #[inline(always)]
    fn from(variant: DDRE_A) -> Self {
        variant as u8 != 0
    }
}
impl DDRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DDRE_A {
        match self.bits {
            false => DDRE_A::NOT_DISABLED,
            true => DDRE_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_DISABLED`"]
    #[inline(always)]
    pub fn is_not_disabled(&self) -> bool {
        *self == DDRE_A::NOT_DISABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DDRE_A::DISABLED
    }
}
#[doc = "Field `DDRE` writer - DMA Disable on Reception Error"]
pub type DDRE_W<'a, const O: u8> = crate::BitWriter<'a, CR3_SPEC, O, DDRE_A>;
impl<'a, const O: u8> DDRE_W<'a, O> {
    #[doc = "DMA is not disabled in case of reception error"]
    #[inline(always)]
    pub fn not_disabled(self) -> &'a mut W {
        self.variant(DDRE_A::NOT_DISABLED)
    }
    #[doc = "DMA is disabled following a reception error"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DDRE_A::DISABLED)
    }
}
#[doc = "Field `DEM` reader - Driver enable mode"]
pub type DEM_R = crate::BitReader<DEM_A>;
#[doc = "Driver enable mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEM_A {
    #[doc = "0: DE function is disabled"]
    DISABLED = 0,
    #[doc = "1: The DE signal is output on the RTS pin"]
    ENABLED = 1,
}
impl From<DEM_A> for bool {
    #[inline(always)]
    fn from(variant: DEM_A) -> Self {
        variant as u8 != 0
    }
}
impl DEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEM_A {
        match self.bits {
            false => DEM_A::DISABLED,
            true => DEM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == DEM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == DEM_A::ENABLED
    }
}
#[doc = "Field `DEM` writer - Driver enable mode"]
pub type DEM_W<'a, const O: u8> = crate::BitWriter<'a, CR3_SPEC, O, DEM_A>;
impl<'a, const O: u8> DEM_W<'a, O> {
    #[doc = "DE function is disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(DEM_A::DISABLED)
    }
    #[doc = "The DE signal is output on the RTS pin"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(DEM_A::ENABLED)
    }
}
#[doc = "Field `DEP` reader - Driver enable polarity selection"]
pub type DEP_R = crate::BitReader<DEP_A>;
#[doc = "Driver enable polarity selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DEP_A {
    #[doc = "0: DE signal is active high"]
    HIGH = 0,
    #[doc = "1: DE signal is active low"]
    LOW = 1,
}
impl From<DEP_A> for bool {
    #[inline(always)]
    fn from(variant: DEP_A) -> Self {
        variant as u8 != 0
    }
}
impl DEP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DEP_A {
        match self.bits {
            false => DEP_A::HIGH,
            true => DEP_A::LOW,
        }
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == DEP_A::HIGH
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == DEP_A::LOW
    }
}
#[doc = "Field `DEP` writer - Driver enable polarity selection"]
pub type DEP_W<'a, const O: u8> = crate::BitWriter<'a, CR3_SPEC, O, DEP_A>;
impl<'a, const O: u8> DEP_W<'a, O> {
    #[doc = "DE signal is active high"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(DEP_A::HIGH)
    }
    #[doc = "DE signal is active low"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(DEP_A::LOW)
    }
}
#[doc = "Field `SCARCNT` reader - Smartcard auto-retry count"]
pub type SCARCNT_R = crate::FieldReader;
#[doc = "Field `SCARCNT` writer - Smartcard auto-retry count"]
pub type SCARCNT_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CR3_SPEC, 3, O>;
#[doc = "Field `WUS` reader - Wakeup from Stop mode interrupt flag selection"]
pub type WUS_R = crate::FieldReader<WUS_A>;
#[doc = "Wakeup from Stop mode interrupt flag selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WUS_A {
    #[doc = "0: WUF active on address match"]
    ADDRESS = 0,
    #[doc = "2: WuF active on Start bit detection"]
    START = 2,
    #[doc = "3: WUF active on RXNE"]
    RXNE = 3,
}
impl From<WUS_A> for u8 {
    #[inline(always)]
    fn from(variant: WUS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for WUS_A {
    type Ux = u8;
}
impl WUS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WUS_A> {
        match self.bits {
            0 => Some(WUS_A::ADDRESS),
            2 => Some(WUS_A::START),
            3 => Some(WUS_A::RXNE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ADDRESS`"]
    #[inline(always)]
    pub fn is_address(&self) -> bool {
        *self == WUS_A::ADDRESS
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == WUS_A::START
    }
    #[doc = "Checks if the value of the field is `RXNE`"]
    #[inline(always)]
    pub fn is_rxne(&self) -> bool {
        *self == WUS_A::RXNE
    }
}
#[doc = "Field `WUS` writer - Wakeup from Stop mode interrupt flag selection"]
pub type WUS_W<'a, const O: u8> = crate::FieldWriter<'a, CR3_SPEC, 2, O, WUS_A>;
impl<'a, const O: u8> WUS_W<'a, O> {
    #[doc = "WUF active on address match"]
    #[inline(always)]
    pub fn address(self) -> &'a mut W {
        self.variant(WUS_A::ADDRESS)
    }
    #[doc = "WuF active on Start bit detection"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(WUS_A::START)
    }
    #[doc = "WUF active on RXNE"]
    #[inline(always)]
    pub fn rxne(self) -> &'a mut W {
        self.variant(WUS_A::RXNE)
    }
}
#[doc = "Field `WUFIE` reader - Wakeup from Stop mode interrupt enable"]
pub type WUFIE_R = crate::BitReader<WUFIE_A>;
#[doc = "Wakeup from Stop mode interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WUFIE_A {
    #[doc = "0: Interrupt is inhibited"]
    DISABLED = 0,
    #[doc = "1: An USART interrupt is generated whenever WUF=1 in the ISR register"]
    ENABLED = 1,
}
impl From<WUFIE_A> for bool {
    #[inline(always)]
    fn from(variant: WUFIE_A) -> Self {
        variant as u8 != 0
    }
}
impl WUFIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WUFIE_A {
        match self.bits {
            false => WUFIE_A::DISABLED,
            true => WUFIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == WUFIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == WUFIE_A::ENABLED
    }
}
#[doc = "Field `WUFIE` writer - Wakeup from Stop mode interrupt enable"]
pub type WUFIE_W<'a, const O: u8> = crate::BitWriter<'a, CR3_SPEC, O, WUFIE_A>;
impl<'a, const O: u8> WUFIE_W<'a, O> {
    #[doc = "Interrupt is inhibited"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(WUFIE_A::DISABLED)
    }
    #[doc = "An USART interrupt is generated whenever WUF=1 in the ISR register"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(WUFIE_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    pub fn eie(&self) -> EIE_R {
        EIE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    pub fn iren(&self) -> IREN_R {
        IREN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    pub fn irlp(&self) -> IRLP_R {
        IRLP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    pub fn hdsel(&self) -> HDSEL_R {
        HDSEL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    pub fn scen(&self) -> SCEN_R {
        SCEN_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    pub fn dmar(&self) -> DMAR_R {
        DMAR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    pub fn dmat(&self) -> DMAT_R {
        DMAT_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    pub fn rtse(&self) -> RTSE_R {
        RTSE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    pub fn ctse(&self) -> CTSE_R {
        CTSE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    pub fn ctsie(&self) -> CTSIE_R {
        CTSIE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - One sample bit method enable"]
    #[inline(always)]
    pub fn onebit(&self) -> ONEBIT_R {
        ONEBIT_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Overrun Disable"]
    #[inline(always)]
    pub fn ovrdis(&self) -> OVRDIS_R {
        OVRDIS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error"]
    #[inline(always)]
    pub fn ddre(&self) -> DDRE_R {
        DDRE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    pub fn dem(&self) -> DEM_R {
        DEM_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Driver enable polarity selection"]
    #[inline(always)]
    pub fn dep(&self) -> DEP_R {
        DEP_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry count"]
    #[inline(always)]
    pub fn scarcnt(&self) -> SCARCNT_R {
        SCARCNT_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bits 20:21 - Wakeup from Stop mode interrupt flag selection"]
    #[inline(always)]
    pub fn wus(&self) -> WUS_R {
        WUS_R::new(((self.bits >> 20) & 3) as u8)
    }
    #[doc = "Bit 22 - Wakeup from Stop mode interrupt enable"]
    #[inline(always)]
    pub fn wufie(&self) -> WUFIE_R {
        WUFIE_R::new(((self.bits >> 22) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn eie(&mut self) -> EIE_W<0> {
        EIE_W::new(self)
    }
    #[doc = "Bit 1 - IrDA mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn iren(&mut self) -> IREN_W<1> {
        IREN_W::new(self)
    }
    #[doc = "Bit 2 - IrDA low-power"]
    #[inline(always)]
    #[must_use]
    pub fn irlp(&mut self) -> IRLP_W<2> {
        IRLP_W::new(self)
    }
    #[doc = "Bit 3 - Half-duplex selection"]
    #[inline(always)]
    #[must_use]
    pub fn hdsel(&mut self) -> HDSEL_W<3> {
        HDSEL_W::new(self)
    }
    #[doc = "Bit 4 - Smartcard NACK enable"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<4> {
        NACK_W::new(self)
    }
    #[doc = "Bit 5 - Smartcard mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn scen(&mut self) -> SCEN_W<5> {
        SCEN_W::new(self)
    }
    #[doc = "Bit 6 - DMA enable receiver"]
    #[inline(always)]
    #[must_use]
    pub fn dmar(&mut self) -> DMAR_W<6> {
        DMAR_W::new(self)
    }
    #[doc = "Bit 7 - DMA enable transmitter"]
    #[inline(always)]
    #[must_use]
    pub fn dmat(&mut self) -> DMAT_W<7> {
        DMAT_W::new(self)
    }
    #[doc = "Bit 8 - RTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn rtse(&mut self) -> RTSE_W<8> {
        RTSE_W::new(self)
    }
    #[doc = "Bit 9 - CTS enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctse(&mut self) -> CTSE_W<9> {
        CTSE_W::new(self)
    }
    #[doc = "Bit 10 - CTS interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn ctsie(&mut self) -> CTSIE_W<10> {
        CTSIE_W::new(self)
    }
    #[doc = "Bit 11 - One sample bit method enable"]
    #[inline(always)]
    #[must_use]
    pub fn onebit(&mut self) -> ONEBIT_W<11> {
        ONEBIT_W::new(self)
    }
    #[doc = "Bit 12 - Overrun Disable"]
    #[inline(always)]
    #[must_use]
    pub fn ovrdis(&mut self) -> OVRDIS_W<12> {
        OVRDIS_W::new(self)
    }
    #[doc = "Bit 13 - DMA Disable on Reception Error"]
    #[inline(always)]
    #[must_use]
    pub fn ddre(&mut self) -> DDRE_W<13> {
        DDRE_W::new(self)
    }
    #[doc = "Bit 14 - Driver enable mode"]
    #[inline(always)]
    #[must_use]
    pub fn dem(&mut self) -> DEM_W<14> {
        DEM_W::new(self)
    }
    #[doc = "Bit 15 - Driver enable polarity selection"]
    #[inline(always)]
    #[must_use]
    pub fn dep(&mut self) -> DEP_W<15> {
        DEP_W::new(self)
    }
    #[doc = "Bits 17:19 - Smartcard auto-retry count"]
    #[inline(always)]
    #[must_use]
    pub fn scarcnt(&mut self) -> SCARCNT_W<17> {
        SCARCNT_W::new(self)
    }
    #[doc = "Bits 20:21 - Wakeup from Stop mode interrupt flag selection"]
    #[inline(always)]
    #[must_use]
    pub fn wus(&mut self) -> WUS_W<20> {
        WUS_W::new(self)
    }
    #[doc = "Bit 22 - Wakeup from Stop mode interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn wufie(&mut self) -> WUFIE_W<22> {
        WUFIE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr3](index.html) module"]
pub struct CR3_SPEC;
impl crate::RegisterSpec for CR3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr3::R](R) reader structure"]
impl crate::Readable for CR3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr3::W](W) writer structure"]
impl crate::Writable for CR3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR3 to value 0"]
impl crate::Resettable for CR3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
