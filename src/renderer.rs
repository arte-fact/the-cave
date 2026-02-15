use web_sys::CanvasRenderingContext2d;

use crate::game::Game;

pub struct Renderer {
    ctx: CanvasRenderingContext2d,
    cell_size: f64,
    offset_x: f64,
    offset_y: f64,
    canvas_w: f64,
    canvas_h: f64,
}

impl Renderer {
    pub fn new(ctx: CanvasRenderingContext2d) -> Self {
        Self {
            ctx,
            cell_size: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
            canvas_w: 0.0,
            canvas_h: 0.0,
        }
    }

    pub fn resize(&mut self, width: f64, height: f64, game: &Game) {
        self.canvas_w = width;
        self.canvas_h = height;

        // Fit the grid into the viewport, pick the smaller axis
        let cell_w = width / game.grid_cols as f64;
        let cell_h = height / game.grid_rows as f64;
        self.cell_size = cell_w.min(cell_h).floor();

        // Center the grid
        let grid_w = self.cell_size * game.grid_cols as f64;
        let grid_h = self.cell_size * game.grid_rows as f64;
        self.offset_x = ((width - grid_w) / 2.0).floor();
        self.offset_y = ((height - grid_h) / 2.0).floor();
    }

    pub fn draw(&self, game: &Game) {
        let ctx = &self.ctx;

        // Clear
        ctx.set_fill_style_str("#000");
        ctx.fill_rect(0.0, 0.0, self.canvas_w, self.canvas_h);

        // Draw grid dots
        ctx.set_fill_style_str("#1a1a1a");
        for y in 0..game.grid_rows {
            for x in 0..game.grid_cols {
                let px = self.offset_x + x as f64 * self.cell_size;
                let py = self.offset_y + y as f64 * self.cell_size;
                ctx.fill_rect(px + 1.0, py + 1.0, self.cell_size - 2.0, self.cell_size - 2.0);
            }
        }

        // Draw player @
        let px = self.offset_x + game.player_x as f64 * self.cell_size;
        let py = self.offset_y + game.player_y as f64 * self.cell_size;

        let font_size = (self.cell_size * 0.8).round();
        ctx.set_font(&format!("{font_size}px monospace"));
        ctx.set_fill_style_str("#fff");
        ctx.set_text_align("center");
        ctx.set_text_baseline("middle");
        let _ = ctx.fill_text(
            "@",
            px + self.cell_size / 2.0,
            py + self.cell_size / 2.0,
        );
    }
}
