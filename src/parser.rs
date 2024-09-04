enum Keywords {
    Updraft(i32, i32),
    Suction(i32, i32),
    Twister(i32, i32),
    Churn(i32, i32),
    Storm,
    Air,
    Gyre,
    Whirl(bool),
    Swirl,
    Typhoon,
    Funnel,
}

fn parse() {
    //TODO: parse input to good format
    let mut function_list: Vec<String> = vec![];

    match p {
        Keywords::Air => function_list.push(todo!()),

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

        Keywords::Storm => todo!(),

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

        Keywords::Funnel => todo!(),
    }
}
