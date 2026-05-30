use makepad_widgets::*;

use super::state::{ACTIVE_DETAIL, DetailTarget};
use infra::infra::mock::MockApi;

script_mod! {
    use mod.prelude.widgets.*

    let CARD_BG = #xffffff
    let CARD_BORDER = #xeeeeee
    let TEXT_PRIMARY = #x222222
    let TEXT_SECONDARY = #x6b6b6b
    let TEXT_MUTED = #x9a9a9a
    let ACCENT = #xff7a45
    let PAGE_BG = #xfafaf7

    let DetailHeader = SolidView{
        width: Fill height: Fit
        padding: Inset{top: 12. bottom: 12. left: 12. right: 12.}
        flow: Right spacing: 8
        align: Align{y: 0.5}
        draw_bg.color: PAGE_BG
        detail_back := Label{
            text: "‹ 返回"
            draw_text.color: ACCENT
            draw_text.text_style.font_size: 14.
        }
        Filler{}
        detail_kind := Label{
            text: "详情"
            draw_text.color: TEXT_PRIMARY
            draw_text.text_style: theme.font_bold{font_size: 15.}
        }
        Filler{}
        Label{
            text: " "
            draw_text.text_style.font_size: 14.
        }
    }

    let DetailRow = View{
        width: Fill height: Fit
        margin: Inset{top: 4. bottom: 4. left: 16. right: 16.}
        flow: Right spacing: 8
        align: Align{y: 0.5}
        row_label := Label{
            text: "—"
            draw_text.color: TEXT_MUTED
            draw_text.text_style.font_size: 12.
        }
        row_value := Label{
            width: Fill height: Fit
            text: "—"
            draw_text.color: TEXT_PRIMARY
            draw_text.text_style.font_size: 12.
        }
    }

    mod.widgets.DetailOverlayBase = #(DetailOverlay::register_widget(vm))
    mod.widgets.DetailOverlay = set_type_default() do mod.widgets.DetailOverlayBase{
        width: Fill height: Fill
        visible: false
        flow: Down
        show_bg: true
        draw_bg.color: PAGE_BG

        DetailHeader{}

        body_view := View{
            width: Fill height: Fill
            flow: Down spacing: 8
            padding: Inset{top: 12. bottom: 12. left: 0. right: 0.}

            View{
                width: Fill height: Fit
                flow: Right spacing: 12
                padding: Inset{top: 4. bottom: 4. left: 16. right: 16.}
                align: Align{y: 0.5}
                cover := Label{
                    text: "📦"
                    draw_text.text_style.font_size: 56.
                }
                View{
                    width: Fill height: Fit
                    flow: Down spacing: 4
                    title := Label{
                        text: "—"
                        draw_text.color: TEXT_PRIMARY
                        draw_text.text_style: theme.font_bold{font_size: 18.}
                    }
                    subtitle := Label{
                        text: "—"
                        draw_text.color: TEXT_SECONDARY
                        draw_text.text_style.font_size: 12.
                    }
                    price_label := Label{
                        text: ""
                        draw_text.color: ACCENT
                        draw_text.text_style: theme.font_bold{font_size: 16.}
                    }
                }
            }

            row_a := DetailRow{}
            row_b := DetailRow{}
            row_c := DetailRow{}
            row_d := DetailRow{}

            View{
                width: Fill height: Fit
                margin: Inset{top: 8. bottom: 0. left: 16. right: 16.}
                flow: Down spacing: 4
                Label{
                    text: "简介"
                    draw_text.color: TEXT_MUTED
                    draw_text.text_style.font_size: 12.
                }
                intro := Label{
                    width: Fill height: Fit
                    text: "—"
                    draw_text.color: TEXT_PRIMARY
                    draw_text.text_style.font_size: 13.
                }
            }
        }

        SolidView{
            width: Fill height: Fit
            padding: Inset{top: 8. bottom: 8. left: 12. right: 12.}
            flow: Right spacing: 8
            align: Align{y: 0.5}
            draw_bg.color: CARD_BG
            primary_btn := Button{
                text: "联系"
                width: Fill height: Fit
            }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct DetailOverlay {
    #[deref]
    view: View,
}

impl Widget for DetailOverlay {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let target = ACTIVE_DETAIL.read().unwrap().clone();
        let visible = target.is_some();
        self.view.set_visible(cx, visible);
        if let Some(t) = target {
            self.populate(cx, &t);
        }
        self.view.draw_walk(cx, scope, walk)
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl DetailOverlay {
    fn populate(&mut self, cx: &mut Cx2d, target: &DetailTarget) {
        match target {
            DetailTarget::Goods(id) => {
                self.view.label(cx, ids!(detail_kind)).set_text(cx, "商品详情");
                if let Some(g) = MockApi.goods_by_id(id) {
                    let cover = if g.cover_emoji.is_empty() {
                        "📦"
                    } else {
                        g.cover_emoji.as_str()
                    };
                    self.view.label(cx, ids!(cover)).set_text(cx, cover);
                    self.view.label(cx, ids!(title)).set_text(cx, &g.title);
                    self.view
                        .label(cx, ids!(subtitle))
                        .set_text(cx, &g.merchant_name);
                    let price_text = if g.has_discount() {
                        format!("¥{:.0}  原价 ¥{:.0}", g.price, g.original_price)
                    } else {
                        format!("¥{:.0}", g.price)
                    };
                    self.view.label(cx, ids!(price_label)).set_text(cx, &price_text);

                    let row_a = self.view.view(cx, ids!(row_a));
                    row_a.label(cx, ids!(row_label)).set_text(cx, "库存");
                    row_a
                        .label(cx, ids!(row_value))
                        .set_text(cx, g.stock_state.label());
                    let row_b = self.view.view(cx, ids!(row_b));
                    row_b.label(cx, ids!(row_label)).set_text(cx, "距离");
                    row_b
                        .label(cx, ids!(row_value))
                        .set_text(cx, &format!("{} 米", g.distance_m));
                    let row_c = self.view.view(cx, ids!(row_c));
                    row_c.label(cx, ids!(row_label)).set_text(cx, "配送");
                    row_c
                        .label(cx, ids!(row_value))
                        .set_text(cx, if g.deliverable { "可配送" } else { "仅自提" });
                    let row_d = self.view.view(cx, ids!(row_d));
                    row_d.label(cx, ids!(row_label)).set_text(cx, "标签");
                    row_d
                        .label(cx, ids!(row_value))
                        .set_text(cx, &g.tags.join(" · "));

                    self.view.label(cx, ids!(intro)).set_text(cx, &g.intro);
                    self.view
                        .button(cx, ids!(primary_btn))
                        .set_text(cx, "联系商家");
                }
            }
            DetailTarget::Merchant(id) => {
                self.view.label(cx, ids!(detail_kind)).set_text(cx, "店铺详情");
                if let Some(m) = MockApi.merchant_by_id(id) {
                    let cover = if m.cover_emoji.is_empty() {
                        "🏪"
                    } else {
                        m.cover_emoji.as_str()
                    };
                    self.view.label(cx, ids!(cover)).set_text(cx, cover);
                    self.view.label(cx, ids!(title)).set_text(cx, &m.name);
                    self.view
                        .label(cx, ids!(subtitle))
                        .set_text(cx, &format!("{} · ⭐ {:.1}", m.category, m.rating));
                    self.view.label(cx, ids!(price_label)).set_text(cx, "");

                    let row_a = self.view.view(cx, ids!(row_a));
                    row_a.label(cx, ids!(row_label)).set_text(cx, "营业");
                    row_a
                        .label(cx, ids!(row_value))
                        .set_text(cx, if m.open_now { "营业中" } else { "已打烊" });
                    let row_b = self.view.view(cx, ids!(row_b));
                    row_b.label(cx, ids!(row_label)).set_text(cx, "时间");
                    row_b.label(cx, ids!(row_value)).set_text(cx, &m.hours);
                    let row_c = self.view.view(cx, ids!(row_c));
                    row_c.label(cx, ids!(row_label)).set_text(cx, "地址");
                    row_c.label(cx, ids!(row_value)).set_text(cx, &m.address);
                    let row_d = self.view.view(cx, ids!(row_d));
                    row_d.label(cx, ids!(row_label)).set_text(cx, "电话");
                    row_d.label(cx, ids!(row_value)).set_text(cx, &m.phone);

                    self.view.label(cx, ids!(intro)).set_text(cx, &m.intro);
                    self.view
                        .button(cx, ids!(primary_btn))
                        .set_text(cx, "联系店铺");
                }
            }
        }
    }

    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) -> bool {
        let mut closed = false;
        if self
            .view
            .label(cx, ids!(detail_back))
            .as_view()
            .finger_up(actions)
            .is_some()
        {
            *ACTIVE_DETAIL.write().unwrap() = None;
            closed = true;
        }
        if closed {
            self.view.redraw(cx);
        }
        closed
    }

    pub fn is_open() -> bool {
        ACTIVE_DETAIL.read().unwrap().is_some()
    }
}
