use std::fmt;

use super::MenuComponent;

pub struct Menu {
    name: String,
    description: String,
    menu_components: Vec<Box<dyn MenuComponent>>,
}
impl Menu {
    pub fn new(name: &str, description: &str) -> Menu {
        Menu {
            name: name.to_owned(),
            description: description.to_owned(),
            menu_components: vec![],
        }
    }
}
impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut buff = String::new();
        buff.push_str(&format!("\n{} - {}", self.name(), self.description()));
        buff.push_str("\n--------\n");

        for menu_component in self.menu_components.iter() {
            buff.push_str(&format!("{}\n", menu_component.as_ref()));
        }

        write!(f, "{}", buff)
    }
}
impl MenuComponent for Menu {
    fn name(&self) -> String {
        self.name.to_owned()
    }
    fn description(&self) -> String {
        self.description.to_owned()
    }
    fn is_vegetarian(&self) -> bool {
        unimplemented!("Unsupported method")
    }
    fn price(&self) -> f32 {
        unimplemented!("Unsupported method")
    }
    fn add(&mut self, menu: Box<dyn MenuComponent>) {
        self.menu_components.push(menu);
    }
    fn remove(&mut self, menu: Box<dyn MenuComponent>) {
        self.menu_components
            .retain(|child| child as *const _ != &menu as *const _);
    }
    fn child(&self, i: usize) -> &Box<dyn MenuComponent> {
        &self.menu_components[i]
    }
}
