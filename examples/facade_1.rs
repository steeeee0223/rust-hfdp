use rust_hfdp::facade::v1::{Amplifier, DVDPlayer, HomeTheaterFacade, TV};

fn main() {
    let amp = Amplifier::new("Top-O-Line");
    let dvd = DVDPlayer::new("Top-O-Line");
    let tv = TV::new("Panasonic");

    let mut home_theater = HomeTheaterFacade::new(amp, dvd, tv);

    home_theater.watch_movie("Raiders of the Lost Ark");

    home_theater.end_movie();
}
