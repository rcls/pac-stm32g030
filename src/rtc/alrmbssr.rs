#[doc = "Register `ALRMBSSR` reader"]
pub type R = crate::R<ALRMBSSR_SPEC>;
#[doc = "Register `ALRMBSSR` writer"]
pub type W = crate::W<ALRMBSSR_SPEC>;
#[doc = "Field `SS` reader - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared."]
pub type SS_R = crate::FieldReader<u16>;
#[doc = "Field `SS` writer - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared."]
pub type SS_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation.\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MASKSS_A {
    #[doc = "0: No comparison on sub seconds for alarm B. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match)."]
    B_0x0 = 0,
    #[doc = "1: SS\\[14:1\\] are don't care in alarm B comparison. Only SS\\[0\\] is compared."]
    B_0x1 = 1,
    #[doc = "2: SS\\[14:2\\] are don't care in alarm B comparison. Only SS\\[1:0\\] are compared."]
    B_0x2 = 2,
    #[doc = "3: SS\\[14:3\\] are don't care in alarm B comparison. Only SS\\[2:0\\] are compared."]
    B_0x3 = 3,
    #[doc = "12: SS\\[14:12\\] are don't care in alarm B comparison. SS\\[11:0\\] are compared."]
    B_0xC = 12,
    #[doc = "13: SS\\[14:13\\] are don't care in alarm B comparison. SS\\[12:0\\] are compared."]
    B_0xD = 13,
    #[doc = "14: SS\\[14\\] is don't care in alarm B comparison. SS\\[13:0\\] are compared."]
    B_0xE = 14,
    #[doc = "15: All 15 SS bits are compared and must match to activate alarm."]
    B_0xF = 15,
}
impl From<MASKSS_A> for u8 {
    #[inline(always)]
    fn from(variant: MASKSS_A) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for MASKSS_A {
    type Ux = u8;
}
impl crate::IsEnum for MASKSS_A {}
#[doc = "Field `MASKSS` reader - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
pub type MASKSS_R = crate::FieldReader<MASKSS_A>;
impl MASKSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<MASKSS_A> {
        match self.bits {
            0 => Some(MASKSS_A::B_0x0),
            1 => Some(MASKSS_A::B_0x1),
            2 => Some(MASKSS_A::B_0x2),
            3 => Some(MASKSS_A::B_0x3),
            12 => Some(MASKSS_A::B_0xC),
            13 => Some(MASKSS_A::B_0xD),
            14 => Some(MASKSS_A::B_0xE),
            15 => Some(MASKSS_A::B_0xF),
            _ => None,
        }
    }
    #[doc = "No comparison on sub seconds for alarm B. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match)."]
    #[inline(always)]
    pub fn is_B_0x0(&self) -> bool {
        *self == MASKSS_A::B_0x0
    }
    #[doc = "SS\\[14:1\\] are don't care in alarm B comparison. Only SS\\[0\\] is compared."]
    #[inline(always)]
    pub fn is_B_0x1(&self) -> bool {
        *self == MASKSS_A::B_0x1
    }
    #[doc = "SS\\[14:2\\] are don't care in alarm B comparison. Only SS\\[1:0\\] are compared."]
    #[inline(always)]
    pub fn is_B_0x2(&self) -> bool {
        *self == MASKSS_A::B_0x2
    }
    #[doc = "SS\\[14:3\\] are don't care in alarm B comparison. Only SS\\[2:0\\] are compared."]
    #[inline(always)]
    pub fn is_B_0x3(&self) -> bool {
        *self == MASKSS_A::B_0x3
    }
    #[doc = "SS\\[14:12\\] are don't care in alarm B comparison. SS\\[11:0\\] are compared."]
    #[inline(always)]
    pub fn is_B_0xC(&self) -> bool {
        *self == MASKSS_A::B_0xC
    }
    #[doc = "SS\\[14:13\\] are don't care in alarm B comparison. SS\\[12:0\\] are compared."]
    #[inline(always)]
    pub fn is_B_0xD(&self) -> bool {
        *self == MASKSS_A::B_0xD
    }
    #[doc = "SS\\[14\\] is don't care in alarm B comparison. SS\\[13:0\\] are compared."]
    #[inline(always)]
    pub fn is_B_0xE(&self) -> bool {
        *self == MASKSS_A::B_0xE
    }
    #[doc = "All 15 SS bits are compared and must match to activate alarm."]
    #[inline(always)]
    pub fn is_B_0xF(&self) -> bool {
        *self == MASKSS_A::B_0xF
    }
}
#[doc = "Field `MASKSS` writer - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
pub type MASKSS_W<'a, REG> = crate::FieldWriter<'a, REG, 4, MASKSS_A>;
impl<'a, REG> MASKSS_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No comparison on sub seconds for alarm B. The alarm is set when the seconds unit is incremented (assuming that the rest of the fields match)."]
    #[inline(always)]
    pub fn B_0x0(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS_A::B_0x0)
    }
    #[doc = "SS\\[14:1\\] are don't care in alarm B comparison. Only SS\\[0\\] is compared."]
    #[inline(always)]
    pub fn B_0x1(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS_A::B_0x1)
    }
    #[doc = "SS\\[14:2\\] are don't care in alarm B comparison. Only SS\\[1:0\\] are compared."]
    #[inline(always)]
    pub fn B_0x2(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS_A::B_0x2)
    }
    #[doc = "SS\\[14:3\\] are don't care in alarm B comparison. Only SS\\[2:0\\] are compared."]
    #[inline(always)]
    pub fn B_0x3(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS_A::B_0x3)
    }
    #[doc = "SS\\[14:12\\] are don't care in alarm B comparison. SS\\[11:0\\] are compared."]
    #[inline(always)]
    pub fn B_0xC(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS_A::B_0xC)
    }
    #[doc = "SS\\[14:13\\] are don't care in alarm B comparison. SS\\[12:0\\] are compared."]
    #[inline(always)]
    pub fn B_0xD(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS_A::B_0xD)
    }
    #[doc = "SS\\[14\\] is don't care in alarm B comparison. SS\\[13:0\\] are compared."]
    #[inline(always)]
    pub fn B_0xE(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS_A::B_0xE)
    }
    #[doc = "All 15 SS bits are compared and must match to activate alarm."]
    #[inline(always)]
    pub fn B_0xF(self) -> &'a mut crate::W<REG> {
        self.variant(MASKSS_A::B_0xF)
    }
}
impl R {
    #[doc = "Bits 0:14 - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared."]
    #[inline(always)]
    pub fn SS(&self) -> SS_R {
        SS_R::new((self.bits & 0x7fff) as u16)
    }
    #[doc = "Bits 24:27 - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
    #[inline(always)]
    pub fn MASKSS(&self) -> MASKSS_R {
        MASKSS_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:14 - Sub seconds value This value is compared with the contents of the synchronous prescaler counter to determine if alarm B is to be activated. Only bits 0 up to MASKSS-1 are compared."]
    #[inline(always)]
    pub fn SS(&mut self) -> SS_W<'_, ALRMBSSR_SPEC> {
        SS_W::new(self, 0)
    }
    #[doc = "Bits 24:27 - Mask the most-significant bits starting at this bit ... The overflow bits of the synchronous counter (bits 15) is never compared. This bit can be different from 0 only after a shift operation."]
    #[inline(always)]
    pub fn MASKSS(&mut self) -> MASKSS_W<'_, ALRMBSSR_SPEC> {
        MASKSS_W::new(self, 24)
    }
}
#[doc = "RTC alarm B sub second register\n\nYou can [`read`](crate::Reg::read) this register and get [`alrmbssr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`alrmbssr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ALRMBSSR_SPEC;
impl crate::RegisterSpec for ALRMBSSR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`alrmbssr::R`](R) reader structure"]
impl crate::Readable for ALRMBSSR_SPEC {}
#[doc = "`write(|w| ..)` method takes [`alrmbssr::W`](W) writer structure"]
impl crate::Writable for ALRMBSSR_SPEC {
    type Safety = crate::Safe;
}
#[doc = "`reset()` method sets ALRMBSSR to value 0"]
impl crate::Resettable for ALRMBSSR_SPEC {}
