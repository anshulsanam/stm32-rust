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
#[doc = "Field `SADD` reader - Slave address bit 9:8 (master mode)"]
pub type SADD_R = crate::FieldReader<u16>;
#[doc = "Field `SADD` writer - Slave address bit 9:8 (master mode)"]
pub type SADD_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CR2_SPEC, 10, O, u16>;
#[doc = "Field `RD_WRN` reader - Transfer direction (master mode)"]
pub type RD_WRN_R = crate::BitReader<RD_WRN_A>;
#[doc = "Transfer direction (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RD_WRN_A {
    #[doc = "0: Master requests a write transfer"]
    WRITE = 0,
    #[doc = "1: Master requests a read transfer"]
    READ = 1,
}
impl From<RD_WRN_A> for bool {
    #[inline(always)]
    fn from(variant: RD_WRN_A) -> Self {
        variant as u8 != 0
    }
}
impl RD_WRN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RD_WRN_A {
        match self.bits {
            false => RD_WRN_A::WRITE,
            true => RD_WRN_A::READ,
        }
    }
    #[doc = "Checks if the value of the field is `WRITE`"]
    #[inline(always)]
    pub fn is_write(&self) -> bool {
        *self == RD_WRN_A::WRITE
    }
    #[doc = "Checks if the value of the field is `READ`"]
    #[inline(always)]
    pub fn is_read(&self) -> bool {
        *self == RD_WRN_A::READ
    }
}
#[doc = "Field `RD_WRN` writer - Transfer direction (master mode)"]
pub type RD_WRN_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, RD_WRN_A>;
impl<'a, const O: u8> RD_WRN_W<'a, O> {
    #[doc = "Master requests a write transfer"]
    #[inline(always)]
    pub fn write(self) -> &'a mut W {
        self.variant(RD_WRN_A::WRITE)
    }
    #[doc = "Master requests a read transfer"]
    #[inline(always)]
    pub fn read(self) -> &'a mut W {
        self.variant(RD_WRN_A::READ)
    }
}
#[doc = "Field `ADD10` reader - 10-bit addressing mode (master mode)"]
pub type ADD10_R = crate::BitReader<ADD10_A>;
#[doc = "10-bit addressing mode (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ADD10_A {
    #[doc = "0: The master operates in 7-bit addressing mode"]
    BIT7 = 0,
    #[doc = "1: The master operates in 10-bit addressing mode"]
    BIT10 = 1,
}
impl From<ADD10_A> for bool {
    #[inline(always)]
    fn from(variant: ADD10_A) -> Self {
        variant as u8 != 0
    }
}
impl ADD10_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADD10_A {
        match self.bits {
            false => ADD10_A::BIT7,
            true => ADD10_A::BIT10,
        }
    }
    #[doc = "Checks if the value of the field is `BIT7`"]
    #[inline(always)]
    pub fn is_bit7(&self) -> bool {
        *self == ADD10_A::BIT7
    }
    #[doc = "Checks if the value of the field is `BIT10`"]
    #[inline(always)]
    pub fn is_bit10(&self) -> bool {
        *self == ADD10_A::BIT10
    }
}
#[doc = "Field `ADD10` writer - 10-bit addressing mode (master mode)"]
pub type ADD10_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, ADD10_A>;
impl<'a, const O: u8> ADD10_W<'a, O> {
    #[doc = "The master operates in 7-bit addressing mode"]
    #[inline(always)]
    pub fn bit7(self) -> &'a mut W {
        self.variant(ADD10_A::BIT7)
    }
    #[doc = "The master operates in 10-bit addressing mode"]
    #[inline(always)]
    pub fn bit10(self) -> &'a mut W {
        self.variant(ADD10_A::BIT10)
    }
}
#[doc = "Field `HEAD10R` reader - 10-bit address header only read direction (master receiver mode)"]
pub type HEAD10R_R = crate::BitReader<HEAD10R_A>;
#[doc = "10-bit address header only read direction (master receiver mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HEAD10R_A {
    #[doc = "0: The master sends the complete 10 bit slave address read sequence"]
    COMPLETE = 0,
    #[doc = "1: The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction"]
    PARTIAL = 1,
}
impl From<HEAD10R_A> for bool {
    #[inline(always)]
    fn from(variant: HEAD10R_A) -> Self {
        variant as u8 != 0
    }
}
impl HEAD10R_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> HEAD10R_A {
        match self.bits {
            false => HEAD10R_A::COMPLETE,
            true => HEAD10R_A::PARTIAL,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETE`"]
    #[inline(always)]
    pub fn is_complete(&self) -> bool {
        *self == HEAD10R_A::COMPLETE
    }
    #[doc = "Checks if the value of the field is `PARTIAL`"]
    #[inline(always)]
    pub fn is_partial(&self) -> bool {
        *self == HEAD10R_A::PARTIAL
    }
}
#[doc = "Field `HEAD10R` writer - 10-bit address header only read direction (master receiver mode)"]
pub type HEAD10R_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, HEAD10R_A>;
impl<'a, const O: u8> HEAD10R_W<'a, O> {
    #[doc = "The master sends the complete 10 bit slave address read sequence"]
    #[inline(always)]
    pub fn complete(self) -> &'a mut W {
        self.variant(HEAD10R_A::COMPLETE)
    }
    #[doc = "The master only sends the 1st 7 bits of the 10 bit address, followed by Read direction"]
    #[inline(always)]
    pub fn partial(self) -> &'a mut W {
        self.variant(HEAD10R_A::PARTIAL)
    }
}
#[doc = "Field `START` reader - Start generation"]
pub type START_R = crate::BitReader<START_A>;
#[doc = "Start generation\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum START_A {
    #[doc = "0: No Start generation"]
    NO_START = 0,
    #[doc = "1: Restart/Start generation"]
    START = 1,
}
impl From<START_A> for bool {
    #[inline(always)]
    fn from(variant: START_A) -> Self {
        variant as u8 != 0
    }
}
impl START_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> START_A {
        match self.bits {
            false => START_A::NO_START,
            true => START_A::START,
        }
    }
    #[doc = "Checks if the value of the field is `NO_START`"]
    #[inline(always)]
    pub fn is_no_start(&self) -> bool {
        *self == START_A::NO_START
    }
    #[doc = "Checks if the value of the field is `START`"]
    #[inline(always)]
    pub fn is_start(&self) -> bool {
        *self == START_A::START
    }
}
#[doc = "Field `START` writer - Start generation"]
pub type START_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, START_A>;
impl<'a, const O: u8> START_W<'a, O> {
    #[doc = "No Start generation"]
    #[inline(always)]
    pub fn no_start(self) -> &'a mut W {
        self.variant(START_A::NO_START)
    }
    #[doc = "Restart/Start generation"]
    #[inline(always)]
    pub fn start(self) -> &'a mut W {
        self.variant(START_A::START)
    }
}
#[doc = "Field `STOP` reader - Stop generation (master mode)"]
pub type STOP_R = crate::BitReader<STOP_A>;
#[doc = "Stop generation (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum STOP_A {
    #[doc = "0: No Stop generation"]
    NO_STOP = 0,
    #[doc = "1: Stop generation after current byte transfer"]
    STOP = 1,
}
impl From<STOP_A> for bool {
    #[inline(always)]
    fn from(variant: STOP_A) -> Self {
        variant as u8 != 0
    }
}
impl STOP_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> STOP_A {
        match self.bits {
            false => STOP_A::NO_STOP,
            true => STOP_A::STOP,
        }
    }
    #[doc = "Checks if the value of the field is `NO_STOP`"]
    #[inline(always)]
    pub fn is_no_stop(&self) -> bool {
        *self == STOP_A::NO_STOP
    }
    #[doc = "Checks if the value of the field is `STOP`"]
    #[inline(always)]
    pub fn is_stop(&self) -> bool {
        *self == STOP_A::STOP
    }
}
#[doc = "Field `STOP` writer - Stop generation (master mode)"]
pub type STOP_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, STOP_A>;
impl<'a, const O: u8> STOP_W<'a, O> {
    #[doc = "No Stop generation"]
    #[inline(always)]
    pub fn no_stop(self) -> &'a mut W {
        self.variant(STOP_A::NO_STOP)
    }
    #[doc = "Stop generation after current byte transfer"]
    #[inline(always)]
    pub fn stop(self) -> &'a mut W {
        self.variant(STOP_A::STOP)
    }
}
#[doc = "Field `NACK` reader - NACK generation (slave mode)"]
pub type NACK_R = crate::BitReader<NACK_A>;
#[doc = "NACK generation (slave mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NACK_A {
    #[doc = "0: an ACK is sent after current received byte"]
    ACK = 0,
    #[doc = "1: a NACK is sent after current received byte"]
    NACK = 1,
}
impl From<NACK_A> for bool {
    #[inline(always)]
    fn from(variant: NACK_A) -> Self {
        variant as u8 != 0
    }
}
impl NACK_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> NACK_A {
        match self.bits {
            false => NACK_A::ACK,
            true => NACK_A::NACK,
        }
    }
    #[doc = "Checks if the value of the field is `ACK`"]
    #[inline(always)]
    pub fn is_ack(&self) -> bool {
        *self == NACK_A::ACK
    }
    #[doc = "Checks if the value of the field is `NACK`"]
    #[inline(always)]
    pub fn is_nack(&self) -> bool {
        *self == NACK_A::NACK
    }
}
#[doc = "Field `NACK` writer - NACK generation (slave mode)"]
pub type NACK_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, NACK_A>;
impl<'a, const O: u8> NACK_W<'a, O> {
    #[doc = "an ACK is sent after current received byte"]
    #[inline(always)]
    pub fn ack(self) -> &'a mut W {
        self.variant(NACK_A::ACK)
    }
    #[doc = "a NACK is sent after current received byte"]
    #[inline(always)]
    pub fn nack(self) -> &'a mut W {
        self.variant(NACK_A::NACK)
    }
}
#[doc = "Field `NBYTES` reader - Number of bytes"]
pub type NBYTES_R = crate::FieldReader;
#[doc = "Field `NBYTES` writer - Number of bytes"]
pub type NBYTES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, CR2_SPEC, 8, O>;
#[doc = "Field `RELOAD` reader - NBYTES reload mode"]
pub type RELOAD_R = crate::BitReader<RELOAD_A>;
#[doc = "NBYTES reload mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RELOAD_A {
    #[doc = "0: The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)"]
    COMPLETED = 0,
    #[doc = "1: The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)"]
    NOT_COMPLETED = 1,
}
impl From<RELOAD_A> for bool {
    #[inline(always)]
    fn from(variant: RELOAD_A) -> Self {
        variant as u8 != 0
    }
}
impl RELOAD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RELOAD_A {
        match self.bits {
            false => RELOAD_A::COMPLETED,
            true => RELOAD_A::NOT_COMPLETED,
        }
    }
    #[doc = "Checks if the value of the field is `COMPLETED`"]
    #[inline(always)]
    pub fn is_completed(&self) -> bool {
        *self == RELOAD_A::COMPLETED
    }
    #[doc = "Checks if the value of the field is `NOT_COMPLETED`"]
    #[inline(always)]
    pub fn is_not_completed(&self) -> bool {
        *self == RELOAD_A::NOT_COMPLETED
    }
}
#[doc = "Field `RELOAD` writer - NBYTES reload mode"]
pub type RELOAD_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, RELOAD_A>;
impl<'a, const O: u8> RELOAD_W<'a, O> {
    #[doc = "The transfer is completed after the NBYTES data transfer (STOP or RESTART will follow)"]
    #[inline(always)]
    pub fn completed(self) -> &'a mut W {
        self.variant(RELOAD_A::COMPLETED)
    }
    #[doc = "The transfer is not completed after the NBYTES data transfer (NBYTES will be reloaded)"]
    #[inline(always)]
    pub fn not_completed(self) -> &'a mut W {
        self.variant(RELOAD_A::NOT_COMPLETED)
    }
}
#[doc = "Field `AUTOEND` reader - Automatic end mode (master mode)"]
pub type AUTOEND_R = crate::BitReader<AUTOEND_A>;
#[doc = "Automatic end mode (master mode)\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AUTOEND_A {
    #[doc = "0: Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low"]
    SOFTWARE = 0,
    #[doc = "1: Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred"]
    AUTOMATIC = 1,
}
impl From<AUTOEND_A> for bool {
    #[inline(always)]
    fn from(variant: AUTOEND_A) -> Self {
        variant as u8 != 0
    }
}
impl AUTOEND_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> AUTOEND_A {
        match self.bits {
            false => AUTOEND_A::SOFTWARE,
            true => AUTOEND_A::AUTOMATIC,
        }
    }
    #[doc = "Checks if the value of the field is `SOFTWARE`"]
    #[inline(always)]
    pub fn is_software(&self) -> bool {
        *self == AUTOEND_A::SOFTWARE
    }
    #[doc = "Checks if the value of the field is `AUTOMATIC`"]
    #[inline(always)]
    pub fn is_automatic(&self) -> bool {
        *self == AUTOEND_A::AUTOMATIC
    }
}
#[doc = "Field `AUTOEND` writer - Automatic end mode (master mode)"]
pub type AUTOEND_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, AUTOEND_A>;
impl<'a, const O: u8> AUTOEND_W<'a, O> {
    #[doc = "Software end mode: TC flag is set when NBYTES data are transferred, stretching SCL low"]
    #[inline(always)]
    pub fn software(self) -> &'a mut W {
        self.variant(AUTOEND_A::SOFTWARE)
    }
    #[doc = "Automatic end mode: a STOP condition is automatically sent when NBYTES data are transferred"]
    #[inline(always)]
    pub fn automatic(self) -> &'a mut W {
        self.variant(AUTOEND_A::AUTOMATIC)
    }
}
#[doc = "Field `PECBYTE` reader - Packet error checking byte"]
pub type PECBYTE_R = crate::BitReader<PECBYTE_A>;
#[doc = "Packet error checking byte\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PECBYTE_A {
    #[doc = "0: No PEC transfer"]
    NO_PEC = 0,
    #[doc = "1: PEC transmission/reception is requested"]
    PEC = 1,
}
impl From<PECBYTE_A> for bool {
    #[inline(always)]
    fn from(variant: PECBYTE_A) -> Self {
        variant as u8 != 0
    }
}
impl PECBYTE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> PECBYTE_A {
        match self.bits {
            false => PECBYTE_A::NO_PEC,
            true => PECBYTE_A::PEC,
        }
    }
    #[doc = "Checks if the value of the field is `NO_PEC`"]
    #[inline(always)]
    pub fn is_no_pec(&self) -> bool {
        *self == PECBYTE_A::NO_PEC
    }
    #[doc = "Checks if the value of the field is `PEC`"]
    #[inline(always)]
    pub fn is_pec(&self) -> bool {
        *self == PECBYTE_A::PEC
    }
}
#[doc = "Field `PECBYTE` writer - Packet error checking byte"]
pub type PECBYTE_W<'a, const O: u8> = crate::BitWriter<'a, CR2_SPEC, O, PECBYTE_A>;
impl<'a, const O: u8> PECBYTE_W<'a, O> {
    #[doc = "No PEC transfer"]
    #[inline(always)]
    pub fn no_pec(self) -> &'a mut W {
        self.variant(PECBYTE_A::NO_PEC)
    }
    #[doc = "PEC transmission/reception is requested"]
    #[inline(always)]
    pub fn pec(self) -> &'a mut W {
        self.variant(PECBYTE_A::PEC)
    }
}
impl R {
    #[doc = "Bits 0:9 - Slave address bit 9:8 (master mode)"]
    #[inline(always)]
    pub fn sadd(&self) -> SADD_R {
        SADD_R::new((self.bits & 0x03ff) as u16)
    }
    #[doc = "Bit 10 - Transfer direction (master mode)"]
    #[inline(always)]
    pub fn rd_wrn(&self) -> RD_WRN_R {
        RD_WRN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode)"]
    #[inline(always)]
    pub fn add10(&self) -> ADD10_R {
        ADD10_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode)"]
    #[inline(always)]
    pub fn head10r(&self) -> HEAD10R_R {
        HEAD10R_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Start generation"]
    #[inline(always)]
    pub fn start(&self) -> START_R {
        START_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Stop generation (master mode)"]
    #[inline(always)]
    pub fn stop(&self) -> STOP_R {
        STOP_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - NACK generation (slave mode)"]
    #[inline(always)]
    pub fn nack(&self) -> NACK_R {
        NACK_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:23 - Number of bytes"]
    #[inline(always)]
    pub fn nbytes(&self) -> NBYTES_R {
        NBYTES_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bit 24 - NBYTES reload mode"]
    #[inline(always)]
    pub fn reload(&self) -> RELOAD_R {
        RELOAD_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode)"]
    #[inline(always)]
    pub fn autoend(&self) -> AUTOEND_R {
        AUTOEND_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Packet error checking byte"]
    #[inline(always)]
    pub fn pecbyte(&self) -> PECBYTE_R {
        PECBYTE_R::new(((self.bits >> 26) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:9 - Slave address bit 9:8 (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn sadd(&mut self) -> SADD_W<0> {
        SADD_W::new(self)
    }
    #[doc = "Bit 10 - Transfer direction (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn rd_wrn(&mut self) -> RD_WRN_W<10> {
        RD_WRN_W::new(self)
    }
    #[doc = "Bit 11 - 10-bit addressing mode (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn add10(&mut self) -> ADD10_W<11> {
        ADD10_W::new(self)
    }
    #[doc = "Bit 12 - 10-bit address header only read direction (master receiver mode)"]
    #[inline(always)]
    #[must_use]
    pub fn head10r(&mut self) -> HEAD10R_W<12> {
        HEAD10R_W::new(self)
    }
    #[doc = "Bit 13 - Start generation"]
    #[inline(always)]
    #[must_use]
    pub fn start(&mut self) -> START_W<13> {
        START_W::new(self)
    }
    #[doc = "Bit 14 - Stop generation (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn stop(&mut self) -> STOP_W<14> {
        STOP_W::new(self)
    }
    #[doc = "Bit 15 - NACK generation (slave mode)"]
    #[inline(always)]
    #[must_use]
    pub fn nack(&mut self) -> NACK_W<15> {
        NACK_W::new(self)
    }
    #[doc = "Bits 16:23 - Number of bytes"]
    #[inline(always)]
    #[must_use]
    pub fn nbytes(&mut self) -> NBYTES_W<16> {
        NBYTES_W::new(self)
    }
    #[doc = "Bit 24 - NBYTES reload mode"]
    #[inline(always)]
    #[must_use]
    pub fn reload(&mut self) -> RELOAD_W<24> {
        RELOAD_W::new(self)
    }
    #[doc = "Bit 25 - Automatic end mode (master mode)"]
    #[inline(always)]
    #[must_use]
    pub fn autoend(&mut self) -> AUTOEND_W<25> {
        AUTOEND_W::new(self)
    }
    #[doc = "Bit 26 - Packet error checking byte"]
    #[inline(always)]
    #[must_use]
    pub fn pecbyte(&mut self) -> PECBYTE_W<26> {
        PECBYTE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Control register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cr2](index.html) module"]
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
