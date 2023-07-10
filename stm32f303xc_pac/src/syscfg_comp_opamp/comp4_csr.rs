#[doc = "Register `COMP4_CSR` reader"]
pub struct R(crate::R<COMP4_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP4_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP4_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP4_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP4_CSR` writer"]
pub struct W(crate::W<COMP4_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP4_CSR_SPEC>;
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
impl From<crate::W<COMP4_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP4_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP4EN` reader - Comparator 4 enable"]
pub type COMP4EN_R = crate::BitReader;
#[doc = "Field `COMP4EN` writer - Comparator 4 enable"]
pub type COMP4EN_W<'a, const O: u8> = crate::BitWriter<'a, COMP4_CSR_SPEC, O>;
#[doc = "Field `COMP4MODE` reader - Comparator 4 mode"]
pub type COMP4MODE_R = crate::FieldReader;
#[doc = "Field `COMP4MODE` writer - Comparator 4 mode"]
pub type COMP4MODE_W<'a, const O: u8> = crate::FieldWriter<'a, COMP4_CSR_SPEC, 2, O>;
#[doc = "Field `COMP4INSEL` reader - Comparator 4 inverting input selection"]
pub type COMP4INSEL_R = crate::FieldReader;
#[doc = "Field `COMP4INSEL` writer - Comparator 4 inverting input selection"]
pub type COMP4INSEL_W<'a, const O: u8> = crate::FieldWriter<'a, COMP4_CSR_SPEC, 3, O>;
#[doc = "Field `COMP4INPSEL` reader - Comparator 4 non inverted input selection"]
pub type COMP4INPSEL_R = crate::BitReader;
#[doc = "Field `COMP4INPSEL` writer - Comparator 4 non inverted input selection"]
pub type COMP4INPSEL_W<'a, const O: u8> = crate::BitWriter<'a, COMP4_CSR_SPEC, O>;
#[doc = "Field `COM4WINMODE` reader - Comparator 4 window mode"]
pub type COM4WINMODE_R = crate::BitReader;
#[doc = "Field `COM4WINMODE` writer - Comparator 4 window mode"]
pub type COM4WINMODE_W<'a, const O: u8> = crate::BitWriter<'a, COMP4_CSR_SPEC, O>;
#[doc = "Field `COMP4_OUT_SEL` reader - Comparator 4 output selection"]
pub type COMP4_OUT_SEL_R = crate::FieldReader;
#[doc = "Field `COMP4_OUT_SEL` writer - Comparator 4 output selection"]
pub type COMP4_OUT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, COMP4_CSR_SPEC, 4, O>;
#[doc = "Field `COMP4POL` reader - Comparator 4 output polarity"]
pub type COMP4POL_R = crate::BitReader;
#[doc = "Field `COMP4POL` writer - Comparator 4 output polarity"]
pub type COMP4POL_W<'a, const O: u8> = crate::BitWriter<'a, COMP4_CSR_SPEC, O>;
#[doc = "Field `COMP4HYST` reader - Comparator 4 hysteresis"]
pub type COMP4HYST_R = crate::FieldReader;
#[doc = "Field `COMP4HYST` writer - Comparator 4 hysteresis"]
pub type COMP4HYST_W<'a, const O: u8> = crate::FieldWriter<'a, COMP4_CSR_SPEC, 2, O>;
#[doc = "Field `COMP4_BLANKING` reader - Comparator 4 blanking source"]
pub type COMP4_BLANKING_R = crate::FieldReader;
#[doc = "Field `COMP4_BLANKING` writer - Comparator 4 blanking source"]
pub type COMP4_BLANKING_W<'a, const O: u8> = crate::FieldWriter<'a, COMP4_CSR_SPEC, 3, O>;
#[doc = "Field `COMP4OUT` reader - Comparator 4 output"]
pub type COMP4OUT_R = crate::BitReader;
#[doc = "Field `COMP4LOCK` reader - Comparator 4 lock"]
pub type COMP4LOCK_R = crate::BitReader;
#[doc = "Field `COMP4LOCK` writer - Comparator 4 lock"]
pub type COMP4LOCK_W<'a, const O: u8> = crate::BitWriter<'a, COMP4_CSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Comparator 4 enable"]
    #[inline(always)]
    pub fn comp4en(&self) -> COMP4EN_R {
        COMP4EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 4 mode"]
    #[inline(always)]
    pub fn comp4mode(&self) -> COMP4MODE_R {
        COMP4MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 4 inverting input selection"]
    #[inline(always)]
    pub fn comp4insel(&self) -> COMP4INSEL_R {
        COMP4INSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator 4 non inverted input selection"]
    #[inline(always)]
    pub fn comp4inpsel(&self) -> COMP4INPSEL_R {
        COMP4INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator 4 window mode"]
    #[inline(always)]
    pub fn com4winmode(&self) -> COM4WINMODE_R {
        COM4WINMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 4 output selection"]
    #[inline(always)]
    pub fn comp4_out_sel(&self) -> COMP4_OUT_SEL_R {
        COMP4_OUT_SEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 4 output polarity"]
    #[inline(always)]
    pub fn comp4pol(&self) -> COMP4POL_R {
        COMP4POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 4 hysteresis"]
    #[inline(always)]
    pub fn comp4hyst(&self) -> COMP4HYST_R {
        COMP4HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 4 blanking source"]
    #[inline(always)]
    pub fn comp4_blanking(&self) -> COMP4_BLANKING_R {
        COMP4_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator 4 output"]
    #[inline(always)]
    pub fn comp4out(&self) -> COMP4OUT_R {
        COMP4OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 4 lock"]
    #[inline(always)]
    pub fn comp4lock(&self) -> COMP4LOCK_R {
        COMP4LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 4 enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp4en(&mut self) -> COMP4EN_W<0> {
        COMP4EN_W::new(self)
    }
    #[doc = "Bits 2:3 - Comparator 4 mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp4mode(&mut self) -> COMP4MODE_W<2> {
        COMP4MODE_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator 4 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp4insel(&mut self) -> COMP4INSEL_W<4> {
        COMP4INSEL_W::new(self)
    }
    #[doc = "Bit 7 - Comparator 4 non inverted input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp4inpsel(&mut self) -> COMP4INPSEL_W<7> {
        COMP4INPSEL_W::new(self)
    }
    #[doc = "Bit 9 - Comparator 4 window mode"]
    #[inline(always)]
    #[must_use]
    pub fn com4winmode(&mut self) -> COM4WINMODE_W<9> {
        COM4WINMODE_W::new(self)
    }
    #[doc = "Bits 10:13 - Comparator 4 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp4_out_sel(&mut self) -> COMP4_OUT_SEL_W<10> {
        COMP4_OUT_SEL_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 4 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn comp4pol(&mut self) -> COMP4POL_W<15> {
        COMP4POL_W::new(self)
    }
    #[doc = "Bits 16:17 - Comparator 4 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn comp4hyst(&mut self) -> COMP4HYST_W<16> {
        COMP4HYST_W::new(self)
    }
    #[doc = "Bits 18:20 - Comparator 4 blanking source"]
    #[inline(always)]
    #[must_use]
    pub fn comp4_blanking(&mut self) -> COMP4_BLANKING_W<18> {
        COMP4_BLANKING_W::new(self)
    }
    #[doc = "Bit 31 - Comparator 4 lock"]
    #[inline(always)]
    #[must_use]
    pub fn comp4lock(&mut self) -> COMP4LOCK_W<31> {
        COMP4LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp4_csr](index.html) module"]
pub struct COMP4_CSR_SPEC;
impl crate::RegisterSpec for COMP4_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp4_csr::R](R) reader structure"]
impl crate::Readable for COMP4_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp4_csr::W](W) writer structure"]
impl crate::Writable for COMP4_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP4_CSR to value 0"]
impl crate::Resettable for COMP4_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
