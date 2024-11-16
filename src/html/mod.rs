pub struct El {
    tag: String,
    text: String,
    children: Vec<El>,
    depth: usize,
    attrs: Vec<(String, String)>,
    style: Vec<(String, String)>,
}

impl El {
    pub fn text(mut self, text: &str) -> Self {
        self.text = text.to_string();
        self
    }

    pub fn class(mut self, class: &str) -> Self {
        self.attrs.push(("class".to_string(), class.to_string()));
        self
    }

    pub fn child(mut self, child: El) -> Self {
        self.children.push(El {
            depth: self.depth + 1,
            ..child
        });
        self
    }

    pub fn children(mut self, children: Vec<El>) -> Self {
        self.children = self.children.into_iter().chain(children).collect();
        self
    }

    pub fn attr(mut self, key: &str, value: &str) -> Self {
        self.attrs.push((key.to_string(), value.to_string()));
        self
    }

    pub fn style(mut self, key: &str, value: &str) -> Self {
        self.style.push((key.to_string(), value.to_string()));
        self
    }

    pub fn render(&self) -> String {
        let mut res = [
            self.indent(),
            "<".to_string(),
            self.tag.clone(),
            self.render_attrs(),
            self.render_style(),
            ">".to_string(),
        ]
        .concat();

        let optional_indent = if self.children.len() > 0 {
            &self.indent()
        } else {
            ""
        };

        if self.text.len() > 0 {
            res.push_str(&self.text);
        }

        if self.children.len() > 0 {
            res = [
                res,
                "\n".to_string(),
                self.render_children(),
                "\n".to_string(),
                optional_indent.to_string(),
            ].concat();

        }

        res = [
            res,
            "</".to_string(),
            self.tag.clone(),
            ">".to_string(),
        ].concat();

        res
    }

    fn indent(&self) -> String {
        "  ".repeat(self.depth)
    }

    fn render_attrs(&self) -> String {
        let mut res = "".to_string();
        for (k, v) in &self.attrs {
            res = [
                res,
                " ".to_string(),
                k.clone(),
                "=\"".to_string(),
                v.clone(),
                "\"".to_string(),
            ].concat();
        }
        res
    }

    fn render_style(&self) -> String {
        let mut res = "".to_string();
        for (k, v) in &self.style {
            res = [
                res,
                k.clone(),
                ":".to_string(),
                v.clone(),
                ";".to_string(),
            ].concat();

        }
        if res.len() > 0 {
            res = [" style=\"".to_string(), res, "\"".to_string()].concat();
        }
        res
    }

    fn render_children(&self) -> String {
        let mut res = "".to_string();
        for c in &self.children {
            res = [
                res,
                "\n".to_string(),
                c.render(),
                self.indent().to_string(),
            ].concat();
        }
        res
    }
}

pub fn el(tag: &str) -> El {
    El {
        tag: tag.to_string(),
        text: "".to_string(),
        children: vec![],
        depth: 0,
        attrs: vec![],
        style: vec![],
    }
}

pub fn html() -> El {
    el("html")
}

pub fn head() -> El {
    el("head")
}

pub fn body() -> El {
    el("body")
}

pub fn div() -> El {
    el("div")
}

pub fn span(text: &str) -> El {
    el("span").text(text)
}

pub fn script(script: &str) -> El {
    el("script").text(script)
}

pub fn style(style: &str) -> El {
    el("style").text(style)
}
