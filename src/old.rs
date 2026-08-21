// fn draw_raqote() -> std::io::Result<()> {
//     let mut dt = raqote::DrawTarget::new(WIDTH as i32, HEIGHT as i32);
//     dt.clear(SolidSource {
//         r: 255,
//         g: 255,
//         b: 255,
//         a: 255,
//     });

//     let mut file = File::open("./hello_world.txt").unwrap();
//     let mut contents = String::new();
//     file.read_to_string(&mut contents).unwrap();

//     let buffer = text::text(
//         &text::TextProps {
//             text: contents,
//             size: 9,
//         },
//         Color::new(255, 0, 0, 0),
//     );

//     buffer.render(&mut dt, raqote::Point::new(10., 10.));
//     dt.write_png("./out.png").unwrap();

//     let buf: &[u8] = dt.get_data_u8();

//     let mut text_file = File::create("./print.txt")?;
//     write!(text_file, "{:#?}", &buf)?;

//     let mut raw_file = File::create("./img.raw")?;
//     raw_file.write(buf)?;

//     Ok(())
// }

// fn draw_talmud() -> std::io::Result<()> {
//     let img = image::open("talmud_crop.png").unwrap().into_luma8();
//     let buf = img.into_raw();
//     let mut text_file = File::create("./print_talmud.txt")?;
//     write!(text_file, "{:#?}", &buf)?;

//     let mut raw_file = File::create("./img.raw")?;
//     raw_file.write(&buf)?;

//     // file.write(&buf)?;
//     // Command::new("sh")
//     //     .arg("./display_raw.sh")
//     //     .output()
//     //     .expect("Failed to execute shell");

//     Ok(())
// }
