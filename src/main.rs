use std::process::{exit, Command};
use std::{env, fs};

fn split_linewise(buffer: String, is_debug: bool) -> Vec<String> {
    let mut lines: Vec<String> = Vec::new();
    let mut line = String::new();
    let mut inside_quote = false;

    for c in buffer.chars() {
        if c == '"' {
            inside_quote = !inside_quote;
            line.push('"');
        } else if c == '!' && !inside_quote {
            line = line.trim().to_string();
            lines.push(line);
            line = String::new();
        } else {
            line.push(c);
        }
    }
    if line.trim().len() > 0 {
        lines.push(line.trim().to_string());
    }

    if is_debug {
        println!("{lines:#?}");
    }
    lines
}

fn parse(line: String, is_debug: bool) -> Vec<String> {
    let mut token = String::new();
    let mut tokens: Vec<String> = Vec::new();
    let mut inside_quote = false;

    for c in line.chars() {
        if c == '"' {
            inside_quote = !inside_quote;
        } else if c == ' ' && !inside_quote {
            if token.len() > 0 {
                tokens.push(token);
            }
            token = String::new();
        } else {
            token.push(c);
        }
    }
    tokens.push(token);

    if is_debug {
        println!("{tokens:?}");
    }
    tokens
}

fn to_python(tokens: Vec<String>, line_no: i32) -> String {
    if tokens[0] != "jon," {
        println!("{line_no} :: lines should start with `jon,`");
        exit(1);
    }
    match tokens[1].as_str() {
        "say" => {
            if tokens.len() == 3 {
                return format!("print('{}')\n", tokens[2]);
            } else if tokens.len() > 3 {
                if tokens[3] != "and" || tokens[4] != "read" {
                    println!("{line_no} :: jon did not understand this line!");
                    exit(1);
                } else {
                    match tokens[5].as_str() {
                        "into" => return format!("{} = eval(input('{}'))\n", tokens[6], tokens[2]),
                        "aloud" => return format!("print('{}', {})\n", tokens[2], tokens[6]),
                        _ => {
                            println!("{line_no} :: jon did not understand this line!");
                            exit(1);
                        }
                    }
                }
            } else {
                String::new()
            }
        }

        "remember" => {
            if tokens[2] != "that" {
                println!("{line_no} :: jon did not understand this line | did you mean `remember that` ?");
                exit(1);
            } else {
                match tokens[4].as_str() {
                    "is" => return format!("{} = {}\n", tokens[3], tokens[5]),
                    "will" => {
                        if tokens[5] != "be" {
                            println!("{line_no} :: jon did not understand this line | did you mean `will be` ?");
                            exit(1);
                        } else {
                            let mut operator = "";
                            match tokens[7].as_str() {
                                "plus" => operator = "+",
                                "minus" => operator = "-",
                                "times" => operator = "*",
                                "by" => operator = "/",
                                _ => {
                                    println!("{line_no} :: jon did not understand this line!");
                                    exit(1);
                                }
                            }

                            return format!(
                                "{} = {} {} {}\n",
                                tokens[3], tokens[6], operator, tokens[8]
                            );
                        }
                    }
                    _ => {
                        println!("{line_no} :: jon did not understand this line!");
                        exit(1);
                    }
                }
            }
        }
        _ => {
            println!("{line_no} :: jon did not understand this line!");
            exit(1);
        }
    }
}
fn main() {
    let mut is_jon_awake = false;
    let mut is_debug = false;

    let args = env::args().skip(1).collect::<Vec<String>>();
    for arg in &args {
        let mut tokens_list: Vec<Vec<String>> = Vec::new();
        if args.contains(&String::from("--debug")) {
            is_debug = true;
        }
        let buffer =
            fs::read_to_string(format!("./{arg}")).expect(&format!("./{arg} does not exist..."));
        let lines = split_linewise(buffer, is_debug);

        for line in lines {
            if line == "hi, jon" {
                is_jon_awake = true;
            } else if line == "bye, jon" {
                is_jon_awake = false;
            } else if is_jon_awake {
                tokens_list.push(parse(line, is_debug));
            }
        }

        let mut line_no = 1;
        let mut out: Vec<String> = Vec::new();
        for tokens in tokens_list {
            let mut line = to_python(tokens, line_no);
            if is_debug {
                println!("{}", line);
            }
            out.push(line);
            line_no += 1;
        }
        let mut program = String::new();
        for line in out {
            program.push_str(line.as_str());
        }
        println!("{program}");
        fs::write("./output.py", program).expect("failed to write temp file ");
        Command::new("python3")
            .arg("./output.py")
            .spawn()
            .expect("failed to run ");
    }
}
