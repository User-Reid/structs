// Hours, Minutes
struct ShortDuration(u32, u32);

// Years, Months
struct LongDuration(u32, u32);

fn main() {
    let work_shift: ShortDuration = ShortDuration(8, 0);
    let era: LongDuration = LongDuration(5, 3);
    println!("{} hours, {} minutes", work_shift.0, work_shift.1);
    println!("{} years, {} months", era.0, era.1);
}
