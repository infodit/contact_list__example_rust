
use super::ui::Console;


#[derive(Debug)]
pub struct Contact {
    name: String,
    number: u32,
}

impl Contact {
    pub fn new(name: String, number: u32) -> Self {
        Self { name, number }
    }
    
    //Get e set....

    
    pub fn name(&self) -> &str {
        &self.name
    }
    
    pub fn number(&self) -> u32 {
        self.number
    }
    
    pub fn set_name(&mut self, name: String) {
        self.name = name;
    }
    
    pub fn set_number(&mut self, number: u32) {
        self.number = number;
    }
}

impl Console for Contact {
    fn show(&self) {
        println!("  Nome: {:#?}",self.name);
        println!("  Numero: {:#?}",self.number);
    }
}
