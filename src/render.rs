//! Draws the scene as an animated SVG: a 96×64 pixel grid scaled up 4×,
//! animated with CSS only, since GitHub strips scripts from READMEs.

use crate::art::{self, Art, FONT_CHARS, GLYPH_H, GLYPH_W, Rgb, Sprite};
use crate::languages;
use crate::model::Book;

const W: usize = 96;
const H: usize = 64;
const SCALE: usize = 4;

const OWL_X: i32 = 32;
const OWL_Y: i32 = 16;
const HEAD_X: i32 = OWL_X + 4;
const BODY_Y: i32 = OWL_Y + 14;
const BOOK_X: i32 = 40;
const BOOK_Y: i32 = 29;
const BOOK_W: usize = 16;
const BOOK_H: usize = 20;
/// Logos are square; the fallback label is centered in the same box.
const LOGO_SIZE: i32 = 12;
const LOGO_X: i32 = BOOK_X + 3;
const BRANCH_Y: i32 = 50;
const STACK_X: i32 = 68;
const SPINE_W: usize = 22;
const SPINE_H: usize = 7;
const SIGN_Y: i32 = 57;
const SIGN_W: usize = 52;
const MAX_SIGN_CHARS: usize = 12;

const SKY: [(usize, usize, &str); 3] = [(0, 20, "#10152b"), (20, 20, "#151c38"), (40, 24, "#1b2445")];
const STARS: [(i32, i32); 14] = [
    (4, 4), (12, 10), (20, 3), (27, 14), (6, 22), (15, 30), (70, 20),
    (88, 24), (62, 6), (92, 14), (50, 4), (24, 40), (86, 36), (8, 44),
];
const STAR: Rgb = [0xe8, 0xe6, 0xc8];
const NO_COLOR: Rgb = [0x8b, 0x94, 0x9e];
const SIGN_WOOD: Rgb = [0x8a, 0x5a, 0x34];
const SIGN_EDGE: Rgb = [0x4a, 0x2e, 0x1a];
const SIGN_TEXT: Rgb = [0xf4, 0xe3, 0xc1];

const STYLE: &str = "\
.lid{opacity:0;animation:blink 8s step-end infinite}\
@keyframes blink{0%{opacity:0}30%{opacity:1}32.5%{opacity:0}70%{opacity:1}72.5%{opacity:0}}\
.p1,.p2{opacity:0;animation:8s step-end infinite}.p1{animation-name:p1}.p2{animation-name:p2}\
@keyframes p1{0%{opacity:0}50%{opacity:1}53%{opacity:0}}\
@keyframes p2{0%{opacity:0}53%{opacity:1}56%{opacity:0}}\
.head{animation:tilt 16s step-end infinite}\
@keyframes tilt{0%{transform:translate(0,0)}20%{transform:translate(1px,-1px)}35%{transform:translate(0,0)}}\
.tw{animation:tw 4s step-end infinite}.tw2{animation-delay:-2s}\
@keyframes tw{0%{opacity:1}50%{opacity:.35}}\
@media (prefers-reduced-motion:reduce){*{animation:none!important}}";

/// Renders the owl reading `books` (largest first) from the repo `repo_name`.
pub fn render(repo_name: &str, books: &[Book]) -> String {
    let art = Art::load();

    let mut back = Layer::new();
    let mut twinkle = [Layer::new(), Layer::new()];
    // Every third star twinkles, alternating between two out-of-phase groups.
    for (i, &(x, y)) in STARS.iter().enumerate() {
        match i % 3 {
            0 => twinkle[i / 3 % 2].dot(x, y, STAR),
            _ => back.dot(x, y, STAR),
        }
    }
    back.sprite(&art.moon, 80, 5);
    back.sprite(&art.branch, 0, BRANCH_Y);
    for (i, book) in books.iter().skip(1).enumerate() {
        // Nudge alternate books sideways so the stack looks hand-placed.
        let x = STACK_X + if i == 0 { 1 } else { -1 };
        let y = BRANCH_Y - SPINE_H as i32 * (i as i32 + 1);
        spine(&mut back, &art, book, x, y);
    }
    back.sprite(&art.body, OWL_X, BODY_Y);

    let mut head = Layer::new();
    head.sprite(&art.head, HEAD_X, OWL_Y);
    let mut lid = Layer::new();
    lid.sprite(&art.eyelids, HEAD_X, OWL_Y);

    let mut front = Layer::new();
    if let Some(book) = books.first() {
        cover(&mut front, &art, book);
    }
    front.sprite(&art.hands, OWL_X, BODY_Y);
    sign(&mut front, &art, repo_name);

    // Offset right so the turning page doesn't land on the beak.
    let mut page1 = Layer::new();
    page1.sprite(&art.page1, BOOK_X + 4, BOOK_Y - 5);
    let mut page2 = Layer::new();
    page2.sprite(&art.page2, BOOK_X + 4, BOOK_Y - 5);

    let names: Vec<&str> = books.iter().map(|b| b.name.as_str()).collect();
    let mut svg = format!(
        "<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{}\" height=\"{}\" viewBox=\"0 0 {W} {H}\" \
         shape-rendering=\"crispEdges\" role=\"img\">\
         <title>An owl reading {}: {}</title><style>{STYLE}</style>",
        W * SCALE,
        H * SCALE,
        xml_escape(repo_name),
        xml_escape(&names.join(", ")),
    );
    for (y, h, color) in SKY {
        svg += &format!("<rect y=\"{y}\" width=\"{W}\" height=\"{h}\" fill=\"{color}\"/>");
    }
    svg += &back.to_rects();
    svg += &group("tw", &twinkle[0].to_rects());
    svg += &group("tw tw2", &twinkle[1].to_rects());
    svg += &group("head", &(head.to_rects() + &group("lid", &lid.to_rects())));
    svg += &front.to_rects();
    svg += &group("p1", &page1.to_rects());
    svg += &group("p2", &page2.to_rects());
    svg += "</svg>\n";
    svg
}

/// Wraps content in a `<g>` whose class picks its CSS animation.
fn group(class: &str, content: &str) -> String {
    format!("<g class=\"{class}\">{content}</g>")
}

/// The book the owl holds: its hand-drawn logo if there is one, otherwise
/// a label on GitHub's color.
fn cover(layer: &mut Layer, art: &Art, book: &Book) {
    let color = book_color(book);
    book_shape(layer, BOOK_X, BOOK_Y, BOOK_W, BOOK_H, color);
    for y in 1..BOOK_H as i32 - 1 {
        layer.dot(BOOK_X + 1, y + BOOK_Y, shade(color, 0.75));
        layer.dot(BOOK_X + 2, y + BOOK_Y, shade(color, 0.85));
    }
    match art::logo(&book.name) {
        Some(logo) => layer.sprite(&logo, LOGO_X, BOOK_Y + 4),
        None => {
            let label = languages::label(&book.name);
            layer.centered_text(&art.font, &label, LOGO_X, LOGO_SIZE, BOOK_Y + 8, ink_for(color));
        }
    }
}

/// A book lying in the stack, seen from its spine.
fn spine(layer: &mut Layer, art: &Art, book: &Book, x: i32, y: i32) {
    let color = book_color(book);
    book_shape(layer, x, y, SPINE_W, SPINE_H, color);
    for band_x in [x + 2, x + SPINE_W as i32 - 3] {
        for dy in 1..SPINE_H as i32 - 1 {
            layer.dot(band_x, y + dy, shade(color, 1.3));
        }
    }
    let label = languages::label(&book.name);
    layer.centered_text(&art.font, &label, x, SPINE_W as i32, y + 1, ink_for(color));
}

fn sign(layer: &mut Layer, art: &Art, repo_name: &str) {
    let x = (W - SIGN_W) as i32 / 2;
    for rope_x in [x + 4, x + SIGN_W as i32 - 5] {
        layer.dot(rope_x, SIGN_Y - 1, SIGN_EDGE);
    }
    layer.rect(x, SIGN_Y, SIGN_W, 7, SIGN_EDGE);
    layer.rect(x + 1, SIGN_Y + 1, SIGN_W - 2, 5, SIGN_WOOD);
    let label = truncate(&repo_name.to_uppercase(), MAX_SIGN_CHARS);
    layer.centered_text(&art.font, &label, x, SIGN_W as i32, SIGN_Y + 1, SIGN_TEXT);
}

fn book_shape(layer: &mut Layer, x: i32, y: i32, w: usize, h: usize, color: Rgb) {
    layer.rect(x, y, w, h, shade(color, 0.45));
    layer.rect(x + 1, y + 1, w - 2, h - 2, color);
}

fn book_color(book: &Book) -> Rgb {
    book.color.as_deref().and_then(parse_hex).unwrap_or(NO_COLOR)
}

fn truncate(text: &str, max: usize) -> String {
    if text.chars().count() <= max {
        text.to_string()
    } else {
        text.chars().take(max - 2).chain("..".chars()).collect()
    }
}

fn text_width(text: &str) -> i32 {
    (text.chars().count() * (GLYPH_W + 1)).saturating_sub(1) as i32
}

/// Dark ink on light covers, light ink on dark ones.
fn ink_for(color: Rgb) -> Rgb {
    let [r, g, b] = color.map(|c| c as f64 / 255.0);
    if 0.299 * r + 0.587 * g + 0.114 * b > 0.6 { [0x1a, 0x1a, 0x1a] } else { [0xf5, 0xf5, 0xf5] }
}

fn shade(color: Rgb, factor: f64) -> Rgb {
    color.map(|c| (c as f64 * factor).round().min(255.0) as u8)
}

fn parse_hex(hex: &str) -> Option<Rgb> {
    let hex = hex.strip_prefix('#')?;
    if hex.len() != 6 {
        return None;
    }
    let channel = |i: usize| u8::from_str_radix(&hex[i..i + 2], 16).ok();
    Some([channel(0)?, channel(2)?, channel(4)?])
}

fn xml_escape(text: &str) -> String {
    text.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;")
}

/// One full-canvas pixel layer. Each animated part of the scene gets its own
/// layer so it can be wrapped in its own `<g>`.
struct Layer {
    pixels: Vec<Option<Rgb>>,
}

impl Layer {
    fn new() -> Layer {
        Layer { pixels: vec![None; W * H] }
    }

    fn dot(&mut self, x: i32, y: i32, color: Rgb) {
        if (0..W as i32).contains(&x) && (0..H as i32).contains(&y) {
            self.pixels[y as usize * W + x as usize] = Some(color);
        }
    }

    fn rect(&mut self, x: i32, y: i32, w: usize, h: usize, color: Rgb) {
        for dy in 0..h as i32 {
            for dx in 0..w as i32 {
                self.dot(x + dx, y + dy, color);
            }
        }
    }

    fn sprite(&mut self, sprite: &Sprite, x: i32, y: i32) {
        for sy in 0..sprite.height {
            for sx in 0..sprite.width {
                if let Some(color) = sprite.get(sx, sy) {
                    self.dot(x + sx as i32, y + sy as i32, color);
                }
            }
        }
    }

    fn text(&mut self, font: &Sprite, text: &str, x: i32, y: i32, color: Rgb) {
        for (i, ch) in text.chars().enumerate() {
            let index = FONT_CHARS.find(ch.to_ascii_uppercase()).or_else(|| FONT_CHARS.find('?'));
            let gx = index.unwrap() * GLYPH_W;
            for gy in 0..GLYPH_H {
                for dx in 0..GLYPH_W {
                    if font.get(gx + dx, gy).is_some() {
                        self.dot(x + (i * (GLYPH_W + 1) + dx) as i32, y + gy as i32, color);
                    }
                }
            }
        }
    }

    /// Draws `text` horizontally centered in the `width` pixels from `x`.
    fn centered_text(&mut self, font: &Sprite, text: &str, x: i32, width: i32, y: i32, color: Rgb) {
        self.text(font, text, x + (width - text_width(text)) / 2, y, color);
    }

    /// Emits the layer as `<rect>`s, merging runs of same-colored pixels.
    fn to_rects(&self) -> String {
        let mut out = String::new();
        for y in 0..H {
            let mut x = 0;
            while x < W {
                let Some(color) = self.pixels[y * W + x] else {
                    x += 1;
                    continue;
                };
                let start = x;
                while x < W && self.pixels[y * W + x] == Some(color) {
                    x += 1;
                }
                let [r, g, b] = color;
                out += &format!(
                    "<rect x=\"{start}\" y=\"{y}\" width=\"{}\" height=\"1\" fill=\"#{r:02x}{g:02x}{b:02x}\"/>",
                    x - start
                );
            }
        }
        out
    }
}
