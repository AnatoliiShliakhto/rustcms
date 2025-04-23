#[macro_export]
macro_rules! hide_dialog {
    () => {
        $crate::app::use_state().dialog_args.set(None)
    };
}

#[macro_export]
macro_rules! alert_dialog {
    ($caption:expr, $message:expr) => {
        $crate::app::use_state().dialog_args.set(Some($crate::components::DialogArgs {
            kind: $crate::components::MessageKind::Alert,
            caption: ::dioxus_i18n::tid!($caption),
            message: ::dioxus_i18n::tid!($message),
            handler: None,
        }))
    };

    ($caption:expr, $message:expr, $handler:ident) => {
        $crate::app::use_state().dialog_args.set(Some($crate::components::DialogArgs {
            kind: $crate::components::MessageKind::Alert,
            caption: ::dioxus_i18n::tid!($caption),
            message: ::dioxus_i18n::tid!($message),
            handler: Some($handler),
        }))
    };
}
