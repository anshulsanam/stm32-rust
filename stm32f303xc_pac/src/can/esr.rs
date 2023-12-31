#[doc = "Register `ESR` reader"]
pub struct R(crate::R<ESR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ESR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ESR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ESR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ESR` writer"]
pub struct W(crate::W<ESR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ESR_SPEC>;
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
impl From<crate::W<ESR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ESR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `EWGF` reader - EWGF"]
pub type EWGF_R = crate::BitReader;
#[doc = "Field `EPVF` reader - EPVF"]
pub type EPVF_R = crate::BitReader;
#[doc = "Field `BOFF` reader - BOFF"]
pub type BOFF_R = crate::BitReader;
#[doc = "Field `LEC` reader - LEC"]
pub type LEC_R = crate::FieldReader<LEC_A>;
#[doc = "LEC\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LEC_A {
    #[doc = "0: No Error"]
    NO_ERROR = 0,
    #[doc = "1: Stuff Error"]
    STUFF = 1,
    #[doc = "2: Form Error"]
    FORM = 2,
    #[doc = "3: Acknowledgment Error"]
    ACK = 3,
    #[doc = "4: Bit recessive Error"]
    BIT_RECESSIVE = 4,
    #[doc = "5: Bit dominant Error"]
    BIT_DOMINANT = 5,
    #[doc = "6: CRC Error"]
    CRC = 6,
    #[doc = "7: Set by software"]
    CUSTOM = 7,
}
impl From<LEC_A> for u8 {
    #[inline(always)]
    fn from(variant: LEC_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for LEC_A {
    type Ux = u8;
}
impl LEC_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> LEC_A {
        match self.bits {
            0 => LEC_A::NO_ERROR,
            1 => LEC_A::STUFF,
            2 => LEC_A::FORM,
            3 => LEC_A::ACK,
            4 => LEC_A::BIT_RECESSIVE,
            5 => LEC_A::BIT_DOMINANT,
            6 => LEC_A::CRC,
            7 => LEC_A::CUSTOM,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_ERROR`"]
    #[inline(always)]
    pub fn is_no_error(&self) -> bool {
        *self == LEC_A::NO_ERROR
    }
    #[doc = "Checks if the value of the field is `STUFF`"]
    #[inline(always)]
    pub fn is_stuff(&self) -> bool {
        *self == LEC_A::STUFF
    }
    #[doc = "Checks if the value of the field is `FORM`"]
    #[inline(always)]
    pub fn is_form(&self) -> bool {
        *self == LEC_A::FORM
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == LEC_A::ACK
    }
    #[doc = "Checks if the value of the field is `BIT_RECESSIVE`"]
    #[inline(always)]
    pub fn is_bit_recessive(&self) -> bool {
        *self == LEC_A::BIT_RECESSIVE
    }
    #[doc = "Checks if the value of the field is `BIT_DOMINANT`"]
    #[inline(always)]
    pub fn is_bit_dominant(&self) -> bool {
        *self == LEC_A::BIT_DOMINANT
    }
    #[doc = "Checks if the value of the field is `CRC`"]
    #[inline(always)]
    pub fn is_crc(&self) -> bool {
        *self == LEC_A::CRC
    }
    #[doc = "Checks if the value of the field is `CUSTOM`"]
    #[inline(always)]
    pub fn is_custom(&self) -> bool {
        *self == LEC_A::CUSTOM
    }
}
#[doc = "Field `LEC` writer - LEC"]
pub type LEC_W<'a, const O: u8> = crate::FieldWriterSafe<'a, ESR_SPEC, 3, O, LEC_A>;
impl<'a, const O: u8> LEC_W<'a, O> {
    #[doc = "No Error"]
    #[inline(always)]
    pub fn no_error(self) -> &'a mut W {
        self.variant(LEC_A::NO_ERROR)
    }
    #[doc = "Stuff Error"]
    #[inline(always)]
    pub fn stuff(self) -> &'a mut W {
        self.variant(LEC_A::STUFF)
    }
    #[doc = "Form Error"]
    #[inline(always)]
    pub fn form(self) -> &'a mut W {
        self.variant(LEC_A::FORM)
    }
    #[doc = "Acknowledgment Error"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(LEC_A::ACK)
    }
    #[doc = "Bit recessive Error"]
    #[inline(always)]
    pub fn bit_recessive(self) -> &'a mut W {
        self.variant(LEC_A::BIT_RECESSIVE)
    }
    #[doc = "Bit dominant Error"]
    #[inline(always)]
    pub fn bit_dominant(self) -> &'a mut W {
        self.variant(LEC_A::BIT_DOMINANT)
    }
    #[doc = "CRC Error"]
    #[inline(always)]
    pub fn crc(self) -> &'a mut W {
        self.variant(LEC_A::CRC)
    }
    #[doc = "Set by software"]
    #[inline(always)]
    pub fn custom(self) -> &'a mut W {
        self.variant(LEC_A::CUSTOM)
    }
}
#[doc = "Field `TEC` reader - TEC"]
pub type TEC_R = crate::FieldReader;
#[doc = "Field `REC` reader - REC"]
pub type REC_R = crate::FieldReader;
impl R {
    #[doc = "Bit 0 - EWGF"]
    #[inline(always)]
    pub fn ewgf(&self) -> EWGF_R {
        EWGF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - EPVF"]
    #[inline(always)]
    pub fn epvf(&self) -> EPVF_R {
        EPVF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - BOFF"]
    #[inline(always)]
    pub fn boff(&self) -> BOFF_R {
        BOFF_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 4:6 - LEC"]
    #[inline(always)]
    pub fn lec(&self) -> LEC_R {
        LEC_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 16:23 - TEC"]
    #[inline(always)]
    pub fn tec(&self) -> TEC_R {
        TEC_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - REC"]
    #[inline(always)]
    pub fn rec(&self) -> REC_R {
        REC_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 4:6 - LEC"]
    #[inline(always)]
    #[must_use]
    pub fn lec(&mut self) -> LEC_W<4> {
        LEC_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "error status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [esr](index.html) module"]
pub struct ESR_SPEC;
impl crate::RegisterSpec for ESR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [esr::R](R) reader structure"]
impl crate::Readable for ESR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [esr::W](W) writer structure"]
impl crate::Writable for ESR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ESR to value 0"]
impl crate::Resettable for ESR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
