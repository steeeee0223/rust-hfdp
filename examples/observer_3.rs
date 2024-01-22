use std::rc::Rc;

use rust_hfdp::observer_3::{
    observers::{CurrentConditionDisplay, Observer, PredictDisplay},
    subject::WeatherData,
};

fn main() {
    let mut weather_data = WeatherData::new();

    let observer_1: Rc<dyn Observer> = Rc::new(CurrentConditionDisplay);
    let observer_2: Rc<dyn Observer> = Rc::new(CurrentConditionDisplay);
    let observer_3: Rc<dyn Observer> = Rc::new(PredictDisplay);

    println!("-----add three observers-----");
    weather_data.register_observer(&observer_1);
    weather_data.register_observer(&observer_2);
    weather_data.register_observer(&observer_3);

    weather_data.set_measurements(10.0);

    println!("-----remove the first observer----");
    weather_data.remove_observer(&observer_1);
    weather_data.set_measurements(11.0);
}
