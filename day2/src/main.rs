fn load_data() -> Vec::<Vec<i64>>
{
    const FILE_NAME : &str = "day2.txt";
    let file_contents = std::fs::read_to_string(FILE_NAME).unwrap();
    let mut data = Vec::<Vec<i64>>::new();
    file_contents.lines().for_each(|line|{
        data.push(Vec::new());
        line.split_whitespace().into_iter().for_each(|line_item|{
            data.last_mut().unwrap().push(i64::from_str_radix(line_item, 10).unwrap());
        });
    });
    data
}

fn is_record_safe(record: &Vec<i64>) -> bool
{
    let mut increasing = true;
    for i in 0..record.len()
    {
        if i == 0
        {
            continue;
        }

        if record[i] == record[i - 1]
        {
            return false;
        }

        let abs_diff = record[i].abs_diff(record[i - 1]);
        if abs_diff < 1 || abs_diff > 3
        {
            return false;
        }

        if i == 1
        {
            increasing = record[i] > record[i - 1];
            continue;
        }

        if increasing != (record[i] > record[i - 1])
        {
            return false;
        }
    }

    true
}

fn day_2_part_1()
{
    let data = load_data();
    let mut num_safe_records = 0;
    for record in &data
    {
        if is_record_safe(&record)
        {
            num_safe_records += 1;
        }
    }
    println!("answer2.1={}", num_safe_records);
}

fn day_2_part_2()

{
    let data = load_data();
    let mut num_safe_records = 0;
    for record in &data
    {
        if is_record_safe(&record)
        {
            num_safe_records += 1;
        }
        else
        {
            // Iterate over all combinations of 1 element removed
            // Until we get a safe record
            for removed_item_index in 0..record.len()
            {
                if is_record_safe(
                    &record.iter().enumerate().filter_map(|(index, item)|{
                        if index != removed_item_index 
                        {
                            Some(*item) 
                        }else{
                            None 
                        } 
                    }).collect())
                {
                    num_safe_records += 1;
                    break;
                }
            }
        }
    }
    println!("answer2.2={}", num_safe_records);
}

fn main() {
    day_2_part_1();
    day_2_part_2();
    
}
