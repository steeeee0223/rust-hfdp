pub struct DVDPlayer {
    description: String,
    has_dvd: bool,
    movie: Option<String>,
}
impl DVDPlayer {
    pub fn new(description: &str) -> Self {
        DVDPlayer {
            description: description.to_owned(),
            has_dvd: false,
            movie: None,
        }
    }
    pub fn on(&self) {
        println!("{} DVD player on", self.description);
    }
    pub fn off(&self) {
        println!("{} DVD player off", self.description);
    }
    pub fn insert(&mut self) {
        self.has_dvd = true;
        println!("{} DVD player insert DVD", self.description)
    }
    pub fn eject(&mut self) {
        self.has_dvd = false;
        println!("{} DVD player eject DVD", self.description)
    }
    pub fn play(&mut self, movie: &str) {
        self.movie = Some(movie.to_owned());
        println!("{} DVD player playing movie: {}", self.description, movie);
    }
    pub fn stop(&mut self) {
        if let Some(movie) = self.movie.take() {
            println!("{} stopped movie: {}", self.description, movie);
            self.movie = None;
        }
    }
}
