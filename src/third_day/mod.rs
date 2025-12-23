use std::fs;

#[cfg(test)]
mod tests;

#[derive(Debug, Clone, Eq, Copy)]
struct Battery {
    position: usize,
    joltage: usize,
}

impl Ord for Battery {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.joltage
            .cmp(&other.joltage)
            .then(other.position.cmp(&self.position))
    }
}

impl PartialEq for Battery {
    fn eq(&self, other: &Self) -> bool {
        (self.position, self.joltage) == (other.position, other.joltage)
    }
}

impl PartialOrd for Battery {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug, Clone)]
struct PowerBank {
    batteries: Vec<Battery>,
}

impl PowerBank {
    fn new(input: &str) -> Self {
        PowerBank {
            batteries: input
                .chars()
                .enumerate()
                .map(|(index, battery)| Battery {
                    position: index,
                    joltage: battery.to_string().parse::<usize>().unwrap(),
                })
                .collect::<Vec<Battery>>(),
        }
    }

    fn get_maximum_joltage(&self, n_active_batteries: usize) -> usize {
        let n_batteries = self.batteries.len();

        let ten: usize = 10;
        let mut active_joltage = 0;
        let mut last_active_battery_position = 0;
        for i in 1..n_active_batteries + 1 {
            let best_available_battery = self.batteries
                [last_active_battery_position..n_batteries - n_active_batteries + i]
                .iter()
                .max()
                .expect("A max to be found");
            active_joltage +=
                best_available_battery.joltage * ten.pow((n_active_batteries - i) as u32);
            last_active_battery_position = best_available_battery.position + 1;
        }
        active_joltage
    }
}

pub fn main() {
    let power_banks = fs::read_to_string("data/day_3.txt")
        .expect("The input to be correctly read")
        .lines()
        .map(PowerBank::new)
        .collect::<Vec<PowerBank>>();
    let first_output_joltage = power_banks
        .iter()
        .map(|power_bank| power_bank.get_maximum_joltage(2))
        .sum::<usize>();
    dbg!(first_output_joltage);
    let unsafe_output_joltage = power_banks
        .iter()
        .map(|power_bank| power_bank.get_maximum_joltage(12))
        .sum::<usize>();
    dbg!(unsafe_output_joltage);
}
