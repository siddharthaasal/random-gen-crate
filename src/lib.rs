use rand::seq::SliceRandom;
use rand::Rng;
use chrono::{NaiveDate, Duration};
use rand::prelude::IndexedRandom;
use chrono::Datelike;

/// Generates a random first name.
pub fn random_first_name() -> String {
    let names = ["Alice", "Bob", "Charlie", "Diana", "Eve", "Frank", "Grace", "Hank"];
    let mut rng = rand::thread_rng();
    names.choose(&mut rng).unwrap().to_string()
}

/// Generates a random last name.
pub fn random_last_name() -> String {
    let names = ["Smith", "Johnson", "Williams", "Brown", "Jones", "Garcia", "Miller", "Davis"];
    let mut rng = rand::thread_rng();
    names.choose(&mut rng).unwrap().to_string()
}

/// Generates a random full name.
pub fn random_full_name() -> String {
    format!("{} {}", random_first_name(), random_last_name())
}

/// Generates a random email address.
pub fn random_email() -> String {
    let domains = ["example.com", "test.com", "demo.com", "sample.org"];
    let mut rng = rand::thread_rng();
    let username = random_first_name().to_lowercase();
    let domain = domains.choose(&mut rng).unwrap();
    format!("{}@{}", username, domain)
}

/// Generates a random phone number in the format (XXX) XXX-XXXX.
pub fn random_phone_number() -> String {
    let mut rng = rand::thread_rng();
    let area_code = rng.random_range(100..=999);
    let exchange = rng.random_range(100..=999);
    let subscriber = rng.random_range(1000..=9999);
    format!("({}) {}-{}", area_code, exchange, subscriber)
}

/// Generates a random date between two years.
pub fn random_date(start_year: i32, end_year: i32) -> NaiveDate {
    // if(start_year > end_year){
    //     return Err("start_year must be less than or equal to end_year");
    // }
    let mut rng = rand::thread_rng();
    let start = NaiveDate::from_ymd_opt(start_year, 1, 1).unwrap();
    let end = NaiveDate::from_ymd_opt(end_year, 12, 31).unwrap();
    let days = (end - start).num_days();
    start + Duration::days(rng.random_range(0..=days))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_first_name() {
        let name = random_first_name();
        assert!(!name.is_empty());
    }

    #[test]
    fn test_random_last_name() {
        let name = random_last_name();
        assert!(!name.is_empty());
    }

    #[test]
    fn test_random_full_name() {
        let name = random_full_name();
        assert!(name.contains(' '));
    }

    #[test]
    fn test_random_email() {
        let email = random_email();
        assert!(email.contains('@'));
    }

    #[test]
    fn test_random_phone_number() {
        let phone = random_phone_number();
        assert_eq!(phone.len(), 14); // Format: (XXX) XXX-XXXX
    }

    #[test]
    fn test_random_date() {
        let date = random_date(2023, 2022);
        assert!(date.year() >= 2000 && date.year() <= 2024);
    }
}