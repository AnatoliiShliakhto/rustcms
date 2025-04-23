#[macro_export]
macro_rules! get {
    ($path:expr) => {
        {
            use $crate::{app::use_state, services::ResponseService};
            let state = use_state();
            state
                .client()
                .get(state.url($path))
                .send()
                .await
                .check()
                .await
        }
    };
}
