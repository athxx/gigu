use makepad_widgets::*;

use super::state::{ME, cycle_role};

script_mod! {
    use mod.prelude.widgets.*

    let CARD_BG = #xffffff
    let CARD_BORDER = #xeeeeee
    let TEXT_PRIMARY = #x222222
    let TEXT_SECONDARY = #x6b6b6b
    let TEXT_MUTED = #x9a9a9a
    let ACCENT = #xff7a45
    let PAGE_BG = #xfafaf7

    let ProfileHeader = SolidView{
        width: Fill height: Fit
        padding: Inset{top: 24. bottom: 16. left: 16. right: 16.}
        flow: Right spacing: 12
        align: Align{y: 0.5}
        draw_bg.color: PAGE_BG

        avatar := Label{
            text: "🦊"
            draw_text.text_style.font_size: 44.
        }
        View{
            width: Fill height: Fit
            flow: Down spacing: 4
            View{
                width: Fill height: Fit
                flow: Right spacing: 6
                align: Align{y: 0.5}
                nickname := Label{
                    text: "—"
                    draw_text.color: TEXT_PRIMARY
                    draw_text.text_style: theme.font_bold{font_size: 18.}
                }
                verified_badge := RoundedView{
                    width: Fit height: Fit
                    visible: false
                    padding: Inset{top: 1. bottom: 1. left: 5. right: 5.}
                    show_bg: true
                    draw_bg.color: #xfff1ec
                    draw_bg.border_radius: 6.
                    Label{
                        text: "已认证"
                        draw_text.color: ACCENT
                        draw_text.text_style.font_size: 10.
                    }
                }
            }
            city_label := Label{
                text: "—"
                draw_text.color: TEXT_MUTED
                draw_text.text_style.font_size: 12.
            }
            credit_label := Label{
                text: "信用 0"
                draw_text.color: TEXT_SECONDARY
                draw_text.text_style.font_size: 12.
            }
        }
    }

    let StatsCard = RoundedView{
        width: Fill height: Fit
        margin: Inset{top: 0. bottom: 12. left: 12. right: 12.}
        padding: Inset{top: 14. bottom: 14. left: 12. right: 12.}
        show_bg: true
        draw_bg.color: CARD_BG
        draw_bg.border_radius: 12.
        draw_bg.border_size: 1.
        draw_bg.border_color: CARD_BORDER
        flow: Right
        align: Align{y: 0.5}

        View{
            width: Fill height: Fit
            flow: Down spacing: 4
            align: Align{x: 0.5}
            stat_fav := Label{
                text: "0"
                draw_text.color: TEXT_PRIMARY
                draw_text.text_style: theme.font_bold{font_size: 16.}
            }
            Label{
                text: "收藏"
                draw_text.color: TEXT_MUTED
                draw_text.text_style.font_size: 11.
            }
        }
        View{
            width: Fill height: Fit
            flow: Down spacing: 4
            align: Align{x: 0.5}
            stat_pub := Label{
                text: "0"
                draw_text.color: TEXT_PRIMARY
                draw_text.text_style: theme.font_bold{font_size: 16.}
            }
            Label{
                text: "发布"
                draw_text.color: TEXT_MUTED
                draw_text.text_style.font_size: 11.
            }
        }
        View{
            width: Fill height: Fit
            flow: Down spacing: 4
            align: Align{x: 0.5}
            stat_follow := Label{
                text: "0"
                draw_text.color: TEXT_PRIMARY
                draw_text.text_style: theme.font_bold{font_size: 16.}
            }
            Label{
                text: "关注"
                draw_text.color: TEXT_MUTED
                draw_text.text_style.font_size: 11.
            }
        }
    }

    let RoleCard = RoundedView{
        width: Fill height: Fit
        margin: Inset{top: 0. bottom: 12. left: 12. right: 12.}
        padding: Inset{top: 14. bottom: 14. left: 14. right: 14.}
        show_bg: true
        draw_bg.color: CARD_BG
        draw_bg.border_radius: 12.
        draw_bg.border_size: 1.
        draw_bg.border_color: CARD_BORDER
        flow: Right spacing: 8
        align: Align{y: 0.5}

        View{
            width: Fill height: Fit
            flow: Down spacing: 4
            Label{
                text: "当前身份"
                draw_text.color: TEXT_MUTED
                draw_text.text_style.font_size: 11.
            }
            role_label := Label{
                text: "—"
                draw_text.color: TEXT_PRIMARY
                draw_text.text_style: theme.font_bold{font_size: 14.}
            }
        }
        switch_role_btn := Button{
            text: "切换身份"
            width: Fit height: Fit
        }
    }

    let MenuRow = View{
        width: Fill height: Fit
        padding: Inset{top: 14. bottom: 14. left: 16. right: 16.}
        flow: Right spacing: 12
        align: Align{y: 0.5}
        cursor: MouseCursor.Hand
        show_bg: true
        draw_bg.color: CARD_BG
        menu_icon := Label{
            text: "•"
            draw_text.text_style.font_size: 16.
        }
        menu_label := Label{
            text: "—"
            draw_text.color: TEXT_PRIMARY
            draw_text.text_style.font_size: 14.
        }
        Filler{}
        Label{
            text: "›"
            draw_text.color: TEXT_MUTED
            draw_text.text_style.font_size: 14.
        }
    }

    let MenuDivider = SolidView{
        width: Fill height: 1
        margin: Inset{left: 44. right: 0.}
        draw_bg.color: CARD_BORDER
    }

    mod.widgets.MePageBase = #(MePage::register_widget(vm))
    mod.widgets.MePage = set_type_default() do mod.widgets.MePageBase{
        width: Fill height: Fill
        flow: Down
        show_bg: true
        draw_bg.color: PAGE_BG

        ProfileHeader{}
        StatsCard{}
        RoleCard{}

        RoundedView{
            width: Fill height: Fit
            margin: Inset{top: 0. bottom: 12. left: 12. right: 12.}
            show_bg: true
            draw_bg.color: CARD_BG
            draw_bg.border_radius: 12.
            draw_bg.border_size: 1.
            draw_bg.border_color: CARD_BORDER
            flow: Down

            menu_favorites := MenuRow{
                menu_icon: { text: "⭐" }
                menu_label: { text: "我的收藏" }
            }
            MenuDivider{}
            menu_published := MenuRow{
                menu_icon: { text: "📝" }
                menu_label: { text: "我的发布" }
            }
            MenuDivider{}
            menu_orders := MenuRow{
                menu_icon: { text: "🧾" }
                menu_label: { text: "订单与交易" }
            }
            MenuDivider{}
            menu_settings := MenuRow{
                menu_icon: { text: "⚙️" }
                menu_label: { text: "设置" }
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct MePage {
    #[deref]
    view: View,
}

impl Widget for MePage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let me = ME.read().unwrap();
        let avatar = if me.avatar_emoji.is_empty() {
            "🙂"
        } else {
            me.avatar_emoji.as_str()
        };
        self.view.label(cx, ids!(avatar)).set_text(cx, avatar);
        self.view.label(cx, ids!(nickname)).set_text(cx, &me.nickname);
        self.view
            .view(cx, ids!(verified_badge))
            .set_visible(cx, me.verified);
        self.view
            .label(cx, ids!(city_label))
            .set_text(cx, &me.city);
        self.view
            .label(cx, ids!(credit_label))
            .set_text(cx, &format!("信用 {}", me.credit_score));
        self.view
            .label(cx, ids!(stat_fav))
            .set_text(cx, &me.stats.favorites.to_string());
        self.view
            .label(cx, ids!(stat_pub))
            .set_text(cx, &me.stats.published.to_string());
        self.view
            .label(cx, ids!(stat_follow))
            .set_text(cx, &me.stats.following.to_string());
        self.view
            .label(cx, ids!(role_label))
            .set_text(cx, me.role.label());
        drop(me);
        self.view.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl MePage {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) -> bool {
        let mut needs_redraw = false;
        if self
            .view
            .button(cx, ids!(switch_role_btn))
            .clicked(actions)
        {
            cycle_role();
            needs_redraw = true;
        }
        if needs_redraw {
            self.view.redraw(cx);
        }
        needs_redraw
    }
}
