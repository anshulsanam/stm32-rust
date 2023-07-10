#[doc = "Register `COMP2_CSR` reader"]
pub struct R(crate::R<COMP2_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP2_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP2_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP2_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP2_CSR` writer"]
pub struct W(crate::W<COMP2_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP2_CSR_SPEC>;
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
impl From<crate::W<COMP2_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP2_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP2EN` reader - Comparator 2 enable"]
pub type COMP2EN_R = crate::BitReader;
#[doc = "Field `COMP2EN` writer - Comparator 2 enable"]
pub type COMP2EN_W<'a, const O: u8> = crate::BitWriter<'a, COMP2_CSR_SPEC, O>;
#[doc = "Field `COMP2MODE` reader - Comparator 2 mode"]
pub type COMP2MODE_R = crate::FieldReader;
#[doc = "Field `COMP2MODE` writer - Comparator 2 mode"]
pub type COMP2MODE_W<'a, const O: u8> = crate::FieldWriter<'a, COMP2_CSR_SPEC, 2, O>;
#[doc = "Field `COMP2INSEL` reader - Comparator 2 inverting input selection"]
pub type COMP2INSEL_R = crate::FieldReader;
#[doc = "Field `COMP2INSEL` writer - Comparator 2 inverting input selection"]
pub type COMP2INSEL_W<'a, const O: u8> = crate::FieldWriter<'a, COMP2_CSR_SPEC, 3, O>;
#[doc = "Field `COMP2INPSEL` reader - Comparator 2 non inverted input selection"]
pub type COMP2INPSEL_R = crate::BitReader;
#[doc = "Field `COMP2INPSEL` writer - Comparator 2 non inverted input selection"]
pub type COMP2INPSEL_W<'a, const O: u8> = crate::BitWriter<'a, COMP2_CSR_SPEC, O>;
#[doc = "Field `COMP2INMSEL` reader - Comparator 1inverting input selection"]
pub type COMP2INMSEL_R = crate::BitReader;
#[doc = "Field `COMP2INMSEL` writer - Comparator 1inverting input selection"]
pub type COMP2INMSEL_W<'a, const O: u8> = crate::BitWriter<'a, COMP2_CSR_SPEC, O>;
#[doc = "Field `COMP2_OUT_SEL` reader - Comparator 2 output selection"]
pub type COMP2_OUT_SEL_R = crate::FieldReader;
#[doc = "Field `COMP2_OUT_SEL` writer - Comparator 2 output selection"]
pub type COMP2_OUT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, COMP2_CSR_SPEC, 4, O>;
#[doc = "Field `COMP2POL` reader - Comparator 2 output polarity"]
pub type COMP2POL_R = crate::BitReader;
#[doc = "Field `COMP2POL` writer - Comparator 2 output polarity"]
pub type COMP2POL_W<'a, const O: u8> = crate::BitWriter<'a, COMP2_CSR_SPEC, O>;
#[doc = "Field `COMP2HYST` reader - Comparator 2 hysteresis"]
pub type COMP2HYST_R = crate::FieldReader;
#[doc = "Field `COMP2HYST` writer - Comparator 2 hysteresis"]
pub type COMP2HYST_W<'a, const O: u8> = crate::FieldWriter<'a, COMP2_CSR_SPEC, 2, O>;
#[doc = "Field `COMP2_BLANKING` reader - Comparator 2 blanking source"]
pub type COMP2_BLANKING_R = crate::FieldReader;
#[doc = "Field `COMP2_BLANKING` writer - Comparator 2 blanking source"]
pub type COMP2_BLANKING_W<'a, const O: u8> = crate::FieldWriter<'a, COMP2_CSR_SPEC, 3, O>;
#[doc = "Field `COMP2LOCK` reader - Comparator 2 lock"]
pub type COMP2LOCK_R = crate::BitReader;
#[doc = "Field `COMP2LOCK` writer - Comparator 2 lock"]
pub type COMP2LOCK_W<'a, const O: u8> = crate::BitWriter<'a, COMP2_CSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Comparator 2 enable"]
    #[inline(always)]
    pub fn comp2en(&self) -> COMP2EN_R {
        COMP2EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 2 mode"]
    #[inline(always)]
    pub fn comp2mode(&self) -> COMP2MODE_R {
        COMP2MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 2 inverting input selection"]
    #[inline(always)]
    pub fn comp2insel(&self) -> COMP2INSEL_R {
        COMP2INSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator 2 non inverted input selection"]
    #[inline(always)]
    pub fn comp2inpsel(&self) -> COMP2INPSEL_R {
        COMP2INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator 1inverting input selection"]
    #[inline(always)]
    pub fn comp2inmsel(&self) -> COMP2INMSEL_R {
        COMP2INMSEL_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 2 output selection"]
    #[inline(always)]
    pub fn comp2_out_sel(&self) -> COMP2_OUT_SEL_R {
        COMP2_OUT_SEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 2 output polarity"]
    #[inline(always)]
    pub fn comp2pol(&self) -> COMP2POL_R {
        COMP2POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis"]
    #[inline(always)]
    pub fn comp2hyst(&self) -> COMP2HYST_R {
        COMP2HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source"]
    #[inline(always)]
    pub fn comp2_blanking(&self) -> COMP2_BLANKING_R {
        COMP2_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    pub fn comp2lock(&self) -> COMP2LOCK_R {
        COMP2LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 2 enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp2en(&mut self) -> COMP2EN_W<0> {
        COMP2EN_W::new(self)
    }
    #[doc = "Bits 2:3 - Comparator 2 mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp2mode(&mut self) -> COMP2MODE_W<2> {
        COMP2MODE_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator 2 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2insel(&mut self) -> COMP2INSEL_W<4> {
        COMP2INSEL_W::new(self)
    }
    #[doc = "Bit 7 - Comparator 2 non inverted input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2inpsel(&mut self) -> COMP2INPSEL_W<7> {
        COMP2INPSEL_W::new(self)
    }
    #[doc = "Bit 9 - Comparator 1inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2inmsel(&mut self) -> COMP2INMSEL_W<9> {
        COMP2INMSEL_W::new(self)
    }
    #[doc = "Bits 10:13 - Comparator 2 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp2_out_sel(&mut self) -> COMP2_OUT_SEL_W<10> {
        COMP2_OUT_SEL_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 2 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn comp2pol(&mut self) -> COMP2POL_W<15> {
        COMP2POL_W::new(self)
    }
    #[doc = "Bits 16:17 - Comparator 2 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn comp2hyst(&mut self) -> COMP2HYST_W<16> {
        COMP2HYST_W::new(self)
    }
    #[doc = "Bits 18:20 - Comparator 2 blanking source"]
    #[inline(always)]
    #[must_use]
    pub fn comp2_blanking(&mut self) -> COMP2_BLANKING_W<18> {
        COMP2_BLANKING_W::new(self)
    }
    #[doc = "Bit 31 - Comparator 2 lock"]
    #[inline(always)]
    #[must_use]
    pub fn comp2lock(&mut self) -> COMP2LOCK_W<31> {
        COMP2LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp2_csr](index.html) module"]
pub struct COMP2_CSR_SPEC;
impl crate::RegisterSpec for COMP2_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp2_csr::R](R) reader structure"]
impl crate::Readable for COMP2_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp2_csr::W](W) writer structure"]
impl crate::Writable for COMP2_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP2_CSR to value 0"]
impl crate::Resettable for COMP2_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
