impl WorkspaceApp {
    pub(in crate::workspace) fn render_ai_sidebar_disabled(
        &self,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        div()
            .w_full()
            .min_w_0()
            .px(px(self.tokens.spacing.four))
            .h_full()
            .flex()
            .flex_col()
            .items_center()
            .justify_center()
            .gap(px(self.tokens.spacing.two_half))
            .text_center()
            .child(
                div()
                    .size(px(48.0))
                    .flex()
                    .items_center()
                    .justify_center()
                    .bg(rgba((self.tokens.ui.accent << 8) | 0x0d))
                    .child(Self::render_lucide_icon(
                        LucideIcon::MessageSquare,
                        24.0,
                        rgba((self.tokens.ui.text_muted << 8) | 0x66),
                    )),
            )
            .child(
                div()
                    .text_size(px(13.0))
                    .font_weight(gpui::FontWeight::BOLD)
                    .text_color(rgb(self.tokens.ui.text))
                    .child(self.render_display_text_with_role(
                        SelectableTextRole::PlainDocument,
                        "ai-chat-disabled",
                        "title",
                        self.i18n.t("ai.chat.title"),
                        self.tokens.ui.text,
                        cx,
                    )),
            )
            .child(
                div()
                    .max_w(px(220.0))
                    .text_size(px(self.tokens.metrics.ui_text_sm))
                    .text_color(rgb(self.tokens.ui.text_muted))
                    .line_height(px(18.0))
                    .child(self.render_display_text_with_role(
                        SelectableTextRole::PlainDocument,
                        "ai-chat-disabled",
                        "message",
                        self.i18n.t("ai.chat.disabled_message"),
                        self.tokens.ui.text_muted,
                        cx,
                    )),
            )
            .child(
                div()
                    .mt(px(self.tokens.spacing.one))
                    .rounded(px(self.tokens.radii.md))
                    .px(px(self.tokens.spacing.two_half))
                    .py(px(self.tokens.spacing.one_half))
                    .bg(rgb(self.tokens.ui.accent))
                    .text_color(rgb(self.tokens.ui.bg))
                    .text_size(px(11.0))
                    .font_weight(gpui::FontWeight::MEDIUM)
                    .cursor_pointer()
                    .child(self.render_display_text_with_role(
                        SelectableTextRole::NonSelectable,
                        "ai-chat-disabled",
                        "open-settings",
                        self.i18n.t("ai.chat.open_settings"),
                        self.tokens.ui.bg,
                        cx,
                    ))
                    .on_mouse_down(
                        MouseButton::Left,
                        cx.listener(|this, _event, window, cx| {
                            this.open_ai_settings(window, cx);
                            cx.stop_propagation();
                        }),
                    ),
            )
            .into_any_element()
    }
}
