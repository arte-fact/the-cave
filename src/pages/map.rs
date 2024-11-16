use crate::html::{body, div, el, head, html, style, El};

pub fn map_page() -> String {
    html().children(vec![
        head().children(vec![
            el("title")
                .text("👑 A map 🐉"),
            style(".tile { display: inline-block; width: 3px; height: 3px; font-size: 2px; line-height: 6px; text-align: center; }"),
        ]),
        body().children(vec![
            div()
                .attr("id", "content")
                .class("container")
                .text("{{ content }}"),
        ]),
    ]).render()
}

