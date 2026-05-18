use crate::app::module_registry::{ModuleDescriptor, product_modules};
use crate::app::router::RouterState;
use crate::app::shell::AppShell;
use crate::app::state::AppState;

pub struct AppModule {
    pub state: AppState,
    pub shell: AppShell,
    pub modules: &'static [ModuleDescriptor],
}

impl Default for AppModule {
    fn default() -> Self {
        Self {
            state: AppState::default(),
            shell: AppShell::default(),
            modules: product_modules(),
        }
    }
}

impl AppModule {
    pub fn current_route(&self) -> &str {
        self.state.router.current.as_str()
    }

    pub fn router(&self) -> &RouterState {
        &self.state.router
    }

    pub fn modules(&self) -> &'static [ModuleDescriptor] {
        self.modules
    }
}
