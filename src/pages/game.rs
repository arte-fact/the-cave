use crate::assets::game_page_script::SCRIPT;
use crate::assets::game_page_style::STYLE;
use crate::html::{body, div, el, head, html, script, style, El};

pub fn game_page() -> String {
    html().children(vec![
        head().children(vec![
            el("title")
                .text("👑 The Cave 🐉"),
            style(STYLE),
        ]),
        body().children(vec![
            div()
                .attr("id", "content")
                .class("container")
                .text("{{ content }}"),
            script(SCRIPT),
        ]),
    ]).render()
}

