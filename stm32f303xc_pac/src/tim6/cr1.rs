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
#[doc = "Field `CEN` reader - Counter enable"]
pub type CEN_R = crate::BitReader<CEN_A>;
#[doc = "Counter enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CEN_A {
    #[doc = "0: Counter disabled"]
    DISABLED = 0,
    #[doc = "1: Counter enabled"]
    ENABLED = 1,
}
impl From<CEN_A> for bool {
    #[inline(always)]
    fn from(variant: CEN_A) -> Self {
        variant as u8 != 0
    }
}
impl CEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CEN_A {
        match self.bits {
            false => CEN_A::DISABLED,
            true => CEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == CEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == CEN_A::ENABLED
    }
}
#[doc = "Field `CEN` writer - Counter enable"]
pub type CEN_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, CEN_A>;
impl<'a, const O: u8> CEN_W<'a, O> {
    #[doc = "Counter disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(CEN_A::DISABLED)
    }
    #[doc = "Counter enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(CEN_A::ENABLED)
    }
}
#[doc = "Field `UDIS` reader - Update disable"]
pub type UDIS_R = crate::BitReader<UDIS_A>;
#[doc = "Update disable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum UDIS_A {
    #[doc = "0: Update event enabled"]
    ENABLED = 0,
    #[doc = "1: Update event disabled"]
    DISABLED = 1,
}
impl From<UDIS_A> for bool {
    #[inline(always)]
    fn from(variant: UDIS_A) -> Self {
        variant as u8 != 0
    }
}
impl UDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> UDIS_A {
        match self.bits {
            false => UDIS_A::ENABLED,
            true => UDIS_A::DISABLED,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == UDIS_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == UDIS_A::DISABLED
    }
}
#[doc = "Field `UDIS` writer - Update disable"]
pub type UDIS_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, UDIS_A>;
impl<'a, const O: u8> UDIS_W<'a, O> {
    #[doc = "Update event enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(UDIS_A::ENABLED)
    }
    #[doc = "Update event disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(UDIS_A::DISABLED)
    }
}
#[doc = "Field `URS` reader - Update request source"]
pub type URS_R = crate::BitReader<URS_A>;
#[doc = "Update request source\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum URS_A {
    #[doc = "0: Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
    ANY_EVENT = 0,
    #[doc = "1: Only counter overflow/underflow generates an update interrupt or DMA request"]
    COUNTER_ONLY = 1,
}
impl From<URS_A> for bool {
    #[inline(always)]
    fn from(variant: URS_A) -> Self {
        variant as u8 != 0
    }
}
impl URS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> URS_A {
        match self.bits {
            false => URS_A::ANY_EVENT,
            true => URS_A::COUNTER_ONLY,
        }
    }
    #[doc = "Checks if the value of the field is `ANY_EVENT`"]
    #[inline(always)]
    pub fn is_any_event(&self) -> bool {
        *self == URS_A::ANY_EVENT
    }
    #[doc = "Checks if the value of the field is `COUNTER_ONLY`"]
    #[inline(always)]
    pub fn is_counter_only(&self) -> bool {
        *self == URS_A::COUNTER_ONLY
    }
}
#[doc = "Field `URS` writer - Update request source"]
pub type URS_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, URS_A>;
impl<'a, const O: u8> URS_W<'a, O> {
    #[doc = "Any of counter overflow/underflow, setting UG, or update through slave mode, generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn any_event(self) -> &'a mut W {
        self.variant(URS_A::ANY_EVENT)
    }
    #[doc = "Only counter overflow/underflow generates an update interrupt or DMA request"]
    #[inline(always)]
    pub fn counter_only(self) -> &'a mut W {
        self.variant(URS_A::COUNTER_ONLY)
    }
}
#[doc = "Field `OPM` reader - One-pulse mode"]
pub type OPM_R = crate::BitReader<OPM_A>;
#[doc = "One-pulse mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OPM_A {
    #[doc = "0: Counter is not stopped at update event"]
    DISABLED = 0,
    #[doc = "1: Counter stops counting at the next update event (clearing the CEN bit)"]
    ENABLED = 1,
}
impl From<OPM_A> for bool {
    #[inline(always)]
    fn from(variant: OPM_A) -> Self {
        variant as u8 != 0
    }
}
impl OPM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> OPM_A {
        match self.bits {
            false => OPM_A::DISABLED,
            true => OPM_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == OPM_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == OPM_A::ENABLED
    }
}
#[doc = "Field `OPM` writer - One-pulse mode"]
pub type OPM_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, OPM_A>;
impl<'a, const O: u8> OPM_W<'a, O> {
    #[doc = "Counter is not stopped at update event"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(OPM_A::DISABLED)
    }
    #[doc = "Counter stops counting at the next update event (clearing the CEN bit)"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(OPM_A::ENABLED)
    }
}
#[doc = "Field `ARPE` reader - Auto-reload preload enable"]
pub type ARPE_R = crate::BitReader<ARPE_A>;
#[doc = "Auto-reload preload enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ARPE_A {
    #[doc = "0: TIMx_APRR register is not buffered"]
    DISABLED = 0,
    #[doc = "1: TIMx_APRR register is buffered"]
    ENABLED = 1,
}
impl From<ARPE_A> for bool {
    #[inline(always)]
    fn from(variant: ARPE_A) -> Self {
        variant as u8 != 0
    }
}
impl ARPE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ARPE_A {
        match self.bits {
            false => ARPE_A::DISABLED,
            true => ARPE_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ARPE_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ARPE_A::ENABLED
    }
}
#[doc = "Field `ARPE` writer - Auto-reload preload enable"]
pub type ARPE_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O, ARPE_A>;
impl<'a, const O: u8> ARPE_W<'a, O> {
    #[doc = "TIMx_APRR register is not buffered"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ARPE_A::DISABLED)
    }
    #[doc = "TIMx_APRR register is buffered"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ARPE_A::ENABLED)
    }
}
#[doc = "Field `UIFREMAP` reader - UIF status bit remapping"]
pub type UIFREMAP_R = crate::BitReader;
#[doc = "Field `UIFREMAP` writer - UIF status bit remapping"]
pub type UIFREMAP_W<'a, const O: u8> = crate::BitWriter<'a, CR1_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    pub fn cen(&self) -> CEN_R {
        CEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    pub fn udis(&self) -> UDIS_R {
        UDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    pub fn urs(&self) -> URS_R {
        URS_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    pub fn opm(&self) -> OPM_R {
        OPM_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    pub fn arpe(&self) -> ARPE_R {
        ARPE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 11 - UIF status bit remapping"]
    #[inline(always)]
    pub fn uifremap(&self) -> UIFREMAP_R {
        UIFREMAP_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Counter enable"]
    #[inline(always)]
    #[must_use]
    pub fn cen(&mut self) -> CEN_W<0> {
        CEN_W::new(self)
    }
    #[doc = "Bit 1 - Update disable"]
    #[inline(always)]
    #[must_use]
    pub fn udis(&mut self) -> UDIS_W<1> {
        UDIS_W::new(self)
    }
    #[doc = "Bit 2 - Update request source"]
    #[inline(always)]
    #[must_use]
    pub fn urs(&mut self) -> URS_W<2> {
        URS_W::new(self)
    }
    #[doc = "Bit 3 - One-pulse mode"]
    #[inline(always)]
    #[must_use]
    pub fn opm(&mut self) -> OPM_W<3> {
        OPM_W::new(self)
    }
    #[doc = "Bit 7 - Auto-reload preload enable"]
    #[inline(always)]
    #[must_use]
    pub fn arpe(&mut self) -> ARPE_W<7> {
        ARPE_W::new(self)
    }
    #[doc = "Bit 11 - UIF status bit remapping"]
    #[inline(always)]
    #[must_use]
    pub fn uifremap(&mut self) -> UIFREMAP_W<11> {
        UIFREMAP_W::new(self)
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
