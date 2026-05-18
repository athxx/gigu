#[derive(Clone, Debug)]
pub struct ModuleDescriptor {
    pub key: &'static str,
    pub display_name: &'static str,
    pub priority: ModulePriority,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ModulePriority {
    P0,
    P1,
    P2,
    P3,
}

pub const PRODUCT_MODULES: &[ModuleDescriptor] = &[
    ModuleDescriptor {
        key: "auth",
        display_name: "账号与身份",
        priority: ModulePriority::P0,
    },
    ModuleDescriptor {
        key: "map",
        display_name: "地图与定位",
        priority: ModulePriority::P0,
    },
    ModuleDescriptor {
        key: "discovery",
        display_name: "搜索与发现",
        priority: ModulePriority::P0,
    },
    ModuleDescriptor {
        key: "applet",
        display_name: "小程序生态",
        priority: ModulePriority::P0,
    },
    ModuleDescriptor {
        key: "webview",
        display_name: "WebView 容器",
        priority: ModulePriority::P0,
    },
    ModuleDescriptor {
        key: "local_goods",
        display_name: "附近商品",
        priority: ModulePriority::P0,
    },
    ModuleDescriptor {
        key: "merchant",
        display_name: "商家店铺",
        priority: ModulePriority::P0,
    },
    ModuleDescriptor {
        key: "local_services",
        display_name: "本地服务",
        priority: ModulePriority::P0,
    },
    ModuleDescriptor {
        key: "publish",
        display_name: "发布中心",
        priority: ModulePriority::P0,
    },
    ModuleDescriptor {
        key: "messaging",
        display_name: "即时通讯",
        priority: ModulePriority::P0,
    },
    ModuleDescriptor {
        key: "trust_safety",
        display_name: "风控与审核",
        priority: ModulePriority::P0,
    },
    ModuleDescriptor {
        key: "profile",
        display_name: "我的与设置",
        priority: ModulePriority::P0,
    },
];

pub fn product_modules() -> &'static [ModuleDescriptor] {
    PRODUCT_MODULES
}
