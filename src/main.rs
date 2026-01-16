use enum_display::EnumDisplay;
use inquire::error::InquireResult;
use inquire_derive::Selectable;

mod eight_day;
mod fifth_day;
mod first_day;
mod fourth_day;
mod ninth_day;
mod second_day;
mod seventh_day;
mod sixth_day;
mod third_day;

#[derive(Debug, Clone, Copy, Selectable, EnumDisplay)]
enum DayOption {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eight,
    Ninth,
    Exit,
}

fn main() -> InquireResult<()> {
    let mut prompt_for_selection = true;
    while prompt_for_selection {
        let day =
            DayOption::select("Select the day over which to run the problem, or exit").prompt()?;

        match day {
            DayOption::Exit => prompt_for_selection = false,
            DayOption::First => first_day::main(),
            DayOption::Second => second_day::main(),
            DayOption::Third => third_day::main(),
            DayOption::Fourth => fourth_day::main(),
            DayOption::Fifth => fifth_day::main(),
            DayOption::Sixth => sixth_day::main(),
            DayOption::Seventh => seventh_day::main(),
            DayOption::Eight => eight_day::main(),
            DayOption::Ninth => ninth_day::main(),
        }
    }
    Ok(())
}
