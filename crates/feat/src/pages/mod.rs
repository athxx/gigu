use makepad_widgets::*;

pub mod around;
pub mod chats;
pub mod detail;
pub mod discover;
pub mod me;
pub mod state;

pub use around::AroundPage;
pub use chats::ChatsPage;
pub use detail::DetailOverlay;
pub use discover::DiscoverPage;
pub use me::MePage;

pub fn script_mod(vm: &mut ScriptVm) {
    around::script_mod(vm);
    discover::script_mod(vm);
    chats::script_mod(vm);
    me::script_mod(vm);
    detail::script_mod(vm);
}
