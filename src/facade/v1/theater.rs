use std::cell::RefCell;
use std::rc::Rc;

use super::{Amplifier, DVDPlayer, TV};

pub struct HomeTheaterFacade {
    amp: Amplifier,
    dvd: Rc<RefCell<DVDPlayer>>,
    tv: TV,
}
impl HomeTheaterFacade {
    pub fn new(amp: Amplifier, dvd: DVDPlayer, tv: TV) -> Self {
        HomeTheaterFacade {
            amp,
            dvd: Rc::new(RefCell::new(dvd)),
            tv,
        }
    }
    pub fn watch_movie(&mut self, movie: &str) {
        println!("Ready to watch a movie: {}", movie);
        self.tv.on();

        self.dvd.borrow().on();
        self.dvd.borrow_mut().insert();

        self.amp.on();
        self.amp.set_volume(5);
        self.amp.set_dvd(Rc::clone(&self.dvd));

        self.dvd.borrow_mut().play(movie);
    }
    pub fn end_movie(&mut self) {
        println!("Shutting movie theater down...");
        self.amp.off();

        self.dvd.borrow_mut().stop();
        self.dvd.borrow_mut().eject();
        self.dvd.borrow_mut().off();

        self.tv.off();
    }
}
