#[doc = "Register `CCMR1_Input` reader"]
pub struct R(crate::R<CCMR1_INPUT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCMR1_INPUT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCMR1_INPUT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCMR1_INPUT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCMR1_Input` writer"]
pub struct W(crate::W<CCMR1_INPUT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCMR1_INPUT_SPEC>;
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
impl From<crate::W<CCMR1_INPUT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCMR1_INPUT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CC1S` reader - Capture/Compare 1 selection"]
pub type CC1S_R = crate::FieldReader<CC1S_A>;
#[doc = "Capture/Compare 1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC1S_A {
    #[doc = "1: CC1 channel is configured as input, IC1 is mapped on TI1"]
    TI1 = 1,
    #[doc = "2: CC1 channel is configured as input, IC1 is mapped on TI2"]
    TI2 = 2,
    #[doc = "3: CC1 channel is configured as input, IC1 is mapped on TRC"]
    TRC = 3,
}
impl From<CC1S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC1S_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC1S_A {
    type Ux = u8;
}
impl CC1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CC1S_A> {
        match self.bits {
            1 => Some(CC1S_A::TI1),
            2 => Some(CC1S_A::TI2),
            3 => Some(CC1S_A::TRC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TI1`"]
    #[inline(always)]
    pub fn is_ti1(&self) -> bool {
        *self == CC1S_A::TI1
    }
    #[doc = "Checks if the value of the field is `TI2`"]
    #[inline(always)]
    pub fn is_ti2(&self) -> bool {
        *self == CC1S_A::TI2
    }
    #[doc = "Checks if the value of the field is `TRC`"]
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        *self == CC1S_A::TRC
    }
}
#[doc = "Field `CC1S` writer - Capture/Compare 1 selection"]
pub type CC1S_W<'a, const O: u8> = crate::FieldWriter<'a, CCMR1_INPUT_SPEC, 2, O, CC1S_A>;
impl<'a, const O: u8> CC1S_W<'a, O> {
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI1"]
    #[inline(always)]
    pub fn ti1(self) -> &'a mut W {
        self.variant(CC1S_A::TI1)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TI2"]
    #[inline(always)]
    pub fn ti2(self) -> &'a mut W {
        self.variant(CC1S_A::TI2)
    }
    #[doc = "CC1 channel is configured as input, IC1 is mapped on TRC"]
    #[inline(always)]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC1S_A::TRC)
    }
}
#[doc = "Field `IC1PSC` reader - Input capture 1 prescaler"]
pub type IC1PSC_R = crate::FieldReader;
#[doc = "Field `IC1PSC` writer - Input capture 1 prescaler"]
pub type IC1PSC_W<'a, const O: u8> = crate::FieldWriter<'a, CCMR1_INPUT_SPEC, 2, O>;
#[doc = "Field `IC1F` reader - Input capture 1 filter"]
pub type IC1F_R = crate::FieldReader<IC1F_A>;
#[doc = "Input capture 1 filter\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IC1F_A {
    #[doc = "0: No filter, sampling is done at fDTS"]
    NO_FILTER = 0,
    #[doc = "1: fSAMPLING=fCK_INT, N=2"]
    FCK_INT_N2 = 1,
    #[doc = "2: fSAMPLING=fCK_INT, N=4"]
    FCK_INT_N4 = 2,
    #[doc = "3: fSAMPLING=fCK_INT, N=8"]
    FCK_INT_N8 = 3,
    #[doc = "4: fSAMPLING=fDTS/2, N=6"]
    FDTS_DIV2_N6 = 4,
    #[doc = "5: fSAMPLING=fDTS/2, N=8"]
    FDTS_DIV2_N8 = 5,
    #[doc = "6: fSAMPLING=fDTS/4, N=6"]
    FDTS_DIV4_N6 = 6,
    #[doc = "7: fSAMPLING=fDTS/4, N=8"]
    FDTS_DIV4_N8 = 7,
    #[doc = "8: fSAMPLING=fDTS/8, N=6"]
    FDTS_DIV8_N6 = 8,
    #[doc = "9: fSAMPLING=fDTS/8, N=8"]
    FDTS_DIV8_N8 = 9,
    #[doc = "10: fSAMPLING=fDTS/16, N=5"]
    FDTS_DIV16_N5 = 10,
    #[doc = "11: fSAMPLING=fDTS/16, N=6"]
    FDTS_DIV16_N6 = 11,
    #[doc = "12: fSAMPLING=fDTS/16, N=8"]
    FDTS_DIV16_N8 = 12,
    #[doc = "13: fSAMPLING=fDTS/32, N=5"]
    FDTS_DIV32_N5 = 13,
    #[doc = "14: fSAMPLING=fDTS/32, N=6"]
    FDTS_DIV32_N6 = 14,
    #[doc = "15: fSAMPLING=fDTS/32, N=8"]
    FDTS_DIV32_N8 = 15,
}
impl From<IC1F_A> for u8 {
    #[inline(always)]
    fn from(variant: IC1F_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for IC1F_A {
    type Ux = u8;
}
impl IC1F_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> IC1F_A {
        match self.bits {
            0 => IC1F_A::NO_FILTER,
            1 => IC1F_A::FCK_INT_N2,
            2 => IC1F_A::FCK_INT_N4,
            3 => IC1F_A::FCK_INT_N8,
            4 => IC1F_A::FDTS_DIV2_N6,
            5 => IC1F_A::FDTS_DIV2_N8,
            6 => IC1F_A::FDTS_DIV4_N6,
            7 => IC1F_A::FDTS_DIV4_N8,
            8 => IC1F_A::FDTS_DIV8_N6,
            9 => IC1F_A::FDTS_DIV8_N8,
            10 => IC1F_A::FDTS_DIV16_N5,
            11 => IC1F_A::FDTS_DIV16_N6,
            12 => IC1F_A::FDTS_DIV16_N8,
            13 => IC1F_A::FDTS_DIV32_N5,
            14 => IC1F_A::FDTS_DIV32_N6,
            15 => IC1F_A::FDTS_DIV32_N8,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `NO_FILTER`"]
    #[inline(always)]
    pub fn is_no_filter(&self) -> bool {
        *self == IC1F_A::NO_FILTER
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N2`"]
    #[inline(always)]
    pub fn is_fck_int_n2(&self) -> bool {
        *self == IC1F_A::FCK_INT_N2
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N4`"]
    #[inline(always)]
    pub fn is_fck_int_n4(&self) -> bool {
        *self == IC1F_A::FCK_INT_N4
    }
    #[doc = "Checks if the value of the field is `FCK_INT_N8`"]
    #[inline(always)]
    pub fn is_fck_int_n8(&self) -> bool {
        *self == IC1F_A::FCK_INT_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV2_N6`"]
    #[inline(always)]
    pub fn is_fdts_div2_n6(&self) -> bool {
        *self == IC1F_A::FDTS_DIV2_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV2_N8`"]
    #[inline(always)]
    pub fn is_fdts_div2_n8(&self) -> bool {
        *self == IC1F_A::FDTS_DIV2_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV4_N6`"]
    #[inline(always)]
    pub fn is_fdts_div4_n6(&self) -> bool {
        *self == IC1F_A::FDTS_DIV4_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV4_N8`"]
    #[inline(always)]
    pub fn is_fdts_div4_n8(&self) -> bool {
        *self == IC1F_A::FDTS_DIV4_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV8_N6`"]
    #[inline(always)]
    pub fn is_fdts_div8_n6(&self) -> bool {
        *self == IC1F_A::FDTS_DIV8_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV8_N8`"]
    #[inline(always)]
    pub fn is_fdts_div8_n8(&self) -> bool {
        *self == IC1F_A::FDTS_DIV8_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV16_N5`"]
    #[inline(always)]
    pub fn is_fdts_div16_n5(&self) -> bool {
        *self == IC1F_A::FDTS_DIV16_N5
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV16_N6`"]
    #[inline(always)]
    pub fn is_fdts_div16_n6(&self) -> bool {
        *self == IC1F_A::FDTS_DIV16_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV16_N8`"]
    #[inline(always)]
    pub fn is_fdts_div16_n8(&self) -> bool {
        *self == IC1F_A::FDTS_DIV16_N8
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV32_N5`"]
    #[inline(always)]
    pub fn is_fdts_div32_n5(&self) -> bool {
        *self == IC1F_A::FDTS_DIV32_N5
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV32_N6`"]
    #[inline(always)]
    pub fn is_fdts_div32_n6(&self) -> bool {
        *self == IC1F_A::FDTS_DIV32_N6
    }
    #[doc = "Checks if the value of the field is `FDTS_DIV32_N8`"]
    #[inline(always)]
    pub fn is_fdts_div32_n8(&self) -> bool {
        *self == IC1F_A::FDTS_DIV32_N8
    }
}
#[doc = "Field `IC1F` writer - Input capture 1 filter"]
pub type IC1F_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CCMR1_INPUT_SPEC, 4, O, IC1F_A>;
impl<'a, const O: u8> IC1F_W<'a, O> {
    #[doc = "No filter, sampling is done at fDTS"]
    #[inline(always)]
    pub fn no_filter(self) -> &'a mut W {
        self.variant(IC1F_A::NO_FILTER)
    }
    #[doc = "fSAMPLING=fCK_INT, N=2"]
    #[inline(always)]
    pub fn fck_int_n2(self) -> &'a mut W {
        self.variant(IC1F_A::FCK_INT_N2)
    }
    #[doc = "fSAMPLING=fCK_INT, N=4"]
    #[inline(always)]
    pub fn fck_int_n4(self) -> &'a mut W {
        self.variant(IC1F_A::FCK_INT_N4)
    }
    #[doc = "fSAMPLING=fCK_INT, N=8"]
    #[inline(always)]
    pub fn fck_int_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FCK_INT_N8)
    }
    #[doc = "fSAMPLING=fDTS/2, N=6"]
    #[inline(always)]
    pub fn fdts_div2_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV2_N6)
    }
    #[doc = "fSAMPLING=fDTS/2, N=8"]
    #[inline(always)]
    pub fn fdts_div2_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV2_N8)
    }
    #[doc = "fSAMPLING=fDTS/4, N=6"]
    #[inline(always)]
    pub fn fdts_div4_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV4_N6)
    }
    #[doc = "fSAMPLING=fDTS/4, N=8"]
    #[inline(always)]
    pub fn fdts_div4_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV4_N8)
    }
    #[doc = "fSAMPLING=fDTS/8, N=6"]
    #[inline(always)]
    pub fn fdts_div8_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV8_N6)
    }
    #[doc = "fSAMPLING=fDTS/8, N=8"]
    #[inline(always)]
    pub fn fdts_div8_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV8_N8)
    }
    #[doc = "fSAMPLING=fDTS/16, N=5"]
    #[inline(always)]
    pub fn fdts_div16_n5(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV16_N5)
    }
    #[doc = "fSAMPLING=fDTS/16, N=6"]
    #[inline(always)]
    pub fn fdts_div16_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV16_N6)
    }
    #[doc = "fSAMPLING=fDTS/16, N=8"]
    #[inline(always)]
    pub fn fdts_div16_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV16_N8)
    }
    #[doc = "fSAMPLING=fDTS/32, N=5"]
    #[inline(always)]
    pub fn fdts_div32_n5(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV32_N5)
    }
    #[doc = "fSAMPLING=fDTS/32, N=6"]
    #[inline(always)]
    pub fn fdts_div32_n6(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV32_N6)
    }
    #[doc = "fSAMPLING=fDTS/32, N=8"]
    #[inline(always)]
    pub fn fdts_div32_n8(self) -> &'a mut W {
        self.variant(IC1F_A::FDTS_DIV32_N8)
    }
}
#[doc = "Field `CC2S` reader - Capture/Compare 2 selection"]
pub type CC2S_R = crate::FieldReader<CC2S_A>;
#[doc = "Capture/Compare 2 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CC2S_A {
    #[doc = "1: CC2 channel is configured as input, IC2 is mapped on TI2"]
    TI2 = 1,
    #[doc = "2: CC2 channel is configured as input, IC2 is mapped on TI1"]
    TI1 = 2,
    #[doc = "3: CC2 channel is configured as input, IC2 is mapped on TRC"]
    TRC = 3,
}
impl From<CC2S_A> for u8 {
    #[inline(always)]
    fn from(variant: CC2S_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CC2S_A {
    type Ux = u8;
}
impl CC2S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<CC2S_A> {
        match self.bits {
            1 => Some(CC2S_A::TI2),
            2 => Some(CC2S_A::TI1),
            3 => Some(CC2S_A::TRC),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `TI2`"]
    #[inline(always)]
    pub fn is_ti2(&self) -> bool {
        *self == CC2S_A::TI2
    }
    #[doc = "Checks if the value of the field is `TI1`"]
    #[inline(always)]
    pub fn is_ti1(&self) -> bool {
        *self == CC2S_A::TI1
    }
    #[doc = "Checks if the value of the field is `TRC`"]
    #[inline(always)]
    pub fn is_trc(&self) -> bool {
        *self == CC2S_A::TRC
    }
}
#[doc = "Field `CC2S` writer - Capture/Compare 2 selection"]
pub type CC2S_W<'a, const O: u8> = crate::FieldWriter<'a, CCMR1_INPUT_SPEC, 2, O, CC2S_A>;
impl<'a, const O: u8> CC2S_W<'a, O> {
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI2"]
    #[inline(always)]
    pub fn ti2(self) -> &'a mut W {
        self.variant(CC2S_A::TI2)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TI1"]
    #[inline(always)]
    pub fn ti1(self) -> &'a mut W {
        self.variant(CC2S_A::TI1)
    }
    #[doc = "CC2 channel is configured as input, IC2 is mapped on TRC"]
    #[inline(always)]
    pub fn trc(self) -> &'a mut W {
        self.variant(CC2S_A::TRC)
    }
}
#[doc = "Field `IC2PSC` reader - Input capture 2 prescaler"]
pub type IC2PSC_R = crate::FieldReader;
#[doc = "Field `IC2PSC` writer - Input capture 2 prescaler"]
pub type IC2PSC_W<'a, const O: u8> = crate::FieldWriter<'a, CCMR1_INPUT_SPEC, 2, O>;
#[doc = "Field `IC2F` reader - Input capture 2 filter"]
pub type IC2F_R = crate::FieldReader;
#[doc = "Field `IC2F` writer - Input capture 2 filter"]
pub type IC2F_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CCMR1_INPUT_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    pub fn cc1s(&self) -> CC1S_R {
        CC1S_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    pub fn ic1psc(&self) -> IC1PSC_R {
        IC1PSC_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline(always)]
    pub fn ic1f(&self) -> IC1F_R {
        IC1F_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    pub fn cc2s(&self) -> CC2S_R {
        CC2S_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline(always)]
    pub fn ic2psc(&self) -> IC2PSC_R {
        IC2PSC_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:15 - Input capture 2 filter"]
    #[inline(always)]
    pub fn ic2f(&self) -> IC2F_R {
        IC2F_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Capture/Compare 1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc1s(&mut self) -> CC1S_W<0> {
        CC1S_W::new(self)
    }
    #[doc = "Bits 2:3 - Input capture 1 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ic1psc(&mut self) -> IC1PSC_W<2> {
        IC1PSC_W::new(self)
    }
    #[doc = "Bits 4:7 - Input capture 1 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ic1f(&mut self) -> IC1F_W<4> {
        IC1F_W::new(self)
    }
    #[doc = "Bits 8:9 - Capture/Compare 2 selection"]
    #[inline(always)]
    #[must_use]
    pub fn cc2s(&mut self) -> CC2S_W<8> {
        CC2S_W::new(self)
    }
    #[doc = "Bits 10:11 - Input capture 2 prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn ic2psc(&mut self) -> IC2PSC_W<10> {
        IC2PSC_W::new(self)
    }
    #[doc = "Bits 12:15 - Input capture 2 filter"]
    #[inline(always)]
    #[must_use]
    pub fn ic2f(&mut self) -> IC2F_W<12> {
        IC2F_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "capture/compare mode register 1 (input mode)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccmr1_input](index.html) module"]
pub struct CCMR1_INPUT_SPEC;
impl crate::RegisterSpec for CCMR1_INPUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccmr1_input::R](R) reader structure"]
impl crate::Readable for CCMR1_INPUT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccmr1_input::W](W) writer structure"]
impl crate::Writable for CCMR1_INPUT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCMR1_Input to value 0"]
impl crate::Resettable for CCMR1_INPUT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
