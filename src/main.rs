use std::fs::read_to_string;

fn main() {
    let numbers: [&str;9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut sum: u32 = 0;

    for line in read_to_string("./src/input.txt").unwrap().lines() {
        // println!("{}",line);
        sum = process_line(line,numbers,sum);
    }
    println!("total: {}",sum);
}

fn process_line(line: &str,numbers: [&str;9], mut sum: u32) -> u32 {
    let mut l3: String = String::new();
    let mut l4: String = String::new();
    let mut l5: String = String::new();
    let mut first_digit: Option<u16> = None;
    let mut last_digit: Option<u16> = None;
    let mut first: bool = true;
    for c in line.chars() {
        let is_number: Result<u16, _> = c.to_string().parse();
        match is_number {
            Ok(number) => {
                if first {
                    first_digit = Some(number);
                    first = false;
                }
                last_digit = Some(number);
                l3 = String::new();
                l4 = String::new();
                l5 = String::new();
            },
            Err(_err) => {
                l3 = format!("{}{}",l3,c);
                if l3.len() > 3 {
                    l3.remove(0);
                }
                l4 = format!("{}{}",l4,c);
                if l4.len() > 4 {
                    l4.remove(0);
                }
                l5 = format!("{}{}",l5,c);
                if l5.len() > 5 {
                    l5.remove(0);
                }
                if l3.len() == 3 {
                    for (key,number) in numbers.iter().enumerate() {
                        if number.len() == 3 && number.to_string() == l3 {
                            if first {
                                first = false;
                                first_digit = Some((key + 1).try_into().unwrap_or(0));
                            }
                            last_digit = Some((key + 1).try_into().unwrap_or(0));
                        }
                    }
                }
                if l4.len() == 4 {
                    for (key,number) in numbers.iter().enumerate() {
                        if number.len() == 4 && number.to_string() == l4 {
                            if first {
                                first = false;
                                first_digit = Some((key + 1).try_into().unwrap_or(0));
                            }
                            last_digit = Some((key + 1).try_into().unwrap_or(0));
                        }
                    }
                }
                if l5.len() == 5 {
                    for (key,number) in numbers.iter().enumerate() {
                        if number.len() == 5 && number.to_string() == l5 {
                            if first {
                                first = false;
                                first_digit = Some((key + 1).try_into().unwrap_or(0));
                            }
                            last_digit = Some((key + 1).try_into().unwrap_or(0));
                        }
                    }
                }
            }
        }
    }
    if let (Some(first_digit),Some(last_digit)) = (first_digit,last_digit) {
        let sumword = format!("{}{}",first_digit,last_digit).parse().unwrap_or(0);
        sum = sum + sumword;
        // println!("result of the line {}", sumword);
    }
    sum
}
