use std::io::{stdin, BufRead};

#[derive(Debug)]
struct Passport {
    byr: Option<usize>,  // (Birth Year)
    iyr: Option<usize>,  // (Issue Year)
    eyr: Option<usize>,  // (Expiration Year)
    hgt: Option<String>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<usize>, // (Passport ID)
    cid: Option<usize>,  // (Country ID)
    had_bad_fields: bool,
}

impl Passport {
    fn new() -> Passport {
        Passport {
            byr: None,
            iyr: None,
            eyr: None,
            hgt: None,
            hcl: None,
            ecl: None,
            pid: None,
            cid: None,
            had_bad_fields: false,
        }
    }
}

fn main() {
    let (last_ppt, mut ppt_list) = stdin().lock().lines().filter_map(|s| s.ok()).fold(
        (Passport::new(), Vec::<Passport>::new()),
        |(mut ppt, mut acc_vec), line| {
            if line.is_empty() {
                acc_vec.push(ppt);
                (Passport::new(), acc_vec)
            } else {
                line.split_ascii_whitespace()
                    .filter_map(|s| Some(s.split_at(s.find(':')? + 1)))
                    .for_each(|(field, value)| match field {
                        "byr:" => ppt.byr = usize::from_str_radix(value, 10).ok(),
                        "iyr:" => ppt.iyr = usize::from_str_radix(value, 10).ok(),
                        "eyr:" => ppt.eyr = usize::from_str_radix(value, 10).ok(),
                        "hgt:" => ppt.hgt = Some(String::from(value)),
                        "hcl:" => ppt.hcl = Some(String::from(value)),
                        "ecl:" => ppt.ecl = Some(String::from(value)),
                        "pid:" => ppt.pid = usize::from_str_radix(value, 10).ok(),
                        "cid:" => ppt.cid = usize::from_str_radix(value, 10).ok(),
                        _ => ppt.had_bad_fields = true,
                    });
                (ppt, acc_vec)
            }
        },
    );

    ppt_list.push(last_ppt);

    let valid_count = ppt_list
        .into_iter()
        .filter(|ppt| {
            ppt.byr.is_some()
                && ppt.iyr.is_some()
                && ppt.eyr.is_some()
                && ppt.hgt.is_some()
                && ppt.hcl.is_some()
                && ppt.ecl.is_some()
                && ppt.pid.is_some()
                && !ppt.had_bad_fields
        })
        .count();

    println!("Valid number of passports = {}", valid_count);
}
