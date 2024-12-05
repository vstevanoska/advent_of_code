use std::fs;

fn main() {
    let path = String::from("input.txt");

    let mut counter: i32 = 0;

    for line in fs::read_to_string(path).unwrap().lines() {

        let temp: Vec<&str> = line.split_whitespace().collect();

        let mut ascending: bool = false;

        if temp[0].parse::<i32>().unwrap() < temp[1].parse::<i32>().unwrap() {
            ascending = true;
        } else if temp[0].parse::<i32>().unwrap() == temp[1].parse::<i32>().unwrap() {

            continue;
        }

        let mut cont: bool = false;

        for (index, value) in temp.iter().enumerate() {
            
            if index + 1 < temp.len() {
                if (ascending && value.parse::<i32>().unwrap() >= temp[index + 1].parse::<i32>().unwrap()) || 
                (!ascending && value.parse::<i32>().unwrap() <= temp[index + 1].parse::<i32>().unwrap()) {
                    cont = true;
                    break;
                } 

                if (i32::abs(value.parse::<i32>().unwrap() - temp[index + 1].parse::<i32>().unwrap()) < 1) || 
                (i32::abs(value.parse::<i32>().unwrap() - temp[index + 1].parse::<i32>().unwrap()) > 3) {
                    cont = true;
                    break;
                }
            }
        }

        if cont {
            continue;
        }

        counter += 1;
    }

    println!("{} reports are safe!", counter);
}
