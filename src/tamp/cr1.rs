#[doc = "Register `CR1` reader"]
pub type R = crate::R<CR1_SPEC>;
#[doc = "Register `CR1` writer"]
pub type W = crate::W<CR1_SPEC>;
#[doc = "Tamper detection on TAMP_IN1 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP1E_A {
    #[doc = "0: Tamper detection on TAMP_IN1 is disabled."]
    B_0x0 = 0,
    #[doc = "1: Tamper detection on TAMP_IN1 is enabled."]
    B_0x1 = 1,
}
impl From<TAMP1E_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP1E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP1E` reader - Tamper detection on TAMP_IN1 enable"]
pub type TAMP1E_R = crate::BitReader<TAMP1E_A>;
impl TAMP1E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP1E_A {
        match self.bits {
            false => TAMP1E_A::B_0x0,
            true => TAMP1E_A::B_0x1,
        }
    }
    #[doc = "Tamper detection on TAMP_IN1 is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TAMP1E_A::B_0x0
    }
    #[doc = "Tamper detection on TAMP_IN1 is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TAMP1E_A::B_0x1
    }
}
#[doc = "Field `TAMP1E` writer - Tamper detection on TAMP_IN1 enable"]
pub type TAMP1E_W<'a, REG> = crate::BitWriter<'a, REG, TAMP1E_A>;
impl<'a, REG> TAMP1E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper detection on TAMP_IN1 is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1E_A::B_0x0)
    }
    #[doc = "Tamper detection on TAMP_IN1 is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP1E_A::B_0x1)
    }
}
#[doc = "Tamper detection on TAMP_IN2 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP2E_A {
    #[doc = "0: Tamper detection on TAMP_IN2 is disabled."]
    B_0x0 = 0,
    #[doc = "1: Tamper detection on TAMP_IN2 is enabled."]
    B_0x1 = 1,
}
impl From<TAMP2E_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP2E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP2E` reader - Tamper detection on TAMP_IN2 enable"]
pub type TAMP2E_R = crate::BitReader<TAMP2E_A>;
impl TAMP2E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP2E_A {
        match self.bits {
            false => TAMP2E_A::B_0x0,
            true => TAMP2E_A::B_0x1,
        }
    }
    #[doc = "Tamper detection on TAMP_IN2 is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TAMP2E_A::B_0x0
    }
    #[doc = "Tamper detection on TAMP_IN2 is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TAMP2E_A::B_0x1
    }
}
#[doc = "Field `TAMP2E` writer - Tamper detection on TAMP_IN2 enable"]
pub type TAMP2E_W<'a, REG> = crate::BitWriter<'a, REG, TAMP2E_A>;
impl<'a, REG> TAMP2E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper detection on TAMP_IN2 is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP2E_A::B_0x0)
    }
    #[doc = "Tamper detection on TAMP_IN2 is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP2E_A::B_0x1)
    }
}
#[doc = "Tamper detection on TAMP_IN3 enable\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TAMP3E_A {
    #[doc = "0: Tamper detection on TAMP_IN3 is disabled."]
    B_0x0 = 0,
    #[doc = "1: Tamper detection on TAMP_IN3 is enabled."]
    B_0x1 = 1,
}
impl From<TAMP3E_A> for bool {
    #[inline(always)]
    fn from(variant: TAMP3E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TAMP3E` reader - Tamper detection on TAMP_IN3 enable"]
pub type TAMP3E_R = crate::BitReader<TAMP3E_A>;
impl TAMP3E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> TAMP3E_A {
        match self.bits {
            false => TAMP3E_A::B_0x0,
            true => TAMP3E_A::B_0x1,
        }
    }
    #[doc = "Tamper detection on TAMP_IN3 is disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == TAMP3E_A::B_0x0
    }
    #[doc = "Tamper detection on TAMP_IN3 is enabled."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == TAMP3E_A::B_0x1
    }
}
#[doc = "Field `TAMP3E` writer - Tamper detection on TAMP_IN3 enable"]
pub type TAMP3E_W<'a, REG> = crate::BitWriter<'a, REG, TAMP3E_A>;
impl<'a, REG> TAMP3E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Tamper detection on TAMP_IN3 is disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP3E_A::B_0x0)
    }
    #[doc = "Tamper detection on TAMP_IN3 is enabled."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(TAMP3E_A::B_0x1)
    }
}
#[doc = "Internal tamper 3 enable: LSE monitoring\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP3E_A {
    #[doc = "0: Internal tamper 3 disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 3 enabled: a tamper is generated when the LSE frequency is below or above thresholds."]
    B_0x1 = 1,
}
impl From<ITAMP3E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP3E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP3E` reader - Internal tamper 3 enable: LSE monitoring"]
pub type ITAMP3E_R = crate::BitReader<ITAMP3E_A>;
impl ITAMP3E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP3E_A {
        match self.bits {
            false => ITAMP3E_A::B_0x0,
            true => ITAMP3E_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 3 disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP3E_A::B_0x0
    }
    #[doc = "Internal tamper 3 enabled: a tamper is generated when the LSE frequency is below or above thresholds."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP3E_A::B_0x1
    }
}
#[doc = "Field `ITAMP3E` writer - Internal tamper 3 enable: LSE monitoring"]
pub type ITAMP3E_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP3E_A>;
impl<'a, REG> ITAMP3E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 3 disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3E_A::B_0x0)
    }
    #[doc = "Internal tamper 3 enabled: a tamper is generated when the LSE frequency is below or above thresholds."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP3E_A::B_0x1)
    }
}
#[doc = "Internal tamper 4 enable: HSE monitoring\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP4E_A {
    #[doc = "0: Internal tamper 4 disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 4 enabled. a tamper is generated when the HSE frequency is below or above thresholds."]
    B_0x1 = 1,
}
impl From<ITAMP4E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP4E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP4E` reader - Internal tamper 4 enable: HSE monitoring"]
pub type ITAMP4E_R = crate::BitReader<ITAMP4E_A>;
impl ITAMP4E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP4E_A {
        match self.bits {
            false => ITAMP4E_A::B_0x0,
            true => ITAMP4E_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 4 disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP4E_A::B_0x0
    }
    #[doc = "Internal tamper 4 enabled. a tamper is generated when the HSE frequency is below or above thresholds."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP4E_A::B_0x1
    }
}
#[doc = "Field `ITAMP4E` writer - Internal tamper 4 enable: HSE monitoring"]
pub type ITAMP4E_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP4E_A>;
impl<'a, REG> ITAMP4E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 4 disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP4E_A::B_0x0)
    }
    #[doc = "Internal tamper 4 enabled. a tamper is generated when the HSE frequency is below or above thresholds."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP4E_A::B_0x1)
    }
}
#[doc = "Internal tamper 5 enable: RTC calendar overflow\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP5E_A {
    #[doc = "0: Internal tamper 5 disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 5 enabled: a tamper is generated when the RTC calendar reaches its maximum value, on the 31st of December 99, at 23:59:59. The calendar is then frozen and cannot overflow."]
    B_0x1 = 1,
}
impl From<ITAMP5E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP5E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP5E` reader - Internal tamper 5 enable: RTC calendar overflow"]
pub type ITAMP5E_R = crate::BitReader<ITAMP5E_A>;
impl ITAMP5E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP5E_A {
        match self.bits {
            false => ITAMP5E_A::B_0x0,
            true => ITAMP5E_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 5 disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP5E_A::B_0x0
    }
    #[doc = "Internal tamper 5 enabled: a tamper is generated when the RTC calendar reaches its maximum value, on the 31st of December 99, at 23:59:59. The calendar is then frozen and cannot overflow."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP5E_A::B_0x1
    }
}
#[doc = "Field `ITAMP5E` writer - Internal tamper 5 enable: RTC calendar overflow"]
pub type ITAMP5E_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP5E_A>;
impl<'a, REG> ITAMP5E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 5 disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP5E_A::B_0x0)
    }
    #[doc = "Internal tamper 5 enabled: a tamper is generated when the RTC calendar reaches its maximum value, on the 31st of December 99, at 23:59:59. The calendar is then frozen and cannot overflow."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP5E_A::B_0x1)
    }
}
#[doc = "Internal tamper 6 enable: ST manufacturer readout\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ITAMP6E_A {
    #[doc = "0: Internal tamper 6 disabled."]
    B_0x0 = 0,
    #[doc = "1: Internal tamper 6 enabled: a tamper is generated in case of ST manufacturer readout."]
    B_0x1 = 1,
}
impl From<ITAMP6E_A> for bool {
    #[inline(always)]
    fn from(variant: ITAMP6E_A) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ITAMP6E` reader - Internal tamper 6 enable: ST manufacturer readout"]
pub type ITAMP6E_R = crate::BitReader<ITAMP6E_A>;
impl ITAMP6E_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITAMP6E_A {
        match self.bits {
            false => ITAMP6E_A::B_0x0,
            true => ITAMP6E_A::B_0x1,
        }
    }
    #[doc = "Internal tamper 6 disabled."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == ITAMP6E_A::B_0x0
    }
    #[doc = "Internal tamper 6 enabled: a tamper is generated in case of ST manufacturer readout."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == ITAMP6E_A::B_0x1
    }
}
#[doc = "Field `ITAMP6E` writer - Internal tamper 6 enable: ST manufacturer readout"]
pub type ITAMP6E_W<'a, REG> = crate::BitWriter<'a, REG, ITAMP6E_A>;
impl<'a, REG> ITAMP6E_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Internal tamper 6 disabled."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP6E_A::B_0x0)
    }
    #[doc = "Internal tamper 6 enabled: a tamper is generated in case of ST manufacturer readout."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(ITAMP6E_A::B_0x1)
    }
}
impl R {
    #[doc = "Bit 0 - Tamper detection on TAMP_IN1 enable"]
    #[inline(always)]
    pub fn TAMP1E(&self) -> TAMP1E_R {
        TAMP1E_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Tamper detection on TAMP_IN2 enable"]
    #[inline(always)]
    pub fn TAMP2E(&self) -> TAMP2E_R {
        TAMP2E_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Tamper detection on TAMP_IN3 enable"]
    #[inline(always)]
    pub fn TAMP3E(&self) -> TAMP3E_R {
        TAMP3E_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 18 - Internal tamper 3 enable: LSE monitoring"]
    #[inline(always)]
    pub fn ITAMP3E(&self) -> ITAMP3E_R {
        ITAMP3E_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Internal tamper 4 enable: HSE monitoring"]
    #[inline(always)]
    pub fn ITAMP4E(&self) -> ITAMP4E_R {
        ITAMP4E_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Internal tamper 5 enable: RTC calendar overflow"]
    #[inline(always)]
    pub fn ITAMP5E(&self) -> ITAMP5E_R {
        ITAMP5E_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Internal tamper 6 enable: ST manufacturer readout"]
    #[inline(always)]
    pub fn ITAMP6E(&self) -> ITAMP6E_R {
        ITAMP6E_R::new(((self.bits >> 21) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Tamper detection on TAMP_IN1 enable"]
    #[inline(always)]
    pub fn TAMP1E(&mut self) -> TAMP1E_W<'_, CR1_SPEC> {
        TAMP1E_W::new(self, 0)
    }
    #[doc = "Bit 1 - Tamper detection on TAMP_IN2 enable"]
    #[inline(always)]
    pub fn TAMP2E(&mut self) -> TAMP2E_W<'_, CR1_SPEC> {
        TAMP2E_W::new(self, 1)
    }
    #[doc = "Bit 2 - Tamper detection on TAMP_IN3 enable"]
    #[inline(always)]
    pub fn TAMP3E(&mut self) -> TAMP3E_W<'_, CR1_SPEC> {
        TAMP3E_W::new(self, 2)
    }
    #[doc = "Bit 18 - Internal tamper 3 enable: LSE monitoring"]
    #[inline(always)]
    pub fn ITAMP3E(&mut self) -> ITAMP3E_W<'_, CR1_SPEC> {
        ITAMP3E_W::new(self, 18)
    }
    #[doc = "Bit 19 - Internal tamper 4 enable: HSE monitoring"]
    #[inline(always)]
    pub fn ITAMP4E(&mut self) -> ITAMP4E_W<'_, CR1_SPEC> {
        ITAMP4E_W::new(self, 19)
    }
    #[doc = "Bit 20 - Internal tamper 5 enable: RTC calendar overflow"]
    #[inline(always)]
    pub fn ITAMP5E(&mut self) -> ITAMP5E_W<'_, CR1_SPEC> {
        ITAMP5E_W::new(self, 20)
    }
    #[doc = "Bit 21 - Internal tamper 6 enable: ST manufacturer readout"]
    #[inline(always)]
    pub fn ITAMP6E(&mut self) -> ITAMP6E_W<'_, CR1_SPEC> {
        ITAMP6E_W::new(self, 21)
    }
}
#[doc = "TAMP control register 1\n\nYou can [`read`](crate::Reg::read) this register and get [`cr1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CR1_SPEC;
impl crate::RegisterSpec for CR1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cr1::R`](R) reader structure"]
impl crate::Readable for CR1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cr1::W`](W) writer structure"]
impl crate::Writable for CR1_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets CR1 to value 0xffff_0000"]
impl crate::Resettable for CR1_SPEC {
    const RESET_VALUE: u32 = 0xffff_0000;
}
