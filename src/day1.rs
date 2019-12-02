use std::io::{stdin, Read};

pub fn run() {
    let module_masses = read_module_masses();
    let fuel: i64 = calculate_fuel(module_masses.iter());
    println!("result: {}", fuel);
}

fn read_module_masses() -> Vec<i64> {
    let mut s = Vec::new();
    stdin()
        .read_to_end(&mut s)
        .expect("did not enter a correct string");

    let input_string = std::str::from_utf8(&s).expect("invalid utf8");
    let module_masses: Vec<i64> = input_string
        .lines()
        .filter_map(|line| line.trim().parse::<i64>().ok())
        .collect();
    module_masses
}

fn calculate_fuel<'a>(module_masses: impl Iterator<Item = &'a i64>) -> i64 {
    module_masses
        .map(|&mass| {
            let fuel = calculate_single_module_fuel(mass);
            calculate_transitive_fuel(fuel)
        })
        .sum()
}

fn calculate_single_module_fuel(module_mass: i64) -> i64 {
    ((module_mass as f64 / 3.0).floor() as i64) - 2
}

fn calculate_transitive_fuel(fuel_mass: i64) -> i64 {
    let mut sum = fuel_mass;
    let mut new_fuel = fuel_mass;
    loop {
        new_fuel = calculate_single_module_fuel(new_fuel);
        if new_fuel <= 0 {
            break;
        }

        sum = sum + new_fuel;
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_fuel_empty() {
        let empty_module_list = vec![];
        assert_eq!(calculate_fuel(empty_module_list.iter()), 0);
    }

    #[test]
    fn test_calculate_fuel_example_1() {
        let sample_module_mass = vec![12];
        assert_eq!(calculate_fuel(sample_module_mass.iter()), 2);
    }

    #[test]
    fn test_calculate_fuel_example_2() {
        let sample_module_mass = vec![14];
        assert_eq!(calculate_fuel(sample_module_mass.iter()), 2);
    }

    #[test]
    fn test_calculate_fuel_example_3() {
        let sample_module_mass = vec![1969];
        assert_eq!(calculate_fuel(sample_module_mass.iter()), 654);
    }

    #[test]
    fn test_calculate_fuel_example_4() {
        let sample_module_mass = vec![100756];
        assert_eq!(calculate_fuel(sample_module_mass.iter()), 33583);
    }

    #[test]
    fn test_calculate_transitive_fuel() {
        assert_eq!(calculate_transitive_fuel(654), 966);
    }

    #[test]
    fn test_calculate_transitive_fuel_2() {
        assert_eq!(calculate_transitive_fuel(33583), 50346);
    }
}
