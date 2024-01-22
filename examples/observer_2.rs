use std::cell::RefCell;
use std::rc::Rc;

use rust_hfdp::observer_2::observers::{CurrentConditionDisplay, Observer, PredictDisplay};
use rust_hfdp::observer_2::subject::WeatherData;

fn main() {
    let mut weather_data = WeatherData::new();
    let observer_1 = Rc::new(RefCell::new(CurrentConditionDisplay::new(1)));
    let observer_2 = Rc::new(RefCell::new(CurrentConditionDisplay::new(2)));
    let observer_3 = Rc::new(RefCell::new(PredictDisplay::new(3)));

    // println!("-----add three observers-----");
    let tmp = Rc::clone(&observer_1);
    let observer_1_clone1: Rc<RefCell<dyn Observer>> = tmp;
    weather_data.register_observer(observer_1_clone1);

    let tmp = Rc::clone(&observer_2);
    let observer_2_clone1: Rc<RefCell<dyn Observer>> = tmp;
    weather_data.register_observer(observer_2_clone1);

    let tmp = Rc::clone(&observer_3);
    let observer_3_clone1: Rc<RefCell<dyn Observer>> = tmp;
    weather_data.register_observer(observer_3_clone1);

    weather_data.set_measurements(10.0);
    println!("------------");
    weather_data.set_measurements(12.0);

    println!("-----remove the third observer-----");
    let tmp = Rc::clone(&observer_1);
    let observer_1_clone2: Rc<RefCell<dyn Observer>> = tmp;
    weather_data.remove_observer(observer_1_clone2);
    weather_data.set_measurements(14.0);
}
