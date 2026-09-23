mod report;
mod supplies;

fn main() {
    println!("Wahhhhhhhhhhhhhh");

    let party: u32 = 4;
    let days: u32 = 30;

    let food = supplies::food_needed(party, days);

    report::print_report(party, days, food);
}
