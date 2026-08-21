use crate::node::HabitNode;
use crate::parse::{add_todo_symbols, parse_emacs, sort_todos};
use crate::save::save_file;
use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, SwashCache};
use std::io::Result;

use std::fs::File;
use std::io::Read;
use tiny_skia::{Paint, PixmapMut, Rect, Transform};

use crate::{DIM, HEIGHT, MARGIN, WIDTH};

pub fn draw_org(file_paths: Vec<String>, run_script: bool) -> Result<()> {
    let buffer: &mut [u8; DIM * 4] = &mut [0xFF; DIM * 4];
    let mut pixmap = PixmapMut::from_bytes(buffer, WIDTH, HEIGHT).unwrap();
    let mut paint = Paint {
        anti_alias: false,
        ..Default::default()
    };

    pixmap.fill(tiny_skia::Color::from_rgba8(0xFF, 0xFF, 0xFF, 0xFF));
    let mut font_system = FontSystem::new();
    let mut swash_cache = SwashCache::new();
    let metrics = Metrics::new(20., 24.);
    let mut buffer = Buffer::new(&mut font_system, metrics);
    let mut buffer = buffer.borrow_with(&mut font_system);
    buffer.set_size(
        Some((WIDTH - 2 * MARGIN as u32) as f32),
        Some((HEIGHT - 2 * MARGIN as u32) as f32),
    );
    let mut attrs = Attrs::new();
    attrs = attrs.family(cosmic_text::Family::Name("Victor Mono"));

    // grab file text
    let mut output_text: Vec<HabitNode> = Vec::new();

    for file_path in file_paths {
        let mut file = File::open(file_path).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();

        // Add some text!
        let mut file_nodes: Vec<HabitNode> = parse_emacs(&contents, &attrs);
        add_todo_symbols(&mut file_nodes);
        output_text.append(&mut file_nodes);
    }
    output_text = sort_todos(&output_text);

    let iter = output_text
        .iter()
        .map(|x| (x.title.as_str(), x.attrs.clone()));
    buffer.set_rich_text(iter, &attrs, Shaping::Advanced, None);

    // // Perform shaping as desired
    buffer.shape_until_scroll(true);

    // // Create a default text color
    let text_color = cosmic_text::Color::rgb(0, 0, 0);

    // Draw the buffer (for performance, instead use SwashCache directly)
    buffer.draw(&mut swash_cache, text_color, |x, y, w, h, color| {
        // Fill in your code here for drawing rectangles
        paint.set_color_rgba8(color.b(), color.g(), color.r(), color.a());
        pixmap.fill_rect(
            Rect::from_xywh((x + MARGIN) as f32, (y + MARGIN) as f32, w as f32, h as f32).unwrap(),
            &paint,
            Transform::identity(),
            None,
        );
    });

    save_file(pixmap.to_owned(), run_script)?;
    Ok(())
}
