use makepad_widgets::*;

use super::state::{ACTIVE_CHAT, ActiveChat, CONVERSATIONS};

script_mod! {
    use mod.prelude.widgets.*

    let CARD_BG = #xffffff
    let CARD_BORDER = #xeeeeee
    let TEXT_PRIMARY = #x222222
    let TEXT_SECONDARY = #x6b6b6b
    let TEXT_MUTED = #x9a9a9a
    let ACCENT = #xff7a45
    let BADGE_BG = #xff3b30
    let BADGE_TEXT = #xffffff
    let PAGE_BG = #xfafaf7
    let MINE_BG = #xffeadc
    let PEER_BG = #xeeeeee

    let ChatsHeader = SolidView{
        width: Fill height: Fit
        padding: Inset{top: 14. bottom: 12. left: 16. right: 16.}
        flow: Right
        align: Align{y: 0.5}
        draw_bg.color: PAGE_BG
        Label{
            text: "消息"
            draw_text.color: TEXT_PRIMARY
            draw_text.text_style: theme.font_bold{font_size: 18.}
        }
    }

    let ConversationRow = View{
        width: Fill height: Fit
        padding: Inset{top: 12. bottom: 12. left: 16. right: 16.}
        flow: Right spacing: 12
        align: Align{y: 0.5}
        cursor: MouseCursor.Hand

        avatar := Label{
            text: "🦊"
            draw_text.text_style.font_size: 28.
        }
        View{
            width: Fill height: Fit
            flow: Down spacing: 4
            View{
                width: Fill height: Fit
                flow: Right
                align: Align{y: 0.5}
                name := Label{
                    text: "—"
                    draw_text.color: TEXT_PRIMARY
                    draw_text.text_style: theme.font_bold{font_size: 14.}
                }
                Filler{}
                last_at := Label{
                    text: "—"
                    draw_text.color: TEXT_MUTED
                    draw_text.text_style.font_size: 11.
                }
            }
            View{
                width: Fill height: Fit
                flow: Right
                align: Align{y: 0.5}
                last_text := Label{
                    width: Fill height: Fit
                    text: "—"
                    draw_text.color: TEXT_SECONDARY
                    draw_text.text_style.font_size: 12.
                }
                badge := RoundedView{
                    width: Fit height: Fit
                    visible: false
                    padding: Inset{top: 1. bottom: 1. left: 6. right: 6.}
                    show_bg: true
                    draw_bg.color: BADGE_BG
                    draw_bg.border_radius: 8.
                    badge_label := Label{
                        text: "0"
                        draw_text.color: BADGE_TEXT
                        draw_text.text_style: theme.font_bold{font_size: 10.}
                    }
                }
            }
        }
    }

    let Divider = SolidView{
        width: Fill height: 1
        margin: Inset{left: 76. right: 0.}
        draw_bg.color: CARD_BORDER
    }

    let ChatHeader = SolidView{
        width: Fill height: Fit
        padding: Inset{top: 12. bottom: 12. left: 12. right: 12.}
        flow: Right spacing: 8
        align: Align{y: 0.5}
        draw_bg.color: PAGE_BG
        chat_back := Label{
            text: "‹ 返回"
            draw_text.color: ACCENT
            draw_text.text_style.font_size: 14.
        }
        Filler{}
        chat_title := Label{
            text: "—"
            draw_text.color: TEXT_PRIMARY
            draw_text.text_style: theme.font_bold{font_size: 15.}
        }
        Filler{}
        Label{
            text: "•••"
            draw_text.color: TEXT_MUTED
            draw_text.text_style.font_size: 14.
        }
    }

    let MineBubble = View{
        width: Fill height: Fit
        padding: Inset{top: 4. bottom: 4. left: 60. right: 12.}
        flow: Right
        align: Align{x: 1.0}
        RoundedView{
            width: Fit height: Fit
            padding: Inset{top: 8. bottom: 8. left: 12. right: 12.}
            show_bg: true
            draw_bg.color: MINE_BG
            draw_bg.border_radius: 12.
            bubble_text := Label{
                text: "—"
                draw_text.color: TEXT_PRIMARY
                draw_text.text_style.font_size: 13.
            }
        }
    }

    let PeerBubble = View{
        width: Fill height: Fit
        padding: Inset{top: 4. bottom: 4. left: 12. right: 60.}
        flow: Right
        align: Align{x: 0.}
        RoundedView{
            width: Fit height: Fit
            padding: Inset{top: 8. bottom: 8. left: 12. right: 12.}
            show_bg: true
            draw_bg.color: PEER_BG
            draw_bg.border_radius: 12.
            bubble_text := Label{
                text: "—"
                draw_text.color: TEXT_PRIMARY
                draw_text.text_style.font_size: 13.
            }
        }
    }

    let Composer = SolidView{
        width: Fill height: Fit
        padding: Inset{top: 8. bottom: 8. left: 12. right: 12.}
        flow: Right spacing: 8
        align: Align{y: 0.5}
        draw_bg.color: CARD_BG

        compose_input := TextInput{
            width: Fill height: Fit
            empty_text: "发送消息…"
            return_key_type: Send
        }
        compose_send := Button{
            text: "发送"
            width: Fit height: Fit
        }
    }

    mod.widgets.ChatsPageBase = #(ChatsPage::register_widget(vm))
    mod.widgets.ChatsPage = set_type_default() do mod.widgets.ChatsPageBase{
        width: Fill height: Fill
        flow: Down
        show_bg: true
        draw_bg.color: PAGE_BG

        list_view := View{
            width: Fill height: Fill
            flow: Down
            ChatsHeader{}
            list := PortalList{
                width: Fill height: Fill
                flow: Down
                Conv := ConversationRow{}
                Sep := Divider{}
            }
        }
        chat_view := View{
            width: Fill height: Fill
            visible: false
            flow: Down
            ChatHeader{}
            chat_list := PortalList{
                width: Fill height: Fill
                flow: Down
                Mine := MineBubble{}
                Peer := PeerBubble{}
            }
            Composer{}
        }
    }
}

#[derive(Script, ScriptHook, Widget)]
pub struct ChatsPage {
    #[deref]
    view: View,
}

impl Widget for ChatsPage {
    fn draw_walk(&mut self, cx: &mut Cx2d, scope: &mut Scope, walk: Walk) -> DrawStep {
        let in_chat = ACTIVE_CHAT.read().unwrap().is_some();
        self.view.view(cx, ids!(list_view)).set_visible(cx, !in_chat);
        self.view.view(cx, ids!(chat_view)).set_visible(cx, in_chat);

        while let Some(step) = self.view.draw_walk(cx, scope, walk).step() {
            if let Some(mut list) = step.as_portal_list().borrow_mut() {
                if !in_chat {
                    let convs = CONVERSATIONS.read().unwrap();
                    let n = convs.len();
                    let total = if n == 0 { 0 } else { n * 2 - 1 };
                    list.set_item_range(cx, 0, total);
                    while let Some(item_id) = list.next_visible_item(cx) {
                        if item_id % 2 == 1 {
                            let item = list.item(cx, item_id, id!(Sep));
                            item.draw_all_unscoped(cx);
                            continue;
                        }
                        let idx = item_id / 2;
                        let Some(c) = convs.get(idx) else {
                            continue;
                        };
                        let item = list.item(cx, item_id, id!(Conv));
                        let avatar = if c.peer_emoji.is_empty() {
                            "🙂"
                        } else {
                            c.peer_emoji.as_str()
                        };
                        item.label(cx, ids!(avatar)).set_text(cx, avatar);
                        item.label(cx, ids!(name)).set_text(cx, &c.peer_name);
                        item.label(cx, ids!(last_at)).set_text(cx, &c.last_at);
                        item.label(cx, ids!(last_text)).set_text(cx, &c.last_text);
                        let has_unread = c.unread > 0;
                        item.view(cx, ids!(badge)).set_visible(cx, has_unread);
                        if has_unread {
                            item.label(cx, ids!(badge_label))
                                .set_text(cx, &c.unread.to_string());
                        }
                        item.draw_all_unscoped(cx);
                    }
                } else {
                    let active = ACTIVE_CHAT.read().unwrap();
                    let messages = active
                        .as_ref()
                        .map(|c| c.messages.clone())
                        .unwrap_or_default();
                    list.set_item_range(cx, 0, messages.len());
                    while let Some(item_id) = list.next_visible_item(cx) {
                        let Some(msg) = messages.get(item_id) else {
                            continue;
                        };
                        let template = match msg.from {
                            domain::domain::messaging::MessageFrom::Me => id!(Mine),
                            domain::domain::messaging::MessageFrom::Peer => id!(Peer),
                        };
                        let item = list.item(cx, item_id, template);
                        item.label(cx, ids!(bubble_text)).set_text(cx, &msg.text);
                        item.draw_all_unscoped(cx);
                    }
                }
            }
        }

        if in_chat {
            let active = ACTIVE_CHAT.read().unwrap();
            if let Some(chat) = active.as_ref() {
                let convs = CONVERSATIONS.read().unwrap();
                if let Some(c) = convs.iter().find(|c| c.id == chat.conversation_id) {
                    self.view.label(cx, ids!(chat_title)).set_text(cx, &c.peer_name);
                }
            }
        }
        DrawStep::done()
    }

    fn handle_event(&mut self, cx: &mut Cx, event: &Event, scope: &mut Scope) {
        self.view.handle_event(cx, event, scope);
    }
}

impl ChatsPage {
    pub fn handle_actions(&mut self, cx: &mut Cx, actions: &Actions) -> bool {
        let mut needs_redraw = false;
        let in_chat = ACTIVE_CHAT.read().unwrap().is_some();

        if !in_chat {
            let list = self.view.portal_list(cx, ids!(list));
            let convs = CONVERSATIONS.read().unwrap().clone();
            for (item_id, item) in list.items_with_actions(actions) {
                if item.as_view().finger_up(actions).is_some() && item_id % 2 == 0 {
                    let idx = item_id / 2;
                    if let Some(c) = convs.get(idx) {
                        *ACTIVE_CHAT.write().unwrap() = Some(ActiveChat::open(&c.id));
                        needs_redraw = true;
                    }
                }
            }
        } else {
            if self.view.label(cx, ids!(chat_back)).as_view().finger_up(actions).is_some() {
                *ACTIVE_CHAT.write().unwrap() = None;
                needs_redraw = true;
            }
            let mut send_text: Option<String> = None;
            if self.view.button(cx, ids!(compose_send)).clicked(actions) {
                let t = self.view.text_input(cx, ids!(compose_input)).text();
                send_text = Some(t);
            }
            if let Some((text, _mods)) =
                self.view.text_input(cx, ids!(compose_input)).returned(actions)
            {
                send_text = Some(text);
            }
            if let Some(text) = send_text {
                let trimmed = text.trim().to_string();
                if !trimmed.is_empty() {
                    if let Some(active) = ACTIVE_CHAT.write().unwrap().as_mut() {
                        active.append_me(&trimmed);
                    }
                    self.view.text_input(cx, ids!(compose_input)).set_text(cx, "");
                    needs_redraw = true;
                }
            }
        }
        if needs_redraw {
            self.view.redraw(cx);
        }
        needs_redraw
    }
}
