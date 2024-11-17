use crate::assets::game_page_script::SCRIPT;
use crate::assets::game_page_style::STYLE;
use crate::html::{body, div, el, head, html, script, style};

pub fn game_page() -> String {
    html()
        .children(vec![
            head().children(vec![el("title").text("👑 The Cave 🐉"), style(STYLE)]),
            body().children(vec![
                div()
                    .attr("id", "content")
                    .class("container")
                    .text("{{ content }}"),
                script(SCRIPT),
            ]),
        ])
        .render()
}

pub fn display_controls() -> String {
    div()
        .class("controls")
        .children(vec![
            el("button")
                .attr("id", "left")
                .attr("onclick", "handleKeyDown('ArrowLeft')")
                .text("👈"),
            el("button")
                .attr("id", "down")
                .attr("onclick", "handleKeyDown('ArrowDown')")
                .text("👇"),
            el("button")
                .attr("id", "up")
                .attr("onclick", "handleKeyDown('ArrowUp')")
                .text("👆"),
            el("button")
                .attr("id", "right")
                .attr("onclick", "handleKeyDown('ArrowRight')")
                .text("👉"),
        ])
        .render()
}
