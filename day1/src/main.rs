use std::collections::HashMap;
use std::{fs, i64};
use std::cmp::min;


fn load_data() -> Vec::<Vec<i64>>
{
    const FILE_NAME : &str = "day1.txt";
    let file_contents = fs::read_to_string(FILE_NAME).unwrap();
    let mut data = Vec::<Vec<i64>>::new();
    file_contents.lines().for_each(|line|{
        data.push(Vec::new());
        line.split_whitespace().into_iter().for_each(|line_item|{
            data.last_mut().unwrap().push(i64::from_str_radix(line_item, 10).unwrap());
        });
    });
    data
}

fn day_1()
{
    let (mut list1, mut list2) : (Vec<i64>, Vec<i64>) = load_data().iter().map(|row| {
        (row[0], row[1])
    }).unzip();

    list1.sort();
    list2.sort();

    let mut sum = 0;
    for i in 0..min(list1.len(), list2.len())
    {
        sum += list1[i].abs_diff(list2[i]);
    }
    println!("==============\n# Day 1\nanswer={}", sum);
}

fn day_1_part_2()
{
    let (list1, list2) : (Vec<i64>, Vec<i64>) = load_data().iter().map(|row| {
        (row[0], row[1])
    }).unzip();

    let mut list2_item_counts = HashMap::new();
    for item in &list2
    {
        list2_item_counts.entry(item).and_modify(|val|{*val += 1}).or_insert(1);
    }

    let mut sum = 0;
    for item in &list1
    {
        let count = list2_item_counts.get(&item).unwrap_or(&0);
        sum += item * count;
    }
    println!("==============\n# Day 1.2\nanswer={}", sum);
}

fn main() {
    day_1();
    day_1_part_2();
}
