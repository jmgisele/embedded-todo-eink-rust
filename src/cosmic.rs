use cosmic_text::{Attrs, Buffer, FontSystem, Metrics, Shaping, Style, SwashCache, Weight};
use std::{fs::File, io::Read};
use tiny_skia::{Paint, PixmapMut, Rect, Transform};

use crate::{node::MarkdownNode, save::save_file, DIM, HEIGHT, MARGIN, WIDTH};

pub fn draw_cosmic(file_path: &str, run_script: bool) -> std::io::Result<()> {
    // set up buffer
    let buffer: &mut [u8; DIM * 4] = &mut [0xFF; DIM * 4];
    let mut pixmap = PixmapMut::from_bytes(buffer, WIDTH, HEIGHT).unwrap();
    let mut paint = Paint {
        anti_alias: false,
        ..Default::default()
    };

    // clear screen
    pixmap.fill(tiny_skia::Color::from_rgba8(0xFF, 0xFF, 0xFF, 0xFF));

    // A FontSystem provides access to detected system fonts, create one per application
    let mut font_system = FontSystem::new();
    // A SwashCache stores rasterized glyphs, create one per application
    let mut swash_cache = SwashCache::new();

    // Text metrics indicate the font size and line height of a buffer
    let metrics = Metrics::new(20., 24.);

    // A Buffer provides shaping and layout for a UTF-8 string, create one per text widget
    let mut buffer = Buffer::new(&mut font_system, metrics);
    // Borrow buffer together with the font system for more convenient method calls
    let mut buffer = buffer.borrow_with(&mut font_system);
    // Set a size for the text buffer, in pixels
    buffer.set_size(
        Some((WIDTH - 2 * MARGIN as u32) as f32),
        Some((HEIGHT - 2 * MARGIN as u32) as f32),
    );

    // Attributes indicate what font to choose
    let mut attrs = Attrs::new();
    attrs = attrs.family(cosmic_text::Family::Name("Victor Mono"));

    // grab file text
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    // Add some text!
    let output_text = parse_text_markdown(&contents, &attrs);
    let iter = output_text
        .iter()
        .map(|x| (x.text.as_str(), x.attrs.clone()));
    buffer.set_rich_text(iter, &attrs, Shaping::Advanced, None);

    // Perform shaping as desired
    buffer.shape_until_scroll(true);

    // Create a default text color
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

pub fn parse_text_markdown<'a>(
    input: &String,
    default_attributes: &Attrs<'a>,
) -> Vec<MarkdownNode<'a>> {
    let mut output: Vec<MarkdownNode> = Vec::new();

    for line in input.lines() {
        let mut curr_line: MarkdownNode<'a> = MarkdownNode {
            attrs: default_attributes.clone(),
            text: line.to_owned() + "\n",
        };

        if let Some(_) = line.chars().nth(4) {
            match &line[0..5] {
                "- [x]" => {
                    curr_line.text = "     ".to_owned() + &curr_line.text[5..];
                    output.push(curr_line);

                    continue;
                }
                "- [ ]" => {
                    curr_line.text = "     ".to_owned() + &curr_line.text[5..];
                    output.push(curr_line);

                    continue;
                }
                _ => {}
            }
        }

        if let Some(_) = line.chars().nth(0) {
            match &line[0..1] {
                "*" => {
                    curr_line.text = "     ".to_owned() + &curr_line.text[1..];
                    output.push(curr_line);

                    continue;
                }
                "#" => {
                    curr_line.attrs = curr_line
                        .attrs
                        .weight(Weight::BOLD)
                        .style(Style::Italic)
                        .metrics(Metrics::new(28., 34.));
                    curr_line.text = curr_line.text[1..].to_owned();
                    output.push(curr_line);
                    continue;
                }
                _ => {}
            }
        }
        output.push(curr_line);
    }
    output
}
