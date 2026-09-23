mod report;
mod supplies;

fn main() {
    println!("Have you ever been so hungry you could eat a horse?");

    let party: u32 = 4;
    let days: u32 = 30;

    let food = supplies::food_needed(party, days);

    report::print_report(party, days, food);
}
