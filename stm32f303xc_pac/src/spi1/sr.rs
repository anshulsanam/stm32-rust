#[doc = "Register `SR` reader"]
pub struct R(crate::R<SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SR` writer"]
pub struct W(crate::W<SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SR_SPEC>;
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
impl From<crate::W<SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXNE` reader - Receive buffer not empty"]
pub type RXNE_R = crate::BitReader<RXNE_A>;
#[doc = "Receive buffer not empty\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNE_A {
    #[doc = "0: Rx buffer empty"]
    EMPTY = 0,
    #[doc = "1: Rx buffer not empty"]
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
#[doc = "Field `TXE` reader - Transmit buffer empty"]
pub type TXE_R = crate::BitReader<TXE_A>;
#[doc = "Transmit buffer empty\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXE_A {
    #[doc = "0: Tx buffer not empty"]
    NOT_EMPTY = 0,
    #[doc = "1: Tx buffer empty"]
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
#[doc = "Field `CHSIDE` reader - Channel side"]
pub type CHSIDE_R = crate::BitReader<CHSIDE_A>;
#[doc = "Channel side\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CHSIDE_A {
    #[doc = "0: Channel left has to be transmitted or has been received"]
    LEFT = 0,
    #[doc = "1: Channel right has to be transmitted or has been received"]
    RIGHT = 1,
}
impl From<CHSIDE_A> for bool {
    #[inline(always)]
    fn from(variant: CHSIDE_A) -> Self {
        variant as u8 != 0
    }
}
impl CHSIDE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CHSIDE_A {
        match self.bits {
            false => CHSIDE_A::LEFT,
            true => CHSIDE_A::RIGHT,
        }
    }
    #[doc = "Checks if the value of the field is `LEFT`"]
    #[inline(always)]
    pub fn is_left(&self) -> bool {
        *self == CHSIDE_A::LEFT
    }
    #[doc = "Checks if the value of the field is `RIGHT`"]
    #[inline(always)]
    pub fn is_right(&self) -> bool {
        *self == CHSIDE_A::RIGHT
    }
}
#[doc = "Field `UDR` reader - Underrun flag"]
pub type UDR_R = crate::BitReader<UDRR_A>;
#[doc = "Underrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDRR_A {
    #[doc = "0: No underrun occurred"]
    NO_UNDERRUN = 0,
    #[doc = "1: Underrun occurred"]
    UNDERRUN = 1,
}
impl From<UDRR_A> for bool {
    #[inline(always)]
    fn from(variant: UDRR_A) -> Self {
        variant as u8 != 0
    }
}
impl UDR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UDRR_A {
        match self.bits {
            false => UDRR_A::NO_UNDERRUN,
            true => UDRR_A::UNDERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_UNDERRUN`"]
    #[inline(always)]
    pub fn is_no_underrun(&self) -> bool {
        *self == UDRR_A::NO_UNDERRUN
    }
    #[doc = "Checks if the value of the field is `UNDERRUN`"]
    #[inline(always)]
    pub fn is_underrun(&self) -> bool {
        *self == UDRR_A::UNDERRUN
    }
}
#[doc = "Field `CRCERR` reader - CRC error flag"]
pub type CRCERR_R = crate::BitReader<CRCERR_A>;
#[doc = "CRC error flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCERR_A {
    #[doc = "0: CRC value received matches the SPIx_RXCRCR value"]
    MATCH = 0,
    #[doc = "1: CRC value received does not match the SPIx_RXCRCR value"]
    NO_MATCH = 1,
}
impl From<CRCERR_A> for bool {
    #[inline(always)]
    fn from(variant: CRCERR_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCERR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCERR_A {
        match self.bits {
            false => CRCERR_A::MATCH,
            true => CRCERR_A::NO_MATCH,
        }
    }
    #[doc = "Checks if the value of the field is `MATCH`"]
    #[inline(always)]
    pub fn is_match(&self) -> bool {
        *self == CRCERR_A::MATCH
    }
    #[doc = "Checks if the value of the field is `NO_MATCH`"]
    #[inline(always)]
    pub fn is_no_match(&self) -> bool {
        *self == CRCERR_A::NO_MATCH
    }
}
#[doc = "Field `CRCERR` writer - CRC error flag"]
pub type CRCERR_W<'a, const O: u8> = crate::BitWriter<'a, SR_SPEC, O, CRCERR_A>;
impl<'a, const O: u8> CRCERR_W<'a, O> {
    #[doc = "CRC value received matches the SPIx_RXCRCR value"]
    #[inline(always)]
    pub fn match_(self) -> &'a mut W {
        self.variant(CRCERR_A::MATCH)
    }
    #[doc = "CRC value received does not match the SPIx_RXCRCR value"]
    #[inline(always)]
    pub fn no_match(self) -> &'a mut W {
        self.variant(CRCERR_A::NO_MATCH)
    }
}
#[doc = "Field `MODF` reader - Mode fault"]
pub type MODF_R = crate::BitReader<MODFR_A>;
#[doc = "Mode fault\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MODFR_A {
    #[doc = "0: No mode fault occurred"]
    NO_FAULT = 0,
    #[doc = "1: Mode fault occurred"]
    FAULT = 1,
}
impl From<MODFR_A> for bool {
    #[inline(always)]
    fn from(variant: MODFR_A) -> Self {
        variant as u8 != 0
    }
}
impl MODF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MODFR_A {
        match self.bits {
            false => MODFR_A::NO_FAULT,
            true => MODFR_A::FAULT,
        }
    }
    #[doc = "Checks if the value of the field is `NO_FAULT`"]
    #[inline(always)]
    pub fn is_no_fault(&self) -> bool {
        *self == MODFR_A::NO_FAULT
    }
    #[doc = "Checks if the value of the field is `FAULT`"]
    #[inline(always)]
    pub fn is_fault(&self) -> bool {
        *self == MODFR_A::FAULT
    }
}
#[doc = "Field `OVR` reader - Overrun flag"]
pub type OVR_R = crate::BitReader<OVRR_A>;
#[doc = "Overrun flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OVRR_A {
    #[doc = "0: No overrun occurred"]
    NO_OVERRUN = 0,
    #[doc = "1: Overrun occurred"]
    OVERRUN = 1,
}
impl From<OVRR_A> for bool {
    #[inline(always)]
    fn from(variant: OVRR_A) -> Self {
        variant as u8 != 0
    }
}
impl OVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OVRR_A {
        match self.bits {
            false => OVRR_A::NO_OVERRUN,
            true => OVRR_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == OVRR_A::NO_OVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == OVRR_A::OVERRUN
    }
}
#[doc = "Field `BSY` reader - Busy flag"]
pub type BSY_R = crate::BitReader<BSYR_A>;
#[doc = "Busy flag\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BSYR_A {
    #[doc = "0: SPI not busy"]
    NOT_BUSY = 0,
    #[doc = "1: SPI busy"]
    BUSY = 1,
}
impl From<BSYR_A> for bool {
    #[inline(always)]
    fn from(variant: BSYR_A) -> Self {
        variant as u8 != 0
    }
}
impl BSY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BSYR_A {
        match self.bits {
            false => BSYR_A::NOT_BUSY,
            true => BSYR_A::BUSY,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_BUSY`"]
    #[inline(always)]
    pub fn is_not_busy(&self) -> bool {
        *self == BSYR_A::NOT_BUSY
    }
    #[doc = "Checks if the value of the field is `BUSY`"]
    #[inline(always)]
    pub fn is_busy(&self) -> bool {
        *self == BSYR_A::BUSY
    }
}
#[doc = "Field `FRE` reader - TI frame format error"]
pub type FRE_R = crate::BitReader<FRER_A>;
#[doc = "TI frame format error\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRER_A {
    #[doc = "0: No frame format error"]
    NO_ERROR = 0,
    #[doc = "1: A frame format error occurred"]
    ERROR = 1,
}
impl From<FRER_A> for bool {
    #[inline(always)]
    fn from(variant: FRER_A) -> Self {
        variant as u8 != 0
    }
}
impl FRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRER_A {
        match self.bits {
            false => FRER_A::NO_ERROR,
            true => FRER_A::ERROR,
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == FRER_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `ERROR`"]
    #[inline(always)]
    pub fn is_error(&self) -> bool {
        *self == FRER_A::ERROR
    }
}
#[doc = "Field `FRLVL` reader - FIFO reception level"]
pub type FRLVL_R = crate::FieldReader<FRLVLR_A>;
#[doc = "FIFO reception level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FRLVLR_A {
    #[doc = "0: Rx FIFO Empty"]
    EMPTY = 0,
    #[doc = "1: Rx 1/4 FIFO"]
    QUARTER = 1,
    #[doc = "2: Rx 1/2 FIFO"]
    HALF = 2,
    #[doc = "3: Rx FIFO full"]
    FULL = 3,
}
impl From<FRLVLR_A> for u8 {
    #[inline(always)]
    fn from(variant: FRLVLR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FRLVLR_A {
    type Ux = u8;
}
impl FRLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRLVLR_A {
        match self.bits {
            0 => FRLVLR_A::EMPTY,
            1 => FRLVLR_A::QUARTER,
            2 => FRLVLR_A::HALF,
            3 => FRLVLR_A::FULL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FRLVLR_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `QUARTER`"]
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == FRLVLR_A::QUARTER
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == FRLVLR_A::HALF
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FRLVLR_A::FULL
    }
}
#[doc = "Field `FTLVL` reader - FIFO transmission level"]
pub type FTLVL_R = crate::FieldReader<FTLVLR_A>;
#[doc = "FIFO transmission level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FTLVLR_A {
    #[doc = "0: Tx FIFO Empty"]
    EMPTY = 0,
    #[doc = "1: Tx 1/4 FIFO"]
    QUARTER = 1,
    #[doc = "2: Tx 1/2 FIFO"]
    HALF = 2,
    #[doc = "3: Tx FIFO full"]
    FULL = 3,
}
impl From<FTLVLR_A> for u8 {
    #[inline(always)]
    fn from(variant: FTLVLR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for FTLVLR_A {
    type Ux = u8;
}
impl FTLVL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FTLVLR_A {
        match self.bits {
            0 => FTLVLR_A::EMPTY,
            1 => FTLVLR_A::QUARTER,
            2 => FTLVLR_A::HALF,
            3 => FTLVLR_A::FULL,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `EMPTY`"]
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        *self == FTLVLR_A::EMPTY
    }
    #[doc = "Checks if the value of the field is `QUARTER`"]
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == FTLVLR_A::QUARTER
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == FTLVLR_A::HALF
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FTLVLR_A::FULL
    }
}
impl R {
    #[doc = "Bit 0 - Receive buffer not empty"]
    #[inline(always)]
    pub fn rxne(&self) -> RXNE_R {
        RXNE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transmit buffer empty"]
    #[inline(always)]
    pub fn txe(&self) -> TXE_R {
        TXE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Channel side"]
    #[inline(always)]
    pub fn chside(&self) -> CHSIDE_R {
        CHSIDE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Underrun flag"]
    #[inline(always)]
    pub fn udr(&self) -> UDR_R {
        UDR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CRC error flag"]
    #[inline(always)]
    pub fn crcerr(&self) -> CRCERR_R {
        CRCERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Mode fault"]
    #[inline(always)]
    pub fn modf(&self) -> MODF_R {
        MODF_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Overrun flag"]
    #[inline(always)]
    pub fn ovr(&self) -> OVR_R {
        OVR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Busy flag"]
    #[inline(always)]
    pub fn bsy(&self) -> BSY_R {
        BSY_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - TI frame format error"]
    #[inline(always)]
    pub fn fre(&self) -> FRE_R {
        FRE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:10 - FIFO reception level"]
    #[inline(always)]
    pub fn frlvl(&self) -> FRLVL_R {
        FRLVL_R::new(((self.bits >> 9) & 3) as u8)
    }
    #[doc = "Bits 11:12 - FIFO transmission level"]
    #[inline(always)]
    pub fn ftlvl(&self) -> FTLVL_R {
        FTLVL_R::new(((self.bits >> 11) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 4 - CRC error flag"]
    #[inline(always)]
    #[must_use]
    pub fn crcerr(&mut self) -> CRCERR_W<4> {
        CRCERR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sr](index.html) module"]
pub struct SR_SPEC;
impl crate::RegisterSpec for SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sr::R](R) reader structure"]
impl crate::Readable for SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sr::W](W) writer structure"]
impl crate::Writable for SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SR to value 0x02"]
impl crate::Resettable for SR_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
