use makepad_widgets::*;

use super::state::{ACTIVE_DETAIL, DISCOVER, DetailTarget, FEED};

script_mod! {
    use mod.prelude.widgets.*

    let CARD_BG = #xffffff
    let CARD_BORDER = #xeeeeee
    let TEXT_PRIMARY = #x222222
    let TEXT_SECONDARY = #x6b6b6b
    let TEXT_MUTED = #x9a9a9a
    let ACCENT = #xff7a45
    let PAGE_BG = #xfafaf7

    let SearchBar = SolidView{
        width: Fill height: Fit
        padding: Inset{top: 12. bottom: 8. left: 12. right: 12.}
        flow: Right spacing: 8
        align: Align{y: 0.5}
        draw_bg.color: PAGE_BG

        RoundedView{
            width: Fill height: Fit
            padding: Inset{top: 6. bottom: 6. left: 12. right: 12.}
            show_bg: true
            draw_bg.color: CARD_BG
            draw_bg.border_radius: 18.
            draw_bg.border_size: 1.
            draw_bg.border_color: CARD_BORDER
            flow: Right spacing: 6
            align: Align{y: 0.5}

            Label{
                text: "🔍"
                draw_text.text_style.font_size: 12.
            }
            search_input := TextInput{
                width: Fill height: Fit
                empty_text: "搜索附近商品 · 服务 · 店铺"
                return_key_type: Search
            }
        }
    }

    let FeedCard = RoundedView{
        width: Fill height: Fit
        margin: Inset{top: 0. bottom: 8. left: 12. right: 12.}
        padding: Inset{top: 12. bottom: 12. left: 12. right: 12.}
        show_bg: true
        draw_bg.color: CARD_BG
        draw_bg.border_radius: 12.
        draw_bg.border_size: 1.
        draw_bg.border_color: CARD_BORDER
        flow: Down spacing: 6

        View{
            width: Fill height: Fit
            flow: Right spacing: 8
            align: Align{y: 0.5}
            avatar := Label{
                text: "🦊"
                draw_text.text_style.font_size: 22.
            }
            View{
                width: Fill height: Fit
                flow: Down spacing: 2
                author := Label{
                    text: "—"
                    draw_text.color: TEXT_PRIMARY
                    draw_text.text_style: theme.font_bold{font_size: 13.}
                }
                meta := Label{
                    text: "—"
                    draw_text.color: TEXT_MUTED
                    draw_text.text_style.font_size: 11.
                }
            }
            spread_pill := RoundedView{
                width: Fit height: Fit
                visible: false
                padding: Inset{top: 1. bottom: 1. left: 6. right: 6.}
                show_bg: true
                draw_bg.color: #xfff1ec
                draw_bg.border_radius: 8.
                spread_label := Label{
                    text: "—"
                    draw_text.color: ACCENT
                    draw_text.text_style.font_size: 10.
                }
            }
        }
        topic_label := Label{
            text: "#话题"
            draw_text.color: ACCENT
            draw_text.text_style.font_size: 11.
        }
        body := Label{
            width: Fill height: Fit
            text: "—"
            draw_text.color: TEXT_PRIMARY
            draw_text.text_style.font_size: 13.
        }
        View{
            width: Fill height: Fit
            flow: Right spacing: 16
            likes := Label{
                text: "♡ 0"
                draw_text.color: TEXT_MUTED
                draw_text.text_style.font_size: 11.
            }
            comments := Label{
                text: "💬 0"
                draw_text.color: TEXT_MUTED
                draw_text.text_style.font_size: 11.
            }
        }
    }

    let GoodsResultCard = RoundedView{
        width: Fill height: Fit
        margin: Inset{top: 0. bottom: 8. left: 12. right: 12.}
        padding: Inset{top: 10. bottom: 10. left: 12. right: 12.}
        show_bg: true
        draw_bg.color: CARD_BG
        draw_bg.border_radius: 12.
        draw_bg.border_size: 1.
        draw_bg.border_color: CARD_BORDER
        flow: Right spacing: 10
        align: Align{y: 0.5}
        cursor: MouseCursor.Hand

        cover := Label{
            text: "📦"
            draw_text.text_style.font_size: 28.
        }
        View{
            width: Fill height: Fit
            flow: Down spacing: 2
            title := Label{
                text: "—"
                draw_text.color: TEXT_PRIMARY
                draw_text.text_style: theme.font_bold{font_size: 13.}
            }
            shop := Label{
                text: "—"
                draw_text.color: TEXT_SECONDARY
                draw_text.text_style.font_size: 11.
            }
            price := Label{
                text: "¥0"
                draw_text.color: ACCENT
                draw_text.text_style: theme.font_bold{font_size: 13.}
            }
        }
    }

    let MerchantResultCard = RoundedView{
        width: Fill height: Fit
        margin: Inset{top: 0. bottom: 8. left: 12. right: 12.}
        padding: Inset{top: 10. bottom: 10. left: 12. right: 12.}
        show_bg: true
        draw_bg.color: CARD_BG
        draw_bg.border_radius: 12.
        draw_bg.border_size: 1.
        draw_bg.border_color: CARD_BORDER
        flow: Right spacing: 10
        align: Align{y: 0.5}
        cursor: MouseCursor.Hand

        cover := Label{
            text: "🏪"
            draw_text.text_style.font_size: 28.
        }
        View{
            width: Fill height: Fit
            flow: Down spacing: 2
            title := Label{
                text: "—"
                draw_text.color: TEXT_PRIMARY
                draw_text.text_style: theme.font_bold{font_size: 13.}
            }
            subtitle := Label{
                text: "—"
                draw_text.color: TEXT_SECONDARY
                draw_text.text_style.font_size: 11.
            }
        }
        rating := Label{
            text: "⭐ 0.0"
            draw_text.color: TEXT_MUTED
            draw_text.text_style.font_size: 11.
        }
    }

    let ServiceResultCard = RoundedView{
        width: Fill height: Fit
        margin: Inset{top: 0. bottom: 8. left: 12. right: 12.}
        padding: Inset{top: 10. bottom: 10. left: 12. right: 12.}
        show_bg: true
        draw_bg.color: CARD_BG
        draw_bg.border_radius: 12.
        draw_bg.border_size: 1.
        draw_bg.border_color: CARD_BORDER
        flow: Right spacing: 10
        align: Align{y: 0.5}

        cover := Label{
            text: "🧰"
            draw_text.text_style.font_size: 28.
        }
        View{
            width: Fill height: Fit
            flow: Down spacing: 2
            title := Label{
                text: "—"
                draw_text.color: TEXT_PRIMARY
                draw_text.text_style: theme.font_bold{font_size: 13.}
            }
            provider := Label{
                text: "—"
                draw_text.color: TEXT_SECONDARY
                draw_text.text_style.font_size: 11.
            }
            mode := Label{
                text: "—"
                draw_text.color: TEXT_MUTED
                draw_text.text_style.font_size: 11.
            }
        }
        price := Label{
            text: "起 ¥0"
            draw_text.color: ACCENT
            draw_text.text_style: theme.font_bold{font_size: 13.}
        }
    }

    let SectionLabel = Label{
        margin: Inset{top: 8. bottom: 6. left: 16. right: 16.}
        text: "—"
        draw_text.color: TEXT_PRIMARY
        draw_text.text_style: theme.font_bold{font_size: 14.}
    }

    mod.widgets.DiscoverPageBase = #(DiscoverPage::register_widget(vm))
    mod.widgets.DiscoverPage = set_type_default() do mod.widgets.DiscoverPageBase{
        width: Fill height: Fill
        flow: Down
        show_bg: true
        draw_bg.color: PAGE_BG

        SearchBar{}

        list := PortalList{
            width: Fill height: Fill
            flow: Down

            FeedHeader := SectionLabel{ text: "邻里动态" }
            FeedItem := FeedCard{}
            MerchantsHeader := SectionLabel{ text: "店铺" }
            MerchantItem := MerchantResultCard{}
            GoodsHeader := SectionLabel{ text: "商品" }
            GoodsItem := GoodsResultCard{}
            ServicesHeader := SectionLabel{ text: "服务" }
            ServiceItem := ServiceResultCard{}
            BottomSpacer := View{ width: Fill height: 24 }
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct DiscoverPage {
    #[deref]
    view: View,
}

#[derive(Clone, Copy)]
enum SectionKind {
    Feed,
    Merchants,
    Goods,
    Services,
}

impl Widget for DiscoverPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        while let Some(step) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = step.as_portal_list().borrow_mut() {
                let state = DISCOVER.read().unwrap();
                let feed = &*FEED;
                let m = &state.results.merchants;
                let g = &state.results.goods;
                let s = &state.results.services;

                let show_feed = state.query.is_empty();
                let mut sections: Vec<(usize, SectionKind)> = Vec::new();
                if show_feed && !feed.is_empty() {
                    sections.push((feed.len(), SectionKind::Feed));
                }
                if !m.is_empty() {
                    sections.push((m.len(), SectionKind::Merchants));
                }
                if !g.is_empty() {
                    sections.push((g.len(), SectionKind::Goods));
                }
                if !s.is_empty() {
                    sections.push((s.len(), SectionKind::Services));
                }

                let mut total = 0usize;
                for (n, _) in &sections {
                    total += 1 + n;
                }
                total += 1;
                list.set_item_range(cx, 0, total);

                while let Some(item_id) = list.next_visible_item(cx) {
                    let mut cursor = 0usize;
                    let mut handled = false;
                    for (count, kind) in &sections {
                        if item_id == cursor {
                            let header_id = match kind {
                                SectionKind::Feed => id!(FeedHeader),
                                SectionKind::Merchants => id!(MerchantsHeader),
                                SectionKind::Goods => id!(GoodsHeader),
                                SectionKind::Services => id!(ServicesHeader),
                            };
                            let item = list.item(cx, item_id, header_id);
                            item.draw_all_unscoped(cx);
                            handled = true;
                            break;
                        }
                        let body_start = cursor + 1;
                        let body_end = body_start + count;
                        if item_id >= body_start && item_id < body_end {
                            let idx = item_id - body_start;
                            match kind {
                                SectionKind::Feed => {
                                    let p = &feed[idx];
                                    let item = list.item(cx, item_id, id!(FeedItem));
                                    let avatar = if p.author_emoji.is_empty() {
                                        "🙂"
                                    } else {
                                        p.author_emoji.as_str()
                                    };
                                    item.label(cx, ids!(avatar)).set_text(cx, avatar);
                                    item.label(cx, ids!(author)).set_text(cx, &p.author);
                                    item.label(cx, ids!(meta)).set_text(
                                        cx,
                                        &format!(
                                            "{} · {}m · {}",
                                            p.city, p.distance_m, p.posted_at
                                        ),
                                    );
                                    item.label(cx, ids!(topic_label))
                                        .set_text(cx, &format!("#{}", p.topic));
                                    item.label(cx, ids!(body)).set_text(cx, &p.text);
                                    item.label(cx, ids!(likes))
                                        .set_text(cx, &format!("♡ {}", p.likes));
                                    item.label(cx, ids!(comments))
                                        .set_text(cx, &format!("💬 {}", p.comments));
                                    let layer = p.spread_layer;
                                    item.view(cx, ids!(spread_pill))
                                        .set_visible(cx, layer > 0);
                                    if layer > 0 {
                                        item.label(cx, ids!(spread_label))
                                            .set_text(cx, &format!("跨 {} 层扩散", layer));
                                    }
                                    item.draw_all_unscoped(cx);
                                }
                                SectionKind::Merchants => {
                                    let mr = &m[idx];
                                    let item = list.item(cx, item_id, id!(MerchantItem));
                                    let cover = if mr.cover_emoji.is_empty() {
                                        "🏪"
                                    } else {
                                        mr.cover_emoji.as_str()
                                    };
                                    item.label(cx, ids!(cover)).set_text(cx, cover);
                                    item.label(cx, ids!(title)).set_text(cx, &mr.name);
                                    item.label(cx, ids!(subtitle)).set_text(
                                        cx,
                                        &format!("{} · {}m", mr.category, mr.distance_m),
                                    );
                                    item.label(cx, ids!(rating))
                                        .set_text(cx, &format!("⭐ {:.1}", mr.rating));
                                    item.draw_all_unscoped(cx);
                                }
                                SectionKind::Goods => {
                                    let gd = &g[idx];
                                    let item = list.item(cx, item_id, id!(GoodsItem));
                                    let cover = if gd.cover_emoji.is_empty() {
                                        "📦"
                                    } else {
                                        gd.cover_emoji.as_str()
                                    };
                                    item.label(cx, ids!(cover)).set_text(cx, cover);
                                    item.label(cx, ids!(title)).set_text(cx, &gd.title);
                                    item.label(cx, ids!(shop))
                                        .set_text(cx, &gd.merchant_name);
                                    item.label(cx, ids!(price))
                                        .set_text(cx, &format!("¥{:.0}", gd.price));
                                    item.draw_all_unscoped(cx);
                                }
                                SectionKind::Services => {
                                    let sv = &s[idx];
                                    let item = list.item(cx, item_id, id!(ServiceItem));
                                    let cover = if sv.cover_emoji.is_empty() {
                                        "🧰"
                                    } else {
                                        sv.cover_emoji.as_str()
                                    };
                                    item.label(cx, ids!(cover)).set_text(cx, cover);
                                    item.label(cx, ids!(title)).set_text(cx, &sv.title);
                                    item.label(cx, ids!(provider)).set_text(
                                        cx,
                                        &format!("{} · ⭐ {:.1}", sv.provider_name, sv.rating),
                                    );
                                    item.label(cx, ids!(mode)).set_text(
                                        cx,
                                        &format!(
                                            "{} · {}m",
                                            sv.service_mode, sv.distance_m
                                        ),
                                    );
                                    item.label(cx, ids!(price))
                                        .set_text(cx, &format!("起 ¥{:.0}", sv.price_from));
                                    item.draw_all_unscoped(cx);
                                }
                            }
                            handled = true;
                            break;
                        }
                        cursor = body_end;
                    }
                    if !handled {
                        let item = list.item(cx, item_id, id!(BottomSpacer));
                        item.draw_all_unscoped(cx);
                    }
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl DiscoverPage {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) -> bool {
        let mut opened = false;
        if let Some(text) = self.view.text_input(cx, ids!(search_input)).changed(actions) {
            let mut state = DISCOVER.write().unwrap();
            state.query = text;
            state.refresh();
            drop(state);
            self.view.redraw(cx);
        }

        let list = self.view.portal_list(cx, ids!(list));
        let state = DISCOVER.read().unwrap();
        let show_feed = state.query.is_empty();
        let feed_n = if show_feed { FEED.len() } else { 0 };
        let m_n = state.results.merchants.len();
        let g_n = state.results.goods.len();

        let mut cursor = 0usize;
        let (m_start, m_end);
        let (g_start, g_end);
        if feed_n > 0 {
            cursor += 1 + feed_n;
        }
        if m_n > 0 {
            m_start = cursor + 1;
            m_end = m_start + m_n;
            cursor = m_end;
        } else {
            m_start = usize::MAX;
            m_end = 0;
        }
        if g_n > 0 {
            g_start = cursor + 1;
            g_end = g_start + g_n;
        } else {
            g_start = usize::MAX;
            g_end = 0;
        }

        for (item_id, item) in list.items_with_actions(actions) {
            if item.as_view().finger_up(actions).is_some() {
                if item_id >= m_start && item_id < m_end {
                    let id = state.results.merchants[item_id - m_start].id.clone();
                    *ACTIVE_DETAIL.write().unwrap() = Some(DetailTarget::Merchant(id));
                    opened = true;
                } else if item_id >= g_start && item_id < g_end {
                    let id = state.results.goods[item_id - g_start].id.clone();
                    *ACTIVE_DETAIL.write().unwrap() = Some(DetailTarget::Goods(id));
                    opened = true;
                }
            }
        }
        opened
    }
}
