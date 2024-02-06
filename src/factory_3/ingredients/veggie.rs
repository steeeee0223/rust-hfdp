pub trait Veggies {
    fn name(&self) -> &str;
}

pub struct Garlic;
impl Veggies for Garlic {
    fn name(&self) -> &str {
        "Garlic"
    }
}

pub struct Onion;
impl Veggies for Onion {
    fn name(&self) -> &str {
        "Onion"
    }
}

pub struct Mushroom;
impl Veggies for Mushroom {
    fn name(&self) -> &str {
        "Mushroom"
    }
}

pub struct RedPepper;
impl Veggies for RedPepper {
    fn name(&self) -> &str {
        "Red Pepper"
    }
}

pub struct BlackOlives;
impl Veggies for BlackOlives {
    fn name(&self) -> &str {
        "Black Olives"
    }
}

pub struct Spinach;
impl Veggies for Spinach {
    fn name(&self) -> &str {
        "Spinach"
    }
}

pub struct Eggplant;
impl Veggies for Eggplant {
    fn name(&self) -> &str {
        "Eggplant"
    }
}
