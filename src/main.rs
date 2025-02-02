#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl TaylorSwiftSong {
    fn new(title: String, release_year: u32, duration_secs: u32) -> Self {
        Self {
            title,
            release_year,
            duration_secs,
        }
    }
}

impl TaylorSwiftSong {
    fn display_song_info(&self) {
        println!("Title: {}", self.title);
        println!("Years since release: {}", self.years_since_release());
        println!("Duration: {} seconds", self.duration_secs);
    }

    fn double_length(&mut self) {
        self.duration_secs = self.duration_secs * 2;
    }

    fn is_longer_than(&self, x: &Self) -> bool {
        self.duration_secs > x.duration_secs
    }

    fn years_since_release(&self) -> u32 {
        2024 - &self.release_year
    }
}

fn main() {
    let blank_space = TaylorSwiftSong::new(String::from("Bad Bitch"), 2024, 399);

    let all_too_well: TaylorSwiftSong = TaylorSwiftSong {
        title: String::from("All Too Well"),
        release_year: 2014,
        duration_secs: 327,
    };

    if blank_space.is_longer_than(&all_too_well) {
        println!(
            "{} is longer than {}",
            blank_space.title, all_too_well.title
        )
    } else {
        println!(
            "{} is longer than {}",
            all_too_well.title, blank_space.title
        )
    }

    all_too_well.display_song_info();
    blank_space.display_song_info();
}
