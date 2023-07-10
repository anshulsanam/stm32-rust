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
#[doc = "Field `ADEN` reader - ADEN"]
pub type ADEN_R = crate::BitReader<ADENW_A>;
#[doc = "ADEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADENW_A {
    #[doc = "1: Enable ADC"]
    ENABLE = 1,
}
impl From<ADENW_A> for bool {
    #[inline(always)]
    fn from(variant: ADENW_A) -> Self {
        variant as u8 != 0
    }
}
impl ADEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADENW_A> {
        match self.bits {
            true => Some(ADENW_A::ENABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `ENABLE`"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == ADENW_A::ENABLE
    }
}
#[doc = "Field `ADEN` writer - ADEN"]
pub type ADEN_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, ADENW_A>;
impl<'a, const O: u8> ADEN_W<'a, O> {
    #[doc = "Enable ADC"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut W {
        self.variant(ADENW_A::ENABLE)
    }
}
#[doc = "Field `ADDIS` reader - ADDIS"]
pub type ADDIS_R = crate::BitReader<ADDISW_A>;
#[doc = "ADDIS\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADDISW_A {
    #[doc = "0: Disable ADC conversion and go to power down mode"]
    DISABLE = 0,
}
impl From<ADDISW_A> for bool {
    #[inline(always)]
    fn from(variant: ADDISW_A) -> Self {
        variant as u8 != 0
    }
}
impl ADDIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADDISW_A> {
        match self.bits {
            false => Some(ADDISW_A::DISABLE),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLE`"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == ADDISW_A::DISABLE
    }
}
#[doc = "Field `ADDIS` writer - ADDIS"]
pub type ADDIS_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, ADDISW_A>;
impl<'a, const O: u8> ADDIS_W<'a, O> {
    #[doc = "Disable ADC conversion and go to power down mode"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut W {
        self.variant(ADDISW_A::DISABLE)
    }
}
#[doc = "Field `ADSTART` reader - ADSTART"]
pub type ADSTART_R = crate::BitReader<ADSTART_A>;
#[doc = "ADSTART\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTART_A {
    #[doc = "1: Starts conversion of channel"]
    START = 1,
}
impl From<ADSTART_A> for bool {
    #[inline(always)]
    fn from(variant: ADSTART_A) -> Self {
        variant as u8 != 0
    }
}
impl ADSTART_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADSTART_A> {
        match self.bits {
            true => Some(ADSTART_A::START),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == ADSTART_A::START
    }
}
#[doc = "Field `ADSTART` writer - ADSTART"]
pub type ADSTART_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, ADSTART_A>;
impl<'a, const O: u8> ADSTART_W<'a, O> {
    #[doc = "Starts conversion of channel"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(ADSTART_A::START)
    }
}
#[doc = "Field `JADSTART` reader - JADSTART"]
pub use ADSTART_R as JADSTART_R;
#[doc = "Field `JADSTART` writer - JADSTART"]
pub use ADSTART_W as JADSTART_W;
#[doc = "Field `ADSTP` reader - ADSTP"]
pub type ADSTP_R = crate::BitReader<ADSTP_A>;
#[doc = "ADSTP\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADSTP_A {
    #[doc = "1: Stop conversion of channel"]
    STOP = 1,
}
impl From<ADSTP_A> for bool {
    #[inline(always)]
    fn from(variant: ADSTP_A) -> Self {
        variant as u8 != 0
    }
}
impl ADSTP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADSTP_A> {
        match self.bits {
            true => Some(ADSTP_A::STOP),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == ADSTP_A::STOP
    }
}
#[doc = "Field `ADSTP` writer - ADSTP"]
pub type ADSTP_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, ADSTP_A>;
impl<'a, const O: u8> ADSTP_W<'a, O> {
    #[doc = "Stop conversion of channel"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(ADSTP_A::STOP)
    }
}
#[doc = "Field `JADSTP` reader - JADSTP"]
pub use ADSTP_R as JADSTP_R;
#[doc = "Field `JADSTP` writer - JADSTP"]
pub use ADSTP_W as JADSTP_W;
#[doc = "Field `ADVREGEN` reader - ADVREGEN"]
pub type ADVREGEN_R = crate::FieldReader<ADVREGEN_A>;
#[doc = "ADVREGEN\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADVREGEN_A {
    #[doc = "0: Intermediate state required when moving the ADC voltage regulator between states"]
    INTERMEDIATE = 0,
    #[doc = "1: ADC voltage regulator enabled"]
    ENABLED = 1,
    #[doc = "2: ADC voltage regulator disabled"]
    DISABLED = 2,
}
impl From<ADVREGEN_A> for u8 {
    #[inline(always)]
    fn from(variant: ADVREGEN_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ADVREGEN_A {
    type Ux = u8;
}
impl ADVREGEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<ADVREGEN_A> {
        match self.bits {
            0 => Some(ADVREGEN_A::INTERMEDIATE),
            1 => Some(ADVREGEN_A::ENABLED),
            2 => Some(ADVREGEN_A::DISABLED),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INTERMEDIATE`"]
    #[inline(always)]
    pub fn is_intermediate(&self) -> bool {
        *self == ADVREGEN_A::INTERMEDIATE
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == ADVREGEN_A::ENABLED
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == ADVREGEN_A::DISABLED
    }
}
#[doc = "Field `ADVREGEN` writer - ADVREGEN"]
pub type ADVREGEN_W<'a, const O: u8> = crate::FieldWriter<'a, CR_SPEC, 2, O, ADVREGEN_A>;
impl<'a, const O: u8> ADVREGEN_W<'a, O> {
    #[doc = "Intermediate state required when moving the ADC voltage regulator between states"]
    #[inline(always)]
    pub fn intermediate(self) -> &'a mut W {
        self.variant(ADVREGEN_A::INTERMEDIATE)
    }
    #[doc = "ADC voltage regulator enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(ADVREGEN_A::ENABLED)
    }
    #[doc = "ADC voltage regulator disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(ADVREGEN_A::DISABLED)
    }
}
#[doc = "Field `ADCALDIF` reader - ADCALDIF"]
pub type ADCALDIF_R = crate::BitReader<ADCALDIF_A>;
#[doc = "ADCALDIF\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCALDIF_A {
    #[doc = "0: Calibration for single-ended mode"]
    SINGLE_ENDED = 0,
    #[doc = "1: Calibration for differential mode"]
    DIFFERENTIAL = 1,
}
impl From<ADCALDIF_A> for bool {
    #[inline(always)]
    fn from(variant: ADCALDIF_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCALDIF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCALDIF_A {
        match self.bits {
            false => ADCALDIF_A::SINGLE_ENDED,
            true => ADCALDIF_A::DIFFERENTIAL,
        }
    }
    #[doc = "Checks if the value of the field is `SINGLE_ENDED`"]
    #[inline(always)]
    pub fn is_single_ended(&self) -> bool {
        *self == ADCALDIF_A::SINGLE_ENDED
    }
    #[doc = "Checks if the value of the field is `DIFFERENTIAL`"]
    #[inline(always)]
    pub fn is_differential(&self) -> bool {
        *self == ADCALDIF_A::DIFFERENTIAL
    }
}
#[doc = "Field `ADCALDIF` writer - ADCALDIF"]
pub type ADCALDIF_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, ADCALDIF_A>;
impl<'a, const O: u8> ADCALDIF_W<'a, O> {
    #[doc = "Calibration for single-ended mode"]
    #[inline(always)]
    pub fn single_ended(self) -> &'a mut W {
        self.variant(ADCALDIF_A::SINGLE_ENDED)
    }
    #[doc = "Calibration for differential mode"]
    #[inline(always)]
    pub fn differential(self) -> &'a mut W {
        self.variant(ADCALDIF_A::DIFFERENTIAL)
    }
}
#[doc = "Field `ADCAL` reader - ADCAL"]
pub type ADCAL_R = crate::BitReader<ADCAL_A>;
#[doc = "ADCAL\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADCAL_A {
    #[doc = "0: Calibration complete"]
    COMPLETE = 0,
    #[doc = "1: Start the calibration of the ADC"]
    CALIBRATION = 1,
}
impl From<ADCAL_A> for bool {
    #[inline(always)]
    fn from(variant: ADCAL_A) -> Self {
        variant as u8 != 0
    }
}
impl ADCAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCAL_A {
        match self.bits {
            false => ADCAL_A::COMPLETE,
            true => ADCAL_A::CALIBRATION,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == ADCAL_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `CALIBRATION`"]
    #[inline(always)]
    pub fn is_calibration(&self) -> bool {
        *self == ADCAL_A::CALIBRATION
    }
}
#[doc = "Field `ADCAL` writer - ADCAL"]
pub type ADCAL_W<'a, const O: u8> = crate::BitWriter<'a, CR_SPEC, O, ADCAL_A>;
impl<'a, const O: u8> ADCAL_W<'a, O> {
    #[doc = "Calibration complete"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(ADCAL_A::COMPLETE)
    }
    #[doc = "Start the calibration of the ADC"]
    #[inline(always)]
    pub fn calibration(self) -> &'a mut W {
        self.variant(ADCAL_A::CALIBRATION)
    }
}
impl R {
    #[doc = "Bit 0 - ADEN"]
    #[inline(always)]
    pub fn aden(&self) -> ADEN_R {
        ADEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADDIS"]
    #[inline(always)]
    pub fn addis(&self) -> ADDIS_R {
        ADDIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - ADSTART"]
    #[inline(always)]
    pub fn adstart(&self) -> ADSTART_R {
        ADSTART_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - JADSTART"]
    #[inline(always)]
    pub fn jadstart(&self) -> JADSTART_R {
        JADSTART_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - ADSTP"]
    #[inline(always)]
    pub fn adstp(&self) -> ADSTP_R {
        ADSTP_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - JADSTP"]
    #[inline(always)]
    pub fn jadstp(&self) -> JADSTP_R {
        JADSTP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bits 28:29 - ADVREGEN"]
    #[inline(always)]
    pub fn advregen(&self) -> ADVREGEN_R {
        ADVREGEN_R::new(((self.bits >> 28) & 3) as u8)
    }
    #[doc = "Bit 30 - ADCALDIF"]
    #[inline(always)]
    pub fn adcaldif(&self) -> ADCALDIF_R {
        ADCALDIF_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - ADCAL"]
    #[inline(always)]
    pub fn adcal(&self) -> ADCAL_R {
        ADCAL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - ADEN"]
    #[inline(always)]
    #[must_use]
    pub fn aden(&mut self) -> ADEN_W<0> {
        ADEN_W::new(self)
    }
    #[doc = "Bit 1 - ADDIS"]
    #[inline(always)]
    #[must_use]
    pub fn addis(&mut self) -> ADDIS_W<1> {
        ADDIS_W::new(self)
    }
    #[doc = "Bit 2 - ADSTART"]
    #[inline(always)]
    #[must_use]
    pub fn adstart(&mut self) -> ADSTART_W<2> {
        ADSTART_W::new(self)
    }
    #[doc = "Bit 3 - JADSTART"]
    #[inline(always)]
    #[must_use]
    pub fn jadstart(&mut self) -> JADSTART_W<3> {
        JADSTART_W::new(self)
    }
    #[doc = "Bit 4 - ADSTP"]
    #[inline(always)]
    #[must_use]
    pub fn adstp(&mut self) -> ADSTP_W<4> {
        ADSTP_W::new(self)
    }
    #[doc = "Bit 5 - JADSTP"]
    #[inline(always)]
    #[must_use]
    pub fn jadstp(&mut self) -> JADSTP_W<5> {
        JADSTP_W::new(self)
    }
    #[doc = "Bits 28:29 - ADVREGEN"]
    #[inline(always)]
    #[must_use]
    pub fn advregen(&mut self) -> ADVREGEN_W<28> {
        ADVREGEN_W::new(self)
    }
    #[doc = "Bit 30 - ADCALDIF"]
    #[inline(always)]
    #[must_use]
    pub fn adcaldif(&mut self) -> ADCALDIF_W<30> {
        ADCALDIF_W::new(self)
    }
    #[doc = "Bit 31 - ADCAL"]
    #[inline(always)]
    #[must_use]
    pub fn adcal(&mut self) -> ADCAL_W<31> {
        ADCAL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr](index.html) module"]
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
