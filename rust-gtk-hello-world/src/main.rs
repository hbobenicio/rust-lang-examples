extern crate gtk;

use gtk::prelude::*;
use gtk::{Button, Window, WindowType};

fn main() {
    if gtk::init().is_err() {
        println!("Failed to initialize GTK.");
        return;
    }

    let glade_src = include_str!("resources/ui/main_window.glade");
    let builder = gtk::Builder::new_from_string(glade_src);
    let window: gtk::Window = builder.get_object("main_window")
        .expect("Não foi possível obter objeto 'main_window' da interface main_window.glade");
    window.show_all();

    // let window = Window::new(WindowType::Toplevel);
    // window.set_title("First GTK+ Program");
    // window.set_default_size(350, 70);
    // let button = Button::new_with_label("Click me!");
    // window.add(&button);
    // window.show_all();
    //
    // window.connect_delete_event(|_, _| {
    //     gtk::main_quit();
    //     Inhibit(false)
    // });
    //
    // button.connect_clicked(|_| {
    //     println!("Clicked!");
    // });

    gtk::main();
}