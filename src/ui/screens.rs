#[derive(Clone, PartialEq)]
pub enum Screen {
    VaultCheck,
    CreateVault,
    UnlockVault,
    MainMenu,
    ViewPasswords,
    AddPassword,
    EditPassword,
    ViewHistory,
    SearchPassword,
    GeneratePassword,
    DeletePassword,
    FilterByTag,
    ThemeSelector,
    OptionsMenu,
    Help,
    Settings,
}

#[derive(Clone, PartialEq)]
pub enum InputField {
    None,
    Password,
    PasswordConfirm,
}

#[derive(Clone, PartialEq)]
pub enum MessageType {
    None,
    Success,
    Error,
    Info,
}
