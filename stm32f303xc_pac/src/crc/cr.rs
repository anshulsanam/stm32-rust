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
#[doc = "Field `RESET` reader - reset bit"]
pub type RESET_R = crate::BitReader<RESETW_A>;
#[doc = "reset bit\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RESETW_A {
    #[doc = "1: Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF"]
    RESET = 1,
}
impl From<RESETW_A> for bool {
    #[inline(always)]
    fn from(variant: RESETW_A) -> Self {
        variant as u8 != 0
    }
}
impl RESET_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RESETW_A> {
        match self.bits {
            true => Some(RESETW_A::RESET),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == RESETW_A::RESET
    }
}
#[doc = "Field `RESET` writer - reset bit"]
pub type RESET_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, RESETW_A>;
impl<'a, const O: u8> RESET_W<'a, O> {
    #[doc = "Resets the CRC calculation unit and sets the data register to 0xFFFF FFFF"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(RESETW_A::RESET)
    }
}
#[doc = "Field `POLYSIZE` reader - Polynomial size"]
pub type POLYSIZE_R = crate::FieldReader<POLYSIZE_A>;
#[doc = "Polynomial size\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum POLYSIZE_A {
    #[doc = "0: 32-bit polynomial"]
    POLYSIZE32 = 0,
    #[doc = "1: 16-bit polynomial"]
    POLYSIZE16 = 1,
    #[doc = "2: 8-bit polynomial"]
    POLYSIZE8 = 2,
    #[doc = "3: 7-bit polynomial"]
    POLYSIZE7 = 3,
}
impl From<POLYSIZE_A> for u8 {
    #[inline(always)]
    fn from(variant: POLYSIZE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for POLYSIZE_A {
    type Ux = u8;
}
impl POLYSIZE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> POLYSIZE_A {
        match self.bits {
            0 => POLYSIZE_A::POLYSIZE32,
            1 => POLYSIZE_A::POLYSIZE16,
            2 => POLYSIZE_A::POLYSIZE8,
            3 => POLYSIZE_A::POLYSIZE7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `POLYSIZE32`"]
    #[inline(always)]
    pub fn is_polysize32(&self) -> bool {
        *self == POLYSIZE_A::POLYSIZE32
    }
    #[doc = "Checks if the value of the field is `POLYSIZE16`"]
    #[inline(always)]
    pub fn is_polysize16(&self) -> bool {
        *self == POLYSIZE_A::POLYSIZE16
    }
    #[doc = "Checks if the value of the field is `POLYSIZE8`"]
    #[inline(always)]
    pub fn is_polysize8(&self) -> bool {
        *self == POLYSIZE_A::POLYSIZE8
    }
    #[doc = "Checks if the value of the field is `POLYSIZE7`"]
    #[inline(always)]
    pub fn is_polysize7(&self) -> bool {
        *self == POLYSIZE_A::POLYSIZE7
    }
}
#[doc = "Field `POLYSIZE` writer - Polynomial size"]
pub type POLYSIZE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CR_SPEC, 2, O, POLYSIZE_A>;
impl<'a, const O: u8> POLYSIZE_W<'a, O> {
    #[doc = "32-bit polynomial"]
    #[inline(always)]
    pub fn polysize32(self) -> &'a mut W {
        self.variant(POLYSIZE_A::POLYSIZE32)
    }
    #[doc = "16-bit polynomial"]
    #[inline(always)]
    pub fn polysize16(self) -> &'a mut W {
        self.variant(POLYSIZE_A::POLYSIZE16)
    }
    #[doc = "8-bit polynomial"]
    #[inline(always)]
    pub fn polysize8(self) -> &'a mut W {
        self.variant(POLYSIZE_A::POLYSIZE8)
    }
    #[doc = "7-bit polynomial"]
    #[inline(always)]
    pub fn polysize7(self) -> &'a mut W {
        self.variant(POLYSIZE_A::POLYSIZE7)
    }
}
#[doc = "Field `REV_IN` reader - Reverse input data"]
pub type REV_IN_R = crate::FieldReader<REV_IN_A>;
#[doc = "Reverse input data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum REV_IN_A {
    #[doc = "0: Bit order not affected"]
    NORMAL = 0,
    #[doc = "1: Bit reversal done by byte"]
    BYTE = 1,
    #[doc = "2: Bit reversal done by half-word"]
    HALF_WORD = 2,
    #[doc = "3: Bit reversal done by word"]
    WORD = 3,
}
impl From<REV_IN_A> for u8 {
    #[inline(always)]
    fn from(variant: REV_IN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for REV_IN_A {
    type Ux = u8;
}
impl REV_IN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV_IN_A {
        match self.bits {
            0 => REV_IN_A::NORMAL,
            1 => REV_IN_A::BYTE,
            2 => REV_IN_A::HALF_WORD,
            3 => REV_IN_A::WORD,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == REV_IN_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `BYTE`"]
    #[inline(always)]
    pub fn is_byte(&self) -> bool {
        *self == REV_IN_A::BYTE
    }
    #[doc = "Checks if the value of the field is `HALF_WORD`"]
    #[inline(always)]
    pub fn is_half_word(&self) -> bool {
        *self == REV_IN_A::HALF_WORD
    }
    #[doc = "Checks if the value of the field is `WORD`"]
    #[inline(always)]
    pub fn is_word(&self) -> bool {
        *self == REV_IN_A::WORD
    }
}
#[doc = "Field `REV_IN` writer - Reverse input data"]
pub type REV_IN_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CR_SPEC, 2, O, REV_IN_A>;
impl<'a, const O: u8> REV_IN_W<'a, O> {
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(REV_IN_A::NORMAL)
    }
    #[doc = "Bit reversal done by byte"]
    #[inline(always)]
    pub fn byte(self) -> &'a mut W {
        self.variant(REV_IN_A::BYTE)
    }
    #[doc = "Bit reversal done by half-word"]
    #[inline(always)]
    pub fn half_word(self) -> &'a mut W {
        self.variant(REV_IN_A::HALF_WORD)
    }
    #[doc = "Bit reversal done by word"]
    #[inline(always)]
    pub fn word(self) -> &'a mut W {
        self.variant(REV_IN_A::WORD)
    }
}
#[doc = "Field `REV_OUT` reader - Reverse output data"]
pub type REV_OUT_R = crate::BitReader<REV_OUT_A>;
#[doc = "Reverse output data\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum REV_OUT_A {
    #[doc = "0: Bit order not affected"]
    NORMAL = 0,
    #[doc = "1: Bit reversed output"]
    REVERSED = 1,
}
impl From<REV_OUT_A> for bool {
    #[inline(always)]
    fn from(variant: REV_OUT_A) -> Self {
        variant as u8 != 0
    }
}
impl REV_OUT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> REV_OUT_A {
        match self.bits {
            false => REV_OUT_A::NORMAL,
            true => REV_OUT_A::REVERSED,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == REV_OUT_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `REVERSED`"]
    #[inline(always)]
    pub fn is_reversed(&self) -> bool {
        *self == REV_OUT_A::REVERSED
    }
}
#[doc = "Field `REV_OUT` writer - Reverse output data"]
pub type REV_OUT_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, REV_OUT_A>;
impl<'a, const O: u8> REV_OUT_W<'a, O> {
    #[doc = "Bit order not affected"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(REV_OUT_A::NORMAL)
    }
    #[doc = "Bit reversed output"]
    #[inline(always)]
    pub fn reversed(self) -> &'a mut W {
        self.variant(REV_OUT_A::REVERSED)
    }
}
impl R {
    #[doc = "Bit 0 - reset bit"]
    #[inline(always)]
    pub fn reset(&self) -> RESET_R {
        RESET_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 3:4 - Polynomial size"]
    #[inline(always)]
    pub fn polysize(&self) -> POLYSIZE_R {
        POLYSIZE_R::new(((self.bits >> 3) & 3) as u8)
    }
    #[doc = "Bits 5:6 - Reverse input data"]
    #[inline(always)]
    pub fn rev_in(&self) -> REV_IN_R {
        REV_IN_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - Reverse output data"]
    #[inline(always)]
    pub fn rev_out(&self) -> REV_OUT_R {
        REV_OUT_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - reset bit"]
    #[inline(always)]
    #[must_use]
    pub fn reset(&mut self) -> RESET_W<0> {
        RESET_W::new(self)
    }
    #[doc = "Bits 3:4 - Polynomial size"]
    #[inline(always)]
    #[must_use]
    pub fn polysize(&mut self) -> POLYSIZE_W<3> {
        POLYSIZE_W::new(self)
    }
    #[doc = "Bits 5:6 - Reverse input data"]
    #[inline(always)]
    #[must_use]
    pub fn rev_in(&mut self) -> REV_IN_W<5> {
        REV_IN_W::new(self)
    }
    #[doc = "Bit 7 - Reverse output data"]
    #[inline(always)]
    #[must_use]
    pub fn rev_out(&mut self) -> REV_OUT_W<7> {
        REV_OUT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
