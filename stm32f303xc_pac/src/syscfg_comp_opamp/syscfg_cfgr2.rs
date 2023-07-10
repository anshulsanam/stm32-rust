#[doc = "Register `SYSCFG_CFGR2` reader"]
pub struct R(crate::R<SYSCFG_CFGR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_CFGR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_CFGR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_CFGR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG_CFGR2` writer"]
pub struct W(crate::W<SYSCFG_CFGR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_CFGR2_SPEC>;
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
impl From<crate::W<SYSCFG_CFGR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_CFGR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LOCUP_LOCK` reader - Cortex-M0 LOCKUP bit enable bit"]
pub type LOCUP_LOCK_R = crate::BitReader;
#[doc = "Field `LOCUP_LOCK` writer - Cortex-M0 LOCKUP bit enable bit"]
pub type LOCUP_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR2_SPEC, O>;
#[doc = "Field `SRAM_PARITY_LOCK` reader - SRAM parity lock bit"]
pub type SRAM_PARITY_LOCK_R = crate::BitReader;
#[doc = "Field `SRAM_PARITY_LOCK` writer - SRAM parity lock bit"]
pub type SRAM_PARITY_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR2_SPEC, O>;
#[doc = "Field `PVD_LOCK` reader - PVD lock enable bit"]
pub type PVD_LOCK_R = crate::BitReader;
#[doc = "Field `PVD_LOCK` writer - PVD lock enable bit"]
pub type PVD_LOCK_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR2_SPEC, O>;
#[doc = "Field `BYP_ADD_PAR` reader - Bypass address bit 29 in parity calculation"]
pub type BYP_ADD_PAR_R = crate::BitReader;
#[doc = "Field `BYP_ADD_PAR` writer - Bypass address bit 29 in parity calculation"]
pub type BYP_ADD_PAR_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR2_SPEC, O>;
#[doc = "Field `SRAM_PEF` reader - SRAM parity flag"]
pub type SRAM_PEF_R = crate::BitReader;
#[doc = "Field `SRAM_PEF` writer - SRAM parity flag"]
pub type SRAM_PEF_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR2_SPEC, O>;
impl R {
    #[doc = "Bit 0 - Cortex-M0 LOCKUP bit enable bit"]
    #[inline(always)]
    pub fn locup_lock(&self) -> LOCUP_LOCK_R {
        LOCUP_LOCK_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    pub fn sram_parity_lock(&self) -> SRAM_PARITY_LOCK_R {
        SRAM_PARITY_LOCK_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    pub fn pvd_lock(&self) -> PVD_LOCK_R {
        PVD_LOCK_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Bypass address bit 29 in parity calculation"]
    #[inline(always)]
    pub fn byp_add_par(&self) -> BYP_ADD_PAR_R {
        BYP_ADD_PAR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - SRAM parity flag"]
    #[inline(always)]
    pub fn sram_pef(&self) -> SRAM_PEF_R {
        SRAM_PEF_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Cortex-M0 LOCKUP bit enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn locup_lock(&mut self) -> LOCUP_LOCK_W<0> {
        LOCUP_LOCK_W::new(self)
    }
    #[doc = "Bit 1 - SRAM parity lock bit"]
    #[inline(always)]
    #[must_use]
    pub fn sram_parity_lock(&mut self) -> SRAM_PARITY_LOCK_W<1> {
        SRAM_PARITY_LOCK_W::new(self)
    }
    #[doc = "Bit 2 - PVD lock enable bit"]
    #[inline(always)]
    #[must_use]
    pub fn pvd_lock(&mut self) -> PVD_LOCK_W<2> {
        PVD_LOCK_W::new(self)
    }
    #[doc = "Bit 4 - Bypass address bit 29 in parity calculation"]
    #[inline(always)]
    #[must_use]
    pub fn byp_add_par(&mut self) -> BYP_ADD_PAR_W<4> {
        BYP_ADD_PAR_W::new(self)
    }
    #[doc = "Bit 8 - SRAM parity flag"]
    #[inline(always)]
    #[must_use]
    pub fn sram_pef(&mut self) -> SRAM_PEF_W<8> {
        SRAM_PEF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_cfgr2](index.html) module"]
pub struct SYSCFG_CFGR2_SPEC;
impl crate::RegisterSpec for SYSCFG_CFGR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_cfgr2::R](R) reader structure"]
impl crate::Readable for SYSCFG_CFGR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg_cfgr2::W](W) writer structure"]
impl crate::Writable for SYSCFG_CFGR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG_CFGR2 to value 0"]
impl crate::Resettable for SYSCFG_CFGR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
