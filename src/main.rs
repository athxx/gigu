#![allow(dead_code)]

pub use makepad_widgets;

use makepad_widgets::*;

app_main!(App);

script_mod! {
    use mod.prelude.widgets.*

    let NAV_BG = #xfafaf7
    let NAV_LABEL = #x9a9a9a
    let NAV_LABEL_ACTIVE = #x222222
    let BADGE_BG = #xff3b30
    let BADGE_TEXT = #xffffff
    let PAGE_BG = #xffffff

    let NavTab = View{
        width: Fill
        height: Fill
        flow: Down
        spacing: 2
        align: Align{x: 0.5 y: 0.5}
        padding: Inset{top: 6. bottom: 6. left: 0. right: 0.}
        cursor: MouseCursor.Hand
    }

    startup() do #(App::script_component(vm)){
        ui: Root{
            main_window := Window{
                window.title: "gigu"
                window.inner_size: vec2(390, 844)
                pass.clear_color: vec4(1.0, 1.0, 1.0, 1.0)
                body +: {
                    width: Fill
                    height: Fill
                    flow: Down

                    page_flip := PageFlip{
                        width: Fill
                        height: Fill
                        active_page: @page_around

                        page_around := View{
                            width: Fill
                            height: Fill
                            draw_bg.color: PAGE_BG
                            show_bg: true
                        }
                        page_discover := View{
                            width: Fill
                            height: Fill
                            draw_bg.color: PAGE_BG
                            show_bg: true
                        }
                        page_chats := View{
                            width: Fill
                            height: Fill
                            draw_bg.color: PAGE_BG
                            show_bg: true
                        }
                        page_me := View{
                            width: Fill
                            height: Fill
                            draw_bg.color: PAGE_BG
                            show_bg: true
                        }
                    }

                    bottom_nav := BottomNavBar{
                        width: Fill
                        height: 64
                        padding: Inset{top: 0. bottom: 6. left: 0. right: 0.}
                        draw_bg.color: NAV_BG
                        draw_bg.tint_alpha: 1.0
                        draw_bg.glass_amount: 0.0
                        draw_bg.shadow_color: #0000
                        draw_bg.shadow_radius: 0.0
                        draw_bg.shadow_offset: vec2(0, 0)
                        draw_bg.border_radius: 16

                        tab_around := NavTab{
                            icon_around_inactive := View{
                                width: Fit height: Fit
                                Icon{
                                    icon_walk: Walk{width: 32 height: 32}
                                    draw_icon.svg: crate_resource("self:resources/home.svg")
                                }
                            }
                            icon_around_active := View{
                                width: Fit height: Fit
                                visible: false
                                Icon{
                                    icon_walk: Walk{width: 32 height: 32}
                                    draw_icon.svg: crate_resource("self:resources/home_active.svg")
                                }
                            }
                            label_around_inactive := Label{
                                text: "Around"
                                draw_text.color: NAV_LABEL
                                draw_text.text_style.font_size: 10.
                            }
                            label_around_active := Label{
                                text: "Around"
                                draw_text.color: NAV_LABEL_ACTIVE
                                draw_text.text_style: theme.font_bold{font_size: 10.}
                                visible: false
                            }
                        }

                        tab_discover := NavTab{
                            icon_discover_inactive := View{
                                width: Fit height: Fit
                                Icon{
                                    icon_walk: Walk{width: 32 height: 32}
                                    draw_icon.svg: crate_resource("self:resources/discover.svg")
                                }
                            }
                            icon_discover_active := View{
                                width: Fit height: Fit
                                visible: false
                                Icon{
                                    icon_walk: Walk{width: 32 height: 32}
                                    draw_icon.svg: crate_resource("self:resources/discover_active.svg")
                                }
                            }
                            label_discover_inactive := Label{
                                text: "Discover"
                                draw_text.color: NAV_LABEL
                                draw_text.text_style.font_size: 10.
                            }
                            label_discover_active := Label{
                                text: "Discover"
                                draw_text.color: NAV_LABEL_ACTIVE
                                draw_text.text_style: theme.font_bold{font_size: 10.}
                                visible: false
                            }
                        }

                        tab_chats := NavTab{
                            View{
                                width: Fit
                                height: Fit
                                icon_chats_inactive := View{
                                    width: Fit height: Fit
                                    Icon{
                                        icon_walk: Walk{width: 32 height: 32}
                                        draw_icon.svg: crate_resource("self:resources/chats.svg")
                                    }
                                }
                                icon_chats_active := View{
                                    width: Fit height: Fit
                                    visible: false
                                    Icon{
                                        icon_walk: Walk{width: 32 height: 32}
                                        draw_icon.svg: crate_resource("self:resources/chats_active.svg")
                                    }
                                }
                                chats_badge := RoundedView{
                                    width: Fit
                                    height: Fit
                                    align: Align{x: 0.5 y: 0.5}
                                    padding: Inset{top: 1. bottom: 1. left: 4. right: 4.}
                                    draw_bg.color: BADGE_BG
                                    draw_bg.border_radius: 8.
                                    abs_pos: vec2(14, -4)
                                    Label{
                                        text: "99"
                                        draw_text.color: BADGE_TEXT
                                        draw_text.text_style: theme.font_bold{font_size: 9.}
                                    }
                                }
                            }
                            label_chats_inactive := Label{
                                text: "Chats"
                                draw_text.color: NAV_LABEL
                                draw_text.text_style.font_size: 10.
                            }
                            label_chats_active := Label{
                                text: "Chats"
                                draw_text.color: NAV_LABEL_ACTIVE
                                draw_text.text_style: theme.font_bold{font_size: 10.}
                                visible: false
                            }
                        }

                        tab_me := NavTab{
                            icon_me_inactive := View{
                                width: Fit height: Fit
                                Icon{
                                    icon_walk: Walk{width: 32 height: 32}
                                    draw_icon.svg: crate_resource("self:resources/me.svg")
                                }
                            }
                            icon_me_active := View{
                                width: Fit height: Fit
                                visible: false
                                Icon{
                                    icon_walk: Walk{width: 32 height: 32}
                                    draw_icon.svg: crate_resource("self:resources/me_active.svg")
                                }
                            }
                            label_me_inactive := Label{
                                text: "Me"
                                draw_text.color: NAV_LABEL
                                draw_text.text_style.font_size: 10.
                            }
                            label_me_active := Label{
                                text: "Me"
                                draw_text.color: NAV_LABEL_ACTIVE
                                draw_text.text_style: theme.font_bold{font_size: 10.}
                                visible: false
                            }
                        }
                    }
                }
            }
        }
    }
}

#[derive(Script, ScriptHook)]
pub struct App {
    #[live]
    ui: WidgetRef,
    #[rust]
    active: ActiveTab,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum ActiveTab {
    Around,
    Discover,
    Chats,
    Me,
}

impl Default for ActiveTab {
    fn default() -> Self {
        Self::Around
    }
}

impl MatchEvent for App {
    fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) {
        if self
            .ui
            .view(cx, ids!(tab_around))
            .finger_up(actions)
            .is_some()
        {
            self.switch_page(cx, ActiveTab::Around);
        }
        if self
            .ui
            .view(cx, ids!(tab_discover))
            .finger_up(actions)
            .is_some()
        {
            self.switch_page(cx, ActiveTab::Discover);
        }
        if self
            .ui
            .view(cx, ids!(tab_chats))
            .finger_up(actions)
            .is_some()
        {
            self.switch_page(cx, ActiveTab::Chats);
        }
        if self.ui.view(cx, ids!(tab_me)).finger_up(actions).is_some() {
            self.switch_page(cx, ActiveTab::Me);
        }
    }
}

impl AppMain for App {
    fn script_mod(vm: &mut ScriptVm) -> ScriptValue {
        crate::makepad_widgets::script_mod(vm);
        ::ui::script_mod(vm);
        self::script_mod(vm)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event) {
        self.match_event(cx, event);
        self.ui.handle_event(cx, event, &mut Scope::empty());

        if let Event::Startup = event {
            self.apply_active(cx);
        }
    }
}

impl App {
    fn switch_page(&mut self, cx: &mut Cx, tab: ActiveTab) {
        self.active = tab;
        let page_id = match tab {
            ActiveTab::Around => live_id!(page_around),
            ActiveTab::Discover => live_id!(page_discover),
            ActiveTab::Chats => live_id!(page_chats),
            ActiveTab::Me => live_id!(page_me),
        };
        self.ui
            .page_flip(cx, ids!(page_flip))
            .set_active_page(cx, page_id);
        self.apply_active(cx);
    }

    fn apply_active(&self, cx: &mut Cx) {
        let entries = [
            (
                ActiveTab::Around,
                ids!(icon_around_inactive),
                ids!(icon_around_active),
                ids!(label_around_inactive),
                ids!(label_around_active),
            ),
            (
                ActiveTab::Discover,
                ids!(icon_discover_inactive),
                ids!(icon_discover_active),
                ids!(label_discover_inactive),
                ids!(label_discover_active),
            ),
            (
                ActiveTab::Chats,
                ids!(icon_chats_inactive),
                ids!(icon_chats_active),
                ids!(label_chats_inactive),
                ids!(label_chats_active),
            ),
            (
                ActiveTab::Me,
                ids!(icon_me_inactive),
                ids!(icon_me_active),
                ids!(label_me_inactive),
                ids!(label_me_active),
            ),
        ];

        for (tab, inactive_icon, active_icon, label_inactive, label_active) in entries {
            let is_active = tab == self.active;
            self.ui
                .widget(cx, inactive_icon)
                .set_visible(cx, !is_active);
            self.ui.widget(cx, active_icon).set_visible(cx, is_active);
            self.ui
                .widget(cx, label_inactive)
                .set_visible(cx, !is_active);
            self.ui.widget(cx, label_active).set_visible(cx, is_active);
        }
    }
}
