use rtui::tui::App;

use std::io;

fn main() -> io::Result<()> {
    let mut app = App::new().expect("Error on creating app");

    app.init_terminal().expect("Error on init_terminal");
    app.run()?;
    app.restore_terminal()
        .expect("Error on restoring terminal, please close and open this terminal window");
    println!("Funcionou pra entrar e sair em!!!");
    Ok(())
}
