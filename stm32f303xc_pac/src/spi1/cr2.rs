#[doc = "Register `CR2` reader"]
pub struct R(crate::R<CR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR2` writer"]
pub struct W(crate::W<CR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR2_SPEC>;
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
impl From<crate::W<CR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RXDMAEN` reader - Rx buffer DMA enable"]
pub type RXDMAEN_R = crate::BitReader<RXDMAEN_A>;
#[doc = "Rx buffer DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXDMAEN_A {
    #[doc = "0: Rx buffer DMA disabled"]
    DISABLED = 0,
    #[doc = "1: Rx buffer DMA enabled"]
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
#[doc = "Field `RXDMAEN` writer - Rx buffer DMA enable"]
pub type RXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, RXDMAEN_A>;
impl<'a, const O: u8> RXDMAEN_W<'a, O> {
    #[doc = "Rx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::DISABLED)
    }
    #[doc = "Rx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(RXDMAEN_A::ENABLED)
    }
}
#[doc = "Field `TXDMAEN` reader - Tx buffer DMA enable"]
pub type TXDMAEN_R = crate::BitReader<TXDMAEN_A>;
#[doc = "Tx buffer DMA enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXDMAEN_A {
    #[doc = "0: Tx buffer DMA disabled"]
    DISABLED = 0,
    #[doc = "1: Tx buffer DMA enabled"]
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
#[doc = "Field `TXDMAEN` writer - Tx buffer DMA enable"]
pub type TXDMAEN_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, TXDMAEN_A>;
impl<'a, const O: u8> TXDMAEN_W<'a, O> {
    #[doc = "Tx buffer DMA disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::DISABLED)
    }
    #[doc = "Tx buffer DMA enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TXDMAEN_A::ENABLED)
    }
}
#[doc = "Field `SSOE` reader - SS output enable"]
pub type SSOE_R = crate::BitReader<SSOE_A>;
#[doc = "SS output enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SSOE_A {
    #[doc = "0: SS output is disabled in master mode"]
    DISABLED = 0,
    #[doc = "1: SS output is enabled in master mode"]
    ENABLED = 1,
}
impl From<SSOE_A> for bool {
    #[inline(always)]
    fn from(variant: SSOE_A) -> Self {
        variant as u8 != 0
    }
}
impl SSOE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SSOE_A {
        match self.bits {
            false => SSOE_A::DISABLED,
            true => SSOE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == SSOE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == SSOE_A::ENABLED
    }
}
#[doc = "Field `SSOE` writer - SS output enable"]
pub type SSOE_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, SSOE_A>;
impl<'a, const O: u8> SSOE_W<'a, O> {
    #[doc = "SS output is disabled in master mode"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(SSOE_A::DISABLED)
    }
    #[doc = "SS output is enabled in master mode"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(SSOE_A::ENABLED)
    }
}
#[doc = "Field `NSSP` reader - NSS pulse management"]
pub type NSSP_R = crate::BitReader<NSSP_A>;
#[doc = "NSS pulse management\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NSSP_A {
    #[doc = "0: No NSS pulse"]
    NO_PULSE = 0,
    #[doc = "1: NSS pulse generated"]
    PULSE_GENERATED = 1,
}
impl From<NSSP_A> for bool {
    #[inline(always)]
    fn from(variant: NSSP_A) -> Self {
        variant as u8 != 0
    }
}
impl NSSP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NSSP_A {
        match self.bits {
            false => NSSP_A::NO_PULSE,
            true => NSSP_A::PULSE_GENERATED,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PULSE`"]
    #[inline(always)]
    pub fn is_no_pulse(&self) -> bool {
        *self == NSSP_A::NO_PULSE
    }
    #[doc = "Checks if the value of the field is `PULSE_GENERATED`"]
    #[inline(always)]
    pub fn is_pulse_generated(&self) -> bool {
        *self == NSSP_A::PULSE_GENERATED
    }
}
#[doc = "Field `NSSP` writer - NSS pulse management"]
pub type NSSP_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, NSSP_A>;
impl<'a, const O: u8> NSSP_W<'a, O> {
    #[doc = "No NSS pulse"]
    #[inline(always)]
    pub fn no_pulse(self) -> &'a mut W {
        self.variant(NSSP_A::NO_PULSE)
    }
    #[doc = "NSS pulse generated"]
    #[inline(always)]
    pub fn pulse_generated(self) -> &'a mut W {
        self.variant(NSSP_A::PULSE_GENERATED)
    }
}
#[doc = "Field `FRF` reader - Frame format"]
pub type FRF_R = crate::BitReader<FRF_A>;
#[doc = "Frame format\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRF_A {
    #[doc = "0: SPI Motorola mode"]
    MOTOROLA = 0,
    #[doc = "1: SPI TI mode"]
    TI = 1,
}
impl From<FRF_A> for bool {
    #[inline(always)]
    fn from(variant: FRF_A) -> Self {
        variant as u8 != 0
    }
}
impl FRF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRF_A {
        match self.bits {
            false => FRF_A::MOTOROLA,
            true => FRF_A::TI,
        }
    }
    #[doc = "Checks if the value of the field is `MOTOROLA`"]
    #[inline(always)]
    pub fn is_motorola(&self) -> bool {
        *self == FRF_A::MOTOROLA
    }
    #[doc = "Checks if the value of the field is `TI`"]
    #[inline(always)]
    pub fn is_ti(&self) -> bool {
        *self == FRF_A::TI
    }
}
#[doc = "Field `FRF` writer - Frame format"]
pub type FRF_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, FRF_A>;
impl<'a, const O: u8> FRF_W<'a, O> {
    #[doc = "SPI Motorola mode"]
    #[inline(always)]
    pub fn motorola(self) -> &'a mut W {
        self.variant(FRF_A::MOTOROLA)
    }
    #[doc = "SPI TI mode"]
    #[inline(always)]
    pub fn ti(self) -> &'a mut W {
        self.variant(FRF_A::TI)
    }
}
#[doc = "Field `ERRIE` reader - Error interrupt enable"]
pub type ERRIE_R = crate::BitReader<ERRIE_A>;
#[doc = "Error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ERRIE_A {
    #[doc = "0: Error interrupt masked"]
    MASKED = 0,
    #[doc = "1: Error interrupt not masked"]
    NOT_MASKED = 1,
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
            false => ERRIE_A::MASKED,
            true => ERRIE_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == ERRIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == ERRIE_A::NOT_MASKED
    }
}
#[doc = "Field `ERRIE` writer - Error interrupt enable"]
pub type ERRIE_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, ERRIE_A>;
impl<'a, const O: u8> ERRIE_W<'a, O> {
    #[doc = "Error interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(ERRIE_A::MASKED)
    }
    #[doc = "Error interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(ERRIE_A::NOT_MASKED)
    }
}
#[doc = "Field `RXNEIE` reader - RX buffer not empty interrupt enable"]
pub type RXNEIE_R = crate::BitReader<RXNEIE_A>;
#[doc = "RX buffer not empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RXNEIE_A {
    #[doc = "0: RXE interrupt masked"]
    MASKED = 0,
    #[doc = "1: RXE interrupt not masked"]
    NOT_MASKED = 1,
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
            false => RXNEIE_A::MASKED,
            true => RXNEIE_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == RXNEIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == RXNEIE_A::NOT_MASKED
    }
}
#[doc = "Field `RXNEIE` writer - RX buffer not empty interrupt enable"]
pub type RXNEIE_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, RXNEIE_A>;
impl<'a, const O: u8> RXNEIE_W<'a, O> {
    #[doc = "RXE interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(RXNEIE_A::MASKED)
    }
    #[doc = "RXE interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(RXNEIE_A::NOT_MASKED)
    }
}
#[doc = "Field `TXEIE` reader - Tx buffer empty interrupt enable"]
pub type TXEIE_R = crate::BitReader<TXEIE_A>;
#[doc = "Tx buffer empty interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TXEIE_A {
    #[doc = "0: TXE interrupt masked"]
    MASKED = 0,
    #[doc = "1: TXE interrupt not masked"]
    NOT_MASKED = 1,
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
            false => TXEIE_A::MASKED,
            true => TXEIE_A::NOT_MASKED,
        }
    }
    #[doc = "Checks if the value of the field is `MASKED`"]
    #[inline(always)]
    pub fn is_masked(&self) -> bool {
        *self == TXEIE_A::MASKED
    }
    #[doc = "Checks if the value of the field is `NOT_MASKED`"]
    #[inline(always)]
    pub fn is_not_masked(&self) -> bool {
        *self == TXEIE_A::NOT_MASKED
    }
}
#[doc = "Field `TXEIE` writer - Tx buffer empty interrupt enable"]
pub type TXEIE_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, TXEIE_A>;
impl<'a, const O: u8> TXEIE_W<'a, O> {
    #[doc = "TXE interrupt masked"]
    #[inline(always)]
    pub fn masked(self) -> &'a mut W {
        self.variant(TXEIE_A::MASKED)
    }
    #[doc = "TXE interrupt not masked"]
    #[inline(always)]
    pub fn not_masked(self) -> &'a mut W {
        self.variant(TXEIE_A::NOT_MASKED)
    }
}
#[doc = "Field `DS` reader - Data size"]
pub type DS_R = crate::FieldReader<DS_A>;
#[doc = "Data size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DS_A {
    #[doc = "3: 4-bit"]
    FOUR_BIT = 3,
    #[doc = "4: 5-bit"]
    FIVE_BIT = 4,
    #[doc = "5: 6-bit"]
    SIX_BIT = 5,
    #[doc = "6: 7-bit"]
    SEVEN_BIT = 6,
    #[doc = "7: 8-bit"]
    EIGHT_BIT = 7,
    #[doc = "8: 9-bit"]
    NINE_BIT = 8,
    #[doc = "9: 10-bit"]
    TEN_BIT = 9,
    #[doc = "10: 11-bit"]
    ELEVEN_BIT = 10,
    #[doc = "11: 12-bit"]
    TWELVE_BIT = 11,
    #[doc = "12: 13-bit"]
    THIRTEEN_BIT = 12,
    #[doc = "13: 14-bit"]
    FOURTEEN_BIT = 13,
    #[doc = "14: 15-bit"]
    FIFTEEN_BIT = 14,
    #[doc = "15: 16-bit"]
    SIXTEEN_BIT = 15,
}
impl From<DS_A> for u8 {
    #[inline(always)]
    fn from(variant: DS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DS_A {
    type Ux = u8;
}
impl DS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DS_A> {
        match self.bits {
            3 => Some(DS_A::FOUR_BIT),
            4 => Some(DS_A::FIVE_BIT),
            5 => Some(DS_A::SIX_BIT),
            6 => Some(DS_A::SEVEN_BIT),
            7 => Some(DS_A::EIGHT_BIT),
            8 => Some(DS_A::NINE_BIT),
            9 => Some(DS_A::TEN_BIT),
            10 => Some(DS_A::ELEVEN_BIT),
            11 => Some(DS_A::TWELVE_BIT),
            12 => Some(DS_A::THIRTEEN_BIT),
            13 => Some(DS_A::FOURTEEN_BIT),
            14 => Some(DS_A::FIFTEEN_BIT),
            15 => Some(DS_A::SIXTEEN_BIT),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `FOUR_BIT`"]
    #[inline(always)]
    pub fn is_four_bit(&self) -> bool {
        *self == DS_A::FOUR_BIT
    }
    #[doc = "Checks if the value of the field is `FIVE_BIT`"]
    #[inline(always)]
    pub fn is_five_bit(&self) -> bool {
        *self == DS_A::FIVE_BIT
    }
    #[doc = "Checks if the value of the field is `SIX_BIT`"]
    #[inline(always)]
    pub fn is_six_bit(&self) -> bool {
        *self == DS_A::SIX_BIT
    }
    #[doc = "Checks if the value of the field is `SEVEN_BIT`"]
    #[inline(always)]
    pub fn is_seven_bit(&self) -> bool {
        *self == DS_A::SEVEN_BIT
    }
    #[doc = "Checks if the value of the field is `EIGHT_BIT`"]
    #[inline(always)]
    pub fn is_eight_bit(&self) -> bool {
        *self == DS_A::EIGHT_BIT
    }
    #[doc = "Checks if the value of the field is `NINE_BIT`"]
    #[inline(always)]
    pub fn is_nine_bit(&self) -> bool {
        *self == DS_A::NINE_BIT
    }
    #[doc = "Checks if the value of the field is `TEN_BIT`"]
    #[inline(always)]
    pub fn is_ten_bit(&self) -> bool {
        *self == DS_A::TEN_BIT
    }
    #[doc = "Checks if the value of the field is `ELEVEN_BIT`"]
    #[inline(always)]
    pub fn is_eleven_bit(&self) -> bool {
        *self == DS_A::ELEVEN_BIT
    }
    #[doc = "Checks if the value of the field is `TWELVE_BIT`"]
    #[inline(always)]
    pub fn is_twelve_bit(&self) -> bool {
        *self == DS_A::TWELVE_BIT
    }
    #[doc = "Checks if the value of the field is `THIRTEEN_BIT`"]
    #[inline(always)]
    pub fn is_thirteen_bit(&self) -> bool {
        *self == DS_A::THIRTEEN_BIT
    }
    #[doc = "Checks if the value of the field is `FOURTEEN_BIT`"]
    #[inline(always)]
    pub fn is_fourteen_bit(&self) -> bool {
        *self == DS_A::FOURTEEN_BIT
    }
    #[doc = "Checks if the value of the field is `FIFTEEN_BIT`"]
    #[inline(always)]
    pub fn is_fifteen_bit(&self) -> bool {
        *self == DS_A::FIFTEEN_BIT
    }
    #[doc = "Checks if the value of the field is `SIXTEEN_BIT`"]
    #[inline(always)]
    pub fn is_sixteen_bit(&self) -> bool {
        *self == DS_A::SIXTEEN_BIT
    }
}
#[doc = "Field `DS` writer - Data size"]
pub type DS_W<'a, const O: u8> = crate::FieldWriter<'a, CR2_SPEC, 4, O, DS_A>;
impl<'a, const O: u8> DS_W<'a, O> {
    #[doc = "4-bit"]
    #[inline(always)]
    pub fn four_bit(self) -> &'a mut W {
        self.variant(DS_A::FOUR_BIT)
    }
    #[doc = "5-bit"]
    #[inline(always)]
    pub fn five_bit(self) -> &'a mut W {
        self.variant(DS_A::FIVE_BIT)
    }
    #[doc = "6-bit"]
    #[inline(always)]
    pub fn six_bit(self) -> &'a mut W {
        self.variant(DS_A::SIX_BIT)
    }
    #[doc = "7-bit"]
    #[inline(always)]
    pub fn seven_bit(self) -> &'a mut W {
        self.variant(DS_A::SEVEN_BIT)
    }
    #[doc = "8-bit"]
    #[inline(always)]
    pub fn eight_bit(self) -> &'a mut W {
        self.variant(DS_A::EIGHT_BIT)
    }
    #[doc = "9-bit"]
    #[inline(always)]
    pub fn nine_bit(self) -> &'a mut W {
        self.variant(DS_A::NINE_BIT)
    }
    #[doc = "10-bit"]
    #[inline(always)]
    pub fn ten_bit(self) -> &'a mut W {
        self.variant(DS_A::TEN_BIT)
    }
    #[doc = "11-bit"]
    #[inline(always)]
    pub fn eleven_bit(self) -> &'a mut W {
        self.variant(DS_A::ELEVEN_BIT)
    }
    #[doc = "12-bit"]
    #[inline(always)]
    pub fn twelve_bit(self) -> &'a mut W {
        self.variant(DS_A::TWELVE_BIT)
    }
    #[doc = "13-bit"]
    #[inline(always)]
    pub fn thirteen_bit(self) -> &'a mut W {
        self.variant(DS_A::THIRTEEN_BIT)
    }
    #[doc = "14-bit"]
    #[inline(always)]
    pub fn fourteen_bit(self) -> &'a mut W {
        self.variant(DS_A::FOURTEEN_BIT)
    }
    #[doc = "15-bit"]
    #[inline(always)]
    pub fn fifteen_bit(self) -> &'a mut W {
        self.variant(DS_A::FIFTEEN_BIT)
    }
    #[doc = "16-bit"]
    #[inline(always)]
    pub fn sixteen_bit(self) -> &'a mut W {
        self.variant(DS_A::SIXTEEN_BIT)
    }
}
#[doc = "Field `FRXTH` reader - FIFO reception threshold"]
pub type FRXTH_R = crate::BitReader<FRXTH_A>;
#[doc = "FIFO reception threshold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FRXTH_A {
    #[doc = "0: RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    HALF = 0,
    #[doc = "1: RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    QUARTER = 1,
}
impl From<FRXTH_A> for bool {
    #[inline(always)]
    fn from(variant: FRXTH_A) -> Self {
        variant as u8 != 0
    }
}
impl FRXTH_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FRXTH_A {
        match self.bits {
            false => FRXTH_A::HALF,
            true => FRXTH_A::QUARTER,
        }
    }
    #[doc = "Checks if the value of the field is `HALF`"]
    #[inline(always)]
    pub fn is_half(&self) -> bool {
        *self == FRXTH_A::HALF
    }
    #[doc = "Checks if the value of the field is `QUARTER`"]
    #[inline(always)]
    pub fn is_quarter(&self) -> bool {
        *self == FRXTH_A::QUARTER
    }
}
#[doc = "Field `FRXTH` writer - FIFO reception threshold"]
pub type FRXTH_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, FRXTH_A>;
impl<'a, const O: u8> FRXTH_W<'a, O> {
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/2 (16-bit)"]
    #[inline(always)]
    pub fn half(self) -> &'a mut W {
        self.variant(FRXTH_A::HALF)
    }
    #[doc = "RXNE event is generated if the FIFO level is greater than or equal to 1/4 (8-bit)"]
    #[inline(always)]
    pub fn quarter(self) -> &'a mut W {
        self.variant(FRXTH_A::QUARTER)
    }
}
#[doc = "Field `LDMA_RX` reader - Last DMA transfer for reception"]
pub type LDMA_RX_R = crate::BitReader<LDMA_RX_A>;
#[doc = "Last DMA transfer for reception\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDMA_RX_A {
    #[doc = "0: Number of data to transfer for receive is even"]
    EVEN = 0,
    #[doc = "1: Number of data to transfer for receive is odd"]
    ODD = 1,
}
impl From<LDMA_RX_A> for bool {
    #[inline(always)]
    fn from(variant: LDMA_RX_A) -> Self {
        variant as u8 != 0
    }
}
impl LDMA_RX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDMA_RX_A {
        match self.bits {
            false => LDMA_RX_A::EVEN,
            true => LDMA_RX_A::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == LDMA_RX_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == LDMA_RX_A::ODD
    }
}
#[doc = "Field `LDMA_RX` writer - Last DMA transfer for reception"]
pub type LDMA_RX_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, LDMA_RX_A>;
impl<'a, const O: u8> LDMA_RX_W<'a, O> {
    #[doc = "Number of data to transfer for receive is even"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(LDMA_RX_A::EVEN)
    }
    #[doc = "Number of data to transfer for receive is odd"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(LDMA_RX_A::ODD)
    }
}
#[doc = "Field `LDMA_TX` reader - Last DMA transfer for transmission"]
pub type LDMA_TX_R = crate::BitReader<LDMA_TX_A>;
#[doc = "Last DMA transfer for transmission\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum LDMA_TX_A {
    #[doc = "0: Number of data to transfer for transmit is even"]
    EVEN = 0,
    #[doc = "1: Number of data to transfer for transmit is odd"]
    ODD = 1,
}
impl From<LDMA_TX_A> for bool {
    #[inline(always)]
    fn from(variant: LDMA_TX_A) -> Self {
        variant as u8 != 0
    }
}
impl LDMA_TX_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LDMA_TX_A {
        match self.bits {
            false => LDMA_TX_A::EVEN,
            true => LDMA_TX_A::ODD,
        }
    }
    #[doc = "Checks if the value of the field is `EVEN`"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == LDMA_TX_A::EVEN
    }
    #[doc = "Checks if the value of the field is `ODD`"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == LDMA_TX_A::ODD
    }
}
#[doc = "Field `LDMA_TX` writer - Last DMA transfer for transmission"]
pub type LDMA_TX_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, LDMA_TX_A>;
impl<'a, const O: u8> LDMA_TX_W<'a, O> {
    #[doc = "Number of data to transfer for transmit is even"]
    #[inline(always)]
    pub fn even(self) -> &'a mut W {
        self.variant(LDMA_TX_A::EVEN)
    }
    #[doc = "Number of data to transfer for transmit is odd"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut W {
        self.variant(LDMA_TX_A::ODD)
    }
}
impl R {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    pub fn rxdmaen(&self) -> RXDMAEN_R {
        RXDMAEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    pub fn txdmaen(&self) -> TXDMAEN_R {
        TXDMAEN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline(always)]
    pub fn ssoe(&self) -> SSOE_R {
        SSOE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NSS pulse management"]
    #[inline(always)]
    pub fn nssp(&self) -> NSSP_R {
        NSSP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline(always)]
    pub fn frf(&self) -> FRF_R {
        FRF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    pub fn errie(&self) -> ERRIE_R {
        ERRIE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    pub fn rxneie(&self) -> RXNEIE_R {
        RXNEIE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    pub fn txeie(&self) -> TXEIE_R {
        TXEIE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Data size"]
    #[inline(always)]
    pub fn ds(&self) -> DS_R {
        DS_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 12 - FIFO reception threshold"]
    #[inline(always)]
    pub fn frxth(&self) -> FRXTH_R {
        FRXTH_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Last DMA transfer for reception"]
    #[inline(always)]
    pub fn ldma_rx(&self) -> LDMA_RX_R {
        LDMA_RX_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission"]
    #[inline(always)]
    pub fn ldma_tx(&self) -> LDMA_TX_R {
        LDMA_TX_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Rx buffer DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxdmaen(&mut self) -> RXDMAEN_W<0> {
        RXDMAEN_W::new(self)
    }
    #[doc = "Bit 1 - Tx buffer DMA enable"]
    #[inline(always)]
    #[must_use]
    pub fn txdmaen(&mut self) -> TXDMAEN_W<1> {
        TXDMAEN_W::new(self)
    }
    #[doc = "Bit 2 - SS output enable"]
    #[inline(always)]
    #[must_use]
    pub fn ssoe(&mut self) -> SSOE_W<2> {
        SSOE_W::new(self)
    }
    #[doc = "Bit 3 - NSS pulse management"]
    #[inline(always)]
    #[must_use]
    pub fn nssp(&mut self) -> NSSP_W<3> {
        NSSP_W::new(self)
    }
    #[doc = "Bit 4 - Frame format"]
    #[inline(always)]
    #[must_use]
    pub fn frf(&mut self) -> FRF_W<4> {
        FRF_W::new(self)
    }
    #[doc = "Bit 5 - Error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn errie(&mut self) -> ERRIE_W<5> {
        ERRIE_W::new(self)
    }
    #[doc = "Bit 6 - RX buffer not empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn rxneie(&mut self) -> RXNEIE_W<6> {
        RXNEIE_W::new(self)
    }
    #[doc = "Bit 7 - Tx buffer empty interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn txeie(&mut self) -> TXEIE_W<7> {
        TXEIE_W::new(self)
    }
    #[doc = "Bits 8:11 - Data size"]
    #[inline(always)]
    #[must_use]
    pub fn ds(&mut self) -> DS_W<8> {
        DS_W::new(self)
    }
    #[doc = "Bit 12 - FIFO reception threshold"]
    #[inline(always)]
    #[must_use]
    pub fn frxth(&mut self) -> FRXTH_W<12> {
        FRXTH_W::new(self)
    }
    #[doc = "Bit 13 - Last DMA transfer for reception"]
    #[inline(always)]
    #[must_use]
    pub fn ldma_rx(&mut self) -> LDMA_RX_W<13> {
        LDMA_RX_W::new(self)
    }
    #[doc = "Bit 14 - Last DMA transfer for transmission"]
    #[inline(always)]
    #[must_use]
    pub fn ldma_tx(&mut self) -> LDMA_TX_W<14> {
        LDMA_TX_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
pub struct CR2_SPEC;
impl crate::RegisterSpec for CR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr2::R](R) reader structure"]
impl crate::Readable for CR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr2::W](W) writer structure"]
impl crate::Writable for CR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR2 to value 0"]
impl crate::Resettable for CR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
