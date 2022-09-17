use std::io;

use app::App;

mod app;

fn main() -> crossterm::Result<()> {
    let stdout = io::stdout();

    let mut app = App::new(stdout);

    app.run()
}
