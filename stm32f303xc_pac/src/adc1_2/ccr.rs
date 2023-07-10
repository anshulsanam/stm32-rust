#[doc = "Register `CCR` reader"]
pub struct R(crate::R<CCR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CCR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CCR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CCR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CCR` writer"]
pub struct W(crate::W<CCR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CCR_SPEC>;
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
impl From<crate::W<CCR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CCR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DUAL` reader - Dual ADC mode selection"]
pub type DUAL_R = crate::FieldReader<DUAL_A>;
#[doc = "Dual ADC mode selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DUAL_A {
    #[doc = "0: Independent mode"]
    INDEPENDENT = 0,
    #[doc = "1: Dual, combined regular simultaneous + injected simultaneous mode"]
    DUAL_RJ = 1,
    #[doc = "2: Dual, combined regular simultaneous + alternate trigger mode"]
    DUAL_RA = 2,
    #[doc = "3: Dual, combined interleaved mode + injected simultaneous mode"]
    DUAL_IJ = 3,
    #[doc = "5: Dual, injected simultaneous mode only"]
    DUAL_J = 5,
    #[doc = "6: Dual, regular simultaneous mode only"]
    DUAL_R = 6,
    #[doc = "7: Dual, interleaved mode only"]
    DUAL_I = 7,
    #[doc = "9: Dual, alternate trigger mode only"]
    DUAL_A = 9,
}
impl From<DUAL_A> for u8 {
    #[inline(always)]
    fn from(variant: DUAL_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for DUAL_A {
    type Ux = u8;
}
impl DUAL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<DUAL_A> {
        match self.bits {
            0 => Some(DUAL_A::INDEPENDENT),
            1 => Some(DUAL_A::DUAL_RJ),
            2 => Some(DUAL_A::DUAL_RA),
            3 => Some(DUAL_A::DUAL_IJ),
            5 => Some(DUAL_A::DUAL_J),
            6 => Some(DUAL_A::DUAL_R),
            7 => Some(DUAL_A::DUAL_I),
            9 => Some(DUAL_A::DUAL_A),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `INDEPENDENT`"]
    #[inline(always)]
    pub fn is_independent(&self) -> bool {
        *self == DUAL_A::INDEPENDENT
    }
    #[doc = "Checks if the value of the field is `DUAL_RJ`"]
    #[inline(always)]
    pub fn is_dual_rj(&self) -> bool {
        *self == DUAL_A::DUAL_RJ
    }
    #[doc = "Checks if the value of the field is `DUAL_RA`"]
    #[inline(always)]
    pub fn is_dual_ra(&self) -> bool {
        *self == DUAL_A::DUAL_RA
    }
    #[doc = "Checks if the value of the field is `DUAL_IJ`"]
    #[inline(always)]
    pub fn is_dual_ij(&self) -> bool {
        *self == DUAL_A::DUAL_IJ
    }
    #[doc = "Checks if the value of the field is `DUAL_J`"]
    #[inline(always)]
    pub fn is_dual_j(&self) -> bool {
        *self == DUAL_A::DUAL_J
    }
    #[doc = "Checks if the value of the field is `DUAL_R`"]
    #[inline(always)]
    pub fn is_dual_r(&self) -> bool {
        *self == DUAL_A::DUAL_R
    }
    #[doc = "Checks if the value of the field is `DUAL_I`"]
    #[inline(always)]
    pub fn is_dual_i(&self) -> bool {
        *self == DUAL_A::DUAL_I
    }
    #[doc = "Checks if the value of the field is `DUAL_A`"]
    #[inline(always)]
    pub fn is_dual_a(&self) -> bool {
        *self == DUAL_A::DUAL_A
    }
}
#[doc = "Field `DUAL` writer - Dual ADC mode selection"]
pub type DUAL_W<'a, const O: u8> = crate::FieldWriter<'a, CCR_SPEC, 5, O, DUAL_A>;
impl<'a, const O: u8> DUAL_W<'a, O> {
    #[doc = "Independent mode"]
    #[inline(always)]
    pub fn independent(self) -> &'a mut W {
        self.variant(DUAL_A::INDEPENDENT)
    }
    #[doc = "Dual, combined regular simultaneous + injected simultaneous mode"]
    #[inline(always)]
    pub fn dual_rj(self) -> &'a mut W {
        self.variant(DUAL_A::DUAL_RJ)
    }
    #[doc = "Dual, combined regular simultaneous + alternate trigger mode"]
    #[inline(always)]
    pub fn dual_ra(self) -> &'a mut W {
        self.variant(DUAL_A::DUAL_RA)
    }
    #[doc = "Dual, combined interleaved mode + injected simultaneous mode"]
    #[inline(always)]
    pub fn dual_ij(self) -> &'a mut W {
        self.variant(DUAL_A::DUAL_IJ)
    }
    #[doc = "Dual, injected simultaneous mode only"]
    #[inline(always)]
    pub fn dual_j(self) -> &'a mut W {
        self.variant(DUAL_A::DUAL_J)
    }
    #[doc = "Dual, regular simultaneous mode only"]
    #[inline(always)]
    pub fn dual_r(self) -> &'a mut W {
        self.variant(DUAL_A::DUAL_R)
    }
    #[doc = "Dual, interleaved mode only"]
    #[inline(always)]
    pub fn dual_i(self) -> &'a mut W {
        self.variant(DUAL_A::DUAL_I)
    }
    #[doc = "Dual, alternate trigger mode only"]
    #[inline(always)]
    pub fn dual_a(self) -> &'a mut W {
        self.variant(DUAL_A::DUAL_A)
    }
}
#[doc = "Field `DELAY` reader - Delay between 2 sampling phases"]
pub type DELAY_R = crate::FieldReader;
#[doc = "Field `DELAY` writer - Delay between 2 sampling phases"]
pub type DELAY_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CCR_SPEC, 4, O>;
#[doc = "Field `DMACFG` reader - DMA configuration (for multi-ADC mode)"]
pub type DMACFG_R = crate::BitReader<DMACFG_A>;
#[doc = "DMA configuration (for multi-ADC mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DMACFG_A {
    #[doc = "0: DMA one shot mode selected"]
    ONE_SHOT = 0,
    #[doc = "1: DMA circular mode selected"]
    CIRCULATOR = 1,
}
impl From<DMACFG_A> for bool {
    #[inline(always)]
    fn from(variant: DMACFG_A) -> Self {
        variant as u8 != 0
    }
}
impl DMACFG_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DMACFG_A {
        match self.bits {
            false => DMACFG_A::ONE_SHOT,
            true => DMACFG_A::CIRCULATOR,
        }
    }
    #[doc = "Checks if the value of the field is `ONE_SHOT`"]
    #[inline(always)]
    pub fn is_one_shot(&self) -> bool {
        *self == DMACFG_A::ONE_SHOT
    }
    #[doc = "Checks if the value of the field is `CIRCULATOR`"]
    #[inline(always)]
    pub fn is_circulator(&self) -> bool {
        *self == DMACFG_A::CIRCULATOR
    }
}
#[doc = "Field `DMACFG` writer - DMA configuration (for multi-ADC mode)"]
pub type DMACFG_W<'a, const O: u8> = crate::BitWriter<'a, CCR_SPEC, O, DMACFG_A>;
impl<'a, const O: u8> DMACFG_W<'a, O> {
    #[doc = "DMA one shot mode selected"]
    #[inline(always)]
    pub fn one_shot(self) -> &'a mut W {
        self.variant(DMACFG_A::ONE_SHOT)
    }
    #[doc = "DMA circular mode selected"]
    #[inline(always)]
    pub fn circulator(self) -> &'a mut W {
        self.variant(DMACFG_A::CIRCULATOR)
    }
}
#[doc = "Field `MDMA` reader - Direct memory access mode for multi ADC mode"]
pub type MDMA_R = crate::FieldReader<MDMA_A>;
#[doc = "Direct memory access mode for multi ADC mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MDMA_A {
    #[doc = "0: MDMA mode disabled"]
    DISABLED = 0,
    #[doc = "2: MDMA mode enabled for 12 and 10-bit resolution"]
    BITS12_10 = 2,
    #[doc = "3: MDMA mode enabled for 8 and 6-bit resolution"]
    BITS8_6 = 3,
}
impl From<MDMA_A> for u8 {
    #[inline(always)]
    fn from(variant: MDMA_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MDMA_A {
    type Ux = u8;
}
impl MDMA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<MDMA_A> {
        match self.bits {
            0 => Some(MDMA_A::DISABLED),
            2 => Some(MDMA_A::BITS12_10),
            3 => Some(MDMA_A::BITS8_6),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == MDMA_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `BITS12_10`"]
    #[inline(always)]
    pub fn is_bits12_10(&self) -> bool {
        *self == MDMA_A::BITS12_10
    }
    #[doc = "Checks if the value of the field is `BITS8_6`"]
    #[inline(always)]
    pub fn is_bits8_6(&self) -> bool {
        *self == MDMA_A::BITS8_6
    }
}
#[doc = "Field `MDMA` writer - Direct memory access mode for multi ADC mode"]
pub type MDMA_W<'a, const O: u8> = crate::FieldWriter<'a, CCR_SPEC, 2, O, MDMA_A>;
impl<'a, const O: u8> MDMA_W<'a, O> {
    #[doc = "MDMA mode disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(MDMA_A::DISABLED)
    }
    #[doc = "MDMA mode enabled for 12 and 10-bit resolution"]
    #[inline(always)]
    pub fn bits12_10(self) -> &'a mut W {
        self.variant(MDMA_A::BITS12_10)
    }
    #[doc = "MDMA mode enabled for 8 and 6-bit resolution"]
    #[inline(always)]
    pub fn bits8_6(self) -> &'a mut W {
        self.variant(MDMA_A::BITS8_6)
    }
}
#[doc = "Field `CKMODE` reader - ADC clock mode"]
pub type CKMODE_R = crate::FieldReader<CKMODE_A>;
#[doc = "ADC clock mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CKMODE_A {
    #[doc = "0: Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock"]
    ASYNCHRONOUS = 0,
    #[doc = "1: Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck"]
    SYNC_DIV1 = 1,
    #[doc = "2: Use AHB clock rcc_hclk3 divided by 2"]
    SYNC_DIV2 = 2,
    #[doc = "3: Use AHB clock rcc_hclk3 divided by 4"]
    SYNC_DIV4 = 3,
}
impl From<CKMODE_A> for u8 {
    #[inline(always)]
    fn from(variant: CKMODE_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CKMODE_A {
    type Ux = u8;
}
impl CKMODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> CKMODE_A {
        match self.bits {
            0 => CKMODE_A::ASYNCHRONOUS,
            1 => CKMODE_A::SYNC_DIV1,
            2 => CKMODE_A::SYNC_DIV2,
            3 => CKMODE_A::SYNC_DIV4,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ASYNCHRONOUS`"]
    #[inline(always)]
    pub fn is_asynchronous(&self) -> bool {
        *self == CKMODE_A::ASYNCHRONOUS
    }
    #[doc = "Checks if the value of the field is `SYNC_DIV1`"]
    #[inline(always)]
    pub fn is_sync_div1(&self) -> bool {
        *self == CKMODE_A::SYNC_DIV1
    }
    #[doc = "Checks if the value of the field is `SYNC_DIV2`"]
    #[inline(always)]
    pub fn is_sync_div2(&self) -> bool {
        *self == CKMODE_A::SYNC_DIV2
    }
    #[doc = "Checks if the value of the field is `SYNC_DIV4`"]
    #[inline(always)]
    pub fn is_sync_div4(&self) -> bool {
        *self == CKMODE_A::SYNC_DIV4
    }
}
#[doc = "Field `CKMODE` writer - ADC clock mode"]
pub type CKMODE_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CCR_SPEC, 2, O, CKMODE_A>;
impl<'a, const O: u8> CKMODE_W<'a, O> {
    #[doc = "Use Kernel Clock adc_ker_ck_input divided by PRESC. Asynchronous to AHB clock"]
    #[inline(always)]
    pub fn asynchronous(self) -> &'a mut W {
        self.variant(CKMODE_A::ASYNCHRONOUS)
    }
    #[doc = "Use AHB clock rcc_hclk3. In this case rcc_hclk must equal sys_d1cpre_ck"]
    #[inline(always)]
    pub fn sync_div1(self) -> &'a mut W {
        self.variant(CKMODE_A::SYNC_DIV1)
    }
    #[doc = "Use AHB clock rcc_hclk3 divided by 2"]
    #[inline(always)]
    pub fn sync_div2(self) -> &'a mut W {
        self.variant(CKMODE_A::SYNC_DIV2)
    }
    #[doc = "Use AHB clock rcc_hclk3 divided by 4"]
    #[inline(always)]
    pub fn sync_div4(self) -> &'a mut W {
        self.variant(CKMODE_A::SYNC_DIV4)
    }
}
#[doc = "Field `VREFEN` reader - VREFINT enable"]
pub type VREFEN_R = crate::BitReader<VREFEN_A>;
#[doc = "VREFINT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VREFEN_A {
    #[doc = "0: V_REFINT channel disabled"]
    DISABLED = 0,
    #[doc = "1: V_REFINT channel enabled"]
    ENABLED = 1,
}
impl From<VREFEN_A> for bool {
    #[inline(always)]
    fn from(variant: VREFEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VREFEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VREFEN_A {
        match self.bits {
            false => VREFEN_A::DISABLED,
            true => VREFEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VREFEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VREFEN_A::ENABLED
    }
}
#[doc = "Field `VREFEN` writer - VREFINT enable"]
pub type VREFEN_W<'a, const O: u8> = crate::BitWriter<'a, CCR_SPEC, O, VREFEN_A>;
impl<'a, const O: u8> VREFEN_W<'a, O> {
    #[doc = "V_REFINT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VREFEN_A::DISABLED)
    }
    #[doc = "V_REFINT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VREFEN_A::ENABLED)
    }
}
#[doc = "Field `TSEN` reader - Temperature sensor enable"]
pub type TSEN_R = crate::BitReader<TSEN_A>;
#[doc = "Temperature sensor enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TSEN_A {
    #[doc = "0: Temperature sensor channel disabled"]
    DISABLED = 0,
    #[doc = "1: Temperature sensor channel enabled"]
    ENABLED = 1,
}
impl From<TSEN_A> for bool {
    #[inline(always)]
    fn from(variant: TSEN_A) -> Self {
        variant as u8 != 0
    }
}
impl TSEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> TSEN_A {
        match self.bits {
            false => TSEN_A::DISABLED,
            true => TSEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == TSEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == TSEN_A::ENABLED
    }
}
#[doc = "Field `TSEN` writer - Temperature sensor enable"]
pub type TSEN_W<'a, const O: u8> = crate::BitWriter<'a, CCR_SPEC, O, TSEN_A>;
impl<'a, const O: u8> TSEN_W<'a, O> {
    #[doc = "Temperature sensor channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(TSEN_A::DISABLED)
    }
    #[doc = "Temperature sensor channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(TSEN_A::ENABLED)
    }
}
#[doc = "Field `VBATEN` reader - VBAT enable"]
pub type VBATEN_R = crate::BitReader<VBATEN_A>;
#[doc = "VBAT enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum VBATEN_A {
    #[doc = "0: V_BAT channel disabled"]
    DISABLED = 0,
    #[doc = "1: V_BAT channel enabled"]
    ENABLED = 1,
}
impl From<VBATEN_A> for bool {
    #[inline(always)]
    fn from(variant: VBATEN_A) -> Self {
        variant as u8 != 0
    }
}
impl VBATEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> VBATEN_A {
        match self.bits {
            false => VBATEN_A::DISABLED,
            true => VBATEN_A::ENABLED,
        }
    }
    #[doc = "Checks if the value of the field is `DISABLED`"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == VBATEN_A::DISABLED
    }
    #[doc = "Checks if the value of the field is `ENABLED`"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == VBATEN_A::ENABLED
    }
}
#[doc = "Field `VBATEN` writer - VBAT enable"]
pub type VBATEN_W<'a, const O: u8> = crate::BitWriter<'a, CCR_SPEC, O, VBATEN_A>;
impl<'a, const O: u8> VBATEN_W<'a, O> {
    #[doc = "V_BAT channel disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut W {
        self.variant(VBATEN_A::DISABLED)
    }
    #[doc = "V_BAT channel enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut W {
        self.variant(VBATEN_A::ENABLED)
    }
}
impl R {
    #[doc = "Bits 0:4 - Dual ADC mode selection"]
    #[inline(always)]
    pub fn dual(&self) -> DUAL_R {
        DUAL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    pub fn delay(&self) -> DELAY_R {
        DELAY_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 13 - DMA configuration (for multi-ADC mode)"]
    #[inline(always)]
    pub fn dmacfg(&self) -> DMACFG_R {
        DMACFG_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bits 14:15 - Direct memory access mode for multi ADC mode"]
    #[inline(always)]
    pub fn mdma(&self) -> MDMA_R {
        MDMA_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17 - ADC clock mode"]
    #[inline(always)]
    pub fn ckmode(&self) -> CKMODE_R {
        CKMODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 22 - VREFINT enable"]
    #[inline(always)]
    pub fn vrefen(&self) -> VREFEN_R {
        VREFEN_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Temperature sensor enable"]
    #[inline(always)]
    pub fn tsen(&self) -> TSEN_R {
        TSEN_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - VBAT enable"]
    #[inline(always)]
    pub fn vbaten(&self) -> VBATEN_R {
        VBATEN_R::new(((self.bits >> 24) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:4 - Dual ADC mode selection"]
    #[inline(always)]
    #[must_use]
    pub fn dual(&mut self) -> DUAL_W<0> {
        DUAL_W::new(self)
    }
    #[doc = "Bits 8:11 - Delay between 2 sampling phases"]
    #[inline(always)]
    #[must_use]
    pub fn delay(&mut self) -> DELAY_W<8> {
        DELAY_W::new(self)
    }
    #[doc = "Bit 13 - DMA configuration (for multi-ADC mode)"]
    #[inline(always)]
    #[must_use]
    pub fn dmacfg(&mut self) -> DMACFG_W<13> {
        DMACFG_W::new(self)
    }
    #[doc = "Bits 14:15 - Direct memory access mode for multi ADC mode"]
    #[inline(always)]
    #[must_use]
    pub fn mdma(&mut self) -> MDMA_W<14> {
        MDMA_W::new(self)
    }
    #[doc = "Bits 16:17 - ADC clock mode"]
    #[inline(always)]
    #[must_use]
    pub fn ckmode(&mut self) -> CKMODE_W<16> {
        CKMODE_W::new(self)
    }
    #[doc = "Bit 22 - VREFINT enable"]
    #[inline(always)]
    #[must_use]
    pub fn vrefen(&mut self) -> VREFEN_W<22> {
        VREFEN_W::new(self)
    }
    #[doc = "Bit 23 - Temperature sensor enable"]
    #[inline(always)]
    #[must_use]
    pub fn tsen(&mut self) -> TSEN_W<23> {
        TSEN_W::new(self)
    }
    #[doc = "Bit 24 - VBAT enable"]
    #[inline(always)]
    #[must_use]
    pub fn vbaten(&mut self) -> VBATEN_W<24> {
        VBATEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC common control register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ccr](index.html) module"]
pub struct CCR_SPEC;
impl crate::RegisterSpec for CCR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ccr::R](R) reader structure"]
impl crate::Readable for CCR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ccr::W](W) writer structure"]
impl crate::Writable for CCR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CCR to value 0"]
impl crate::Resettable for CCR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
