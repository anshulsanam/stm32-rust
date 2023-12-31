#[doc = "Register `CFGR` reader"]
pub struct R(crate::R<CFGR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CFGR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CFGR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CFGR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CFGR` writer"]
pub struct W(crate::W<CFGR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CFGR_SPEC>;
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
impl From<crate::W<CFGR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CFGR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SW` reader - System clock Switch"]
pub type SW_R = crate::FieldReader<SW_A>;
#[doc = "System clock Switch\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SW_A {
    #[doc = "0: HSI selected as system clock"]
    HSI = 0,
    #[doc = "1: HSE selected as system clock"]
    HSE = 1,
    #[doc = "2: PLL selected as system clock"]
    PLL = 2,
}
impl From<SW_A> for u8 {
    #[inline(always)]
    fn from(variant: SW_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SW_A {
    type Ux = u8;
}
impl SW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SW_A> {
        match self.bits {
            0 => Some(SW_A::HSI),
            1 => Some(SW_A::HSE),
            2 => Some(SW_A::PLL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SW_A::HSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SW_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SW_A::PLL
    }
}
#[doc = "Field `SW` writer - System clock Switch"]
pub type SW_W<'a, const O: u8> = crate::FieldWriter<'a, CFGR_SPEC, 2, O, SW_A>;
impl<'a, const O: u8> SW_W<'a, O> {
    #[doc = "HSI selected as system clock"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(SW_A::HSI)
    }
    #[doc = "HSE selected as system clock"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(SW_A::HSE)
    }
    #[doc = "PLL selected as system clock"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(SW_A::PLL)
    }
}
#[doc = "Field `SWS` reader - System Clock Switch Status"]
pub type SWS_R = crate::FieldReader<SWSR_A>;
#[doc = "System Clock Switch Status\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SWSR_A {
    #[doc = "0: HSI oscillator used as system clock"]
    HSI = 0,
    #[doc = "1: HSE oscillator used as system clock"]
    HSE = 1,
    #[doc = "2: PLL used as system clock"]
    PLL = 2,
}
impl From<SWSR_A> for u8 {
    #[inline(always)]
    fn from(variant: SWSR_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SWSR_A {
    type Ux = u8;
}
impl SWS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SWSR_A> {
        match self.bits {
            0 => Some(SWSR_A::HSI),
            1 => Some(SWSR_A::HSE),
            2 => Some(SWSR_A::PLL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == SWSR_A::HSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == SWSR_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == SWSR_A::PLL
    }
}
#[doc = "Field `HPRE` reader - AHB prescaler"]
pub type HPRE_R = crate::FieldReader<HPRE_A>;
#[doc = "AHB prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum HPRE_A {
    #[doc = "0: SYSCLK not divided"]
    DIV1 = 0,
    #[doc = "8: SYSCLK divided by 2"]
    DIV2 = 8,
    #[doc = "9: SYSCLK divided by 4"]
    DIV4 = 9,
    #[doc = "10: SYSCLK divided by 8"]
    DIV8 = 10,
    #[doc = "11: SYSCLK divided by 16"]
    DIV16 = 11,
    #[doc = "12: SYSCLK divided by 64"]
    DIV64 = 12,
    #[doc = "13: SYSCLK divided by 128"]
    DIV128 = 13,
    #[doc = "14: SYSCLK divided by 256"]
    DIV256 = 14,
    #[doc = "15: SYSCLK divided by 512"]
    DIV512 = 15,
}
impl From<HPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: HPRE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for HPRE_A {
    type Ux = u8;
}
impl HPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<HPRE_A> {
        match self.bits {
            0 => Some(HPRE_A::DIV1),
            8 => Some(HPRE_A::DIV2),
            9 => Some(HPRE_A::DIV4),
            10 => Some(HPRE_A::DIV8),
            11 => Some(HPRE_A::DIV16),
            12 => Some(HPRE_A::DIV64),
            13 => Some(HPRE_A::DIV128),
            14 => Some(HPRE_A::DIV256),
            15 => Some(HPRE_A::DIV512),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == HPRE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == HPRE_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == HPRE_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == HPRE_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == HPRE_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == HPRE_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == HPRE_A::DIV128
    }
    #[doc = "Checks if the value of the field is `DIV256`"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == HPRE_A::DIV256
    }
    #[doc = "Checks if the value of the field is `DIV512`"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == HPRE_A::DIV512
    }
}
#[doc = "Field `HPRE` writer - AHB prescaler"]
pub type HPRE_W<'a, const O: u8> = crate::FieldWriter<'a, CFGR_SPEC, 4, O, HPRE_A>;
impl<'a, const O: u8> HPRE_W<'a, O> {
    #[doc = "SYSCLK not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(HPRE_A::DIV1)
    }
    #[doc = "SYSCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(HPRE_A::DIV2)
    }
    #[doc = "SYSCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(HPRE_A::DIV4)
    }
    #[doc = "SYSCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(HPRE_A::DIV8)
    }
    #[doc = "SYSCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(HPRE_A::DIV16)
    }
    #[doc = "SYSCLK divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(HPRE_A::DIV64)
    }
    #[doc = "SYSCLK divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(HPRE_A::DIV128)
    }
    #[doc = "SYSCLK divided by 256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut W {
        self.variant(HPRE_A::DIV256)
    }
    #[doc = "SYSCLK divided by 512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut W {
        self.variant(HPRE_A::DIV512)
    }
}
#[doc = "Field `PPRE1` reader - APB Low speed prescaler (APB1)"]
pub type PPRE1_R = crate::FieldReader<PPRE1_A>;
#[doc = "APB Low speed prescaler (APB1)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PPRE1_A {
    #[doc = "0: HCLK not divided"]
    DIV1 = 0,
    #[doc = "4: HCLK divided by 2"]
    DIV2 = 4,
    #[doc = "5: HCLK divided by 4"]
    DIV4 = 5,
    #[doc = "6: HCLK divided by 8"]
    DIV8 = 6,
    #[doc = "7: HCLK divided by 16"]
    DIV16 = 7,
}
impl From<PPRE1_A> for u8 {
    #[inline(always)]
    fn from(variant: PPRE1_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PPRE1_A {
    type Ux = u8;
}
impl PPRE1_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PPRE1_A> {
        match self.bits {
            0 => Some(PPRE1_A::DIV1),
            4 => Some(PPRE1_A::DIV2),
            5 => Some(PPRE1_A::DIV4),
            6 => Some(PPRE1_A::DIV8),
            7 => Some(PPRE1_A::DIV16),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PPRE1_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PPRE1_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == PPRE1_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == PPRE1_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == PPRE1_A::DIV16
    }
}
#[doc = "Field `PPRE1` writer - APB Low speed prescaler (APB1)"]
pub type PPRE1_W<'a, const O: u8> = crate::FieldWriter<'a, CFGR_SPEC, 3, O, PPRE1_A>;
impl<'a, const O: u8> PPRE1_W<'a, O> {
    #[doc = "HCLK not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PPRE1_A::DIV1)
    }
    #[doc = "HCLK divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PPRE1_A::DIV2)
    }
    #[doc = "HCLK divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(PPRE1_A::DIV4)
    }
    #[doc = "HCLK divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(PPRE1_A::DIV8)
    }
    #[doc = "HCLK divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(PPRE1_A::DIV16)
    }
}
#[doc = "Field `PPRE2` reader - APB high speed prescaler (APB2)"]
pub use PPRE1_R as PPRE2_R;
#[doc = "Field `PPRE2` writer - APB high speed prescaler (APB2)"]
pub use PPRE1_W as PPRE2_W;
#[doc = "Field `PLLSRC` reader - PLL entry clock source"]
pub type PLLSRC_R = crate::FieldReader<PLLSRC_A>;
#[doc = "PLL entry clock source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLSRC_A {
    #[doc = "0: HSI divided by 2 selected as PLL input clock"]
    HSI_DIV2 = 0,
    #[doc = "1: HSI divided by PREDIV selected as PLL input clock"]
    HSI_DIV_PREDIV = 1,
    #[doc = "2: HSE divided by PREDIV selected as PLL input clock"]
    HSE_DIV_PREDIV = 2,
}
impl From<PLLSRC_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLSRC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLSRC_A {
    type Ux = u8;
}
impl PLLSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PLLSRC_A> {
        match self.bits {
            0 => Some(PLLSRC_A::HSI_DIV2),
            1 => Some(PLLSRC_A::HSI_DIV_PREDIV),
            2 => Some(PLLSRC_A::HSE_DIV_PREDIV),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `HSI_DIV2`"]
    #[inline(always)]
    pub fn is_hsi_div2(&self) -> bool {
        *self == PLLSRC_A::HSI_DIV2
    }
    #[doc = "Checks if the value of the field is `HSI_DIV_PREDIV`"]
    #[inline(always)]
    pub fn is_hsi_div_prediv(&self) -> bool {
        *self == PLLSRC_A::HSI_DIV_PREDIV
    }
    #[doc = "Checks if the value of the field is `HSE_DIV_PREDIV`"]
    #[inline(always)]
    pub fn is_hse_div_prediv(&self) -> bool {
        *self == PLLSRC_A::HSE_DIV_PREDIV
    }
}
#[doc = "Field `PLLSRC` writer - PLL entry clock source"]
pub type PLLSRC_W<'a, const O: u8> = crate::FieldWriter<'a, CFGR_SPEC, 2, O, PLLSRC_A>;
impl<'a, const O: u8> PLLSRC_W<'a, O> {
    #[doc = "HSI divided by 2 selected as PLL input clock"]
    #[inline(always)]
    pub fn hsi_div2(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSI_DIV2)
    }
    #[doc = "HSI divided by PREDIV selected as PLL input clock"]
    #[inline(always)]
    pub fn hsi_div_prediv(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSI_DIV_PREDIV)
    }
    #[doc = "HSE divided by PREDIV selected as PLL input clock"]
    #[inline(always)]
    pub fn hse_div_prediv(self) -> &'a mut W {
        self.variant(PLLSRC_A::HSE_DIV_PREDIV)
    }
}
#[doc = "Field `PLLXTPRE` reader - HSE divider for PLL entry"]
pub type PLLXTPRE_R = crate::BitReader<PLLXTPRE_A>;
#[doc = "HSE divider for PLL entry\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLXTPRE_A {
    #[doc = "0: HSE clock not divided"]
    DIV1 = 0,
    #[doc = "1: HSE clock divided by 2"]
    DIV2 = 1,
}
impl From<PLLXTPRE_A> for bool {
    #[inline(always)]
    fn from(variant: PLLXTPRE_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLXTPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLXTPRE_A {
        match self.bits {
            false => PLLXTPRE_A::DIV1,
            true => PLLXTPRE_A::DIV2,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLXTPRE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLXTPRE_A::DIV2
    }
}
#[doc = "Field `PLLXTPRE` writer - HSE divider for PLL entry"]
pub type PLLXTPRE_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O, PLLXTPRE_A>;
impl<'a, const O: u8> PLLXTPRE_W<'a, O> {
    #[doc = "HSE clock not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLXTPRE_A::DIV1)
    }
    #[doc = "HSE clock divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLXTPRE_A::DIV2)
    }
}
#[doc = "Field `PLLMUL` reader - PLL Multiplication Factor"]
pub type PLLMUL_R = crate::FieldReader<PLLMUL_A>;
#[doc = "PLL Multiplication Factor\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PLLMUL_A {
    #[doc = "0: PLL input clock x2"]
    MUL2 = 0,
    #[doc = "1: PLL input clock x3"]
    MUL3 = 1,
    #[doc = "2: PLL input clock x4"]
    MUL4 = 2,
    #[doc = "3: PLL input clock x5"]
    MUL5 = 3,
    #[doc = "4: PLL input clock x6"]
    MUL6 = 4,
    #[doc = "5: PLL input clock x7"]
    MUL7 = 5,
    #[doc = "6: PLL input clock x8"]
    MUL8 = 6,
    #[doc = "7: PLL input clock x9"]
    MUL9 = 7,
    #[doc = "8: PLL input clock x10"]
    MUL10 = 8,
    #[doc = "9: PLL input clock x11"]
    MUL11 = 9,
    #[doc = "10: PLL input clock x12"]
    MUL12 = 10,
    #[doc = "11: PLL input clock x13"]
    MUL13 = 11,
    #[doc = "12: PLL input clock x14"]
    MUL14 = 12,
    #[doc = "13: PLL input clock x15"]
    MUL15 = 13,
    #[doc = "14: PLL input clock x16"]
    MUL16 = 14,
    #[doc = "15: PLL input clock x16"]
    MUL16X = 15,
}
impl From<PLLMUL_A> for u8 {
    #[inline(always)]
    fn from(variant: PLLMUL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PLLMUL_A {
    type Ux = u8;
}
impl PLLMUL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLMUL_A {
        match self.bits {
            0 => PLLMUL_A::MUL2,
            1 => PLLMUL_A::MUL3,
            2 => PLLMUL_A::MUL4,
            3 => PLLMUL_A::MUL5,
            4 => PLLMUL_A::MUL6,
            5 => PLLMUL_A::MUL7,
            6 => PLLMUL_A::MUL8,
            7 => PLLMUL_A::MUL9,
            8 => PLLMUL_A::MUL10,
            9 => PLLMUL_A::MUL11,
            10 => PLLMUL_A::MUL12,
            11 => PLLMUL_A::MUL13,
            12 => PLLMUL_A::MUL14,
            13 => PLLMUL_A::MUL15,
            14 => PLLMUL_A::MUL16,
            15 => PLLMUL_A::MUL16X,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `MUL2`"]
    #[inline(always)]
    pub fn is_mul2(&self) -> bool {
        *self == PLLMUL_A::MUL2
    }
    #[doc = "Checks if the value of the field is `MUL3`"]
    #[inline(always)]
    pub fn is_mul3(&self) -> bool {
        *self == PLLMUL_A::MUL3
    }
    #[doc = "Checks if the value of the field is `MUL4`"]
    #[inline(always)]
    pub fn is_mul4(&self) -> bool {
        *self == PLLMUL_A::MUL4
    }
    #[doc = "Checks if the value of the field is `MUL5`"]
    #[inline(always)]
    pub fn is_mul5(&self) -> bool {
        *self == PLLMUL_A::MUL5
    }
    #[doc = "Checks if the value of the field is `MUL6`"]
    #[inline(always)]
    pub fn is_mul6(&self) -> bool {
        *self == PLLMUL_A::MUL6
    }
    #[doc = "Checks if the value of the field is `MUL7`"]
    #[inline(always)]
    pub fn is_mul7(&self) -> bool {
        *self == PLLMUL_A::MUL7
    }
    #[doc = "Checks if the value of the field is `MUL8`"]
    #[inline(always)]
    pub fn is_mul8(&self) -> bool {
        *self == PLLMUL_A::MUL8
    }
    #[doc = "Checks if the value of the field is `MUL9`"]
    #[inline(always)]
    pub fn is_mul9(&self) -> bool {
        *self == PLLMUL_A::MUL9
    }
    #[doc = "Checks if the value of the field is `MUL10`"]
    #[inline(always)]
    pub fn is_mul10(&self) -> bool {
        *self == PLLMUL_A::MUL10
    }
    #[doc = "Checks if the value of the field is `MUL11`"]
    #[inline(always)]
    pub fn is_mul11(&self) -> bool {
        *self == PLLMUL_A::MUL11
    }
    #[doc = "Checks if the value of the field is `MUL12`"]
    #[inline(always)]
    pub fn is_mul12(&self) -> bool {
        *self == PLLMUL_A::MUL12
    }
    #[doc = "Checks if the value of the field is `MUL13`"]
    #[inline(always)]
    pub fn is_mul13(&self) -> bool {
        *self == PLLMUL_A::MUL13
    }
    #[doc = "Checks if the value of the field is `MUL14`"]
    #[inline(always)]
    pub fn is_mul14(&self) -> bool {
        *self == PLLMUL_A::MUL14
    }
    #[doc = "Checks if the value of the field is `MUL15`"]
    #[inline(always)]
    pub fn is_mul15(&self) -> bool {
        *self == PLLMUL_A::MUL15
    }
    #[doc = "Checks if the value of the field is `MUL16`"]
    #[inline(always)]
    pub fn is_mul16(&self) -> bool {
        *self == PLLMUL_A::MUL16
    }
    #[doc = "Checks if the value of the field is `MUL16X`"]
    #[inline(always)]
    pub fn is_mul16x(&self) -> bool {
        *self == PLLMUL_A::MUL16X
    }
}
#[doc = "Field `PLLMUL` writer - PLL Multiplication Factor"]
pub type PLLMUL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CFGR_SPEC, 4, O, PLLMUL_A>;
impl<'a, const O: u8> PLLMUL_W<'a, O> {
    #[doc = "PLL input clock x2"]
    #[inline(always)]
    pub fn mul2(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL2)
    }
    #[doc = "PLL input clock x3"]
    #[inline(always)]
    pub fn mul3(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL3)
    }
    #[doc = "PLL input clock x4"]
    #[inline(always)]
    pub fn mul4(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL4)
    }
    #[doc = "PLL input clock x5"]
    #[inline(always)]
    pub fn mul5(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL5)
    }
    #[doc = "PLL input clock x6"]
    #[inline(always)]
    pub fn mul6(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL6)
    }
    #[doc = "PLL input clock x7"]
    #[inline(always)]
    pub fn mul7(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL7)
    }
    #[doc = "PLL input clock x8"]
    #[inline(always)]
    pub fn mul8(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL8)
    }
    #[doc = "PLL input clock x9"]
    #[inline(always)]
    pub fn mul9(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL9)
    }
    #[doc = "PLL input clock x10"]
    #[inline(always)]
    pub fn mul10(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL10)
    }
    #[doc = "PLL input clock x11"]
    #[inline(always)]
    pub fn mul11(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL11)
    }
    #[doc = "PLL input clock x12"]
    #[inline(always)]
    pub fn mul12(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL12)
    }
    #[doc = "PLL input clock x13"]
    #[inline(always)]
    pub fn mul13(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL13)
    }
    #[doc = "PLL input clock x14"]
    #[inline(always)]
    pub fn mul14(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL14)
    }
    #[doc = "PLL input clock x15"]
    #[inline(always)]
    pub fn mul15(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL15)
    }
    #[doc = "PLL input clock x16"]
    #[inline(always)]
    pub fn mul16(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL16)
    }
    #[doc = "PLL input clock x16"]
    #[inline(always)]
    pub fn mul16x(self) -> &'a mut W {
        self.variant(PLLMUL_A::MUL16X)
    }
}
#[doc = "Field `USBPRE` reader - USB prescaler"]
pub type USBPRE_R = crate::BitReader<USBPRE_A>;
#[doc = "USB prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum USBPRE_A {
    #[doc = "0: PLL clock is divided by 1.5"]
    DIV1_5 = 0,
    #[doc = "1: PLL clock is not divided"]
    DIV1 = 1,
}
impl From<USBPRE_A> for bool {
    #[inline(always)]
    fn from(variant: USBPRE_A) -> Self {
        variant as u8 != 0
    }
}
impl USBPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> USBPRE_A {
        match self.bits {
            false => USBPRE_A::DIV1_5,
            true => USBPRE_A::DIV1,
        }
    }
    #[doc = "Checks if the value of the field is `DIV1_5`"]
    #[inline(always)]
    pub fn is_div1_5(&self) -> bool {
        *self == USBPRE_A::DIV1_5
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == USBPRE_A::DIV1
    }
}
#[doc = "Field `USBPRE` writer - USB prescaler"]
pub type USBPRE_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O, USBPRE_A>;
impl<'a, const O: u8> USBPRE_W<'a, O> {
    #[doc = "PLL clock is divided by 1.5"]
    #[inline(always)]
    pub fn div1_5(self) -> &'a mut W {
        self.variant(USBPRE_A::DIV1_5)
    }
    #[doc = "PLL clock is not divided"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(USBPRE_A::DIV1)
    }
}
#[doc = "Field `I2SSRC` reader - I2S external clock source selection"]
pub type I2SSRC_R = crate::BitReader<I2SSRC_A>;
#[doc = "I2S external clock source selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum I2SSRC_A {
    #[doc = "0: System clock used as I2S clock source"]
    SYSCLK = 0,
    #[doc = "1: External clock mapped on the I2S_CKIN pin used as I2S clock source"]
    CKIN = 1,
}
impl From<I2SSRC_A> for bool {
    #[inline(always)]
    fn from(variant: I2SSRC_A) -> Self {
        variant as u8 != 0
    }
}
impl I2SSRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> I2SSRC_A {
        match self.bits {
            false => I2SSRC_A::SYSCLK,
            true => I2SSRC_A::CKIN,
        }
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == I2SSRC_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `CKIN`"]
    #[inline(always)]
    pub fn is_ckin(&self) -> bool {
        *self == I2SSRC_A::CKIN
    }
}
#[doc = "Field `I2SSRC` writer - I2S external clock source selection"]
pub type I2SSRC_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O, I2SSRC_A>;
impl<'a, const O: u8> I2SSRC_W<'a, O> {
    #[doc = "System clock used as I2S clock source"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(I2SSRC_A::SYSCLK)
    }
    #[doc = "External clock mapped on the I2S_CKIN pin used as I2S clock source"]
    #[inline(always)]
    pub fn ckin(self) -> &'a mut W {
        self.variant(I2SSRC_A::CKIN)
    }
}
#[doc = "Field `MCO` reader - Microcontroller clock output"]
pub type MCO_R = crate::FieldReader<MCO_A>;
#[doc = "Microcontroller clock output\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCO_A {
    #[doc = "0: MCO output disabled, no clock on MCO"]
    NO_MCO = 0,
    #[doc = "2: Internal low speed (LSI) oscillator clock selected"]
    LSI = 2,
    #[doc = "3: External low speed (LSE) oscillator clock selected"]
    LSE = 3,
    #[doc = "4: System clock selected"]
    SYSCLK = 4,
    #[doc = "5: Internal RC 8 MHz (HSI) oscillator clock selected"]
    HSI = 5,
    #[doc = "6: External 4-32 MHz (HSE) oscillator clock selected"]
    HSE = 6,
    #[doc = "7: PLL clock selected (divided by 1 or 2, depending en PLLNODIV)"]
    PLL = 7,
}
impl From<MCO_A> for u8 {
    #[inline(always)]
    fn from(variant: MCO_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCO_A {
    type Ux = u8;
}
impl MCO_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MCO_A> {
        match self.bits {
            0 => Some(MCO_A::NO_MCO),
            2 => Some(MCO_A::LSI),
            3 => Some(MCO_A::LSE),
            4 => Some(MCO_A::SYSCLK),
            5 => Some(MCO_A::HSI),
            6 => Some(MCO_A::HSE),
            7 => Some(MCO_A::PLL),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `NO_MCO`"]
    #[inline(always)]
    pub fn is_no_mco(&self) -> bool {
        *self == MCO_A::NO_MCO
    }
    #[doc = "Checks if the value of the field is `LSI`"]
    #[inline(always)]
    pub fn is_lsi(&self) -> bool {
        *self == MCO_A::LSI
    }
    #[doc = "Checks if the value of the field is `LSE`"]
    #[inline(always)]
    pub fn is_lse(&self) -> bool {
        *self == MCO_A::LSE
    }
    #[doc = "Checks if the value of the field is `SYSCLK`"]
    #[inline(always)]
    pub fn is_sysclk(&self) -> bool {
        *self == MCO_A::SYSCLK
    }
    #[doc = "Checks if the value of the field is `HSI`"]
    #[inline(always)]
    pub fn is_hsi(&self) -> bool {
        *self == MCO_A::HSI
    }
    #[doc = "Checks if the value of the field is `HSE`"]
    #[inline(always)]
    pub fn is_hse(&self) -> bool {
        *self == MCO_A::HSE
    }
    #[doc = "Checks if the value of the field is `PLL`"]
    #[inline(always)]
    pub fn is_pll(&self) -> bool {
        *self == MCO_A::PLL
    }
}
#[doc = "Field `MCO` writer - Microcontroller clock output"]
pub type MCO_W<'a, const O: u8> = crate::FieldWriter<'a, CFGR_SPEC, 3, O, MCO_A>;
impl<'a, const O: u8> MCO_W<'a, O> {
    #[doc = "MCO output disabled, no clock on MCO"]
    #[inline(always)]
    pub fn no_mco(self) -> &'a mut W {
        self.variant(MCO_A::NO_MCO)
    }
    #[doc = "Internal low speed (LSI) oscillator clock selected"]
    #[inline(always)]
    pub fn lsi(self) -> &'a mut W {
        self.variant(MCO_A::LSI)
    }
    #[doc = "External low speed (LSE) oscillator clock selected"]
    #[inline(always)]
    pub fn lse(self) -> &'a mut W {
        self.variant(MCO_A::LSE)
    }
    #[doc = "System clock selected"]
    #[inline(always)]
    pub fn sysclk(self) -> &'a mut W {
        self.variant(MCO_A::SYSCLK)
    }
    #[doc = "Internal RC 8 MHz (HSI) oscillator clock selected"]
    #[inline(always)]
    pub fn hsi(self) -> &'a mut W {
        self.variant(MCO_A::HSI)
    }
    #[doc = "External 4-32 MHz (HSE) oscillator clock selected"]
    #[inline(always)]
    pub fn hse(self) -> &'a mut W {
        self.variant(MCO_A::HSE)
    }
    #[doc = "PLL clock selected (divided by 1 or 2, depending en PLLNODIV)"]
    #[inline(always)]
    pub fn pll(self) -> &'a mut W {
        self.variant(MCO_A::PLL)
    }
}
#[doc = "Field `MCOPRE` reader - Microcontroller Clock Output Prescaler"]
pub type MCOPRE_R = crate::FieldReader<MCOPRE_A>;
#[doc = "Microcontroller Clock Output Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MCOPRE_A {
    #[doc = "0: MCO is divided by 1"]
    DIV1 = 0,
    #[doc = "1: MCO is divided by 2"]
    DIV2 = 1,
    #[doc = "2: MCO is divided by 4"]
    DIV4 = 2,
    #[doc = "3: MCO is divided by 8"]
    DIV8 = 3,
    #[doc = "4: MCO is divided by 16"]
    DIV16 = 4,
    #[doc = "5: MCO is divided by 32"]
    DIV32 = 5,
    #[doc = "6: MCO is divided by 64"]
    DIV64 = 6,
    #[doc = "7: MCO is divided by 128"]
    DIV128 = 7,
}
impl From<MCOPRE_A> for u8 {
    #[inline(always)]
    fn from(variant: MCOPRE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MCOPRE_A {
    type Ux = u8;
}
impl MCOPRE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MCOPRE_A {
        match self.bits {
            0 => MCOPRE_A::DIV1,
            1 => MCOPRE_A::DIV2,
            2 => MCOPRE_A::DIV4,
            3 => MCOPRE_A::DIV8,
            4 => MCOPRE_A::DIV16,
            5 => MCOPRE_A::DIV32,
            6 => MCOPRE_A::DIV64,
            7 => MCOPRE_A::DIV128,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == MCOPRE_A::DIV1
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == MCOPRE_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV4`"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == MCOPRE_A::DIV4
    }
    #[doc = "Checks if the value of the field is `DIV8`"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == MCOPRE_A::DIV8
    }
    #[doc = "Checks if the value of the field is `DIV16`"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == MCOPRE_A::DIV16
    }
    #[doc = "Checks if the value of the field is `DIV32`"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == MCOPRE_A::DIV32
    }
    #[doc = "Checks if the value of the field is `DIV64`"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == MCOPRE_A::DIV64
    }
    #[doc = "Checks if the value of the field is `DIV128`"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == MCOPRE_A::DIV128
    }
}
#[doc = "Field `MCOPRE` writer - Microcontroller Clock Output Prescaler"]
pub type MCOPRE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CFGR_SPEC, 3, O, MCOPRE_A>;
impl<'a, const O: u8> MCOPRE_W<'a, O> {
    #[doc = "MCO is divided by 1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV1)
    }
    #[doc = "MCO is divided by 2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV2)
    }
    #[doc = "MCO is divided by 4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV4)
    }
    #[doc = "MCO is divided by 8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV8)
    }
    #[doc = "MCO is divided by 16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV16)
    }
    #[doc = "MCO is divided by 32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV32)
    }
    #[doc = "MCO is divided by 64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV64)
    }
    #[doc = "MCO is divided by 128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut W {
        self.variant(MCOPRE_A::DIV128)
    }
}
#[doc = "Field `PLLNODIV` reader - Do not divide PLL to MCO"]
pub type PLLNODIV_R = crate::BitReader<PLLNODIV_A>;
#[doc = "Do not divide PLL to MCO\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PLLNODIV_A {
    #[doc = "0: PLL is divided by 2 for MCO"]
    DIV2 = 0,
    #[doc = "1: PLL is not divided for MCO"]
    DIV1 = 1,
}
impl From<PLLNODIV_A> for bool {
    #[inline(always)]
    fn from(variant: PLLNODIV_A) -> Self {
        variant as u8 != 0
    }
}
impl PLLNODIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PLLNODIV_A {
        match self.bits {
            false => PLLNODIV_A::DIV2,
            true => PLLNODIV_A::DIV1,
        }
    }
    #[doc = "Checks if the value of the field is `DIV2`"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == PLLNODIV_A::DIV2
    }
    #[doc = "Checks if the value of the field is `DIV1`"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == PLLNODIV_A::DIV1
    }
}
#[doc = "Field `PLLNODIV` writer - Do not divide PLL to MCO"]
pub type PLLNODIV_W<'a, const O: u8> = crate::BitWriter<'a, CFGR_SPEC, O, PLLNODIV_A>;
impl<'a, const O: u8> PLLNODIV_W<'a, O> {
    #[doc = "PLL is divided by 2 for MCO"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut W {
        self.variant(PLLNODIV_A::DIV2)
    }
    #[doc = "PLL is not divided for MCO"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut W {
        self.variant(PLLNODIV_A::DIV1)
    }
}
impl R {
    #[doc = "Bits 0:1 - System clock Switch"]
    #[inline(always)]
    pub fn sw(&self) -> SW_R {
        SW_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - System Clock Switch Status"]
    #[inline(always)]
    pub fn sws(&self) -> SWS_R {
        SWS_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    pub fn hpre(&self) -> HPRE_R {
        HPRE_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    pub fn ppre1(&self) -> PPRE1_R {
        PPRE1_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 11:13 - APB high speed prescaler (APB2)"]
    #[inline(always)]
    pub fn ppre2(&self) -> PPRE2_R {
        PPRE2_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bits 15:16 - PLL entry clock source"]
    #[inline(always)]
    pub fn pllsrc(&self) -> PLLSRC_R {
        PLLSRC_R::new(((self.bits >> 15) & 3) as u8)
    }
    #[doc = "Bit 17 - HSE divider for PLL entry"]
    #[inline(always)]
    pub fn pllxtpre(&self) -> PLLXTPRE_R {
        PLLXTPRE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor"]
    #[inline(always)]
    pub fn pllmul(&self) -> PLLMUL_R {
        PLLMUL_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bit 22 - USB prescaler"]
    #[inline(always)]
    pub fn usbpre(&self) -> USBPRE_R {
        USBPRE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - I2S external clock source selection"]
    #[inline(always)]
    pub fn i2ssrc(&self) -> I2SSRC_R {
        I2SSRC_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline(always)]
    pub fn mco(&self) -> MCO_R {
        MCO_R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bits 28:30 - Microcontroller Clock Output Prescaler"]
    #[inline(always)]
    pub fn mcopre(&self) -> MCOPRE_R {
        MCOPRE_R::new(((self.bits >> 28) & 7) as u8)
    }
    #[doc = "Bit 31 - Do not divide PLL to MCO"]
    #[inline(always)]
    pub fn pllnodiv(&self) -> PLLNODIV_R {
        PLLNODIV_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:1 - System clock Switch"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<0> {
        SW_W::new(self)
    }
    #[doc = "Bits 4:7 - AHB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn hpre(&mut self) -> HPRE_W<4> {
        HPRE_W::new(self)
    }
    #[doc = "Bits 8:10 - APB Low speed prescaler (APB1)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre1(&mut self) -> PPRE1_W<8> {
        PPRE1_W::new(self)
    }
    #[doc = "Bits 11:13 - APB high speed prescaler (APB2)"]
    #[inline(always)]
    #[must_use]
    pub fn ppre2(&mut self) -> PPRE2_W<11> {
        PPRE2_W::new(self)
    }
    #[doc = "Bits 15:16 - PLL entry clock source"]
    #[inline(always)]
    #[must_use]
    pub fn pllsrc(&mut self) -> PLLSRC_W<15> {
        PLLSRC_W::new(self)
    }
    #[doc = "Bit 17 - HSE divider for PLL entry"]
    #[inline(always)]
    #[must_use]
    pub fn pllxtpre(&mut self) -> PLLXTPRE_W<17> {
        PLLXTPRE_W::new(self)
    }
    #[doc = "Bits 18:21 - PLL Multiplication Factor"]
    #[inline(always)]
    #[must_use]
    pub fn pllmul(&mut self) -> PLLMUL_W<18> {
        PLLMUL_W::new(self)
    }
    #[doc = "Bit 22 - USB prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn usbpre(&mut self) -> USBPRE_W<22> {
        USBPRE_W::new(self)
    }
    #[doc = "Bit 23 - I2S external clock source selection"]
    #[inline(always)]
    #[must_use]
    pub fn i2ssrc(&mut self) -> I2SSRC_W<23> {
        I2SSRC_W::new(self)
    }
    #[doc = "Bits 24:26 - Microcontroller clock output"]
    #[inline(always)]
    #[must_use]
    pub fn mco(&mut self) -> MCO_W<24> {
        MCO_W::new(self)
    }
    #[doc = "Bits 28:30 - Microcontroller Clock Output Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn mcopre(&mut self) -> MCOPRE_W<28> {
        MCOPRE_W::new(self)
    }
    #[doc = "Bit 31 - Do not divide PLL to MCO"]
    #[inline(always)]
    #[must_use]
    pub fn pllnodiv(&mut self) -> PLLNODIV_W<31> {
        PLLNODIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock configuration register (RCC_CFGR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cfgr](index.html) module"]
pub struct CFGR_SPEC;
impl crate::RegisterSpec for CFGR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cfgr::R](R) reader structure"]
impl crate::Readable for CFGR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cfgr::W](W) writer structure"]
impl crate::Writable for CFGR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CFGR to value 0"]
impl crate::Resettable for CFGR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
