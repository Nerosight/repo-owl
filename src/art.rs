//! The pixel art, decoded from the PNGs baked in at compile time.

mod embedded {
    include!(concat!(env!("OUT_DIR"), "/logos.rs"));
}

pub type Rgb = [u8; 3];

/// A grid of pixels; `None` is transparent.
pub struct Sprite {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Option<Rgb>>,
}

impl Sprite {
    pub fn get(&self, x: usize, y: usize) -> Option<Rgb> {
        self.pixels[y * self.width + x]
    }

    /// Decodes a PNG of any color type. Pixels under half opacity count as
    /// transparent, since pixel art has no in-between.
    pub fn from_png(bytes: &[u8]) -> Sprite {
        let mut decoder = png::Decoder::new(bytes);
        decoder.set_transformations(png::Transformations::normalize_to_color8());
        let mut reader = decoder.read_info().expect("valid PNG");
        let mut buf = vec![0; reader.output_buffer_size()];
        let info = reader.next_frame(&mut buf).expect("valid PNG frame");
        let channels = info.color_type.samples();
        let pixels = buf[..info.buffer_size()]
            .chunks(channels)
            .map(|px| match px {
                [g] => Some([*g, *g, *g]),
                [g, a] => (*a >= 128).then_some([*g, *g, *g]),
                [r, g, b] => Some([*r, *g, *b]),
                [r, g, b, a] => (*a >= 128).then_some([*r, *g, *b]),
                _ => unreachable!("normalized PNGs have 1-4 channels"),
            })
            .collect();
        Sprite { width: info.width as usize, height: info.height as usize, pixels }
    }
}

macro_rules! asset {
    ($path:literal) => {
        Sprite::from_png(include_bytes!(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/", $path)))
    };
}

pub struct Art {
    pub head: Sprite,
    pub eyelids: Sprite,
    pub body: Sprite,
    pub hands: Sprite,
    pub page1: Sprite,
    pub page2: Sprite,
    pub moon: Sprite,
    pub branch: Sprite,
    pub font: Sprite,
}

impl Art {
    pub fn load() -> Art {
        Art {
            head: asset!("owl/head.png"),
            eyelids: asset!("owl/eyelids.png"),
            body: asset!("owl/body.png"),
            hands: asset!("owl/hands.png"),
            page1: asset!("owl/page1.png"),
            page2: asset!("owl/page2.png"),
            moon: asset!("scene/moon.png"),
            branch: asset!("scene/branch.png"),
            font: asset!("font.png"),
        }
    }
}

/// The hand-drawn logo for a language, if one exists in `assets/logos`.
pub fn logo(language: &str) -> Option<Sprite> {
    let slug = logo_slug(language);
    embedded::LOGOS.iter().find(|(s, _)| *s == slug).map(|(_, bytes)| Sprite::from_png(bytes))
}

/// `Jupyter Notebook` → `jupyter-notebook`, `C++` → `cplusplus`, `C#` → `csharp`.
pub fn logo_slug(language: &str) -> String {
    language
        .to_lowercase()
        .replace('+', "plus")
        .replace('#', "sharp")
        .replace(' ', "-")
}

/// Glyph order in `font.png`; each glyph is 3×5.
pub const FONT_CHARS: &str = " ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789+#-._!?";
pub const GLYPH_W: usize = 3;
pub const GLYPH_H: usize = 5;
