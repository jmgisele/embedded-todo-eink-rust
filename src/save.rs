use std::fs::File;
use std::io::Result;
use std::io::{self, Write};
use std::process::Command;
use tiny_skia::Pixmap;

pub fn save_file(pixmap: Pixmap, run_script: bool) -> Result<()> {
    pixmap.to_owned().save_png("./output.png")?;
    let img = image::open("output.png").unwrap().into_luma8();
    let buf = img.into_raw();

    let mut text_file = File::create("./data.txt")?;
    write!(text_file, "{:#?}", &buf)?;

    let mut raw_file = File::create("./img.raw")?;
    raw_file.write(&buf)?;

    if run_script {
        let output = execute_shell("./send-to-ereader.sh");
        println!("{}", output.status);
        io::stdout().write_all(&output.stdout)?;
    }

    Ok(())
}

fn execute_shell(shell_path: &str) -> std::process::Output {
    Command::new("sh")
        .arg(shell_path)
        .output()
        .expect("Failed to execute shell")
}
