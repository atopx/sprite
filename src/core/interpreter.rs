use super::instruct::Instruct;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use tracing::{debug, error, trace};

pub const INSTRUCT_POS: &str = "pos";
pub const INSTRUCT_MOVE: &str = "move";
pub const INSTRUCT_MOUSE: &str = "mouse";
pub const INSTRUCT_SLEEP: &str = "sleep";
pub const INSTRUCT_LOOP_START: &str = "loop-start";
pub const INSTRUCT_LOOP_END: &str = "loop-end";

pub struct Interpreter {
    variables: HashMap<String, (i32, i32)>,
    pub instructs: Vec<Instruct>,
}

const COMMENT_START: &str = "**";
const DEFAULT_MOUSE_CLICK_COUNT: usize = 1;

impl Interpreter {
    pub fn new() -> Self {
        Interpreter { variables: HashMap::new(), instructs: Vec::new() }
    }

    pub fn parse_line(
        &mut self,
        line: &str,
        line_number: usize,
        loop_stack: &mut Vec<Instruct>,
    ) -> Result<(), String> {
        let original_line = line.trim().to_string();

        if line.is_empty() || original_line.starts_with(COMMENT_START) {
            return Ok(());
        }

        let line = match original_line.find(COMMENT_START) {
            Some(index) => &original_line[..index],
            None => &original_line,
        };

        let parts: Vec<&str> = line.split_whitespace().collect();
        let args = &parts[1..];

        trace!("parse line {}: {}", line_number, original_line);

        match parts[0] {
            INSTRUCT_POS => {
                if args.len() != 3 {
                    return Err(format!("line {} - ValueError: [pos] requires 3 args", line_number));
                }
                let x = args[1]
                    .parse::<i32>()
                    .map_err(|_| format!("line {} - ValueError: [pos] args must be int", line_number))?;
                let y = args[2]
                    .parse::<i32>()
                    .map_err(|_| format!("line {} - ValueError: [pos] args must be int", line_number))?;
                self.variables.insert(args[0].to_string(), (x, y));
                debug!("line {}: Parsed instruct pos {} {} {}", line_number, args[0], x, y);
            }
            INSTRUCT_LOOP_START => {
                if args.len() != 1 {
                    return Err(format!(
                        "line {} - SyntaxError: [loop-start] requires 1 args",
                        line_number
                    ));
                }
                let loop_count = args[0].parse::<usize>().map_err(|_| {
                    format!("line {} - ValueError: [loop-start] args must be int", line_number)
                })?;
                debug!("line {}: Parsed instruct loop-start {}", line_number, loop_count);
                loop_stack.push(Instruct::Loop(loop_count, Vec::new()));
            }
            INSTRUCT_LOOP_END => {
                debug!("line {}: Parsed instruct loop-end", line_number);
                if let Some(Instruct::Loop(count, instructions)) = loop_stack.pop() {
                    if let Some(Instruct::Loop(_, ref mut parent_instructions)) = loop_stack.last_mut() {
                        parent_instructions.push(Instruct::Loop(count, instructions));
                    } else {
                        self.instructs.push(Instruct::Loop(count, instructions));
                    }
                } else {
                    return Err(format!("line {} - SyntaxError: missing [loop-start].", line_number));
                }
            }
            INSTRUCT_MOVE => match args.len() {
                1 => {
                    let (x, y) = self.variables.get(args[0]).ok_or_else(|| {
                        format!("line {} - ValueError: variable not defined {}", line_number, args[0])
                    })?;
                    debug!("line {}: Parsed instruct [move] {} ({} {})", line_number, args[0], x, y);
                    self.instructs.push(Instruct::Move(*x, *y));
                }
                2 => {
                    let x = args[0].parse::<i32>().map_err(|_| {
                        format!("line {} - ValueError: [move] args must be int", line_number)
                    })?;
                    let y = args[1].parse::<i32>().map_err(|_| {
                        format!("line {} - ValueError: [move] args must be int", line_number)
                    })?;
                    debug!("line {}: Parsed instruct [move] {} {}", line_number, x, y);
                    self.instructs.push(Instruct::Move(x, y));
                }
                _ => return Err(format!("line {} - ValueError: [move] pos or (x, y)", line_number)),
            },
            INSTRUCT_MOUSE => {
                let button = args[0].to_string();
                let count = if args.len() == 2 {
                    args[1].parse::<usize>().map_err(|_| {
                        format!("line {} - ValueError: [mouse] args must be int", line_number)
                    })?
                } else {
                    DEFAULT_MOUSE_CLICK_COUNT
                };
                debug!("line {}: Parsed instruct [mouse] {} {}", line_number, button, count);
                self.instructs.push(Instruct::Mouse(button, count));
            }
            INSTRUCT_SLEEP => {
                if args.len() != 1 {
                    return Err(format!("line {} - SyntaxError: [sleep] missing 1 args", line_number));
                }
                let duration = args[0]
                    .parse::<usize>()
                    .map_err(|_| format!("line {} - ValueError: [sleep] must be int", line_number))?;
                debug!("line {}: Parsed instruct [sleep] {}", line_number, duration);
                self.instructs.push(Instruct::Sleep(duration));
            }
            _ => {
                return Err(format!("line {} - SyntaxError: unknown instruct", line_number));
            }
        }

        Ok(())
    }

    pub fn parse_script(&mut self, filename: &str) -> Result<(), io::Error> {
        let file = File::open(filename)?;
        let reader = io::BufReader::new(file);
        let mut loop_stack = Vec::new();

        for (line_number, line) in reader.lines().enumerate() {
            let line = line?;
            if let Err(err) = self.parse_line(&line, line_number + 1, &mut loop_stack) {
                error!(err);
                std::process::exit(1);
            }
        }

        if !loop_stack.is_empty() {
            error!("SyntaxError: missing context [loop-end]");
            std::process::exit(1);
        }

        Ok(())
    }

    pub fn execute(&mut self) {
        for ins in &self.instructs {
            ins.execute()
        }
    }
}
