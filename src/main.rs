use contact_list_example_rust::{app::App, contact::Contact, ui::Console};

fn main() {
    let mut app = App::new(Vec::new());

    app.add(Contact::new(String::from("Marco"), 33872564));

    app.show();
}