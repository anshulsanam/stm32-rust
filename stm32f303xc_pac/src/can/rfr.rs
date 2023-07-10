#[doc = "Register `RF%sR` reader"]
pub struct R(crate::R<RFR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RFR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RFR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RFR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RF%sR` writer"]
pub struct W(crate::W<RFR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RFR_SPEC>;
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
impl From<crate::W<RFR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RFR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FMP` reader - FMP0"]
pub type FMP_R = crate::FieldReader;
#[doc = "Field `FULL` reader - FULL0"]
pub type FULL_R = crate::BitReader<FULL0R_A>;
#[doc = "FULL0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FULL0R_A {
    #[doc = "0: FIFO x is not full"]
    NOT_FULL = 0,
    #[doc = "1: FIFO x is full"]
    FULL = 1,
}
impl From<FULL0R_A> for bool {
    #[inline(always)]
    fn from(variant: FULL0R_A) -> Self {
        variant as u8 != 0
    }
}
impl FULL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FULL0R_A {
        match self.bits {
            false => FULL0R_A::NOT_FULL,
            true => FULL0R_A::FULL,
        }
    }
    #[doc = "Checks if the value of the field is `NOT_FULL`"]
    #[inline(always)]
    pub fn is_not_full(&self) -> bool {
        *self == FULL0R_A::NOT_FULL
    }
    #[doc = "Checks if the value of the field is `FULL`"]
    #[inline(always)]
    pub fn is_full(&self) -> bool {
        *self == FULL0R_A::FULL
    }
}
#[doc = "FULL0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FULL0W_AW {
    #[doc = "1: Clear flag"]
    CLEAR = 1,
}
impl From<FULL0W_AW> for bool {
    #[inline(always)]
    fn from(variant: FULL0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FULL` writer - FULL0"]
pub type FULL_W<'a, const O: u8> = crate::BitWriter<'a, RFR_SPEC, O, FULL0W_AW>;
impl<'a, const O: u8> FULL_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FULL0W_AW::CLEAR)
    }
}
#[doc = "Field `FOVR` reader - FOVR0"]
pub type FOVR_R = crate::BitReader<FOVR0R_A>;
#[doc = "FOVR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOVR0R_A {
    #[doc = "0: No FIFO x overrun"]
    NO_OVERRUN = 0,
    #[doc = "1: FIFO x overrun"]
    OVERRUN = 1,
}
impl From<FOVR0R_A> for bool {
    #[inline(always)]
    fn from(variant: FOVR0R_A) -> Self {
        variant as u8 != 0
    }
}
impl FOVR_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FOVR0R_A {
        match self.bits {
            false => FOVR0R_A::NO_OVERRUN,
            true => FOVR0R_A::OVERRUN,
        }
    }
    #[doc = "Checks if the value of the field is `NO_OVERRUN`"]
    #[inline(always)]
    pub fn is_no_overrun(&self) -> bool {
        *self == FOVR0R_A::NO_OVERRUN
    }
    #[doc = "Checks if the value of the field is `OVERRUN`"]
    #[inline(always)]
    pub fn is_overrun(&self) -> bool {
        *self == FOVR0R_A::OVERRUN
    }
}
#[doc = "FOVR0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FOVR0W_AW {
    #[doc = "1: Clear flag"]
    CLEAR = 1,
}
impl From<FOVR0W_AW> for bool {
    #[inline(always)]
    fn from(variant: FOVR0W_AW) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `FOVR` writer - FOVR0"]
pub type FOVR_W<'a, const O: u8> = crate::BitWriter<'a, RFR_SPEC, O, FOVR0W_AW>;
impl<'a, const O: u8> FOVR_W<'a, O> {
    #[doc = "Clear flag"]
    #[inline(always)]
    pub fn clear(self) -> &'a mut W {
        self.variant(FOVR0W_AW::CLEAR)
    }
}
#[doc = "Field `RFOM` reader - RFOM0"]
pub type RFOM_R = crate::BitReader<RFOM0W_A>;
#[doc = "RFOM0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RFOM0W_A {
    #[doc = "1: Set by software to release the output mailbox of the FIFO"]
    RELEASE = 1,
}
impl From<RFOM0W_A> for bool {
    #[inline(always)]
    fn from(variant: RFOM0W_A) -> Self {
        variant as u8 != 0
    }
}
impl RFOM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<RFOM0W_A> {
        match self.bits {
            true => Some(RFOM0W_A::RELEASE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `RELEASE`"]
    #[inline(always)]
    pub fn is_release(&self) -> bool {
        *self == RFOM0W_A::RELEASE
    }
}
#[doc = "Field `RFOM` writer - RFOM0"]
pub type RFOM_W<'a, const O: u8> = crate::BitWriter<'a, RFR_SPEC, O, RFOM0W_A>;
impl<'a, const O: u8> RFOM_W<'a, O> {
    #[doc = "Set by software to release the output mailbox of the FIFO"]
    #[inline(always)]
    pub fn release(self) -> &'a mut W {
        self.variant(RFOM0W_A::RELEASE)
    }
}
impl R {
    #[doc = "Bits 0:1 - FMP0"]
    #[inline(always)]
    pub fn fmp(&self) -> FMP_R {
        FMP_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 3 - FULL0"]
    #[inline(always)]
    pub fn full(&self) -> FULL_R {
        FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FOVR0"]
    #[inline(always)]
    pub fn fovr(&self) -> FOVR_R {
        FOVR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - RFOM0"]
    #[inline(always)]
    pub fn rfom(&self) -> RFOM_R {
        RFOM_R::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - FULL0"]
    #[inline(always)]
    #[must_use]
    pub fn full(&mut self) -> FULL_W<3> {
        FULL_W::new(self)
    }
    #[doc = "Bit 4 - FOVR0"]
    #[inline(always)]
    #[must_use]
    pub fn fovr(&mut self) -> FOVR_W<4> {
        FOVR_W::new(self)
    }
    #[doc = "Bit 5 - RFOM0"]
    #[inline(always)]
    #[must_use]
    pub fn rfom(&mut self) -> RFOM_W<5> {
        RFOM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "receive FIFO %s register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rfr](index.html) module"]
pub struct RFR_SPEC;
impl crate::RegisterSpec for RFR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rfr::R](R) reader structure"]
impl crate::Readable for RFR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rfr::W](W) writer structure"]
impl crate::Writable for RFR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RF%sR to value 0"]
impl crate::Resettable for RFR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
