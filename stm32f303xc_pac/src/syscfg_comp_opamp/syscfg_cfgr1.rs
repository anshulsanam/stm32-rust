#[doc = "Register `SYSCFG_CFGR1` reader"]
pub struct R(crate::R<SYSCFG_CFGR1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SYSCFG_CFGR1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SYSCFG_CFGR1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SYSCFG_CFGR1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SYSCFG_CFGR1` writer"]
pub struct W(crate::W<SYSCFG_CFGR1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SYSCFG_CFGR1_SPEC>;
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
impl From<crate::W<SYSCFG_CFGR1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SYSCFG_CFGR1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MEM_MODE` reader - Memory mapping selection bits"]
pub type MEM_MODE_R = crate::FieldReader;
#[doc = "Field `MEM_MODE` writer - Memory mapping selection bits"]
pub type MEM_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, SYSCFG_CFGR1_SPEC, 2, O>;
#[doc = "Field `USB_IT_RMP` reader - USB interrupt remap"]
pub type USB_IT_RMP_R = crate::BitReader;
#[doc = "Field `USB_IT_RMP` writer - USB interrupt remap"]
pub type USB_IT_RMP_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR1_SPEC, O>;
#[doc = "Field `TIM1_ITR_RMP` reader - Timer 1 ITR3 selection"]
pub type TIM1_ITR_RMP_R = crate::BitReader;
#[doc = "Field `TIM1_ITR_RMP` writer - Timer 1 ITR3 selection"]
pub type TIM1_ITR_RMP_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR1_SPEC, O>;
#[doc = "Field `DAC_TRIG_RMP` reader - DAC trigger remap (when TSEL = 001)"]
pub type DAC_TRIG_RMP_R = crate::BitReader;
#[doc = "Field `DAC_TRIG_RMP` writer - DAC trigger remap (when TSEL = 001)"]
pub type DAC_TRIG_RMP_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR1_SPEC, O>;
#[doc = "Field `ADC24_DMA_RMP` reader - ADC24 DMA remapping bit"]
pub type ADC24_DMA_RMP_R = crate::BitReader;
#[doc = "Field `ADC24_DMA_RMP` writer - ADC24 DMA remapping bit"]
pub type ADC24_DMA_RMP_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR1_SPEC, O>;
#[doc = "Field `TIM16_DMA_RMP` reader - TIM16 DMA request remapping bit"]
pub type TIM16_DMA_RMP_R = crate::BitReader;
#[doc = "Field `TIM16_DMA_RMP` writer - TIM16 DMA request remapping bit"]
pub type TIM16_DMA_RMP_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR1_SPEC, O>;
#[doc = "Field `TIM17_DMA_RMP` reader - TIM17 DMA request remapping bit"]
pub type TIM17_DMA_RMP_R = crate::BitReader;
#[doc = "Field `TIM17_DMA_RMP` writer - TIM17 DMA request remapping bit"]
pub type TIM17_DMA_RMP_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR1_SPEC, O>;
#[doc = "Field `TIM6_DAC1_DMA_RMP` reader - TIM6 and DAC1 DMA request remapping bit"]
pub type TIM6_DAC1_DMA_RMP_R = crate::BitReader;
#[doc = "Field `TIM6_DAC1_DMA_RMP` writer - TIM6 and DAC1 DMA request remapping bit"]
pub type TIM6_DAC1_DMA_RMP_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR1_SPEC, O>;
#[doc = "Field `TIM7_DAC2_DMA_RMP` reader - TIM7 and DAC2 DMA request remapping bit"]
pub type TIM7_DAC2_DMA_RMP_R = crate::BitReader;
#[doc = "Field `TIM7_DAC2_DMA_RMP` writer - TIM7 and DAC2 DMA request remapping bit"]
pub type TIM7_DAC2_DMA_RMP_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR1_SPEC, O>;
#[doc = "Field `I2C_PB6_FM` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB6_FM_R = crate::BitReader;
#[doc = "Field `I2C_PB6_FM` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB6_FM_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR1_SPEC, O>;
#[doc = "Field `I2C_PB7_FM` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB7_FM_R = crate::BitReader;
#[doc = "Field `I2C_PB7_FM` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB7_FM_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR1_SPEC, O>;
#[doc = "Field `I2C_PB8_FM` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB8_FM_R = crate::BitReader;
#[doc = "Field `I2C_PB8_FM` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB8_FM_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR1_SPEC, O>;
#[doc = "Field `I2C_PB9_FM` reader - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB9_FM_R = crate::BitReader;
#[doc = "Field `I2C_PB9_FM` writer - Fast Mode Plus (FM+) driving capability activation bits."]
pub type I2C_PB9_FM_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR1_SPEC, O>;
#[doc = "Field `I2C1_FM` reader - I2C1 Fast Mode Plus"]
pub type I2C1_FM_R = crate::BitReader;
#[doc = "Field `I2C1_FM` writer - I2C1 Fast Mode Plus"]
pub type I2C1_FM_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR1_SPEC, O>;
#[doc = "Field `I2C2_FM` reader - I2C2 Fast Mode Plus"]
pub type I2C2_FM_R = crate::BitReader;
#[doc = "Field `I2C2_FM` writer - I2C2 Fast Mode Plus"]
pub type I2C2_FM_W<'a, const O: u8> = crate::BitWriter<'a, SYSCFG_CFGR1_SPEC, O>;
#[doc = "Field `ENCODER_MODE` reader - Encoder mode"]
pub type ENCODER_MODE_R = crate::FieldReader;
#[doc = "Field `ENCODER_MODE` writer - Encoder mode"]
pub type ENCODER_MODE_W<'a, const O: u8> = crate::FieldWriter<'a, SYSCFG_CFGR1_SPEC, 2, O>;
#[doc = "Field `FPU_IT` reader - Interrupt enable bits from FPU"]
pub type FPU_IT_R = crate::FieldReader;
#[doc = "Field `FPU_IT` writer - Interrupt enable bits from FPU"]
pub type FPU_IT_W<'a, const O: u8> = crate::FieldWriter<'a, SYSCFG_CFGR1_SPEC, 6, O>;
impl R {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    pub fn mem_mode(&self) -> MEM_MODE_R {
        MEM_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 5 - USB interrupt remap"]
    #[inline(always)]
    pub fn usb_it_rmp(&self) -> USB_IT_RMP_R {
        USB_IT_RMP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Timer 1 ITR3 selection"]
    #[inline(always)]
    pub fn tim1_itr_rmp(&self) -> TIM1_ITR_RMP_R {
        TIM1_ITR_RMP_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DAC trigger remap (when TSEL = 001)"]
    #[inline(always)]
    pub fn dac_trig_rmp(&self) -> DAC_TRIG_RMP_R {
        DAC_TRIG_RMP_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - ADC24 DMA remapping bit"]
    #[inline(always)]
    pub fn adc24_dma_rmp(&self) -> ADC24_DMA_RMP_R {
        ADC24_DMA_RMP_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 11 - TIM16 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim16_dma_rmp(&self) -> TIM16_DMA_RMP_R {
        TIM16_DMA_RMP_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - TIM17 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim17_dma_rmp(&self) -> TIM17_DMA_RMP_R {
        TIM17_DMA_RMP_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - TIM6 and DAC1 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim6_dac1_dma_rmp(&self) -> TIM6_DAC1_DMA_RMP_R {
        TIM6_DAC1_DMA_RMP_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - TIM7 and DAC2 DMA request remapping bit"]
    #[inline(always)]
    pub fn tim7_dac2_dma_rmp(&self) -> TIM7_DAC2_DMA_RMP_R {
        TIM7_DAC2_DMA_RMP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb6_fm(&self) -> I2C_PB6_FM_R {
        I2C_PB6_FM_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb7_fm(&self) -> I2C_PB7_FM_R {
        I2C_PB7_FM_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb8_fm(&self) -> I2C_PB8_FM_R {
        I2C_PB8_FM_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    pub fn i2c_pb9_fm(&self) -> I2C_PB9_FM_R {
        I2C_PB9_FM_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - I2C1 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c1_fm(&self) -> I2C1_FM_R {
        I2C1_FM_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - I2C2 Fast Mode Plus"]
    #[inline(always)]
    pub fn i2c2_fm(&self) -> I2C2_FM_R {
        I2C2_FM_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bits 22:23 - Encoder mode"]
    #[inline(always)]
    pub fn encoder_mode(&self) -> ENCODER_MODE_R {
        ENCODER_MODE_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 26:31 - Interrupt enable bits from FPU"]
    #[inline(always)]
    pub fn fpu_it(&self) -> FPU_IT_R {
        FPU_IT_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Memory mapping selection bits"]
    #[inline(always)]
    #[must_use]
    pub fn mem_mode(&mut self) -> MEM_MODE_W<0> {
        MEM_MODE_W::new(self)
    }
    #[doc = "Bit 5 - USB interrupt remap"]
    #[inline(always)]
    #[must_use]
    pub fn usb_it_rmp(&mut self) -> USB_IT_RMP_W<5> {
        USB_IT_RMP_W::new(self)
    }
    #[doc = "Bit 6 - Timer 1 ITR3 selection"]
    #[inline(always)]
    #[must_use]
    pub fn tim1_itr_rmp(&mut self) -> TIM1_ITR_RMP_W<6> {
        TIM1_ITR_RMP_W::new(self)
    }
    #[doc = "Bit 7 - DAC trigger remap (when TSEL = 001)"]
    #[inline(always)]
    #[must_use]
    pub fn dac_trig_rmp(&mut self) -> DAC_TRIG_RMP_W<7> {
        DAC_TRIG_RMP_W::new(self)
    }
    #[doc = "Bit 8 - ADC24 DMA remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn adc24_dma_rmp(&mut self) -> ADC24_DMA_RMP_W<8> {
        ADC24_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 11 - TIM16 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim16_dma_rmp(&mut self) -> TIM16_DMA_RMP_W<11> {
        TIM16_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 12 - TIM17 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim17_dma_rmp(&mut self) -> TIM17_DMA_RMP_W<12> {
        TIM17_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 13 - TIM6 and DAC1 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim6_dac1_dma_rmp(&mut self) -> TIM6_DAC1_DMA_RMP_W<13> {
        TIM6_DAC1_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 14 - TIM7 and DAC2 DMA request remapping bit"]
    #[inline(always)]
    #[must_use]
    pub fn tim7_dac2_dma_rmp(&mut self) -> TIM7_DAC2_DMA_RMP_W<14> {
        TIM7_DAC2_DMA_RMP_W::new(self)
    }
    #[doc = "Bit 16 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb6_fm(&mut self) -> I2C_PB6_FM_W<16> {
        I2C_PB6_FM_W::new(self)
    }
    #[doc = "Bit 17 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb7_fm(&mut self) -> I2C_PB7_FM_W<17> {
        I2C_PB7_FM_W::new(self)
    }
    #[doc = "Bit 18 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb8_fm(&mut self) -> I2C_PB8_FM_W<18> {
        I2C_PB8_FM_W::new(self)
    }
    #[doc = "Bit 19 - Fast Mode Plus (FM+) driving capability activation bits."]
    #[inline(always)]
    #[must_use]
    pub fn i2c_pb9_fm(&mut self) -> I2C_PB9_FM_W<19> {
        I2C_PB9_FM_W::new(self)
    }
    #[doc = "Bit 20 - I2C1 Fast Mode Plus"]
    #[inline(always)]
    #[must_use]
    pub fn i2c1_fm(&mut self) -> I2C1_FM_W<20> {
        I2C1_FM_W::new(self)
    }
    #[doc = "Bit 21 - I2C2 Fast Mode Plus"]
    #[inline(always)]
    #[must_use]
    pub fn i2c2_fm(&mut self) -> I2C2_FM_W<21> {
        I2C2_FM_W::new(self)
    }
    #[doc = "Bits 22:23 - Encoder mode"]
    #[inline(always)]
    #[must_use]
    pub fn encoder_mode(&mut self) -> ENCODER_MODE_W<22> {
        ENCODER_MODE_W::new(self)
    }
    #[doc = "Bits 26:31 - Interrupt enable bits from FPU"]
    #[inline(always)]
    #[must_use]
    pub fn fpu_it(&mut self) -> FPU_IT_W<26> {
        FPU_IT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configuration register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [syscfg_cfgr1](index.html) module"]
pub struct SYSCFG_CFGR1_SPEC;
impl crate::RegisterSpec for SYSCFG_CFGR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [syscfg_cfgr1::R](R) reader structure"]
impl crate::Readable for SYSCFG_CFGR1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [syscfg_cfgr1::W](W) writer structure"]
impl crate::Writable for SYSCFG_CFGR1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SYSCFG_CFGR1 to value 0"]
impl crate::Resettable for SYSCFG_CFGR1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
