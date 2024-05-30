use gtk4::prelude::*;
use gtk4::{glib, Application, ApplicationWindow, Button};

const APLICATION_ID: &str = "org.gtk_rs.HelloWorld";

fn build_ui(app: &Application) {
    let button = Button::builder().label("Press Me!").margin_top(50).build();

    button.connect_clicked(|btn| {
        btn.set_label("Hello World!");
    });

    let window = ApplicationWindow::builder()
        .application(app)
        .title("Hello World GTK!")
        .child(&button)
        .build();

    window.present();
}

fn main() -> glib::ExitCode {
    let app = Application::builder().application_id(APLICATION_ID).build();

    app.connect_activate(build_ui);

    app.run()
}
