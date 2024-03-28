use std::fmt;

pub enum Menu {
    Menu(String, String, Vec<Menu>),
    MenuItem(String, String, bool, f32),
}

pub trait MenuComponent
where
    Self: fmt::Display,
{
    fn name(&self) -> String;
    fn description(&self) -> String;
    fn is_vegetarian(&self) -> bool;
    fn price(&self) -> f32;
    fn add(&mut self, menu_component: Menu);
    fn remove(&mut self, menu_component: Menu);
    fn child(&self, i: usize) -> &Menu;
}

impl fmt::Display for Menu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Menu::MenuItem(name, description, vegetarian, price) => write!(
                f,
                "\n{}{}, {} -- {}",
                name,
                if *vegetarian { "(v)" } else { "" },
                price,
                description
            ),
            Menu::Menu(name, description, menu_components) => {
                let mut buff = String::new();
                buff.push_str(&format!("\n{} - {}", name, description));
                buff.push_str("\n--------\n");
                for menu_component in menu_components.iter() {
                    buff.push_str(&format!("{}\n", menu_component));
                }
                write!(f, "{}", buff)
            }
        }
    }
}

impl MenuComponent for Menu {
    fn name(&self) -> String {
        match self {
            Menu::Menu(name, ..) => name.to_owned(),
            Menu::MenuItem(name, ..) => name.to_owned(),
        }
    }
    fn description(&self) -> String {
        match self {
            Menu::Menu(_, description, _) => description.to_owned(),
            Menu::MenuItem(_, description, ..) => description.to_owned(),
        }
    }
    fn is_vegetarian(&self) -> bool {
        match self {
            Menu::MenuItem(.., vegetarian, _) => *vegetarian,
            _ => unimplemented!("Unsupported method"),
        }
    }
    fn price(&self) -> f32 {
        match self {
            Menu::MenuItem(.., price) => *price,
            _ => unimplemented!("Unsupported method"),
        }
    }
    fn add(&mut self, menu_component: Menu) {
        match self {
            Menu::Menu(.., ref mut menu_components) => {
                menu_components.push(menu_component);
            }
            _ => unimplemented!("Unsupported method"),
        }
    }

    fn remove(&mut self, menu_component: Menu) {
        match self {
            Menu::Menu(.., ref mut menu_components) => {
                menu_components.retain(|child| child as *const _ != &menu_component as *const _);
            }
            _ => unimplemented!("Unsupported method"),
        }
    }
    fn child(&self, i: usize) -> &Menu {
        match self {
            Menu::Menu(.., menu_components) => &menu_components[i],
            _ => unimplemented!("Unsupported method"),
        }
    }
}
