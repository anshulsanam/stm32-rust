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
#[doc = "Field `CPHA` reader - Clock phase"]
pub type CPHA_R = crate::BitReader<CPHA_A>;
#[doc = "Clock phase\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPHA_A {
    #[doc = "0: The first clock transition is the first data capture edge"]
    FIRST_EDGE = 0,
    #[doc = "1: The second clock transition is the first data capture edge"]
    SECOND_EDGE = 1,
}
impl From<CPHA_A> for bool {
    #[inline(always)]
    fn from(variant: CPHA_A) -> Self {
        variant as u8 != 0
    }
}
impl CPHA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPHA_A {
        match self.bits {
            false => CPHA_A::FIRST_EDGE,
            true => CPHA_A::SECOND_EDGE,
        }
    }
    #[doc = "Checks if the value of the field is `FIRST_EDGE`"]
    #[inline(always)]
    pub fn is_first_edge(&self) -> bool {
        *self == CPHA_A::FIRST_EDGE
    }
    #[doc = "Checks if the value of the field is `SECOND_EDGE`"]
    #[inline(always)]
    pub fn is_second_edge(&self) -> bool {
        *self == CPHA_A::SECOND_EDGE
    }
}
#[doc = "Field `CPHA` writer - Clock phase"]
pub type CPHA_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, CPHA_A>;
impl<'a, const O: u8> CPHA_W<'a, O> {
    #[doc = "The first clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn first_edge(self) -> &'a mut W {
        self.variant(CPHA_A::FIRST_EDGE)
    }
    #[doc = "The second clock transition is the first data capture edge"]
    #[inline(always)]
    pub fn second_edge(self) -> &'a mut W {
        self.variant(CPHA_A::SECOND_EDGE)
    }
}
#[doc = "Field `CPOL` reader - Clock polarity"]
pub type CPOL_R = crate::BitReader<CPOL_A>;
#[doc = "Clock polarity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CPOL_A {
    #[doc = "0: CK to 0 when idle"]
    IDLE_LOW = 0,
    #[doc = "1: CK to 1 when idle"]
    IDLE_HIGH = 1,
}
impl From<CPOL_A> for bool {
    #[inline(always)]
    fn from(variant: CPOL_A) -> Self {
        variant as u8 != 0
    }
}
impl CPOL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CPOL_A {
        match self.bits {
            false => CPOL_A::IDLE_LOW,
            true => CPOL_A::IDLE_HIGH,
        }
    }
    #[doc = "Checks if the value of the field is `IDLE_LOW`"]
    #[inline(always)]
    pub fn is_idle_low(&self) -> bool {
        *self == CPOL_A::IDLE_LOW
    }
    #[doc = "Checks if the value of the field is `IDLE_HIGH`"]
    #[inline(always)]
    pub fn is_idle_high(&self) -> bool {
        *self == CPOL_A::IDLE_HIGH
    }
}
#[doc = "Field `CPOL` writer - Clock polarity"]
pub type CPOL_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, CPOL_A>;
impl<'a, const O: u8> CPOL_W<'a, O> {
    #[doc = "CK to 0 when idle"]
    #[inline(always)]
    pub fn idle_low(self) -> &'a mut W {
        self.variant(CPOL_A::IDLE_LOW)
    }
    #[doc = "CK to 1 when idle"]
    #[inline(always)]
    pub fn idle_high(self) -> &'a mut W {
        self.variant(CPOL_A::IDLE_HIGH)
    }
}
#[doc = "Field `MSTR` reader - Master selection"]
pub type MSTR_R = crate::BitReader<MSTR_A>;
#[doc = "Master selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MSTR_A {
    #[doc = "0: Slave configuration"]
    SLAVE = 0,
    #[doc = "1: Master configuration"]
    MASTER = 1,
}
impl From<MSTR_A> for bool {
    #[inline(always)]
    fn from(variant: MSTR_A) -> Self {
        variant as u8 != 0
    }
}
impl MSTR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MSTR_A {
        match self.bits {
            false => MSTR_A::SLAVE,
            true => MSTR_A::MASTER,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE`"]
    #[inline(always)]
    pub fn is_slave(&self) -> bool {
        *self == MSTR_A::SLAVE
    }
    #[doc = "Checks if the value of the field is `MASTER`"]
    #[inline(always)]
    pub fn is_master(&self) -> bool {
        *self == MSTR_A::MASTER
    }
}
#[doc = "Field `MSTR` writer - Master selection"]
pub type MSTR_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, MSTR_A>;
impl<'a, const O: u8> MSTR_W<'a, O> {
    #[doc = "Slave configuration"]
    #[inline(always)]
    pub fn slave(self) -> &'a mut W {
        self.variant(MSTR_A::SLAVE)
    }
    #[doc = "Master configuration"]
    #[inline(always)]
    pub fn master(self) -> &'a mut W {
        self.variant(MSTR_A::MASTER)
    }
}
#[doc = "Field `BR` reader - Baud rate control"]
pub type BR_R = crate::FieldReader<BR_A>;
#[doc = "Baud rate control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum BR_A {
    #[doc = "0: f_PCLK / 2"]
    DIV2 = 0,
    #[doc = "1: f_PCLK / 4"]
    DIV4 = 1,
    #[doc = "2: f_PCLK / 8"]
    DIV8 = 2,
    #[doc = "3: f_PCLK / 16"]
    DIV16 = 3,
    #[doc = "4: f_PCLK / 32"]
    DIV32 = 4,
    #[doc = "5: f_PCLK / 64"]
    DIV64 = 5,
    #[doc = "6: f_PCLK / 128"]
    DIV128 = 6,
    #[doc = "7: f_PCLK / 256"]
    DIV256 = 7,
}
impl From<BR_A> for u8 {
    #[inline(always)]
    fn from(variant: BR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for BR_A {
    type Ux = u8;
}
impl BR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BR_A {
        match self.bits {
            0 => BR_A::DIV2,
            1 => BR_A::DIV4,
            2 => BR_A::DIV8,
            3 => BR_A::DIV16,
            4 => BR_A::DIV32,
            5 => BR_A::DIV64,
            6 => BR_A::DIV128,
            7 => BR_A::DIV256,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == BR_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == BR_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == BR_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == BR_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == BR_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == BR_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == BR_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == BR_A::DIV256
    }
}
#[doc = "Field `BR` writer - Baud rate control"]
pub type BR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CR1_SPEC, 3, O, BR_A>;
impl<'a, const O: u8> BR_W<'a, O> {
    #[doc = "f_PCLK / 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(BR_A::DIV2)
    }
    #[doc = "f_PCLK / 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(BR_A::DIV4)
    }
    #[doc = "f_PCLK / 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(BR_A::DIV8)
    }
    #[doc = "f_PCLK / 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(BR_A::DIV16)
    }
    #[doc = "f_PCLK / 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(BR_A::DIV32)
    }
    #[doc = "f_PCLK / 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(BR_A::DIV64)
    }
    #[doc = "f_PCLK / 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(BR_A::DIV128)
    }
    #[doc = "f_PCLK / 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(BR_A::DIV256)
    }
}
#[doc = "Field `SPE` reader - SPI enable"]
pub type SPE_R = crate::BitReader<SPE_A>;
#[doc = "SPI enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SPE_A {
    #[doc = "0: Peripheral disabled"]
    DISABLED = 0,
    #[doc = "1: Peripheral enabled"]
    ENABLED = 1,
}
impl From<SPE_A> for bool {
    #[inline(always)]
    fn from(variant: SPE_A) -> Self {
        variant as u8 != 0
    }
}
impl SPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SPE_A {
        match self.bits {
            false => SPE_A::DISABLED,
            true => SPE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SPE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SPE_A::ENABLED
    }
}
#[doc = "Field `SPE` writer - SPI enable"]
pub type SPE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, SPE_A>;
impl<'a, const O: u8> SPE_W<'a, O> {
    #[doc = "Peripheral disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SPE_A::DISABLED)
    }
    #[doc = "Peripheral enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SPE_A::ENABLED)
    }
}
#[doc = "Field `LSBFIRST` reader - Frame format"]
pub type LSBFIRST_R = crate::BitReader<LSBFIRST_A>;
#[doc = "Frame format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LSBFIRST_A {
    #[doc = "0: Data is transmitted/received with the MSB first"]
    MSBFIRST = 0,
    #[doc = "1: Data is transmitted/received with the LSB first"]
    LSBFIRST = 1,
}
impl From<LSBFIRST_A> for bool {
    #[inline(always)]
    fn from(variant: LSBFIRST_A) -> Self {
        variant as u8 != 0
    }
}
impl LSBFIRST_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LSBFIRST_A {
        match self.bits {
            false => LSBFIRST_A::MSBFIRST,
            true => LSBFIRST_A::LSBFIRST,
        }
    }
    #[doc = "Checks if the value of the field is `MSBFIRST`"]
    #[inline(always)]
    pub fn is_msbfirst(&self) -> bool {
        *self == LSBFIRST_A::MSBFIRST
    }
    #[doc = "Checks if the value of the field is `LSBFIRST`"]
    #[inline(always)]
    pub fn is_lsbfirst(&self) -> bool {
        *self == LSBFIRST_A::LSBFIRST
    }
}
#[doc = "Field `LSBFIRST` writer - Frame format"]
pub type LSBFIRST_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, LSBFIRST_A>;
impl<'a, const O: u8> LSBFIRST_W<'a, O> {
    #[doc = "Data is transmitted/received with the MSB first"]
    #[inline(always)]
    pub fn msbfirst(self) -> &'a mut W {
        self.variant(LSBFIRST_A::MSBFIRST)
    }
    #[doc = "Data is transmitted/received with the LSB first"]
    #[inline(always)]
    pub fn lsbfirst(self) -> &'a mut W {
        self.variant(LSBFIRST_A::LSBFIRST)
    }
}
#[doc = "Field `SSI` reader - Internal slave select"]
pub type SSI_R = crate::BitReader<SSI_A>;
#[doc = "Internal slave select\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSI_A {
    #[doc = "0: 0 is forced onto the NSS pin and the I/O value of the NSS pin is ignored"]
    SLAVE_SELECTED = 0,
    #[doc = "1: 1 is forced onto the NSS pin and the I/O value of the NSS pin is ignored"]
    SLAVE_NOT_SELECTED = 1,
}
impl From<SSI_A> for bool {
    #[inline(always)]
    fn from(variant: SSI_A) -> Self {
        variant as u8 != 0
    }
}
impl SSI_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSI_A {
        match self.bits {
            false => SSI_A::SLAVE_SELECTED,
            true => SSI_A::SLAVE_NOT_SELECTED,
        }
    }
    #[doc = "Checks if the value of the field is `SLAVE_SELECTED`"]
    #[inline(always)]
    pub fn is_slave_selected(&self) -> bool {
        *self == SSI_A::SLAVE_SELECTED
    }
    #[doc = "Checks if the value of the field is `SLAVE_NOT_SELECTED`"]
    #[inline(always)]
    pub fn is_slave_not_selected(&self) -> bool {
        *self == SSI_A::SLAVE_NOT_SELECTED
    }
}
#[doc = "Field `SSI` writer - Internal slave select"]
pub type SSI_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, SSI_A>;
impl<'a, const O: u8> SSI_W<'a, O> {
    #[doc = "0 is forced onto the NSS pin and the I/O value of the NSS pin is ignored"]
    #[inline(always)]
    pub fn slave_selected(self) -> &'a mut W {
        self.variant(SSI_A::SLAVE_SELECTED)
    }
    #[doc = "1 is forced onto the NSS pin and the I/O value of the NSS pin is ignored"]
    #[inline(always)]
    pub fn slave_not_selected(self) -> &'a mut W {
        self.variant(SSI_A::SLAVE_NOT_SELECTED)
    }
}
#[doc = "Field `SSM` reader - Software slave management"]
pub type SSM_R = crate::BitReader<SSM_A>;
#[doc = "Software slave management\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSM_A {
    #[doc = "0: Software slave management disabled"]
    DISABLED = 0,
    #[doc = "1: Software slave management enabled"]
    ENABLED = 1,
}
impl From<SSM_A> for bool {
    #[inline(always)]
    fn from(variant: SSM_A) -> Self {
        variant as u8 != 0
    }
}
impl SSM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSM_A {
        match self.bits {
            false => SSM_A::DISABLED,
            true => SSM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSM_A::ENABLED
    }
}
#[doc = "Field `SSM` writer - Software slave management"]
pub type SSM_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, SSM_A>;
impl<'a, const O: u8> SSM_W<'a, O> {
    #[doc = "Software slave management disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSM_A::DISABLED)
    }
    #[doc = "Software slave management enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSM_A::ENABLED)
    }
}
#[doc = "Field `RXONLY` reader - Receive only"]
pub type RXONLY_R = crate::BitReader<RXONLY_A>;
#[doc = "Receive only\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXONLY_A {
    #[doc = "0: Full duplex (Transmit and receive)"]
    FULL_DUPLEX = 0,
    #[doc = "1: Output disabled (Receive-only mode)"]
    OUTPUT_DISABLED = 1,
}
impl From<RXONLY_A> for bool {
    #[inline(always)]
    fn from(variant: RXONLY_A) -> Self {
        variant as u8 != 0
    }
}
impl RXONLY_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RXONLY_A {
        match self.bits {
            false => RXONLY_A::FULL_DUPLEX,
            true => RXONLY_A::OUTPUT_DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `FULL_DUPLEX`"]
    #[inline(always)]
    pub fn is_full_duplex(&self) -> bool {
        *self == RXONLY_A::FULL_DUPLEX
    }
    #[doc = "Checks if the value of the field is `OUTPUT_DISABLED`"]
    #[inline(always)]
    pub fn is_output_disabled(&self) -> bool {
        *self == RXONLY_A::OUTPUT_DISABLED
    }
}
#[doc = "Field `RXONLY` writer - Receive only"]
pub type RXONLY_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, RXONLY_A>;
impl<'a, const O: u8> RXONLY_W<'a, O> {
    #[doc = "Full duplex (Transmit and receive)"]
    #[inline(always)]
    pub fn full_duplex(self) -> &'a mut W {
        self.variant(RXONLY_A::FULL_DUPLEX)
    }
    #[doc = "Output disabled (Receive-only mode)"]
    #[inline(always)]
    pub fn output_disabled(self) -> &'a mut W {
        self.variant(RXONLY_A::OUTPUT_DISABLED)
    }
}
#[doc = "Field `CRCL` reader - CRC length"]
pub type CRCL_R = crate::BitReader<CRCL_A>;
#[doc = "CRC length\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCL_A {
    #[doc = "0: 8-bit CRC length"]
    EIGHT_BIT = 0,
    #[doc = "1: 16-bit CRC length"]
    SIXTEEN_BIT = 1,
}
impl From<CRCL_A> for bool {
    #[inline(always)]
    fn from(variant: CRCL_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCL_A {
        match self.bits {
            false => CRCL_A::EIGHT_BIT,
            true => CRCL_A::SIXTEEN_BIT,
        }
    }
    #[doc = "Checks if the value of the field is `EIGHT_BIT`"]
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == CRCL_A::EIGHT_BIT
    }
    #[doc = "Checks if the value of the field is `SIXTEEN_BIT`"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == CRCL_A::SIXTEEN_BIT
    }
}
#[doc = "Field `CRCL` writer - CRC length"]
pub type CRCL_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, CRCL_A>;
impl<'a, const O: u8> CRCL_W<'a, O> {
    #[doc = "8-bit CRC length"]
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut W {
        self.variant(CRCL_A::EIGHT_BIT)
    }
    #[doc = "16-bit CRC length"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(CRCL_A::SIXTEEN_BIT)
    }
}
#[doc = "Field `CRCNEXT` reader - CRC transfer next"]
pub type CRCNEXT_R = crate::BitReader<CRCNEXT_A>;
#[doc = "CRC transfer next\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCNEXT_A {
    #[doc = "0: Next transmit value is from Tx buffer"]
    TX_BUFFER = 0,
    #[doc = "1: Next transmit value is from Tx CRC register"]
    CRC = 1,
}
impl From<CRCNEXT_A> for bool {
    #[inline(always)]
    fn from(variant: CRCNEXT_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCNEXT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCNEXT_A {
        match self.bits {
            false => CRCNEXT_A::TX_BUFFER,
            true => CRCNEXT_A::CRC,
        }
    }
    #[doc = "Checks if the value of the field is `TX_BUFFER`"]
    #[inline(always)]
    pub fn is_tx_buffer(&self) -> bool {
        *self == CRCNEXT_A::TX_BUFFER
    }
    #[doc = "Checks if the value of the field is `CRC`"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == CRCNEXT_A::CRC
    }
}
#[doc = "Field `CRCNEXT` writer - CRC transfer next"]
pub type CRCNEXT_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, CRCNEXT_A>;
impl<'a, const O: u8> CRCNEXT_W<'a, O> {
    #[doc = "Next transmit value is from Tx buffer"]
    #[inline(always)]
    pub fn tx_buffer(self) -> &'a mut W {
        self.variant(CRCNEXT_A::TX_BUFFER)
    }
    #[doc = "Next transmit value is from Tx CRC register"]
    #[inline(always)]
    pub fn crc(self) -> &'a mut W {
        self.variant(CRCNEXT_A::CRC)
    }
}
#[doc = "Field `CRCEN` reader - Hardware CRC calculation enable"]
pub type CRCEN_R = crate::BitReader<CRCEN_A>;
#[doc = "Hardware CRC calculation enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CRCEN_A {
    #[doc = "0: CRC calculation disabled"]
    DISABLED = 0,
    #[doc = "1: CRC calculation enabled"]
    ENABLED = 1,
}
impl From<CRCEN_A> for bool {
    #[inline(always)]
    fn from(variant: CRCEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CRCEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CRCEN_A {
        match self.bits {
            false => CRCEN_A::DISABLED,
            true => CRCEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CRCEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CRCEN_A::ENABLED
    }
}
#[doc = "Field `CRCEN` writer - Hardware CRC calculation enable"]
pub type CRCEN_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, CRCEN_A>;
impl<'a, const O: u8> CRCEN_W<'a, O> {
    #[doc = "CRC calculation disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CRCEN_A::DISABLED)
    }
    #[doc = "CRC calculation enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CRCEN_A::ENABLED)
    }
}
#[doc = "Field `BIDIOE` reader - Output enable in bidirectional mode"]
pub type BIDIOE_R = crate::BitReader<BIDIOE_A>;
#[doc = "Output enable in bidirectional mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIDIOE_A {
    #[doc = "0: Output disabled (receive-only mode)"]
    OUTPUT_DISABLED = 0,
    #[doc = "1: Output enabled (transmit-only mode)"]
    OUTPUT_ENABLED = 1,
}
impl From<BIDIOE_A> for bool {
    #[inline(always)]
    fn from(variant: BIDIOE_A) -> Self {
        variant as u8 != 0
    }
}
impl BIDIOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIDIOE_A {
        match self.bits {
            false => BIDIOE_A::OUTPUT_DISABLED,
            true => BIDIOE_A::OUTPUT_ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `OUTPUT_DISABLED`"]
    #[inline(always)]
    pub fn is_output_disabled(&self) -> bool {
        *self == BIDIOE_A::OUTPUT_DISABLED
    }
    #[doc = "Checks if the value of the field is `OUTPUT_ENABLED`"]
    #[inline(always)]
    pub fn is_output_enabled(&self) -> bool {
        *self == BIDIOE_A::OUTPUT_ENABLED
    }
}
#[doc = "Field `BIDIOE` writer - Output enable in bidirectional mode"]
pub type BIDIOE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, BIDIOE_A>;
impl<'a, const O: u8> BIDIOE_W<'a, O> {
    #[doc = "Output disabled (receive-only mode)"]
    #[inline(always)]
    pub fn output_disabled(self) -> &'a mut W {
        self.variant(BIDIOE_A::OUTPUT_DISABLED)
    }
    #[doc = "Output enabled (transmit-only mode)"]
    #[inline(always)]
    pub fn output_enabled(self) -> &'a mut W {
        self.variant(BIDIOE_A::OUTPUT_ENABLED)
    }
}
#[doc = "Field `BIDIMODE` reader - Bidirectional data mode enable"]
pub type BIDIMODE_R = crate::BitReader<BIDIMODE_A>;
#[doc = "Bidirectional data mode enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BIDIMODE_A {
    #[doc = "0: 2-line unidirectional data mode selected"]
    UNIDIRECTIONAL = 0,
    #[doc = "1: 1-line bidirectional data mode selected"]
    BIDIRECTIONAL = 1,
}
impl From<BIDIMODE_A> for bool {
    #[inline(always)]
    fn from(variant: BIDIMODE_A) -> Self {
        variant as u8 != 0
    }
}
impl BIDIMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> BIDIMODE_A {
        match self.bits {
            false => BIDIMODE_A::UNIDIRECTIONAL,
            true => BIDIMODE_A::BIDIRECTIONAL,
        }
    }
    #[doc = "Checks if the value of the field is `UNIDIRECTIONAL`"]
    #[inline(always)]
    pub fn is_unidirectional(&self) -> bool {
        *self == BIDIMODE_A::UNIDIRECTIONAL
    }
    #[doc = "Checks if the value of the field is `BIDIRECTIONAL`"]
    #[inline(always)]
    pub fn is_bidirectional(&self) -> bool {
        *self == BIDIMODE_A::BIDIRECTIONAL
    }
}
#[doc = "Field `BIDIMODE` writer - Bidirectional data mode enable"]
pub type BIDIMODE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, BIDIMODE_A>;
impl<'a, const O: u8> BIDIMODE_W<'a, O> {
    #[doc = "2-line unidirectional data mode selected"]
    #[inline(always)]
    pub fn unidirectional(self) -> &'a mut W {
        self.variant(BIDIMODE_A::UNIDIRECTIONAL)
    }
    #[doc = "1-line bidirectional data mode selected"]
    #[inline(always)]
    pub fn bidirectional(self) -> &'a mut W {
        self.variant(BIDIMODE_A::BIDIRECTIONAL)
    }
}
impl R {
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    pub fn cpha(&self) -> CPHA_R {
        CPHA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    pub fn cpol(&self) -> CPOL_R {
        CPOL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Master selection"]
    #[inline(always)]
    pub fn mstr(&self) -> MSTR_R {
        MSTR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - Baud rate control"]
    #[inline(always)]
    pub fn br(&self) -> BR_R {
        BR_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    pub fn spe(&self) -> SPE_R {
        SPE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline(always)]
    pub fn lsbfirst(&self) -> LSBFIRST_R {
        LSBFIRST_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Internal slave select"]
    #[inline(always)]
    pub fn ssi(&self) -> SSI_R {
        SSI_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Software slave management"]
    #[inline(always)]
    pub fn ssm(&self) -> SSM_R {
        SSM_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    pub fn rxonly(&self) -> RXONLY_R {
        RXONLY_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - CRC length"]
    #[inline(always)]
    pub fn crcl(&self) -> CRCL_R {
        CRCL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline(always)]
    pub fn crcnext(&self) -> CRCNEXT_R {
        CRCNEXT_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline(always)]
    pub fn crcen(&self) -> CRCEN_R {
        CRCEN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode"]
    #[inline(always)]
    pub fn bidioe(&self) -> BIDIOE_R {
        BIDIOE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Bidirectional data mode enable"]
    #[inline(always)]
    pub fn bidimode(&self) -> BIDIMODE_R {
        BIDIMODE_R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Clock phase"]
    #[inline(always)]
    #[must_use]
    pub fn cpha(&mut self) -> CPHA_W<0> {
        CPHA_W::new(self)
    }
    #[doc = "Bit 1 - Clock polarity"]
    #[inline(always)]
    #[must_use]
    pub fn cpol(&mut self) -> CPOL_W<1> {
        CPOL_W::new(self)
    }
    #[doc = "Bit 2 - Master selection"]
    #[inline(always)]
    #[must_use]
    pub fn mstr(&mut self) -> MSTR_W<2> {
        MSTR_W::new(self)
    }
    #[doc = "Bits 3:5 - Baud rate control"]
    #[inline(always)]
    #[must_use]
    pub fn br(&mut self) -> BR_W<3> {
        BR_W::new(self)
    }
    #[doc = "Bit 6 - SPI enable"]
    #[inline(always)]
    #[must_use]
    pub fn spe(&mut self) -> SPE_W<6> {
        SPE_W::new(self)
    }
    #[doc = "Bit 7 - Frame format"]
    #[inline(always)]
    #[must_use]
    pub fn lsbfirst(&mut self) -> LSBFIRST_W<7> {
        LSBFIRST_W::new(self)
    }
    #[doc = "Bit 8 - Internal slave select"]
    #[inline(always)]
    #[must_use]
    pub fn ssi(&mut self) -> SSI_W<8> {
        SSI_W::new(self)
    }
    #[doc = "Bit 9 - Software slave management"]
    #[inline(always)]
    #[must_use]
    pub fn ssm(&mut self) -> SSM_W<9> {
        SSM_W::new(self)
    }
    #[doc = "Bit 10 - Receive only"]
    #[inline(always)]
    #[must_use]
    pub fn rxonly(&mut self) -> RXONLY_W<10> {
        RXONLY_W::new(self)
    }
    #[doc = "Bit 11 - CRC length"]
    #[inline(always)]
    #[must_use]
    pub fn crcl(&mut self) -> CRCL_W<11> {
        CRCL_W::new(self)
    }
    #[doc = "Bit 12 - CRC transfer next"]
    #[inline(always)]
    #[must_use]
    pub fn crcnext(&mut self) -> CRCNEXT_W<12> {
        CRCNEXT_W::new(self)
    }
    #[doc = "Bit 13 - Hardware CRC calculation enable"]
    #[inline(always)]
    #[must_use]
    pub fn crcen(&mut self) -> CRCEN_W<13> {
        CRCEN_W::new(self)
    }
    #[doc = "Bit 14 - Output enable in bidirectional mode"]
    #[inline(always)]
    #[must_use]
    pub fn bidioe(&mut self) -> BIDIOE_W<14> {
        BIDIOE_W::new(self)
    }
    #[doc = "Bit 15 - Bidirectional data mode enable"]
    #[inline(always)]
    #[must_use]
    pub fn bidimode(&mut self) -> BIDIMODE_W<15> {
        BIDIMODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr1](index.html) module"]
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
