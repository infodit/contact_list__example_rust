
use super::{contact::Contact, ui::Console};

pub(crate) type ContactList = Vec<Contact>;

pub struct App {
    pub(crate) contact_list: ContactList
}

impl App {
    pub fn new(contact_list: ContactList) -> Self {
        Self { contact_list }
    }

    pub fn add(&mut self,new_contact:Contact) {
        let _ = &self.contact_list.push(new_contact);
    }
}

impl Console for App {
    fn show(&self) {
        for contact in &self.contact_list {
            contact.show();
        }
    }
}
