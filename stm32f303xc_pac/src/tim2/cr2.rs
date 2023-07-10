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
#[doc = "Field `CCDS` reader - Capture/compare DMA selection"]
pub type CCDS_R = crate::BitReader<CCDS_A>;
#[doc = "Capture/compare DMA selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CCDS_A {
    #[doc = "0: CCx DMA request sent when CCx event occurs"]
    ON_COMPARE = 0,
    #[doc = "1: CCx DMA request sent when update event occurs"]
    ON_UPDATE = 1,
}
impl From<CCDS_A> for bool {
    #[inline(always)]
    fn from(variant: CCDS_A) -> Self {
        variant as u8 != 0
    }
}
impl CCDS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CCDS_A {
        match self.bits {
            false => CCDS_A::ON_COMPARE,
            true => CCDS_A::ON_UPDATE,
        }
    }
    #[doc = "Checks if the value of the field is `ON_COMPARE`"]
    #[inline(always)]
    pub fn is_on_compare(&self) -> bool {
        *self == CCDS_A::ON_COMPARE
    }
    #[doc = "Checks if the value of the field is `ON_UPDATE`"]
    #[inline(always)]
    pub fn is_on_update(&self) -> bool {
        *self == CCDS_A::ON_UPDATE
    }
}
#[doc = "Field `CCDS` writer - Capture/compare DMA selection"]
pub type CCDS_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, CCDS_A>;
impl<'a, const O: u8> CCDS_W<'a, O> {
    #[doc = "CCx DMA request sent when CCx event occurs"]
    #[inline(always)]
    pub fn on_compare(self) -> &'a mut W {
        self.variant(CCDS_A::ON_COMPARE)
    }
    #[doc = "CCx DMA request sent when update event occurs"]
    #[inline(always)]
    pub fn on_update(self) -> &'a mut W {
        self.variant(CCDS_A::ON_UPDATE)
    }
}
#[doc = "Field `MMS` reader - Master mode selection"]
pub type MMS_R = crate::FieldReader<MMS_A>;
#[doc = "Master mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MMS_A {
    #[doc = "0: The UG bit from the TIMx_EGR register is used as trigger output"]
    RESET = 0,
    #[doc = "1: The counter enable signal, CNT_EN, is used as trigger output"]
    ENABLE = 1,
    #[doc = "2: The update event is selected as trigger output"]
    UPDATE = 2,
    #[doc = "3: The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
    COMPARE_PULSE = 3,
    #[doc = "4: OC1REF signal is used as trigger output"]
    COMPARE_OC1 = 4,
    #[doc = "5: OC2REF signal is used as trigger output"]
    COMPARE_OC2 = 5,
    #[doc = "6: OC3REF signal is used as trigger output"]
    COMPARE_OC3 = 6,
    #[doc = "7: OC4REF signal is used as trigger output"]
    COMPARE_OC4 = 7,
}
impl From<MMS_A> for u8 {
    #[inline(always)]
    fn from(variant: MMS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MMS_A {
    type Ux = u8;
}
impl MMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> MMS_A {
        match self.bits {
            0 => MMS_A::RESET,
            1 => MMS_A::ENABLE,
            2 => MMS_A::UPDATE,
            3 => MMS_A::COMPARE_PULSE,
            4 => MMS_A::COMPARE_OC1,
            5 => MMS_A::COMPARE_OC2,
            6 => MMS_A::COMPARE_OC3,
            7 => MMS_A::COMPARE_OC4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RESET`"]
    #[inline(always)]
    pub fn is_reset(&self) -> bool {
        *self == MMS_A::RESET
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == MMS_A::ENABLE
    }
    #[doc = "Checks if the value of the field is `UPDATE`"]
    #[inline(always)]
    pub fn is_update(&self) -> bool {
        *self == MMS_A::UPDATE
    }
    #[doc = "Checks if the value of the field is `COMPARE_PULSE`"]
    #[inline(always)]
    pub fn is_compare_pulse(&self) -> bool {
        *self == MMS_A::COMPARE_PULSE
    }
    #[doc = "Checks if the value of the field is `COMPARE_OC1`"]
    #[inline(always)]
    pub fn is_compare_oc1(&self) -> bool {
        *self == MMS_A::COMPARE_OC1
    }
    #[doc = "Checks if the value of the field is `COMPARE_OC2`"]
    #[inline(always)]
    pub fn is_compare_oc2(&self) -> bool {
        *self == MMS_A::COMPARE_OC2
    }
    #[doc = "Checks if the value of the field is `COMPARE_OC3`"]
    #[inline(always)]
    pub fn is_compare_oc3(&self) -> bool {
        *self == MMS_A::COMPARE_OC3
    }
    #[doc = "Checks if the value of the field is `COMPARE_OC4`"]
    #[inline(always)]
    pub fn is_compare_oc4(&self) -> bool {
        *self == MMS_A::COMPARE_OC4
    }
}
#[doc = "Field `MMS` writer - Master mode selection"]
pub type MMS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CR2_SPEC, 3, O, MMS_A>;
impl<'a, const O: u8> MMS_W<'a, O> {
    #[doc = "The UG bit from the TIMx_EGR register is used as trigger output"]
    #[inline(always)]
    pub fn reset(self) -> &'a mut W {
        self.variant(MMS_A::RESET)
    }
    #[doc = "The counter enable signal, CNT_EN, is used as trigger output"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(MMS_A::ENABLE)
    }
    #[doc = "The update event is selected as trigger output"]
    #[inline(always)]
    pub fn update(self) -> &'a mut W {
        self.variant(MMS_A::UPDATE)
    }
    #[doc = "The trigger output send a positive pulse when the CC1IF flag it to be set, as soon as a capture or a compare match occurred"]
    #[inline(always)]
    pub fn compare_pulse(self) -> &'a mut W {
        self.variant(MMS_A::COMPARE_PULSE)
    }
    #[doc = "OC1REF signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_oc1(self) -> &'a mut W {
        self.variant(MMS_A::COMPARE_OC1)
    }
    #[doc = "OC2REF signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_oc2(self) -> &'a mut W {
        self.variant(MMS_A::COMPARE_OC2)
    }
    #[doc = "OC3REF signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_oc3(self) -> &'a mut W {
        self.variant(MMS_A::COMPARE_OC3)
    }
    #[doc = "OC4REF signal is used as trigger output"]
    #[inline(always)]
    pub fn compare_oc4(self) -> &'a mut W {
        self.variant(MMS_A::COMPARE_OC4)
    }
}
#[doc = "Field `TI1S` reader - TI1 selection"]
pub type TI1S_R = crate::BitReader<TI1S_A>;
#[doc = "TI1 selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TI1S_A {
    #[doc = "0: The TIMx_CH1 pin is connected to TI1 input"]
    NORMAL = 0,
    #[doc = "1: The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
    XOR = 1,
}
impl From<TI1S_A> for bool {
    #[inline(always)]
    fn from(variant: TI1S_A) -> Self {
        variant as u8 != 0
    }
}
impl TI1S_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TI1S_A {
        match self.bits {
            false => TI1S_A::NORMAL,
            true => TI1S_A::XOR,
        }
    }
    #[doc = "Checks if the value of the field is `NORMAL`"]
    #[inline(always)]
    pub fn is_normal(&self) -> bool {
        *self == TI1S_A::NORMAL
    }
    #[doc = "Checks if the value of the field is `XOR`"]
    #[inline(always)]
    pub fn is_xor(&self) -> bool {
        *self == TI1S_A::XOR
    }
}
#[doc = "Field `TI1S` writer - TI1 selection"]
pub type TI1S_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, TI1S_A>;
impl<'a, const O: u8> TI1S_W<'a, O> {
    #[doc = "The TIMx_CH1 pin is connected to TI1 input"]
    #[inline(always)]
    pub fn normal(self) -> &'a mut W {
        self.variant(TI1S_A::NORMAL)
    }
    #[doc = "The TIMx_CH1, CH2, CH3 pins are connected to TI1 input"]
    #[inline(always)]
    pub fn xor(self) -> &'a mut W {
        self.variant(TI1S_A::XOR)
    }
}
impl R {
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    pub fn ccds(&self) -> CCDS_R {
        CCDS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    pub fn mms(&self) -> MMS_R {
        MMS_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    pub fn ti1s(&self) -> TI1S_R {
        TI1S_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 3 - Capture/compare DMA selection"]
    #[inline(always)]
    #[must_use]
    pub fn ccds(&mut self) -> CCDS_W<3> {
        CCDS_W::new(self)
    }
    #[doc = "Bits 4:6 - Master mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn mms(&mut self) -> MMS_W<4> {
        MMS_W::new(self)
    }
    #[doc = "Bit 7 - TI1 selection"]
    #[inline(always)]
    #[must_use]
    pub fn ti1s(&mut self) -> TI1S_W<7> {
        TI1S_W::new(self)
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
