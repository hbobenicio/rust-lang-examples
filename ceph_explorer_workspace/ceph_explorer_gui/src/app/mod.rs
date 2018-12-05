use gtk::{prelude::*, Button, ToolButton, Window, Builder};

#[derive(Debug)]
pub struct App {
  main_window: Window,
  add_toolbutton: ToolButton,
  remove_toolbutton: ToolButton,
  start_button: Button,
}

impl App {
    pub fn new() -> Result<App, String> {
      gtk::init()
          .map_err(|e| format!("Failed to initialize GTK: {}", e))?;

      let glade_src: &str = include_str!("../resources/ui/main_window.glade");
      let builder: Builder = gtk::Builder::new_from_string(glade_src);

      let main_window: Window = get_object_from_builder(&builder, "main_window")?;
      let add_toolbutton: ToolButton = get_object_from_builder(&builder, "add_toolbutton")?;
      let remove_toolbutton: ToolButton = get_object_from_builder(&builder, "remove_toolbutton")?;
      let start_button: Button = get_object_from_builder(&builder, "start_button")?;

      let app = App {
        main_window,
        add_toolbutton,
        remove_toolbutton,
        start_button
      };

      app.connect_signals();
      app.main_window.show_all();

      Ok(app)
    }

    pub fn start(&self) -> Result<(), String> {
      gtk::main();

      Ok(())
    }

    fn connect_signals(&self) {
      self.main_window.connect_delete_event(|_win, _e: &gdk::Event| {
          gtk::main_quit();
          Inhibit(false)
      });

      self.add_toolbutton.connect_clicked(|_btn| {
          println!("Add button clicked");
      });

      self.remove_toolbutton.connect_clicked(|_btn| {
          println!("Remove button clicked");
      });

      self.start_button.connect_clicked(|_btn| {
          println!("Start button clicked");
      });
    }
}

fn get_object_from_builder<O>(builder: &Builder, object_name: &str) -> Result<O, String>
where
    O: gtk::IsA<gtk::Object>
{
    builder
      .get_object(object_name)
      .ok_or(
        format!("Error while getting object with id {} from builder", object_name)
      )
}

