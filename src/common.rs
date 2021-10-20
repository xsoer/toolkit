#[derive(Debug, Clone)]
pub enum Message {
    // 菜单事件
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
}
