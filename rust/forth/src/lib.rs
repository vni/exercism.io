use std::collections::HashMap;

pub type Value = i32;
pub type ForthResult = Result<(), Error>;

pub struct Forth {
    stack: Vec<Value>,
    words: HashMap<String, String>,
}

#[derive(Debug, PartialEq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Forth {
    pub fn new() -> Forth {
        Forth {
            stack: Vec::new(),
            words: HashMap::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        &self.stack
    }

    pub fn eval(&mut self, input: &str) -> ForthResult {
        let in_vec: Vec<String> = input
            .split(char::is_whitespace)
            .map(|s| s.to_lowercase())
            .collect();
        let mut iter = in_vec.iter();
        while let Some(v) = iter.next() {
            if let Ok(num) = v.parse::<Value>() {
                self.stack.push(num);
            } else if self.words.contains_key(v) {
                let program_str = self.words[v].clone();
                self.eval(program_str.as_str())?;
            } else {
                match v.as_str() {
                    "+" => self.add()?,
                    "-" => self.sub()?,
                    "*" => self.mul()?,
                    "/" => self.div()?,
                    "dup" => self.dup()?,
                    "drop" => self.drop()?,
                    "swap" => self.swap()?,
                    "over" => self.over()?,
                    ":" => self.define(&mut iter)?,
                    _ => return Err(Error::UnknownWord),
                }
            }
        }

        Ok(())
    }

    fn add(&mut self) -> ForthResult {
        let (top, bottom) = self.pop2()?;
        self.stack.push(bottom + top);
        Ok(())
    }

    fn sub(&mut self) -> ForthResult {
        let (top, bottom) = self.pop2()?;
        self.stack.push(bottom - top);
        Ok(())
    }

    fn mul(&mut self) -> ForthResult {
        let (top, bottom) = self.pop2()?;
        self.stack.push(bottom * top);
        Ok(())
    }

    fn div(&mut self) -> ForthResult {
        let (top, bottom) = self.pop2()?;
        if top != 0 {
            self.stack.push(bottom / top);
            Ok(())
        } else {
            Err(Error::DivisionByZero)
        }
    }

    fn dup(&mut self) -> ForthResult {
        if let Some(&top) = self.stack.last() {
            self.stack.push(top);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn drop(&mut self) -> ForthResult {
        if None == self.stack.pop() {
            return Err(Error::StackUnderflow);
        }
        Ok(())
    }

    fn swap(&mut self) -> ForthResult {
        let (top, bottom) = self.pop2()?;
        self.stack.push(top);
        self.stack.push(bottom);
        Ok(())
    }

    fn over(&mut self) -> ForthResult {
        let len = self.stack.len();
        if len < 2 {
            return Err(Error::StackUnderflow);
        }
        self.stack.push(self.stack[len - 2]);
        Ok(())
    }

    fn define(&mut self, iter: &mut std::slice::Iter<String>) -> ForthResult {
        let name = iter.next();
        if name.is_none() {
            return Err(Error::InvalidWord);
        }
        let name: &String = name.unwrap();
        if !Self::valid_word_name(name) {
            return Err(Error::InvalidWord);
        }
        let name = name.to_lowercase();

        let mut word: Vec<String> = vec![];
        let mut finished: bool = false;
        while let Some(vv) = iter.next() {
            if vv == ";" {
                finished = true;
                break;
            }

            if let Some(program_str) = self.words.get(vv) {
                word.push(program_str.clone());
            } else {
                word.push(vv.clone());
            }
        }

        if finished && !word.is_empty() {
            self.words.insert(name, word.join(" "));
        } else {
            return Err(Error::InvalidWord);
        }

        Ok(())
    }

    fn valid_word_name(word: &str) -> bool {
        word != ";" && !word.chars().any(|c| c.is_digit(10))
    }

    fn pop(&mut self) -> Result<Value, Error> {
        self.stack.pop().ok_or(Error::StackUnderflow)
    }

    fn pop2(&mut self) -> Result<(Value, Value), Error> {
        Ok((self.pop()?, self.pop()?))
    }
}
