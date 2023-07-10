#[doc = "Register `COMP6_CSR` reader"]
pub struct R(crate::R<COMP6_CSR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<COMP6_CSR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<COMP6_CSR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<COMP6_CSR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `COMP6_CSR` writer"]
pub struct W(crate::W<COMP6_CSR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<COMP6_CSR_SPEC>;
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
impl From<crate::W<COMP6_CSR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<COMP6_CSR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `COMP6EN` reader - Comparator 6 enable"]
pub type COMP6EN_R = crate::BitReader;
#[doc = "Field `COMP6EN` writer - Comparator 6 enable"]
pub type COMP6EN_W<'a, const O: u8> = crate::BitWriter<'a, COMP6_CSR_SPEC, O>;
#[doc = "Field `COMP6MODE` reader - Comparator 6 mode"]
pub type COMP6MODE_R = crate::FieldReader;
#[doc = "Field `COMP6MODE` writer - Comparator 6 mode"]
pub type COMP6MODE_W<'a, const O: u8> = crate::FieldWriter<'a, COMP6_CSR_SPEC, 2, O>;
#[doc = "Field `COMP6INSEL` reader - Comparator 6 inverting input selection"]
pub type COMP6INSEL_R = crate::FieldReader;
#[doc = "Field `COMP6INSEL` writer - Comparator 6 inverting input selection"]
pub type COMP6INSEL_W<'a, const O: u8> = crate::FieldWriter<'a, COMP6_CSR_SPEC, 3, O>;
#[doc = "Field `COMP6INPSEL` reader - Comparator 6 non inverted input selection"]
pub type COMP6INPSEL_R = crate::BitReader;
#[doc = "Field `COMP6INPSEL` writer - Comparator 6 non inverted input selection"]
pub type COMP6INPSEL_W<'a, const O: u8> = crate::BitWriter<'a, COMP6_CSR_SPEC, O>;
#[doc = "Field `COM6WINMODE` reader - Comparator 6 window mode"]
pub type COM6WINMODE_R = crate::BitReader;
#[doc = "Field `COM6WINMODE` writer - Comparator 6 window mode"]
pub type COM6WINMODE_W<'a, const O: u8> = crate::BitWriter<'a, COMP6_CSR_SPEC, O>;
#[doc = "Field `COMP6_OUT_SEL` reader - Comparator 6 output selection"]
pub type COMP6_OUT_SEL_R = crate::FieldReader;
#[doc = "Field `COMP6_OUT_SEL` writer - Comparator 6 output selection"]
pub type COMP6_OUT_SEL_W<'a, const O: u8> = crate::FieldWriter<'a, COMP6_CSR_SPEC, 4, O>;
#[doc = "Field `COMP6POL` reader - Comparator 6 output polarity"]
pub type COMP6POL_R = crate::BitReader;
#[doc = "Field `COMP6POL` writer - Comparator 6 output polarity"]
pub type COMP6POL_W<'a, const O: u8> = crate::BitWriter<'a, COMP6_CSR_SPEC, O>;
#[doc = "Field `COMP6HYST` reader - Comparator 6 hysteresis"]
pub type COMP6HYST_R = crate::FieldReader;
#[doc = "Field `COMP6HYST` writer - Comparator 6 hysteresis"]
pub type COMP6HYST_W<'a, const O: u8> = crate::FieldWriter<'a, COMP6_CSR_SPEC, 2, O>;
#[doc = "Field `COMP6_BLANKING` reader - Comparator 6 blanking source"]
pub type COMP6_BLANKING_R = crate::FieldReader;
#[doc = "Field `COMP6_BLANKING` writer - Comparator 6 blanking source"]
pub type COMP6_BLANKING_W<'a, const O: u8> = crate::FieldWriter<'a, COMP6_CSR_SPEC, 3, O>;
#[doc = "Field `COMP6OUT` reader - Comparator 6 output"]
pub type COMP6OUT_R = crate::BitReader;
#[doc = "Field `COMP6LOCK` reader - Comparator 6 lock"]
pub type COMP6LOCK_R = crate::BitReader;
#[doc = "Field `COMP6LOCK` writer - Comparator 6 lock"]
pub type COMP6LOCK_W<'a, const O: u8> = crate::BitWriter<'a, COMP6_CSR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Comparator 6 enable"]
    #[inline(always)]
    pub fn comp6en(&self) -> COMP6EN_R {
        COMP6EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 2:3 - Comparator 6 mode"]
    #[inline(always)]
    pub fn comp6mode(&self) -> COMP6MODE_R {
        COMP6MODE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:6 - Comparator 6 inverting input selection"]
    #[inline(always)]
    pub fn comp6insel(&self) -> COMP6INSEL_R {
        COMP6INSEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - Comparator 6 non inverted input selection"]
    #[inline(always)]
    pub fn comp6inpsel(&self) -> COMP6INPSEL_R {
        COMP6INPSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Comparator 6 window mode"]
    #[inline(always)]
    pub fn com6winmode(&self) -> COM6WINMODE_R {
        COM6WINMODE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:13 - Comparator 6 output selection"]
    #[inline(always)]
    pub fn comp6_out_sel(&self) -> COMP6_OUT_SEL_R {
        COMP6_OUT_SEL_R::new(((self.bits >> 10) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Comparator 6 output polarity"]
    #[inline(always)]
    pub fn comp6pol(&self) -> COMP6POL_R {
        COMP6POL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:17 - Comparator 6 hysteresis"]
    #[inline(always)]
    pub fn comp6hyst(&self) -> COMP6HYST_R {
        COMP6HYST_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:20 - Comparator 6 blanking source"]
    #[inline(always)]
    pub fn comp6_blanking(&self) -> COMP6_BLANKING_R {
        COMP6_BLANKING_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bit 30 - Comparator 6 output"]
    #[inline(always)]
    pub fn comp6out(&self) -> COMP6OUT_R {
        COMP6OUT_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Comparator 6 lock"]
    #[inline(always)]
    pub fn comp6lock(&self) -> COMP6LOCK_R {
        COMP6LOCK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Comparator 6 enable"]
    #[inline(always)]
    #[must_use]
    pub fn comp6en(&mut self) -> COMP6EN_W<0> {
        COMP6EN_W::new(self)
    }
    #[doc = "Bits 2:3 - Comparator 6 mode"]
    #[inline(always)]
    #[must_use]
    pub fn comp6mode(&mut self) -> COMP6MODE_W<2> {
        COMP6MODE_W::new(self)
    }
    #[doc = "Bits 4:6 - Comparator 6 inverting input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp6insel(&mut self) -> COMP6INSEL_W<4> {
        COMP6INSEL_W::new(self)
    }
    #[doc = "Bit 7 - Comparator 6 non inverted input selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp6inpsel(&mut self) -> COMP6INPSEL_W<7> {
        COMP6INPSEL_W::new(self)
    }
    #[doc = "Bit 9 - Comparator 6 window mode"]
    #[inline(always)]
    #[must_use]
    pub fn com6winmode(&mut self) -> COM6WINMODE_W<9> {
        COM6WINMODE_W::new(self)
    }
    #[doc = "Bits 10:13 - Comparator 6 output selection"]
    #[inline(always)]
    #[must_use]
    pub fn comp6_out_sel(&mut self) -> COMP6_OUT_SEL_W<10> {
        COMP6_OUT_SEL_W::new(self)
    }
    #[doc = "Bit 15 - Comparator 6 output polarity"]
    #[inline(always)]
    #[must_use]
    pub fn comp6pol(&mut self) -> COMP6POL_W<15> {
        COMP6POL_W::new(self)
    }
    #[doc = "Bits 16:17 - Comparator 6 hysteresis"]
    #[inline(always)]
    #[must_use]
    pub fn comp6hyst(&mut self) -> COMP6HYST_W<16> {
        COMP6HYST_W::new(self)
    }
    #[doc = "Bits 18:20 - Comparator 6 blanking source"]
    #[inline(always)]
    #[must_use]
    pub fn comp6_blanking(&mut self) -> COMP6_BLANKING_W<18> {
        COMP6_BLANKING_W::new(self)
    }
    #[doc = "Bit 31 - Comparator 6 lock"]
    #[inline(always)]
    #[must_use]
    pub fn comp6lock(&mut self) -> COMP6LOCK_W<31> {
        COMP6LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "control and status register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [comp6_csr](index.html) module"]
pub struct COMP6_CSR_SPEC;
impl crate::RegisterSpec for COMP6_CSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [comp6_csr::R](R) reader structure"]
impl crate::Readable for COMP6_CSR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [comp6_csr::W](W) writer structure"]
impl crate::Writable for COMP6_CSR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets COMP6_CSR to value 0"]
impl crate::Resettable for COMP6_CSR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
