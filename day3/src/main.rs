use std::i64;

fn load_data() -> String
{
    const FILE_NAME : &str = "day3.txt";
    std::fs::read_to_string(FILE_NAME).unwrap()
}

fn calculate(data: &str) -> i64
{
    let mut answer = 0;
    let re = regex::Regex::new(r"mul\((?<num1>\d+),(?<num2>\d+)\)").unwrap();
    for caps in re.captures_iter(&data)
    {
        answer += i64::from_str_radix(&caps["num1"], 10).unwrap() * i64::from_str_radix(&caps["num2"], 10).unwrap();
    }
    answer
}

fn day3_part1()
{
    let data = load_data();
    println!("answer={}", calculate(&data));
}

#[derive(PartialEq)]
enum Command
{
    Do,
    Dont,
}

fn day3_part2()
{
    let mut data = load_data();
    const DO_STR : &str = "do()";
    const DONT_STR : &str = "don't()";
    data = DO_STR.to_owned() + &data + DONT_STR;
    let mut data_to_process : Vec<&str> = Vec::new();
    let mut start_index = 0;
    let mut commands = Vec::new();
    while let Some(end_index) = data[start_index..].find(DO_STR)
    {
        let index_in_orig_str = start_index + end_index + DO_STR.len();
        commands.push((index_in_orig_str, Command::Do));
        start_index = index_in_orig_str;
        if start_index >= data.len()
        {
            break;
        }
    }

    start_index = 0;
    while let Some(end_index) = data[start_index..].find(DONT_STR)
    {
        let index_in_orig_str = start_index + end_index + DONT_STR.len();
        commands.push((index_in_orig_str, Command::Dont));
        start_index = index_in_orig_str;
        if start_index >= data.len()
        {
            break;
        }
    }

    commands.sort_by(|cmd1, cmd2| {
        cmd1.0.cmp(&cmd2.0)
    } );


    let mut last_do_cmd_index = None;
    for (i, cmd) in commands.iter().enumerate()
    {
        if cmd.1 == Command::Do
        {
            if last_do_cmd_index.is_none() || commands[i - 1].1 != Command::Do
            {
                last_do_cmd_index = Some(cmd.0);
            }
            continue;
        }

        // don't command
        if commands[i - 1].1 != Command::Dont
        {
            data_to_process.push(&data[last_do_cmd_index.unwrap()..cmd.0 - DONT_STR.len()]);
        }
    }

    let mut answer = 0;
    for data in &data_to_process
    {
        answer += calculate(&data);
    }
    println!("answer={}", answer);
}

fn main() {
    day3_part1();
    day3_part2();
}
