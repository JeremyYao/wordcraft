//! The system's animation setting, which File › Options › Accessibility › Animation effects
//! follows when set to "Use system setting" (`Services::animations_enabled`).
//!
//! Windows: Settings › Accessibility › Visual effects › Animation effects, read through the
//! Windows Runtime's `UISettings`, a safe API of the `windows` crate (our workspace forbids
//! unsafe). macOS (Reduce motion) and the Linux and BSD desktops aren't read yet: the answer there
//! is `None`, so animation effects stay on unless they are switched off in Options.

/// Whether the system has animation effects on; `None` when it doesn't say.
#[cfg(target_os = "windows")]
pub fn system_enabled() -> Option<bool> {
    windows::UI::ViewManagement::UISettings::new()
        .and_then(|settings| settings.AnimationsEnabled())
        .map_err(|e| log::debug!("system animation setting: {e}"))
        .ok()
}

/// Whether the system has animation effects on; `None` when it doesn't say.
#[cfg(not(target_os = "windows"))]
pub fn system_enabled() -> Option<bool> {
    None
}
