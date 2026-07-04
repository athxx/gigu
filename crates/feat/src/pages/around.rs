use makepad_widgets::*;

use super::state::{ACTIVE_DETAIL, DetailTarget, GOODS, ME, MERCHANTS};

script_mod! {
    use mod.prelude.widgets.*

    let CARD_BG = #xffffff
    let CARD_BORDER = #xeeeeee
    let TEXT_PRIMARY = #x222222
    let TEXT_SECONDARY = #x6b6b6b
    let TEXT_MUTED = #x9a9a9a
    let ACCENT = #xff7a45
    let PAGE_BG = #xfafaf7

    let LocationBar = SolidView{
        width: Fill height: Fit
        padding: Inset{top: 14. bottom: 12. left: 16. right: 16.}
        flow: Right spacing: 8
        align: Align{y: 0.5}
        draw_bg.color: PAGE_BG

        Label{
            text: "📍"
            draw_text.text_style.font_size: 16.
        }
        View{
            width: Fill height: Fit
            flow: Down spacing: 2
            location_label := Label{
                text: "上海 · 黄浦区"
                draw_text.color: TEXT_PRIMARY
                draw_text.text_style: theme.font_bold{font_size: 14.}
            }
            greeting_label := Label{
                text: "泰 สวัสดี / 天नमस्ते / 阿مرحبا / 韩안녕하세요/ 日こんにちは"
                draw_text.color: TEXT_MUTED
                draw_text.text_style.font_size: 11.
            }
        }
    }

    let SectionHeader = View{
        width: Fill height: Fit
        padding: Inset{top: 14. bottom: 8. left: 16. right: 16.}
        flow: Right
        align: Align{y: 0.5}
        section_title := Label{
            text: "附近"
            draw_text.color: TEXT_PRIMARY
            draw_text.text_style: theme.font_bold{font_size: 15.}
        }
        Filler{}
        section_more := Label{
            text: "更多 ›"
            draw_text.color: TEXT_MUTED
            draw_text.text_style.font_size: 11.
        }
    }

    let MerchantCard = RoundedView{
        width: Fill height: Fit
        margin: Inset{top: 0. bottom: 8. left: 12. right: 12.}
        padding: Inset{top: 12. bottom: 12. left: 12. right: 12.}
        show_bg: true
        draw_bg.color: CARD_BG
        draw_bg.border_radius: 12.
        draw_bg.border_size: 1.
        draw_bg.border_color: CARD_BORDER
        flow: Right spacing: 12
        align: Align{y: 0.5}
        cursor: MouseCursor.Hand

        cover := Label{
            text: "🍜"
            draw_text.text_style.font_size: 32.
        }
        View{
            width: Fill height: Fit
            flow: Down spacing: 4
            View{
                width: Fill height: Fit
                flow: Right spacing: 6
                align: Align{y: 0.5}
                title := Label{
                    text: "—"
                    draw_text.color: TEXT_PRIMARY
                    draw_text.text_style: theme.font_bold{font_size: 14.}
                }
                verified_badge := RoundedView{
                    width: Fit height: Fit
                    visible: false
                    padding: Inset{top: 1. bottom: 1. left: 4. right: 4.}
                    show_bg: true
                    draw_bg.color: #xfff1ec
                    draw_bg.border_radius: 4.
                    Label{
                        text: "认证"
                        draw_text.color: ACCENT
                        draw_text.text_style.font_size: 9.
                    }
                }
            }
            subtitle := Label{
                text: "—"
                draw_text.color: TEXT_SECONDARY
                draw_text.text_style.font_size: 11.
            }
            meta := Label{
                text: "—"
                draw_text.color: TEXT_MUTED
                draw_text.text_style.font_size: 11.
            }
        }
        View{
            width: Fit height: Fit
            flow: Down spacing: 4
            align: Align{x: 1.0}
            distance := Label{
                text: "—"
                draw_text.color: TEXT_MUTED
                draw_text.text_style.font_size: 11.
            }
            open_badge := Label{
                text: "营业中"
                draw_text.color: #x2da44e
                draw_text.text_style.font_size: 11.
            }
        }
    }

    let GoodsCard = RoundedView{
        width: Fill height: Fit
        margin: Inset{top: 0. bottom: 8. left: 12. right: 12.}
        padding: Inset{top: 12. bottom: 12. left: 12. right: 12.}
        show_bg: true
        draw_bg.color: CARD_BG
        draw_bg.border_radius: 12.
        draw_bg.border_size: 1.
        draw_bg.border_color: CARD_BORDER
        flow: Right spacing: 12
        align: Align{y: 0.5}
        cursor: MouseCursor.Hand

        cover := Label{
            text: "📦"
            draw_text.text_style.font_size: 32.
        }
        View{
            width: Fill height: Fit
            flow: Down spacing: 4
            title := Label{
                text: "—"
                draw_text.color: TEXT_PRIMARY
                draw_text.text_style: theme.font_bold{font_size: 14.}
            }
            shop := Label{
                text: "—"
                draw_text.color: TEXT_SECONDARY
                draw_text.text_style.font_size: 11.
            }
            price_row := View{
                width: Fill height: Fit
                flow: Right spacing: 6
                align: Align{y: 0.5}
                price := Label{
                    text: "¥0"
                    draw_text.color: ACCENT
                    draw_text.text_style: theme.font_bold{font_size: 14.}
                }
                original_price := Label{
                    text: ""
                    visible: false
                    draw_text.color: TEXT_MUTED
                    draw_text.text_style.font_size: 11.
                }
                stock := Label{
                    text: ""
                    draw_text.color: TEXT_MUTED
                    draw_text.text_style.font_size: 11.
                }
            }
        }
        distance := Label{
            text: "—"
            draw_text.color: TEXT_MUTED
            draw_text.text_style.font_size: 11.
        }
    }

    let SectionLabel = Label{
        margin: Inset{top: 8. bottom: 6. left: 16. right: 16.}
        text: "—"
        draw_text.color: TEXT_PRIMARY
        draw_text.text_style: theme.font_bold{font_size: 15.}
    }

    mod.widgets.AroundPageBase = #(AroundPage::register_widget(vm))
    mod.widgets.AroundPage = set_type_default() do mod.widgets.AroundPageBase{
        width: Fill height: Fill
        flow: Down
        show_bg: true
        draw_bg.color: PAGE_BG

        list := PortalList{
            width: Fill height: Fill
            flow: Down

            Header := LocationBar{}
            MerchantsHeader := SectionLabel{ text: "附近店铺" }
            Merchant := MerchantCard{}
            GoodsHeader := SectionLabel{ text: "热门商品" }
            Goods := GoodsCard{}
            BottomSpacer := View{ width: Fill height: 24 }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct AroundPage {
    #[deref]
    view: View,
}

impl Widget for AroundPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(step) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = step.as_portal_list().borrow_mut() {
                let merchants = &*MERCHANTS;
                let goods = &*GOODS;
                let header_count = 3usize;
                let total = header_count + merchants.len() + goods.len() + 1;
                list.set_item_range(cx, 0, total);

                while let Some(item_id) = list.next_visible_item(cx) {
                    if item_id == 0 {
                        let item = list.item(cx, item_id, id!(Header));
                        let me = ME.read().unwrap();
                        if !me.city.is_empty() {
                            item.label(cx, ids!(location_label))
                                .set_text(cx, &format!("{} · 附近", me.city));
                        }
                        item.draw_all_unscoped(cx);
                        continue;
                    }
                    if item_id == 1 {
                        let item = list.item(cx, item_id, id!(MerchantsHeader));
                        item.draw_all_unscoped(cx);
                        continue;
                    }
                    let merchants_start = 2usize;
                    let merchants_end = merchants_start + merchants.len();
                    if item_id >= merchants_start && item_id < merchants_end {
                        let m = &merchants[item_id - merchants_start];
                        let item = list.item(cx, item_id, id!(Merchant));
                        let cover = if m.cover_emoji.is_empty() {
                            "🏪"
                        } else {
                            m.cover_emoji.as_str()
                        };
                        item.label(cx, ids!(cover)).set_text(cx, cover);
                        item.label(cx, ids!(title)).set_text(cx, &m.name);
                        item.label(cx, ids!(subtitle))
                            .set_text(cx, &format!("{} · ⭐ {:.1}", m.category, m.rating));
                        item.label(cx, ids!(meta)).set_text(cx, &m.hours);
                        item.label(cx, ids!(distance))
                            .set_text(cx, &format_distance(m.distance_m));
                        item.view(cx, ids!(verified_badge))
                            .set_visible(cx, m.verified);
                        item.label(cx, ids!(open_badge))
                            .set_text(cx, if m.open_now { "营业中" } else { "已打烊" });
                        item.draw_all_unscoped(cx);
                        continue;
                    }
                    if item_id == merchants_end {
                        let item = list.item(cx, item_id, id!(GoodsHeader));
                        item.draw_all_unscoped(cx);
                        continue;
                    }
                    let goods_start = merchants_end + 1;
                    let goods_end = goods_start + goods.len();
                    if item_id >= goods_start && item_id < goods_end {
                        let g = &goods[item_id - goods_start];
                        let item = list.item(cx, item_id, id!(Goods));
                        let cover = if g.cover_emoji.is_empty() {
                            "📦"
                        } else {
                            g.cover_emoji.as_str()
                        };
                        item.label(cx, ids!(cover)).set_text(cx, cover);
                        item.label(cx, ids!(title)).set_text(cx, &g.title);
                        item.label(cx, ids!(shop)).set_text(cx, &g.merchant_name);
                        item.label(cx, ids!(price))
                            .set_text(cx, &format!("¥{:.0}", g.price));
                        let has_discount = g.has_discount();
                        let original = item.label(cx, ids!(original_price));
                        if has_discount {
                            original.set_text(cx, &format!("¥{:.0}", g.original_price));
                            item.view(cx, ids!(original_price)).set_visible(cx, true);
                        } else {
                            item.view(cx, ids!(original_price)).set_visible(cx, false);
                        }
                        item.label(cx, ids!(stock))
                            .set_text(cx, &format!("· {}", g.stock_state.label()));
                        item.label(cx, ids!(distance))
                            .set_text(cx, &format_distance(g.distance_m));
                        item.draw_all_unscoped(cx);
                        continue;
                    }
                    let item = list.item(cx, item_id, id!(BottomSpacer));
                    item.draw_all_unscoped(cx);
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl AroundPage {
    pub fn handle_actions(&self, cx: &mut Cx, actions: &Actions) -> bool {
        let mut opened = false;
        let list = self.view.portal_list(cx, ids!(list));
        let merchants = &*MERCHANTS;
        let goods = &*GOODS;
        let merchants_start = 2usize;
        let merchants_end = merchants_start + merchants.len();
        let goods_start = merchants_end + 1;
        let goods_end = goods_start + goods.len();
        for (item_id, item) in list.items_with_actions(actions) {
            if item.as_view().finger_up(actions).is_some() {
                if item_id >= merchants_start && item_id < merchants_end {
                    let id = merchants[item_id - merchants_start].id.clone();
                    *ACTIVE_DETAIL.write().unwrap() = Some(DetailTarget::Merchant(id));
                    opened = true;
                } else if item_id >= goods_start && item_id < goods_end {
                    let id = goods[item_id - goods_start].id.clone();
                    *ACTIVE_DETAIL.write().unwrap() = Some(DetailTarget::Goods(id));
                    opened = true;
                }
            }
        }
        opened
    }
}

fn format_distance(m: u32) -> String {
    if m < 1000 {
        format!("{}m", m)
    } else {
        format!("{:.1}km", m as f32 / 1000.0)
    }
}
