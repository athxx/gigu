pub mod domain {
    pub use domain::domain::*;
}

pub mod applet;
pub mod auth;
pub mod discovery;
pub mod home;
pub mod local_goods;
pub mod local_services;
pub mod map;
pub mod merchant;
pub mod messaging;
pub mod pages;
pub mod profile;
pub mod publish;
pub mod trust_safety;
pub mod webview;

pub fn script_mod(vm: &mut makepad_widgets::ScriptVm) {
    use makepad_widgets::*;
    pages::script_mod(vm);
    script_eval!(vm, {
        mod.prelude.widgets = {
            ..mod.prelude.widgets,
            ..mod.widgets,
        }
    });
}
