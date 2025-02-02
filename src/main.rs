#[derive(Debug)]
struct TaylorSwiftSong {
    title: String,
    release_year: u32,
    duration_secs: u32,
}

impl TaylorSwiftSong {
    fn display_song_info(self: Self) {
        // Immutable struct value (self param takes ownership)
        // Mutable struct value (self parameter takes ownership, has permission to mutate)
        // Immutable reference to the struct instance (no ownership moved)
        // Mutable reference to the struct instance (no ownership moved, have permission to mutate)
        println!("Title: {}", self.title);
        println!("Release Year {}", self.release_year);
        println!("Duration: {} seconds", self.duration_secs);
    }
}

fn main() {
    let song: TaylorSwiftSong = TaylorSwiftSong {
        title: String::from("Blank Space"),
        release_year: 2014,
        duration_secs: 231,
    };

    song.display_song_info();
}
