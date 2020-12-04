use std::io::{stdin, BufRead};

#[derive(Debug)]
enum Height {
    IN(usize),
    CM(usize),
}

#[derive(Debug)]
struct Passport {
    byr: Option<usize>,  // (Birth Year)
    iyr: Option<usize>,  // (Issue Year)
    eyr: Option<usize>,  // (Expiration Year)
    hgt: Option<Height>, // (Height)
    hcl: Option<String>, // (Hair Color)
    ecl: Option<String>, // (Eye Color)
    pid: Option<usize>,  // (Passport ID)
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
                        "byr:" => {
                            ppt.byr = usize::from_str_radix(value, 10)
                                .ok()
                                .filter(|byr| *byr >= 1920 && *byr <= 2002)
                        }
                        "iyr:" => {
                            ppt.iyr = usize::from_str_radix(value, 10)
                                .ok()
                                .filter(|iyr| *iyr >= 2010 && *iyr <= 2020)
                        }
                        "eyr:" => {
                            ppt.eyr = usize::from_str_radix(value, 10)
                                .ok()
                                .filter(|eyr| *eyr >= 2020 && *eyr <= 2030)
                        }
                        "hgt:" => {
                            ppt.hgt = {
                                if value.ends_with("cm") {
                                    usize::from_str_radix(value.split_at(value.len() - 2).0, 10)
                                        .ok()
                                        .filter(|hgt| *hgt >= 150 && *hgt <= 193)
                                        .map(|hgt| Height::CM(hgt))
                                } else if value.ends_with("in") {
                                    usize::from_str_radix(value.split_at(value.len() - 2).0, 10)
                                        .ok()
                                        .filter(|hgt| *hgt >= 59 && *hgt <= 76)
                                        .map(|hgt| Height::IN(hgt))
                                } else {
                                    None
                                }
                            }
                        }
                        "hcl:" => {
                            ppt.hcl = {
                                if value.starts_with('#') {
                                    let color_code = value.split_at(1).1;
                                    if color_code
                                        .chars()
                                        .all(|ch| ch.is_ascii_digit() || (ch >= 'a' && ch <= 'f'))
                                    {
                                        Some(String::from(value))
                                    } else {
                                        None
                                    }
                                } else {
                                    None
                                }
                            }
                        }
                        "ecl:" => {
                            ppt.ecl = match value {
                                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => {
                                    Some(String::from(value))
                                }
                                _ => None,
                            }
                        }
                        "pid:" => {
                            ppt.pid = if value.len() == 9 {
                                usize::from_str_radix(value, 10).ok()
                            } else {
                                None
                            }
                        }
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
            println!("Validating: {:?}", ppt);
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
