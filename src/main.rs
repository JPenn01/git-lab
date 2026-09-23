mod report;
mod supplies;

fn main() {
    println!("Have you ever been so hungry you could eat a horse? Wahhhhhhhhhhhhhh");
    let party: u32 = 4;
    let days: u32 = 30;
    let rations_per_day: u32 = 3;

    let food = supplies::food_needed(party, days, rations_per_day);

    report::print_report(party, days, food);
}
