#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl TaylorSwiftSong {
    // Immutable struct value (self param takes ownership)
    // -> Immutable reference to the struct instance (no ownership moved)
    fn display_song_info(&self) {
        println!("Title: {}", self.title);
        println!("Release Year {}", self.release_year);
        println!("Duration: {} seconds", self.duration_secs);
    }

    // Mutable struct value (self parameter takes ownership, has permission to mutate)
    // Mutable reference to the struct instance (no ownership moved, have permission to mutate)
    fn double_length(&mut self) {
        self.duration_secs = self.duration_secs * 2;
    }
}

fn main() {
    let mut song: TaylorSwiftSong = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };

    song.display_song_info();
    song.double_length();
    song.display_song_info();
}
