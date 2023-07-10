#[doc = "Register `ISR` reader"]
pub struct R(crate::R<ISR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ISR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ISR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ISR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ISR` writer"]
pub struct W(crate::W<ISR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ISR_SPEC>;
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
impl From<crate::W<ISR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ISR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TXE` reader - Transmit data register empty (transmitters)"]
pub type TXE_R = crate::BitReader<TXE_A>;
#[doc = "Transmit data register empty (transmitters)\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXE_A {
    #[doc = "0: TXDR register not empty"]
    NOT_EMPTY = 0,
    #[doc = "1: TXDR register empty"]
    EMPTY = 1,
}
impl From<TXE_A> for bool {
    #[inline(always)]
    fn from(variant: TXE_A) -> Self {
        variant as u8 != 0
    }
}
impl TXE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXE_A {
        match self.bits {
            false => TXE_A::NOT_EMPTY,
            true => TXE_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXE_A::NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXE_A::EMPTY
    }
}
#[doc = "Field `TXE` writer - Transmit data register empty (transmitters)"]
pub type TXE_W<'a, const O: u8> = crate::BitWriter<'a, ISR_SPEC, O, TXE_A>;
impl<'a, const O: u8> TXE_W<'a, O> {
    #[doc = "TXDR register not empty"]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut W {
        self.variant(TXE_A::NOT_EMPTY)
    }
    #[doc = "TXDR register empty"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TXE_A::EMPTY)
    }
}
#[doc = "Field `TXIS` reader - Transmit interrupt status (transmitters)"]
pub type TXIS_R = crate::BitReader<TXIS_A>;
#[doc = "Transmit interrupt status (transmitters)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXIS_A {
    #[doc = "0: The TXDR register is not empty"]
    NOT_EMPTY = 0,
    #[doc = "1: The TXDR register is empty and the data to be transmitted must be written in the TXDR register"]
    EMPTY = 1,
}
impl From<TXIS_A> for bool {
    #[inline(always)]
    fn from(variant: TXIS_A) -> Self {
        variant as u8 != 0
    }
}
impl TXIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TXIS_A {
        match self.bits {
            false => TXIS_A::NOT_EMPTY,
            true => TXIS_A::EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == TXIS_A::NOT_EMPTY
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == TXIS_A::EMPTY
    }
}
#[doc = "Field `TXIS` writer - Transmit interrupt status (transmitters)"]
pub type TXIS_W<'a, const O: u8> = crate::BitWriter<'a, ISR_SPEC, O, TXIS_A>;
impl<'a, const O: u8> TXIS_W<'a, O> {
    #[doc = "The TXDR register is not empty"]
    #[inline(always)]
    pub fn not_empty(self) -> &'a mut W {
        self.variant(TXIS_A::NOT_EMPTY)
    }
    #[doc = "The TXDR register is empty and the data to be transmitted must be written in the TXDR register"]
    #[inline(always)]
    pub fn empty(self) -> &'a mut W {
        self.variant(TXIS_A::EMPTY)
    }
}
#[doc = "Field `RXNE` reader - Receive data register not empty (receivers)"]
pub type RXNE_R = crate::BitReader<RXNE_A>;
#[doc = "Receive data register not empty (receivers)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNE_A {
    #[doc = "0: The RXDR register is empty"]
    EMPTY = 0,
    #[doc = "1: Received data is copied into the RXDR register, and is ready to be read"]
    NOT_EMPTY = 1,
}
impl From<RXNE_A> for bool {
    #[inline(always)]
    fn from(variant: RXNE_A) -> Self {
        variant as u8 != 0
    }
}
impl RXNE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXNE_A {
        match self.bits {
            false => RXNE_A::EMPTY,
            true => RXNE_A::NOT_EMPTY,
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == RXNE_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `NOT_EMPTY`"]
    #[inline(always)]
    pub fn is_not_empty(&self) -> bool {
        *self == RXNE_A::NOT_EMPTY
    }
}
#[doc = "Field `ADDR` reader - Address matched (slave mode)"]
pub type ADDR_R = crate::BitReader<ADDR_A>;
#[doc = "Address matched (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDR_A {
    #[doc = "0: Adress mismatched or not received"]
    NOT_MATCH = 0,
    #[doc = "1: Received slave address matched with one of the enabled slave addresses"]
    MATCH = 1,
}
impl From<ADDR_A> for bool {
    #[inline(always)]
    fn from(variant: ADDR_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADDR_A {
        match self.bits {
            false => ADDR_A::NOT_MATCH,
            true => ADDR_A::MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_MATCH`"]
    #[inline(always)]
    pub fn is_not_match(&self) -> bool {
        *self == ADDR_A::NOT_MATCH
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == ADDR_A::MATCH
    }
}
#[doc = "Field `NACKF` reader - Not acknowledge received flag"]
pub type NACKF_R = crate::BitReader<NACKF_A>;
#[doc = "Not acknowledge received flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACKF_A {
    #[doc = "0: No NACK has been received"]
    NO_NACK = 0,
    #[doc = "1: NACK has been received"]
    NACK = 1,
}
impl From<NACKF_A> for bool {
    #[inline(always)]
    fn from(variant: NACKF_A) -> Self {
        variant as u8 != 0
    }
}
impl NACKF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACKF_A {
        match self.bits {
            false => NACKF_A::NO_NACK,
            true => NACKF_A::NACK,
        }
    }
    #[doc = "Checks if the value of the field is `NO_NACK`"]
    #[inline(always)]
    pub fn is_no_nack(&self) -> bool {
        *self == NACKF_A::NO_NACK
    }
    #[doc = "Checks if the value of the field is `NACK`"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == NACKF_A::NACK
    }
}
#[doc = "Field `STOPF` reader - Stop detection flag"]
pub type STOPF_R = crate::BitReader<STOPF_A>;
#[doc = "Stop detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOPF_A {
    #[doc = "0: No Stop condition detected"]
    NO_STOP = 0,
    #[doc = "1: Stop condition detected"]
    STOP = 1,
}
impl From<STOPF_A> for bool {
    #[inline(always)]
    fn from(variant: STOPF_A) -> Self {
        variant as u8 != 0
    }
}
impl STOPF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOPF_A {
        match self.bits {
            false => STOPF_A::NO_STOP,
            true => STOPF_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NO_STOP`"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOPF_A::NO_STOP
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOPF_A::STOP
    }
}
#[doc = "Field `TC` reader - Transfer Complete (master mode)"]
pub type TC_R = crate::BitReader<TC_A>;
#[doc = "Transfer Complete (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TC_A {
    #[doc = "0: Transfer is not complete"]
    NOT_COMPLETE = 0,
    #[doc = "1: NBYTES has been transfered"]
    COMPLETE = 1,
}
impl From<TC_A> for bool {
    #[inline(always)]
    fn from(variant: TC_A) -> Self {
        variant as u8 != 0
    }
}
impl TC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TC_A {
        match self.bits {
            false => TC_A::NOT_COMPLETE,
            true => TC_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_COMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TC_A::NOT_COMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TC_A::COMPLETE
    }
}
#[doc = "Field `TCR` reader - Transfer Complete Reload"]
pub type TCR_R = crate::BitReader<TCR_A>;
#[doc = "Transfer Complete Reload\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TCR_A {
    #[doc = "0: Transfer is not complete"]
    NOT_COMPLETE = 0,
    #[doc = "1: NBYTES has been transfered"]
    COMPLETE = 1,
}
impl From<TCR_A> for bool {
    #[inline(always)]
    fn from(variant: TCR_A) -> Self {
        variant as u8 != 0
    }
}
impl TCR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TCR_A {
        match self.bits {
            false => TCR_A::NOT_COMPLETE,
            true => TCR_A::COMPLETE,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_COMPLETE`"]
    #[inline(always)]
    pub fn is_not_complete(&self) -> bool {
        *self == TCR_A::NOT_COMPLETE
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == TCR_A::COMPLETE
    }
}
#[doc = "Field `BERR` reader - Bus error"]
pub type BERR_R = crate::BitReader<BERR_A>;
#[doc = "Bus error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BERR_A {
    #[doc = "0: No bus error"]
    NO_ERROR = 0,
    #[doc = "1: Misplaced Start and Stop condition is detected"]
    ERROR = 1,
}
impl From<BERR_A> for bool {
    #[inline(always)]
    fn from(variant: BERR_A) -> Self {
        variant as u8 != 0
    }
}
impl BERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BERR_A {
        match self.bits {
            false => BERR_A::NO_ERROR,
            true => BERR_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == BERR_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == BERR_A::ERROR
    }
}
#[doc = "Field `ARLO` reader - Arbitration lost"]
pub type ARLO_R = crate::BitReader<ARLO_A>;
#[doc = "Arbitration lost\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARLO_A {
    #[doc = "0: No arbitration lost"]
    NOT_LOST = 0,
    #[doc = "1: Arbitration lost"]
    LOST = 1,
}
impl From<ARLO_A> for bool {
    #[inline(always)]
    fn from(variant: ARLO_A) -> Self {
        variant as u8 != 0
    }
}
impl ARLO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARLO_A {
        match self.bits {
            false => ARLO_A::NOT_LOST,
            true => ARLO_A::LOST,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_LOST`"]
    #[inline(always)]
    pub fn is_not_lost(&self) -> bool {
        *self == ARLO_A::NOT_LOST
    }
    #[doc = "Checks if the value of the field is `LOST`"]
    #[inline(always)]
    pub fn is_lost(&self) -> bool {
        *self == ARLO_A::LOST
    }
}
#[doc = "Field `OVR` reader - Overrun/Underrun (slave mode)"]
pub type OVR_R = crate::BitReader<OVR_A>;
#[doc = "Overrun/Underrun (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVR_A {
    #[doc = "0: No overrun/underrun error occurs"]
    NO_OVERRUN = 0,
    #[doc = "1: slave mode with NOSTRETCH=1, when an overrun/underrun error occurs"]
    OVERRUN = 1,
}
impl From<OVR_A> for bool {
    #[inline(always)]
    fn from(variant: OVR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVR_A {
        match self.bits {
            false => OVR_A::NO_OVERRUN,
            true => OVR_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVR_A::NO_OVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVR_A::OVERRUN
    }
}
#[doc = "Field `PECERR` reader - PEC Error in reception"]
pub type PECERR_R = crate::BitReader<PECERR_A>;
#[doc = "PEC Error in reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECERR_A {
    #[doc = "0: Received PEC does match with PEC register"]
    MATCH = 0,
    #[doc = "1: Received PEC does not match with PEC register"]
    NO_MATCH = 1,
}
impl From<PECERR_A> for bool {
    #[inline(always)]
    fn from(variant: PECERR_A) -> Self {
        variant as u8 != 0
    }
}
impl PECERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECERR_A {
        match self.bits {
            false => PECERR_A::MATCH,
            true => PECERR_A::NO_MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == PECERR_A::MATCH
    }
    #[doc = "Checks if the value of the field is `NO_MATCH`"]
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == PECERR_A::NO_MATCH
    }
}
#[doc = "Field `TIMEOUT` reader - Timeout or t_low detection flag"]
pub type TIMEOUT_R = crate::BitReader<TIMEOUT_A>;
#[doc = "Timeout or t_low detection flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TIMEOUT_A {
    #[doc = "0: No timeout occured"]
    NO_TIMEOUT = 0,
    #[doc = "1: Timeout occured"]
    TIMEOUT = 1,
}
impl From<TIMEOUT_A> for bool {
    #[inline(always)]
    fn from(variant: TIMEOUT_A) -> Self {
        variant as u8 != 0
    }
}
impl TIMEOUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TIMEOUT_A {
        match self.bits {
            false => TIMEOUT_A::NO_TIMEOUT,
            true => TIMEOUT_A::TIMEOUT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_TIMEOUT`"]
    #[inline(always)]
    pub fn is_no_timeout(&self) -> bool {
        *self == TIMEOUT_A::NO_TIMEOUT
    }
    #[doc = "Checks if the value of the field is `TIMEOUT`"]
    #[inline(always)]
    pub fn is_timeout(&self) -> bool {
        *self == TIMEOUT_A::TIMEOUT
    }
}
#[doc = "Field `ALERT` reader - SMBus alert"]
pub type ALERT_R = crate::BitReader<ALERT_A>;
#[doc = "SMBus alert\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ALERT_A {
    #[doc = "0: SMBA alert is not detected"]
    NO_ALERT = 0,
    #[doc = "1: SMBA alert event is detected on SMBA pin"]
    ALERT = 1,
}
impl From<ALERT_A> for bool {
    #[inline(always)]
    fn from(variant: ALERT_A) -> Self {
        variant as u8 != 0
    }
}
impl ALERT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ALERT_A {
        match self.bits {
            false => ALERT_A::NO_ALERT,
            true => ALERT_A::ALERT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ALERT`"]
    #[inline(always)]
    pub fn is_no_alert(&self) -> bool {
        *self == ALERT_A::NO_ALERT
    }
    #[doc = "Checks if the value of the field is `ALERT`"]
    #[inline(always)]
    pub fn is_alert(&self) -> bool {
        *self == ALERT_A::ALERT
    }
}
#[doc = "Field `BUSY` reader - Bus busy"]
pub type BUSY_R = crate::BitReader<BUSY_A>;
#[doc = "Bus busy\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BUSY_A {
    #[doc = "0: No communication is in progress on the bus"]
    NOT_BUSY = 0,
    #[doc = "1: A communication is in progress on the bus"]
    BUSY = 1,
}
impl From<BUSY_A> for bool {
    #[inline(always)]
    fn from(variant: BUSY_A) -> Self {
        variant as u8 != 0
    }
}
impl BUSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BUSY_A {
        match self.bits {
            false => BUSY_A::NOT_BUSY,
            true => BUSY_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BUSY_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BUSY_A::BUSY
    }
}
#[doc = "Field `DIR` reader - Transfer direction (Slave mode)"]
pub type DIR_R = crate::BitReader<DIR_A>;
#[doc = "Transfer direction (Slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: Write transfer, slave enters receiver mode"]
    WRITE = 0,
    #[doc = "1: Read transfer, slave enters transmitter mode"]
    READ = 1,
}
impl From<DIR_A> for bool {
    #[inline(always)]
    fn from(variant: DIR_A) -> Self {
        variant as u8 != 0
    }
}
impl DIR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIR_A {
        match self.bits {
            false => DIR_A::WRITE,
            true => DIR_A::READ,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == DIR_A::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == DIR_A::READ
    }
}
#[doc = "Field `ADDCODE` reader - Address match code (Slave mode)"]
pub type ADDCODE_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - Transmit data register empty (transmitters)"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit interrupt status (transmitters)"]
    #[inline(always)]
    pub fn txis(&self) -> TXIS_R {
        TXIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Receive data register not empty (receivers)"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Address matched (slave mode)"]
    #[inline(always)]
    pub fn addr(&self) -> ADDR_R {
        ADDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Not acknowledge received flag"]
    #[inline(always)]
    pub fn nackf(&self) -> NACKF_R {
        NACKF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Stop detection flag"]
    #[inline(always)]
    pub fn stopf(&self) -> STOPF_R {
        STOPF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Transfer Complete (master mode)"]
    #[inline(always)]
    pub fn tc(&self) -> TC_R {
        TC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Transfer Complete Reload"]
    #[inline(always)]
    pub fn tcr(&self) -> TCR_R {
        TCR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Bus error"]
    #[inline(always)]
    pub fn berr(&self) -> BERR_R {
        BERR_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Arbitration lost"]
    #[inline(always)]
    pub fn arlo(&self) -> ARLO_R {
        ARLO_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Overrun/Underrun (slave mode)"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - PEC Error in reception"]
    #[inline(always)]
    pub fn pecerr(&self) -> PECERR_R {
        PECERR_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Timeout or t_low detection flag"]
    #[inline(always)]
    pub fn timeout(&self) -> TIMEOUT_R {
        TIMEOUT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - SMBus alert"]
    #[inline(always)]
    pub fn alert(&self) -> ALERT_R {
        ALERT_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 15 - Bus busy"]
    #[inline(always)]
    pub fn busy(&self) -> BUSY_R {
        BUSY_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Transfer direction (Slave mode)"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:23 - Address match code (Slave mode)"]
    #[inline(always)]
    pub fn addcode(&self) -> ADDCODE_R {
        ADDCODE_R::new(((self.bits >> 17) & 0x7f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Transmit data register empty (transmitters)"]
    #[inline(always)]
    #[must_use]
    pub fn txe(&mut self) -> TXE_W<0> {
        TXE_W::new(self)
    }
    #[doc = "Bit 1 - Transmit interrupt status (transmitters)"]
    #[inline(always)]
    #[must_use]
    pub fn txis(&mut self) -> TXIS_W<1> {
        TXIS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Interrupt and Status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [isr](index.html) module"]
pub struct ISR_SPEC;
impl crate::RegisterSpec for ISR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [isr::R](R) reader structure"]
impl crate::Readable for ISR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [isr::W](W) writer structure"]
impl crate::Writable for ISR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ISR to value 0x01"]
impl crate::Resettable for ISR_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
