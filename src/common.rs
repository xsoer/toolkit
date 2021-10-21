#[derive(Debug, Clone)]
pub enum Message {
    // 菜单事件
    MenuClockPressed,
    MenuUuidPressed,
    MenuPwdPressed,
    MenuTimePressed,
    MenuEncoderPressed,
    MenuBasePressed,
    MenuRegexPressed,
    MenuQrcodePressed,
    MenuColorPressed,
    MenuCrontabPressed,
    // uuid模块
    UuidBtnPressed,
    UuidCountInputChanged(String),
    UuidCheckUpperToggled(bool),
    UuidCheckSplitToggled(bool),
    UuidCheckBraceToggled(bool),
    // 密码模块
    PwdBtnPressed,
    PwdCheckUpperToggled(bool),
    PwdCheckLowerToggled(bool),
    PwdCheckDigitToggled(bool),
    PwdCheckSpecialToggled(bool),
    PwdSliderChanged(u8),
    PwdLenInputChanged(String),
    PwdNumInputChanged(String),
    // 时间戳模块
    TimeBtnPressed,
    TimeTimeStampChanged(String),
    TimeDateTimeChanged(String),
    // qrcode
    QrCodeDataChanged(String),
    // Clock
    ClockTick(chrono::DateTime<chrono::Local>),
    // Base
    BaseFromValueChanged(String),
    BaseBtnChanged,
    BaseRadioSelected(BaseType),
    // EnDeCode
    EnDeCodeRadioSelected(EnDeCodeType),
    EnDeCodeEncodeChanged(String),
    EnDeCodeDecodeChanged(String),
    EnDeCodeBtnEncodeToggled,
    EnDeCodeBtnDecodeToggled,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BaseType {
    Base2,
    Base8,
    Base10,
    Base16,
    Base26,
    Base32,
    Base36,
    Base52,
    Base58,
    Base62,
    Base64,
}

impl BaseType {
    pub fn all() -> [BaseType; 11] {
        [
            BaseType::Base2,
            BaseType::Base8,
            BaseType::Base10,
            BaseType::Base16,
            BaseType::Base26,
            BaseType::Base32,
            BaseType::Base36,
            BaseType::Base52,
            BaseType::Base58,
            BaseType::Base62,
            BaseType::Base64,
        ]
    }
}

impl From<BaseType> for String {
    fn from(b: BaseType) -> String {
        String::from(match b {
            BaseType::Base2 => "2进制",
            BaseType::Base8 => "8进制",
            BaseType::Base10 => "10进制",
            BaseType::Base16 => "16进制",
            BaseType::Base26 => "26进制",
            BaseType::Base32 => "32进制",
            BaseType::Base36 => "36进制",
            BaseType::Base52 => "52进制",
            BaseType::Base58 => "58进制",
            BaseType::Base62 => "62进制",
            BaseType::Base64 => "64进制",
        })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EnDeCodeType {
    Unicode,
    Uri,
}

impl EnDeCodeType {
    pub fn all() -> [EnDeCodeType; 2] {
        [EnDeCodeType::Unicode, EnDeCodeType::Uri]
    }
}

impl From<EnDeCodeType> for String {
    fn from(c: EnDeCodeType) -> String {
        String::from(match c {
            EnDeCodeType::Unicode => "Unicode",
            EnDeCodeType::Uri => "URI",
        })
    }
}
