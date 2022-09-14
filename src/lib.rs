#![deny(warnings, missing_docs)]
#![doc = include_str!("../README.md")]

use regex::Regex;
use std::collections::HashMap;

const SPLIT_PAT: &str = "||";
const VAR_PAT: &str = r"__\w*";

/// Parses and evaluates simple math expressions.
/// The expressions must look like this: "n1 operator n2"
///
/// Supported operators: +, -, *, /, %, <, >, <=, >=, ==, !=
/// ```
/// # use dialogos::*;
/// assert_eq!(calc("15 + 25").unwrap(), 40.0);
/// assert_eq!(calc("15 - 25").unwrap(), -10.0);
/// assert_eq!(calc("10 * 20").unwrap(), 200.0);
/// assert_eq!(calc("10 / 20").unwrap(), 0.5);
/// assert_eq!(calc("10 % 20").unwrap(), 10.0);
/// assert_eq!(calc("6 < 6").unwrap(), 0.0);
/// assert_eq!(calc("6 > 6").unwrap(), 0.0);
/// assert_eq!(calc("6 <= 6").unwrap(), 1.0);
/// assert_eq!(calc("6 >= 6").unwrap(), 1.0);
/// assert_eq!(calc("6 == 6").unwrap(), 1.0);
/// assert_eq!(calc("6 != 6").unwrap(), 0.0);
/// ```
pub fn calc(s: &str) -> Option<f64> {
    let boolf64 = |b| if b { 1.0 } else { 0.0 };
    let args: Vec<&str> = s.trim().split(' ').collect();
    if args.len() == 3 {
        if let Ok(n1) = args[0].parse::<f64>() {
            if let Ok(n2) = args[2].parse::<f64>() {
                match args[1] {
                    "+" => Some(n1 + n2),
                    "-" => Some(n1 - n2),
                    "*" => Some(n1 * n2),
                    "/" => Some(n1 / n2),
                    "%" => Some(n1 % n2),
                    "<" => Some(boolf64(n1 < n2)),
                    ">" => Some(boolf64(n1 > n2)),
                    "<=" => Some(boolf64(n1 <= n2)),
                    ">=" => Some(boolf64(n1 >= n2)),
                    "==" => Some(boolf64(n1 == n2)),
                    "!=" => Some(boolf64(n1 != n2)),
                    _ => None,
                }
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    }
}

/// Splits a string with the dialogue split pattern.
/// ```
/// # use dialogos::*;
/// assert!(split("Hello||Gamer||Sisters").len() == 3);
/// ```
pub fn split(s: &str) -> Vec<String> {
    s.trim().split(SPLIT_PAT).map(|s| s.to_string()).collect()
}

/// The line types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum LineType {
    /// Represents the end of a dialogue.
    End,
    /// Represents text in a dialogue.
    Text,
    /// Represents a position in a dialogue.
    Label,
    /// Represents a position change in a dialogue.
    Jump,
    /// Represents a choice in a dialogue.
    Menu,
    /// Represents a variable creation in a dialogue.
    Variable,
    /// Represents a conditional statement in a dialogue.
    Check,
}

/// The dialogue line structure.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Line {
    /// The type of a line determines how its data will be used.
    pub t: LineType,
    /// Information about the line.
    pub info: String,
    /// The content of the line.
    pub cont: String,
}

/// Creates an end line.
pub fn end() -> Line {
    Line {
        t: LineType::End,
        info: String::new(),
        cont: String::new(),
    }
}

/// Creates a text line.
pub fn text(info: &str, cont: &str) -> Line {
    Line {
        t: LineType::Text,
        info: String::from(info),
        cont: String::from(cont),
    }
}

/// Creates a label line.
pub fn label(cont: &str) -> Line {
    Line {
        t: LineType::Label,
        info: String::new(),
        cont: String::from(cont),
    }
}

/// Creates a jump line.
pub fn jump(cont: &str) -> Line {
    Line {
        t: LineType::Jump,
        info: String::new(),
        cont: String::from(cont),
    }
}

/// Creates a menu line.
pub fn menu(info: &str, cont: &str) -> Line {
    Line {
        t: LineType::Menu,
        info: String::from(info),
        cont: String::from(cont),
    }
}

/// Creates a variable line.
pub fn variable(info: &str, cont: &str) -> Line {
    Line {
        t: LineType::Variable,
        info: String::from(info),
        cont: String::from(cont),
    }
}

/// Creates a check line.
pub fn check(cont: &str) -> Line {
    Line {
        t: LineType::Check,
        info: String::new(),
        cont: String::from(cont),
    }
}

/// The dialogue structure.
#[derive(Clone, Debug)]
pub struct Dialogue {
    idx: usize,
    lines: Vec<Line>,
    labels: HashMap<String, usize>,
    /// The variables of the dialogue.
    pub vars: HashMap<String, String>,
}

impl Dialogue {
    /// Creates a new dialogue.
    pub fn new(lines: Vec<Line>) -> Self {
        let mut labels = HashMap::new();
        let mut lines = lines;
        lines.push(end());
        for (i, line) in lines.iter().enumerate() {
            if line.t == LineType::Label {
                labels.insert(line.cont.clone(), i);
            }
        }
        let mut d = Dialogue {
            idx: 0,
            lines,
            labels,
            vars: HashMap::new(),
        };
        d.update();
        d
    }

    fn update(&mut self) {
        let line = self.line();
        match line.t {
            LineType::Label => {
                self.next();
            }

            LineType::Jump => {
                self.idx = match self.labels.get(&line.cont) {
                    Some(idx) => *idx,
                    None => self.idx + 1,
                };
                self.update();
            }

            LineType::Variable => {
                let val = if let Some(val) = calc(&line.cont) {
                    val.to_string()
                } else {
                    line.cont
                };
                self.vars.insert(line.info, val);
                self.next();
            }

            LineType::Check => {
                let val = if let Some(val) = calc(&line.cont) {
                    val
                } else {
                    0.0
                };
                if val == 1.0 {
                    self.next();
                } else {
                    self.idx += 2;
                    if self.idx >= self.lines.len() {
                        self.idx = self.lines.len() - 1;
                    }
                    self.update();
                }
            }

            _ => {}
        }
    }

    /// Returns true if the current line is an end line.
    pub fn has_end(&self) -> bool {
        self.lines[self.idx].t == LineType::End
    }

    /// Returns true if the current line is a menu line.
    pub fn has_menu(&self) -> bool {
        self.lines[self.idx].t == LineType::Menu
    }

    /// Resets the dialogue index.
    pub fn reset(&mut self) {
        self.idx = 0;
    }

    /// Changes the lines of the dialogue.
    pub fn change(&mut self, lines: Vec<Line>) {
        self.reset();
        self.labels.clear();
        self.lines = lines;
        self.lines.push(end());
        for (i, line) in self.lines.iter().enumerate() {
            if line.t == LineType::Label {
                self.labels.insert(line.cont.clone(), i);
            }
        }
    }

    /// Returns the current line of the dialogue.
    /// ```
    /// # use dialogos::*;
    /// let mut d = Dialogue::new(vec![
    ///     text("uwu", "My recomendation is..."),
    ///     text("owo", "ubunchu!"),
    /// ]);
    ///
    /// assert_eq!(d.line().info, "uwu");
    /// ```
    pub fn line(&self) -> Line {
        let re = Regex::new(VAR_PAT).unwrap();
        let line = &self.lines[self.idx];

        let mut result = line.clone();
        for caps in re.captures_iter(&line.info) {
            let word = caps.get(0).unwrap().as_str();
            if let Some(val) = self.vars.get(&word[2..]) {
                result.info = line.info.replace(word, val);
            }
        }
        for caps in re.captures_iter(&line.cont) {
            let word = caps.get(0).unwrap().as_str();
            if let Some(val) = self.vars.get(&word[2..]) {
                result.cont = line.cont.replace(word, val);
            }
        }
        result
    }

    /// Advances the index by one.
    /// ```
    /// # use dialogos::*;
    /// let mut d = Dialogue::new(vec![
    ///     text("Line 1", "This is a line."),
    ///     text("Line 2", "And this is also a line."),
    /// ]);
    ///
    /// assert_eq!(d.line().info, "Line 1");
    /// d.next();
    /// assert_eq!(d.line().info, "Line 2");
    /// ```
    pub fn next(&mut self) {
        self.idx += 1;
        self.update();
    }

    /// Changes the index by using a label.
    /// ```
    /// # use dialogos::*;
    /// let merchant = |cont| text("Merchant", cont);
    ///
    /// let mut d = Dialogue::new(vec![
    ///     label("Buy"),
    ///     merchant("What're ya buyin?"),
    ///     jump("End"),
    ///     label("Sell"),
    ///     merchant("What're ya sellin?"),
    ///     label("End"),
    ///     merchant("Heh heh heh... Thank you!"),
    /// ]);
    ///
    /// d.jump("Sell");
    /// assert_eq!(d.line().cont, "What're ya sellin?");
    /// ```
    pub fn jump(&mut self, label: &str) {
        self.idx = self.labels[label];
        self.update();
    }

    /// Returns the choices of a menu line.
    pub fn choices(&self) -> Vec<String> {
        split(&self.line().cont)
    }

    /// Chooses an item from a menu line.
    pub fn choose(&mut self, choice: usize) {
        self.jump(&split(&self.line().info)[choice]);
    }
}
