use std::fs;
use std::collections::HashMap;

fn main() {

    let path = String::from("input.txt");

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();

    for line in fs::read_to_string(path).unwrap().lines() {

        let temp: Vec<&str> = line.split_whitespace().collect();

        list1.push(temp[0].parse().unwrap());
        list2.push(temp[1].parse().unwrap());
    }

    list1.sort();
    list2.sort();

    let mut sum: i32 = 0;

    for (index, value) in list1.iter().enumerate() {
        sum += i32::abs(value - list2[index]);
    }

    println!("The sum of all paths is: {}", sum);
    

    //second part: calculating similarity score

    let mut scores: HashMap<i32, i32> = HashMap::new();

    for value1 in list1.iter() {
        for value2 in list2.iter() {

            if *value1 == *value2 {
                let count = scores.entry(*value1).or_insert(0);
                *count += 1;
            }
        }
    }

    let mut similarity_score : i32 = 0;

    for (key, value) in &scores {
        similarity_score += key * value;
    }

    println!("The similarity score is: {}", similarity_score);

}
