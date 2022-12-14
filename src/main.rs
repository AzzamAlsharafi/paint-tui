mod app;
mod constant;
mod painter;
mod utils;

use std::io;

use app::App;

fn main() -> crossterm::Result<()> {
    let stdout = io::stdout();

    let mut app = App::new(stdout);

    app.run()
}
