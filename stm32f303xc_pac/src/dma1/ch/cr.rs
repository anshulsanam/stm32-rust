#[doc = "Register `CR` reader"]
pub struct R(crate::R<CR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CR` writer"]
pub struct W(crate::W<CR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CR_SPEC>;
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
impl From<crate::W<CR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EN` reader - Channel enable"]
pub type EN_R = crate::BitReader<EN_A>;
#[doc = "Channel enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EN_A {
    #[doc = "0: Channel disabled"]
    DISABLED = 0,
    #[doc = "1: Channel enabled"]
    ENABLED = 1,
}
impl From<EN_A> for bool {
    #[inline(always)]
    fn from(variant: EN_A) -> Self {
        variant as u8 != 0
    }
}
impl EN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> EN_A {
        match self.bits {
            false => EN_A::DISABLED,
            true => EN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EN_A::ENABLED
    }
}
#[doc = "Field `EN` writer - Channel enable"]
pub type EN_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, EN_A>;
impl<'a, const O: u8> EN_W<'a, O> {
    #[doc = "Channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(EN_A::DISABLED)
    }
    #[doc = "Channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(EN_A::ENABLED)
    }
}
#[doc = "Field `TCIE` reader - Transfer complete interrupt enable"]
pub type TCIE_R = crate::BitReader<TCIE_A>;
#[doc = "Transfer complete interrupt enable\n\nValue on reset: 0"]
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
#[doc = "Field `TCIE` writer - Transfer complete interrupt enable"]
pub type TCIE_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, TCIE_A>;
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
#[doc = "Field `HTIE` reader - Half Transfer interrupt enable"]
pub type HTIE_R = crate::BitReader<HTIE_A>;
#[doc = "Half Transfer interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HTIE_A {
    #[doc = "0: Half Transfer interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Half Transfer interrupt enabled"]
    ENABLED = 1,
}
impl From<HTIE_A> for bool {
    #[inline(always)]
    fn from(variant: HTIE_A) -> Self {
        variant as u8 != 0
    }
}
impl HTIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HTIE_A {
        match self.bits {
            false => HTIE_A::DISABLED,
            true => HTIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == HTIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == HTIE_A::ENABLED
    }
}
#[doc = "Field `HTIE` writer - Half Transfer interrupt enable"]
pub type HTIE_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, HTIE_A>;
impl<'a, const O: u8> HTIE_W<'a, O> {
    #[doc = "Half Transfer interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(HTIE_A::DISABLED)
    }
    #[doc = "Half Transfer interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(HTIE_A::ENABLED)
    }
}
#[doc = "Field `TEIE` reader - Transfer error interrupt enable"]
pub type TEIE_R = crate::BitReader<TEIE_A>;
#[doc = "Transfer error interrupt enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TEIE_A {
    #[doc = "0: Transfer Error interrupt disabled"]
    DISABLED = 0,
    #[doc = "1: Transfer Error interrupt enabled"]
    ENABLED = 1,
}
impl From<TEIE_A> for bool {
    #[inline(always)]
    fn from(variant: TEIE_A) -> Self {
        variant as u8 != 0
    }
}
impl TEIE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TEIE_A {
        match self.bits {
            false => TEIE_A::DISABLED,
            true => TEIE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TEIE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TEIE_A::ENABLED
    }
}
#[doc = "Field `TEIE` writer - Transfer error interrupt enable"]
pub type TEIE_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, TEIE_A>;
impl<'a, const O: u8> TEIE_W<'a, O> {
    #[doc = "Transfer Error interrupt disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TEIE_A::DISABLED)
    }
    #[doc = "Transfer Error interrupt enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TEIE_A::ENABLED)
    }
}
#[doc = "Field `DIR` reader - Data transfer direction"]
pub type DIR_R = crate::BitReader<DIR_A>;
#[doc = "Data transfer direction\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DIR_A {
    #[doc = "0: Read from peripheral"]
    FROM_PERIPHERAL = 0,
    #[doc = "1: Read from memory"]
    FROM_MEMORY = 1,
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
            false => DIR_A::FROM_PERIPHERAL,
            true => DIR_A::FROM_MEMORY,
        }
    }
    #[doc = "Checks if the value of the field is `FROM_PERIPHERAL`"]
    #[inline(always)]
    pub fn is_from_peripheral(&self) -> bool {
        *self == DIR_A::FROM_PERIPHERAL
    }
    #[doc = "Checks if the value of the field is `FROM_MEMORY`"]
    #[inline(always)]
    pub fn is_from_memory(&self) -> bool {
        *self == DIR_A::FROM_MEMORY
    }
}
#[doc = "Field `DIR` writer - Data transfer direction"]
pub type DIR_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, DIR_A>;
impl<'a, const O: u8> DIR_W<'a, O> {
    #[doc = "Read from peripheral"]
    #[inline(always)]
    pub fn from_peripheral(self) -> &'a mut W {
        self.variant(DIR_A::FROM_PERIPHERAL)
    }
    #[doc = "Read from memory"]
    #[inline(always)]
    pub fn from_memory(self) -> &'a mut W {
        self.variant(DIR_A::FROM_MEMORY)
    }
}
#[doc = "Field `CIRC` reader - Circular mode"]
pub type CIRC_R = crate::BitReader<CIRC_A>;
#[doc = "Circular mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CIRC_A {
    #[doc = "0: Circular buffer disabled"]
    DISABLED = 0,
    #[doc = "1: Circular buffer enabled"]
    ENABLED = 1,
}
impl From<CIRC_A> for bool {
    #[inline(always)]
    fn from(variant: CIRC_A) -> Self {
        variant as u8 != 0
    }
}
impl CIRC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CIRC_A {
        match self.bits {
            false => CIRC_A::DISABLED,
            true => CIRC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CIRC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CIRC_A::ENABLED
    }
}
#[doc = "Field `CIRC` writer - Circular mode"]
pub type CIRC_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, CIRC_A>;
impl<'a, const O: u8> CIRC_W<'a, O> {
    #[doc = "Circular buffer disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CIRC_A::DISABLED)
    }
    #[doc = "Circular buffer enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CIRC_A::ENABLED)
    }
}
#[doc = "Field `PINC` reader - Peripheral increment mode"]
pub type PINC_R = crate::BitReader<PINC_A>;
#[doc = "Peripheral increment mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PINC_A {
    #[doc = "0: Increment mode disabled"]
    DISABLED = 0,
    #[doc = "1: Increment mode enabled"]
    ENABLED = 1,
}
impl From<PINC_A> for bool {
    #[inline(always)]
    fn from(variant: PINC_A) -> Self {
        variant as u8 != 0
    }
}
impl PINC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PINC_A {
        match self.bits {
            false => PINC_A::DISABLED,
            true => PINC_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == PINC_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == PINC_A::ENABLED
    }
}
#[doc = "Field `PINC` writer - Peripheral increment mode"]
pub type PINC_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, PINC_A>;
impl<'a, const O: u8> PINC_W<'a, O> {
    #[doc = "Increment mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(PINC_A::DISABLED)
    }
    #[doc = "Increment mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(PINC_A::ENABLED)
    }
}
#[doc = "Field `MINC` reader - Memory increment mode"]
pub use PINC_R as MINC_R;
#[doc = "Field `MINC` writer - Memory increment mode"]
pub use PINC_W as MINC_W;
#[doc = "Field `PSIZE` reader - Peripheral size"]
pub type PSIZE_R = crate::FieldReader<PSIZE_A>;
#[doc = "Peripheral size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PSIZE_A {
    #[doc = "0: 8-bit size"]
    BITS8 = 0,
    #[doc = "1: 16-bit size"]
    BITS16 = 1,
    #[doc = "2: 32-bit size"]
    BITS32 = 2,
}
impl From<PSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: PSIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PSIZE_A {
    type Ux = u8;
}
impl PSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PSIZE_A> {
        match self.bits {
            0 => Some(PSIZE_A::BITS8),
            1 => Some(PSIZE_A::BITS16),
            2 => Some(PSIZE_A::BITS32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `BITS8`"]
    #[inline(always)]
    pub fn is_bits8(&self) -> bool {
        *self == PSIZE_A::BITS8
    }
    #[doc = "Checks if the value of the field is `BITS16`"]
    #[inline(always)]
    pub fn is_bits16(&self) -> bool {
        *self == PSIZE_A::BITS16
    }
    #[doc = "Checks if the value of the field is `BITS32`"]
    #[inline(always)]
    pub fn is_bits32(&self) -> bool {
        *self == PSIZE_A::BITS32
    }
}
#[doc = "Field `PSIZE` writer - Peripheral size"]
pub type PSIZE_W<'a, const O: u8> = crate::FieldWriter<'a, CR_SPEC, 2, O, PSIZE_A>;
impl<'a, const O: u8> PSIZE_W<'a, O> {
    #[doc = "8-bit size"]
    #[inline(always)]
    pub fn bits8(self) -> &'a mut W {
        self.variant(PSIZE_A::BITS8)
    }
    #[doc = "16-bit size"]
    #[inline(always)]
    pub fn bits16(self) -> &'a mut W {
        self.variant(PSIZE_A::BITS16)
    }
    #[doc = "32-bit size"]
    #[inline(always)]
    pub fn bits32(self) -> &'a mut W {
        self.variant(PSIZE_A::BITS32)
    }
}
#[doc = "Field `MSIZE` reader - Memory size"]
pub use PSIZE_R as MSIZE_R;
#[doc = "Field `MSIZE` writer - Memory size"]
pub use PSIZE_W as MSIZE_W;
#[doc = "Field `PL` reader - Channel Priority level"]
pub type PL_R = crate::FieldReader<PL_A>;
#[doc = "Channel Priority level\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PL_A {
    #[doc = "0: Low priority"]
    LOW = 0,
    #[doc = "1: Medium priority"]
    MEDIUM = 1,
    #[doc = "2: High priority"]
    HIGH = 2,
    #[doc = "3: Very high priority"]
    VERY_HIGH = 3,
}
impl From<PL_A> for u8 {
    #[inline(always)]
    fn from(variant: PL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for PL_A {
    type Ux = u8;
}
impl PL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PL_A {
        match self.bits {
            0 => PL_A::LOW,
            1 => PL_A::MEDIUM,
            2 => PL_A::HIGH,
            3 => PL_A::VERY_HIGH,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `LOW`"]
    #[inline(always)]
    pub fn is_low(&self) -> bool {
        *self == PL_A::LOW
    }
    #[doc = "Checks if the value of the field is `MEDIUM`"]
    #[inline(always)]
    pub fn is_medium(&self) -> bool {
        *self == PL_A::MEDIUM
    }
    #[doc = "Checks if the value of the field is `HIGH`"]
    #[inline(always)]
    pub fn is_high(&self) -> bool {
        *self == PL_A::HIGH
    }
    #[doc = "Checks if the value of the field is `VERY_HIGH`"]
    #[inline(always)]
    pub fn is_very_high(&self) -> bool {
        *self == PL_A::VERY_HIGH
    }
}
#[doc = "Field `PL` writer - Channel Priority level"]
pub type PL_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CR_SPEC, 2, O, PL_A>;
impl<'a, const O: u8> PL_W<'a, O> {
    #[doc = "Low priority"]
    #[inline(always)]
    pub fn low(self) -> &'a mut W {
        self.variant(PL_A::LOW)
    }
    #[doc = "Medium priority"]
    #[inline(always)]
    pub fn medium(self) -> &'a mut W {
        self.variant(PL_A::MEDIUM)
    }
    #[doc = "High priority"]
    #[inline(always)]
    pub fn high(self) -> &'a mut W {
        self.variant(PL_A::HIGH)
    }
    #[doc = "Very high priority"]
    #[inline(always)]
    pub fn very_high(self) -> &'a mut W {
        self.variant(PL_A::VERY_HIGH)
    }
}
#[doc = "Field `MEM2MEM` reader - Memory to memory mode"]
pub type MEM2MEM_R = crate::BitReader<MEM2MEM_A>;
#[doc = "Memory to memory mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum MEM2MEM_A {
    #[doc = "0: Memory to memory mode disabled"]
    DISABLED = 0,
    #[doc = "1: Memory to memory mode enabled"]
    ENABLED = 1,
}
impl From<MEM2MEM_A> for bool {
    #[inline(always)]
    fn from(variant: MEM2MEM_A) -> Self {
        variant as u8 != 0
    }
}
impl MEM2MEM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MEM2MEM_A {
        match self.bits {
            false => MEM2MEM_A::DISABLED,
            true => MEM2MEM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MEM2MEM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == MEM2MEM_A::ENABLED
    }
}
#[doc = "Field `MEM2MEM` writer - Memory to memory mode"]
pub type MEM2MEM_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, MEM2MEM_A>;
impl<'a, const O: u8> MEM2MEM_W<'a, O> {
    #[doc = "Memory to memory mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MEM2MEM_A::DISABLED)
    }
    #[doc = "Memory to memory mode enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(MEM2MEM_A::ENABLED)
    }
}
impl R {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable"]
    #[inline(always)]
    pub fn tcie(&self) -> TCIE_R {
        TCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Half Transfer interrupt enable"]
    #[inline(always)]
    pub fn htie(&self) -> HTIE_R {
        HTIE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable"]
    #[inline(always)]
    pub fn teie(&self) -> TEIE_R {
        TEIE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Data transfer direction"]
    #[inline(always)]
    pub fn dir(&self) -> DIR_R {
        DIR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Circular mode"]
    #[inline(always)]
    pub fn circ(&self) -> CIRC_R {
        CIRC_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Peripheral increment mode"]
    #[inline(always)]
    pub fn pinc(&self) -> PINC_R {
        PINC_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Memory increment mode"]
    #[inline(always)]
    pub fn minc(&self) -> MINC_R {
        MINC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:9 - Peripheral size"]
    #[inline(always)]
    pub fn psize(&self) -> PSIZE_R {
        PSIZE_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Memory size"]
    #[inline(always)]
    pub fn msize(&self) -> MSIZE_R {
        MSIZE_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - Channel Priority level"]
    #[inline(always)]
    pub fn pl(&self) -> PL_R {
        PL_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    pub fn mem2mem(&self) -> MEM2MEM_R {
        MEM2MEM_R::new(((self.bits >> 14) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Channel enable"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<0> {
        EN_W::new(self)
    }
    #[doc = "Bit 1 - Transfer complete interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn tcie(&mut self) -> TCIE_W<1> {
        TCIE_W::new(self)
    }
    #[doc = "Bit 2 - Half Transfer interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn htie(&mut self) -> HTIE_W<2> {
        HTIE_W::new(self)
    }
    #[doc = "Bit 3 - Transfer error interrupt enable"]
    #[inline(always)]
    #[must_use]
    pub fn teie(&mut self) -> TEIE_W<3> {
        TEIE_W::new(self)
    }
    #[doc = "Bit 4 - Data transfer direction"]
    #[inline(always)]
    #[must_use]
    pub fn dir(&mut self) -> DIR_W<4> {
        DIR_W::new(self)
    }
    #[doc = "Bit 5 - Circular mode"]
    #[inline(always)]
    #[must_use]
    pub fn circ(&mut self) -> CIRC_W<5> {
        CIRC_W::new(self)
    }
    #[doc = "Bit 6 - Peripheral increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn pinc(&mut self) -> PINC_W<6> {
        PINC_W::new(self)
    }
    #[doc = "Bit 7 - Memory increment mode"]
    #[inline(always)]
    #[must_use]
    pub fn minc(&mut self) -> MINC_W<7> {
        MINC_W::new(self)
    }
    #[doc = "Bits 8:9 - Peripheral size"]
    #[inline(always)]
    #[must_use]
    pub fn psize(&mut self) -> PSIZE_W<8> {
        PSIZE_W::new(self)
    }
    #[doc = "Bits 10:11 - Memory size"]
    #[inline(always)]
    #[must_use]
    pub fn msize(&mut self) -> MSIZE_W<10> {
        MSIZE_W::new(self)
    }
    #[doc = "Bits 12:13 - Channel Priority level"]
    #[inline(always)]
    #[must_use]
    pub fn pl(&mut self) -> PL_W<12> {
        PL_W::new(self)
    }
    #[doc = "Bit 14 - Memory to memory mode"]
    #[inline(always)]
    #[must_use]
    pub fn mem2mem(&mut self) -> MEM2MEM_W<14> {
        MEM2MEM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DMA channel configuration register (DMA_CCR)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
pub struct CR_SPEC;
impl crate::RegisterSpec for CR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cr::R](R) reader structure"]
impl crate::Readable for CR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cr::W](W) writer structure"]
impl crate::Writable for CR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CR to value 0"]
impl crate::Resettable for CR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
