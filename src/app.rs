
use std::collections::HashMap;

use super::{contact::Contact, ui::Console};

pub(crate) type ContactList = HashMap<String,Contact>;

pub struct App {
    pub(crate) contact_list: ContactList
}

impl App {
    pub fn new(contact_list: ContactList) -> Self {
        Self { contact_list }
    }

    pub fn add(&mut self,new_contact:Contact) {
        let _ = &self.contact_list.insert(new_contact.name().to_string(), new_contact);
    }
}

impl Console for App {
    fn show(&self) {
        for contact in &self.contact_list {
            println!("{}:", contact.0);
            contact.1.show();
        }
    }
}
