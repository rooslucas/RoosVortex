use std::{collections::HashMap, str::FromStr};

#[derive(Debug, PartialEq)]
enum Keywords {
    Updraft(i32, i32),
    Suction(i32, i32),
    Twister(i32, i32),
    Churn(i32, i32),
    Storm(String, i32),
    Air,
    Gyre,
    Whirl(bool),
    Swirl,
    Typhoon,
    Funnel(i32),
    I32(i32),
}

impl FromStr for Keywords {
    type Err = ();

    fn from_str(input: &str) -> Result<Keywords, Self::Err> {
        let function_line: Vec<&str> = input.split_whitespace().collect();
        let mut value1 = 0;
        let mut value2 = 0;
        let mut variable_name: String = "".to_owned();

        if (function_line.len() > 1) & (function_line[0] == "Storm") {
            variable_name = function_line[1].to_owned();
            value1 = function_line[2].parse::<i32>().unwrap();
        } else if (function_line.len() > 1) & (function_line[0] == "Updraf") {
            value1 = function_line[1].parse::<i32>().unwrap();
            value2 = function_line[2].parse::<i32>().unwrap();
        }
        // Disect value
        // If length .0 > 1
        // Check for storm
        // if storm variable name = .0.1
        // find air value after is value

        match &function {
            "Storm" => Ok(Keywords::Storm(todo!(), todo!())),
            "Updraft" => Ok(Keywords::Updraft(todo!(), todo!())),
            "Suction" => Ok(Keywords::Suction(todo!(), todo!())),
            "Twister" => Ok(Keywords::Twister(todo!(), todo!())),
            "Churn" => Ok(Keywords::Churn(todo!(), todo!())),
            i32 => return I32(&function.parse::<i32>().unwrap()),
            _ => Err(()),
        }
    }
}

fn parse() {
    //TODO: parse input to good format
    let mut function_list: Vec<String> = vec![];
    let mut heap = HashMap::new();

    match p {
        Keywords::Air => {
            todo!()
        }

        Keywords::Updraft(value1, value2) => {
            todo!(); // find value1 on stack
            todo!(); // find value2 on stack
            function_list.push("ADD".to_owned());
        }

        Keywords::Suction(value1, value2) => {
            todo!(); // find value1 on stack
            todo!(); // find value2 on stack
            function_list.push("SUB".to_owned());
        }

        Keywords::Twister(value1, value2) => {
            todo!(); // find value1 on stack
            todo!(); // find value2 on stack
            function_list.push("MUL".to_owned());
        }

        Keywords::Churn(value1, value2) => {
            todo!(); // find value1 on stack
            todo!(); // find value2 on stack
            function_list.push("DIV".to_owned());
        }

        Keywords::Storm(variable, value) => {
            heap.insert(variable, value);
        }

        Keywords::Gyre => todo!(),

        Keywords::Whirl(statement) => {
            todo!(); // find value1 on stack
            todo!(); // find value2 on stack
            function_list.push("IF ".to_owned() + &statement.to_string());
        }

        Keywords::Swirl => {
            todo!(); // find value1 on stack
            todo!(); // find value2 on stack
            function_list.push("ELSE".to_owned());
        }

        Keywords::Typhoon => todo!(),

        Keywords::Funnel(value) => todo!(),

        Keywords::I32(value) => todo!(),
    }
}
