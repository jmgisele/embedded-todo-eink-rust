use draw::draw_org;

const WIDTH: u32 = 600;
const HEIGHT: u32 = 800;
const DIM: usize = (WIDTH * HEIGHT) as usize;
const MARGIN: i32 = 16;

mod cosmic;
mod draw;
mod node;
mod parse;
mod save;

fn main() -> std::io::Result<()> {
    let files_to_parse: Vec<String> = vec![
        // "/home/jgisele/emacs/org/SHARED/habits.org".to_owned(), // these are the files you want to parse
        // "/home/jgisele/emacs/org/SHARED/todos.org".to_owned(),
        "/media/backups/SHARED/habits.org".to_owned(),
        "/media/backups/SHARED/todos.org".to_owned(),
    ];

    draw_org(files_to_parse, true)?;

    Ok(())
}
