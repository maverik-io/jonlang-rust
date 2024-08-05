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
                return format!("println!(\"{}\");\n", tokens[2]);
            } else if tokens.len() > 3 {
                if tokens[3] != "and" || tokens[4] != "read" {
                    println!("{line_no} :: jon did not understand this line!");
                    exit(1);
                } else {
                    match tokens[5].as_str() {
                        "into" => {
                            if tokens.len() < 8 || tokens[7] != "as" {
                                println!("{line_no} :: jon did not understand this line!");
                                exit(1);
                            } else {
                                match tokens[8].as_str() {
                                    "number" => {
                                        return format!(
                                            "let {} = get_number(\"{}\", {line_no});",
                                            tokens[6], tokens[2]
                                        )
                                    }
                                    "text" => {
                                        return format!(
                                            "let {} = get_text(\"{}\", {line_no});",
                                            tokens[6], tokens[2]
                                        )
                                    }
                                    _ => {
                                        println!("{line_no} :: jon did not understand this line!");
                                        exit(1);
                                    }
                                }
                            }
                        }
                        "aloud" => {
                            return format!("println!(\"{} {{}}\", {})\n", tokens[2], tokens[6])
                        }
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
                    "is" => return format!("let {} = {};\n", tokens[3], tokens[5]),
                    "will" => {
                        if tokens[5] != "be" {
                            println!("{line_no} :: jon did not understand this line | did you mean `will be` ?");
                            exit(1);
                        } else {
                            let operator: &str;
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
                                "let {} = {} {} {};\n",
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
    if args.contains(&String::from("--debug")) {
        is_debug = true;
    }

    for arg in &args {
        if arg == "--debug" {
            continue;
        }
        let mut tokens_list: Vec<Vec<String>> = Vec::new();
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
            let line = to_python(tokens, line_no);
            if is_debug {
                println!("{}", line);
            }
            out.push(line);
            line_no += 1;
        }
        let mut program = String::from(
            "
#![allow(dead_code)]
use std::io::{self, Write};
enum StringOrFloat {
    IsString(String),
    IsFloat(f32),
}
fn input(prompt: &str, typ: &str, line_no: u32) -> StringOrFloat {
    print!(\"{prompt}\");
    io::stdout().flush().unwrap();
    let mut out = String::new();
    io::stdin().read_line(&mut out).unwrap();
    let out = out.trim().to_string();
    match typ {
        \"number\" => match out.parse::<f32>() {
            Ok(val) => return StringOrFloat::IsFloat(val),
            Err(_) => {
                println!(\"{line_no} :: jon could not parse this!\");
                std::process::exit(1)
            }
        },
        \"text\" => return StringOrFloat::IsString(out),
        _ => {
            println!(\"{line_no} :: jon did not understand this line!\");
            std::process::exit(1);
        }
    }
}

fn get_text(prompt: &str, line_no: u32) -> String {
    match input(prompt,\"text\", line_no) {
        StringOrFloat::IsString(val) => return val,
        StringOrFloat::IsFloat(_) => {
            println!(\"That's not supposed to happen\");
            std::process::exit(1);
        }
    }
}

fn get_number(prompt: &str, line_no: u32) -> f32 {
    match input(prompt,\"number\", line_no) {
        StringOrFloat::IsFloat(val) => return val,
        StringOrFloat::IsString(_) => {
            println!(\"That's not supposed to happen\");
            std::process::exit(1);
        }
    }
}

fn main() {

",
        );
        for line in out {
            program.push_str(line.as_str());
        }
        program.push('}');
        if is_debug {
            println!("{program}");
        }
        let name = arg.split('/').last().unwrap().split('.').next().unwrap();
        fs::write(format!("./{name}.rs"), program).expect("failed to write temp file ");
        Command::new("rustc")
            .arg(format!("./{name}.rs"))
            .spawn()
            .expect("jon faliled to compile")
            .wait()
            .unwrap();

        Command::new("rm")
            .arg(format!("./{name}.rs"))
            .spawn()
            .expect("jon failed to delete tempfile!");
    }
}
