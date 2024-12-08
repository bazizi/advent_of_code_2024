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

fn main() {
    let data = load_data();
    let mut num_safe_records = 0;
    for record in &data
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
                break;
            }
            
            let abs_diff = record[i].abs_diff(record[i - 1]);
            if abs_diff < 1 || abs_diff > 3
            {
                break;
            }

            if i == 1
            {
                increasing = record[i] > record[i - 1];
                continue;
            }

            if increasing != (record[i] > record[i - 1])
            {
                break;
            }

            if i == record.len() - 1
            {
                num_safe_records += 1;
            }
        }
    }
    println!("answer={}", num_safe_records);
}
