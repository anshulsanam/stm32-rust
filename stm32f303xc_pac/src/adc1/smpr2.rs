#[doc = "Register `SMPR2` reader"]
pub struct R(crate::R<SMPR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SMPR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SMPR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SMPR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SMPR2` writer"]
pub struct W(crate::W<SMPR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SMPR2_SPEC>;
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
impl From<crate::W<SMPR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SMPR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SMP10` reader - SMP10"]
pub type SMP10_R = crate::FieldReader<SMP10_A>;
#[doc = "SMP10\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SMP10_A {
    #[doc = "0: 1.5 ADC clock cycles"]
    CYCLES1_5 = 0,
    #[doc = "1: 2.5 ADC clock cycles"]
    CYCLES2_5 = 1,
    #[doc = "2: 4.5 ADC clock cycles"]
    CYCLES4_5 = 2,
    #[doc = "3: 7.5 ADC clock cycles"]
    CYCLES7_5 = 3,
    #[doc = "4: 19.5 ADC clock cycles"]
    CYCLES19_5 = 4,
    #[doc = "5: 61.5 ADC clock cycles"]
    CYCLES61_5 = 5,
    #[doc = "6: 181.5 ADC clock cycles"]
    CYCLES181_5 = 6,
    #[doc = "7: 601.5 ADC clock cycles"]
    CYCLES601_5 = 7,
}
impl From<SMP10_A> for u8 {
    #[inline(always)]
    fn from(variant: SMP10_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for SMP10_A {
    type Ux = u8;
}
impl SMP10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SMP10_A {
        match self.bits {
            0 => SMP10_A::CYCLES1_5,
            1 => SMP10_A::CYCLES2_5,
            2 => SMP10_A::CYCLES4_5,
            3 => SMP10_A::CYCLES7_5,
            4 => SMP10_A::CYCLES19_5,
            5 => SMP10_A::CYCLES61_5,
            6 => SMP10_A::CYCLES181_5,
            7 => SMP10_A::CYCLES601_5,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `CYCLES1_5`"]
    #[inline(always)]
    pub fn is_cycles1_5(&self) -> bool {
        *self == SMP10_A::CYCLES1_5
    }
    #[doc = "Checks if the value of the field is `CYCLES2_5`"]
    #[inline(always)]
    pub fn is_cycles2_5(&self) -> bool {
        *self == SMP10_A::CYCLES2_5
    }
    #[doc = "Checks if the value of the field is `CYCLES4_5`"]
    #[inline(always)]
    pub fn is_cycles4_5(&self) -> bool {
        *self == SMP10_A::CYCLES4_5
    }
    #[doc = "Checks if the value of the field is `CYCLES7_5`"]
    #[inline(always)]
    pub fn is_cycles7_5(&self) -> bool {
        *self == SMP10_A::CYCLES7_5
    }
    #[doc = "Checks if the value of the field is `CYCLES19_5`"]
    #[inline(always)]
    pub fn is_cycles19_5(&self) -> bool {
        *self == SMP10_A::CYCLES19_5
    }
    #[doc = "Checks if the value of the field is `CYCLES61_5`"]
    #[inline(always)]
    pub fn is_cycles61_5(&self) -> bool {
        *self == SMP10_A::CYCLES61_5
    }
    #[doc = "Checks if the value of the field is `CYCLES181_5`"]
    #[inline(always)]
    pub fn is_cycles181_5(&self) -> bool {
        *self == SMP10_A::CYCLES181_5
    }
    #[doc = "Checks if the value of the field is `CYCLES601_5`"]
    #[inline(always)]
    pub fn is_cycles601_5(&self) -> bool {
        *self == SMP10_A::CYCLES601_5
    }
}
#[doc = "Field `SMP10` writer - SMP10"]
pub type SMP10_W<'a, const O: u8> = crate::FieldWriterSafe<'a, SMPR2_SPEC, 3, O, SMP10_A>;
impl<'a, const O: u8> SMP10_W<'a, O> {
    #[doc = "1.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles1_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES1_5)
    }
    #[doc = "2.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles2_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES2_5)
    }
    #[doc = "4.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles4_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES4_5)
    }
    #[doc = "7.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles7_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES7_5)
    }
    #[doc = "19.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles19_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES19_5)
    }
    #[doc = "61.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles61_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES61_5)
    }
    #[doc = "181.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles181_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES181_5)
    }
    #[doc = "601.5 ADC clock cycles"]
    #[inline(always)]
    pub fn cycles601_5(self) -> &'a mut W {
        self.variant(SMP10_A::CYCLES601_5)
    }
}
#[doc = "Field `SMP11` reader - SMP11"]
pub use SMP10_R as SMP11_R;
#[doc = "Field `SMP12` reader - SMP12"]
pub use SMP10_R as SMP12_R;
#[doc = "Field `SMP13` reader - SMP13"]
pub use SMP10_R as SMP13_R;
#[doc = "Field `SMP14` reader - SMP14"]
pub use SMP10_R as SMP14_R;
#[doc = "Field `SMP15` reader - SMP15"]
pub use SMP10_R as SMP15_R;
#[doc = "Field `SMP16` reader - SMP16"]
pub use SMP10_R as SMP16_R;
#[doc = "Field `SMP17` reader - SMP17"]
pub use SMP10_R as SMP17_R;
#[doc = "Field `SMP18` reader - SMP18"]
pub use SMP10_R as SMP18_R;
#[doc = "Field `SMP11` writer - SMP11"]
pub use SMP10_W as SMP11_W;
#[doc = "Field `SMP12` writer - SMP12"]
pub use SMP10_W as SMP12_W;
#[doc = "Field `SMP13` writer - SMP13"]
pub use SMP10_W as SMP13_W;
#[doc = "Field `SMP14` writer - SMP14"]
pub use SMP10_W as SMP14_W;
#[doc = "Field `SMP15` writer - SMP15"]
pub use SMP10_W as SMP15_W;
#[doc = "Field `SMP16` writer - SMP16"]
pub use SMP10_W as SMP16_W;
#[doc = "Field `SMP17` writer - SMP17"]
pub use SMP10_W as SMP17_W;
#[doc = "Field `SMP18` writer - SMP18"]
pub use SMP10_W as SMP18_W;
impl R {
    #[doc = "Bits 0:2 - SMP10"]
    #[inline(always)]
    pub fn smp10(&self) -> SMP10_R {
        SMP10_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - SMP11"]
    #[inline(always)]
    pub fn smp11(&self) -> SMP11_R {
        SMP11_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - SMP12"]
    #[inline(always)]
    pub fn smp12(&self) -> SMP12_R {
        SMP12_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - SMP13"]
    #[inline(always)]
    pub fn smp13(&self) -> SMP13_R {
        SMP13_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - SMP14"]
    #[inline(always)]
    pub fn smp14(&self) -> SMP14_R {
        SMP14_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - SMP15"]
    #[inline(always)]
    pub fn smp15(&self) -> SMP15_R {
        SMP15_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - SMP16"]
    #[inline(always)]
    pub fn smp16(&self) -> SMP16_R {
        SMP16_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - SMP17"]
    #[inline(always)]
    pub fn smp17(&self) -> SMP17_R {
        SMP17_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26 - SMP18"]
    #[inline(always)]
    pub fn smp18(&self) -> SMP18_R {
        SMP18_R::new(((self.bits >> 24) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - SMP10"]
    #[inline(always)]
    #[must_use]
    pub fn smp10(&mut self) -> SMP10_W<0> {
        SMP10_W::new(self)
    }
    #[doc = "Bits 3:5 - SMP11"]
    #[inline(always)]
    #[must_use]
    pub fn smp11(&mut self) -> SMP11_W<3> {
        SMP11_W::new(self)
    }
    #[doc = "Bits 6:8 - SMP12"]
    #[inline(always)]
    #[must_use]
    pub fn smp12(&mut self) -> SMP12_W<6> {
        SMP12_W::new(self)
    }
    #[doc = "Bits 9:11 - SMP13"]
    #[inline(always)]
    #[must_use]
    pub fn smp13(&mut self) -> SMP13_W<9> {
        SMP13_W::new(self)
    }
    #[doc = "Bits 12:14 - SMP14"]
    #[inline(always)]
    #[must_use]
    pub fn smp14(&mut self) -> SMP14_W<12> {
        SMP14_W::new(self)
    }
    #[doc = "Bits 15:17 - SMP15"]
    #[inline(always)]
    #[must_use]
    pub fn smp15(&mut self) -> SMP15_W<15> {
        SMP15_W::new(self)
    }
    #[doc = "Bits 18:20 - SMP16"]
    #[inline(always)]
    #[must_use]
    pub fn smp16(&mut self) -> SMP16_W<18> {
        SMP16_W::new(self)
    }
    #[doc = "Bits 21:23 - SMP17"]
    #[inline(always)]
    #[must_use]
    pub fn smp17(&mut self) -> SMP17_W<21> {
        SMP17_W::new(self)
    }
    #[doc = "Bits 24:26 - SMP18"]
    #[inline(always)]
    #[must_use]
    pub fn smp18(&mut self) -> SMP18_W<24> {
        SMP18_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "sample time register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [smpr2](index.html) module"]
pub struct SMPR2_SPEC;
impl crate::RegisterSpec for SMPR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [smpr2::R](R) reader structure"]
impl crate::Readable for SMPR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [smpr2::W](W) writer structure"]
impl crate::Writable for SMPR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SMPR2 to value 0"]
impl crate::Resettable for SMPR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
