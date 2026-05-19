pub mod comp;
pub mod theme;
pub mod views;

use makepad_widgets::*;

pub fn script_mod(vm: &mut ScriptVm) {
    crate::comp::bottom_nav::script_mod(vm);
    script_eval!(vm, {
        mod.prelude.widgets = {
            ..mod.prelude.widgets,
            ..mod.widgets,
        }
    });
}
