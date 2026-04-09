//! Expression tokenizer, parser, and evaluator.
//!
//! Supports: arithmetic, comparison, logical operators, member access,
//! indexing, function calls, and string interpolation.

use std::{collections::HashMap, fmt};

use rand::RngExt;

use meme_generator_utils::text::{Text2Image, TextParams};

use crate::error::TemplateError;

// ── TextDrawInfo: deferred text rendering ──

#[derive(Clone)]
pub struct TextDrawInfo {
    pub text: String,
    pub text_params: TextParams,
    pub font_size: FontSizeInfo,
    pub bound: Option<(f32, f32)>,
    pub bbcode: bool,
}

impl TextDrawInfo {
    /// Measure the rendered text dimensions (width, height).
    /// For bounded text returns the bound; for unbounded, uses Text2Image to measure.
    pub fn measure(&self) -> Result<(f32, f32), TemplateError> {
        if let Some(bound) = self.bound {
            return Ok(bound);
        }
        let font_size = match &self.font_size {
            FontSizeInfo::Fixed(s) => *s,
            FontSizeInfo::Range(_, _) => {
                return Err(TemplateError::ExprEval(
                    "Cannot measure text with font_size range and no bound".to_string(),
                ));
            }
        };
        let t = if self.bbcode {
            Text2Image::from_bbcode_text(&self.text, font_size, self.text_params.clone())
        } else {
            Text2Image::from_text(&self.text, font_size, self.text_params.clone())
        };
        Ok((t.longest_line().ceil(), t.height()))
    }
}

#[derive(Clone, Debug)]
pub enum FontSizeInfo {
    Fixed(f32),
    Range(f32, f32),
}

// ── Value ──

#[derive(Clone)]
pub enum Value {
    Number(f64),
    Bool(bool),
    String(String),
    Array(Vec<Value>),
    Map(HashMap<String, Value>),
    Image(skia_safe::Image),
    Text(TextDrawInfo),
}

impl fmt::Debug for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Number(n) => write!(f, "Number({n})"),
            Value::Bool(b) => write!(f, "Bool({b})"),
            Value::String(s) => write!(f, "String({s:?})"),
            Value::Array(a) => write!(f, "Array({a:?})"),
            Value::Map(m) => write!(f, "Map({m:?})"),
            Value::Image(img) => write!(f, "Image({}x{})", img.width(), img.height()),
            Value::Text(info) => write!(f, "Text({:?})", info.text),
        }
    }
}

impl Value {
    pub fn as_number(&self) -> Result<f64, TemplateError> {
        match self {
            Value::Number(n) => Ok(*n),
            Value::Bool(b) => Ok(if *b { 1.0 } else { 0.0 }),
            other => Err(TemplateError::TypeError {
                expected: "number",
                actual: format!("{other:?}"),
            }),
        }
    }

    pub fn as_bool(&self) -> Result<bool, TemplateError> {
        match self {
            Value::Bool(b) => Ok(*b),
            Value::Number(n) => Ok(*n != 0.0),
            Value::String(s) => Ok(!s.is_empty()),
            _ => Ok(true),
        }
    }

    pub fn as_string(&self) -> Result<String, TemplateError> {
        match self {
            Value::String(s) => Ok(s.clone()),
            Value::Number(n) => {
                if n.fract() == 0.0 && n.abs() < i64::MAX as f64 {
                    Ok(format!("{}", *n as i64))
                } else {
                    Ok(format!("{n}"))
                }
            }
            Value::Bool(b) => Ok(format!("{b}")),
            other => Err(TemplateError::TypeError {
                expected: "string",
                actual: format!("{other:?}"),
            }),
        }
    }

    pub fn as_i32(&self) -> Result<i32, TemplateError> {
        Ok(self.as_number()?.round() as i32)
    }

    pub fn as_f32(&self) -> Result<f32, TemplateError> {
        Ok(self.as_number()? as f32)
    }

    pub fn as_array(&self) -> Result<&Vec<Value>, TemplateError> {
        match self {
            Value::Array(a) => Ok(a),
            other => Err(TemplateError::TypeError {
                expected: "array",
                actual: format!("{other:?}"),
            }),
        }
    }

    pub fn as_map(&self) -> Result<&HashMap<String, Value>, TemplateError> {
        match self {
            Value::Map(m) => Ok(m),
            other => Err(TemplateError::TypeError {
                expected: "map",
                actual: format!("{other:?}"),
            }),
        }
    }

    pub fn as_image(&self) -> Result<&skia_safe::Image, TemplateError> {
        match self {
            Value::Image(img) => Ok(img),
            other => Err(TemplateError::TypeError {
                expected: "image",
                actual: format!("{other:?}"),
            }),
        }
    }

    #[allow(dead_code)]
    pub fn as_text(&self) -> Result<&TextDrawInfo, TemplateError> {
        match self {
            Value::Text(info) => Ok(info),
            other => Err(TemplateError::TypeError {
                expected: "text",
                actual: format!("{other:?}"),
            }),
        }
    }

    pub fn member_access(&self, field: &str) -> Result<Value, TemplateError> {
        match self {
            Value::Image(img) => match field {
                "width" => Ok(Value::Number(img.width() as f64)),
                "height" => Ok(Value::Number(img.height() as f64)),
                "size" => Ok(Value::Array(vec![
                    Value::Number(img.width() as f64),
                    Value::Number(img.height() as f64),
                ])),
                _ => Err(TemplateError::ExprEval(format!(
                    "Image has no property '{field}'"
                ))),
            },
            Value::Text(info) => {
                let (w, h) = info.measure()?;
                match field {
                    "width" => Ok(Value::Number(w as f64)),
                    "height" => Ok(Value::Number(h as f64)),
                    "size" => Ok(Value::Array(vec![
                        Value::Number(w as f64),
                        Value::Number(h as f64),
                    ])),
                    _ => Err(TemplateError::ExprEval(format!(
                        "Text has no property '{field}'"
                    ))),
                }
            }
            Value::Map(m) => m
                .get(field)
                .cloned()
                .ok_or_else(|| TemplateError::ExprEval(format!("Map has no key '{field}'"))),
            Value::Array(a) => match field {
                "length" => Ok(Value::Number(a.len() as f64)),
                _ => Err(TemplateError::ExprEval(format!(
                    "Array has no property '{field}'"
                ))),
            },
            _ => Err(TemplateError::ExprEval(format!(
                "Cannot access property '{field}' on {self:?}"
            ))),
        }
    }

    pub fn index_access(&self, index: usize) -> Result<Value, TemplateError> {
        match self {
            Value::Array(a) => a.get(index).cloned().ok_or_else(|| {
                TemplateError::ExprEval(format!("Index {index} out of bounds (len={})", a.len()))
            }),
            _ => Err(TemplateError::ExprEval(format!(
                "Cannot index into {self:?}"
            ))),
        }
    }
}

// ── Tokens ──

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Bool(bool),
    Str(String),
    Ident(String),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Caret,
    Eq,
    NotEq,
    Gt,
    Lt,
    GtEq,
    LtEq,
    And,
    Or,
    Not,
    Dot,
    LBracket,
    RBracket,
    LParen,
    RParen,
    Comma,
    Eof,
}

// ── Tokenizer ──

struct Tokenizer {
    chars: Vec<char>,
    pos: usize,
}

impl Tokenizer {
    fn new(input: &str) -> Self {
        Self {
            chars: input.chars().collect(),
            pos: 0,
        }
    }

    fn peek_char(&self) -> Option<char> {
        self.chars.get(self.pos).copied()
    }

    fn next_char(&mut self) -> Option<char> {
        let ch = self.chars.get(self.pos).copied();
        if ch.is_some() {
            self.pos += 1;
        }
        ch
    }

    fn skip_whitespace(&mut self) {
        while let Some(ch) = self.peek_char() {
            if ch.is_whitespace() {
                self.pos += 1;
            } else {
                break;
            }
        }
    }

    fn tokenize(&mut self) -> Result<Vec<Token>, TemplateError> {
        let mut tokens = Vec::new();
        loop {
            self.skip_whitespace();
            match self.peek_char() {
                None => {
                    tokens.push(Token::Eof);
                    return Ok(tokens);
                }
                Some(ch) => {
                    let tok = match ch {
                        '+' => {
                            self.next_char();
                            Token::Plus
                        }
                        '-' => {
                            self.next_char();
                            Token::Minus
                        }
                        '*' => {
                            self.next_char();
                            Token::Star
                        }
                        '/' => {
                            self.next_char();
                            Token::Slash
                        }
                        '%' => {
                            self.next_char();
                            Token::Percent
                        }
                        '^' => {
                            self.next_char();
                            Token::Caret
                        }
                        '.' => {
                            self.next_char();
                            Token::Dot
                        }
                        '[' => {
                            self.next_char();
                            Token::LBracket
                        }
                        ']' => {
                            self.next_char();
                            Token::RBracket
                        }
                        '(' => {
                            self.next_char();
                            Token::LParen
                        }
                        ')' => {
                            self.next_char();
                            Token::RParen
                        }
                        ',' => {
                            self.next_char();
                            Token::Comma
                        }
                        '=' => {
                            self.next_char();
                            if self.peek_char() == Some('=') {
                                self.next_char();
                                Token::Eq
                            } else {
                                return Err(TemplateError::ExprParse(
                                    "Expected '==' but got '='".to_string(),
                                ));
                            }
                        }
                        '!' => {
                            self.next_char();
                            if self.peek_char() == Some('=') {
                                self.next_char();
                                Token::NotEq
                            } else {
                                Token::Not
                            }
                        }
                        '>' => {
                            self.next_char();
                            if self.peek_char() == Some('=') {
                                self.next_char();
                                Token::GtEq
                            } else {
                                Token::Gt
                            }
                        }
                        '<' => {
                            self.next_char();
                            if self.peek_char() == Some('=') {
                                self.next_char();
                                Token::LtEq
                            } else {
                                Token::Lt
                            }
                        }
                        '&' => {
                            self.next_char();
                            if self.peek_char() == Some('&') {
                                self.next_char();
                                Token::And
                            } else {
                                return Err(TemplateError::ExprParse(
                                    "Expected '&&' but got '&'".to_string(),
                                ));
                            }
                        }
                        '|' => {
                            self.next_char();
                            if self.peek_char() == Some('|') {
                                self.next_char();
                                Token::Or
                            } else {
                                return Err(TemplateError::ExprParse(
                                    "Expected '||' but got '|'".to_string(),
                                ));
                            }
                        }
                        '\'' => {
                            self.next_char();
                            let mut s = String::new();
                            loop {
                                match self.next_char() {
                                    Some('\'') => break,
                                    Some(c) => s.push(c),
                                    None => {
                                        return Err(TemplateError::ExprParse(
                                            "Unterminated string literal".to_string(),
                                        ));
                                    }
                                }
                            }
                            Token::Str(s)
                        }
                        c if c.is_ascii_digit() => {
                            let mut num_str = String::new();
                            while let Some(c) = self.peek_char() {
                                if c.is_ascii_digit() || c == '.' {
                                    num_str.push(c);
                                    self.next_char();
                                } else {
                                    break;
                                }
                            }
                            let n: f64 = num_str.parse().map_err(|_| {
                                TemplateError::ExprParse(format!("Invalid number: {num_str}"))
                            })?;
                            Token::Number(n)
                        }
                        c if c.is_ascii_alphabetic() || c == '_' => {
                            let mut ident = String::new();
                            while let Some(c) = self.peek_char() {
                                if c.is_ascii_alphanumeric() || c == '_' {
                                    ident.push(c);
                                    self.next_char();
                                } else {
                                    break;
                                }
                            }
                            match ident.as_str() {
                                "true" => Token::Bool(true),
                                "false" => Token::Bool(false),
                                _ => Token::Ident(ident),
                            }
                        }
                        _ => {
                            return Err(TemplateError::ExprParse(format!(
                                "Unexpected character: '{ch}'"
                            )));
                        }
                    };
                    tokens.push(tok);
                }
            }
        }
    }
}

// ── AST ──

#[derive(Debug, Clone)]
pub enum Expr {
    Literal(Value),
    Ident(String),
    ArrayLiteral(Vec<Expr>),
    BinOp {
        op: BinOp,
        left: Box<Expr>,
        right: Box<Expr>,
    },
    UnaryOp {
        op: UnaryOp,
        operand: Box<Expr>,
    },
    Member {
        object: Box<Expr>,
        field: String,
    },
    Index {
        object: Box<Expr>,
        index: Box<Expr>,
    },
    Call {
        func: String,
        args: Vec<Expr>,
    },
}

#[derive(Debug, Clone, Copy)]
pub enum BinOp {
    Add,
    Sub,
    Mul,
    Div,
    Mod,
    Pow,
    Eq,
    NotEq,
    Gt,
    Lt,
    GtEq,
    LtEq,
    And,
    Or,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp {
    Neg,
    Not,
}

// ── Parser (recursive descent) ──

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Self { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn advance(&mut self) -> Token {
        let tok = self.tokens.get(self.pos).cloned().unwrap_or(Token::Eof);
        self.pos += 1;
        tok
    }

    fn expect(&mut self, expected: &Token) -> Result<(), TemplateError> {
        let tok = self.advance();
        if &tok == expected {
            Ok(())
        } else {
            Err(TemplateError::ExprParse(format!(
                "Expected {expected:?}, got {tok:?}"
            )))
        }
    }

    fn parse_expr(&mut self) -> Result<Expr, TemplateError> {
        self.parse_or()
    }

    fn parse_or(&mut self) -> Result<Expr, TemplateError> {
        let mut left = self.parse_and()?;
        while *self.peek() == Token::Or {
            self.advance();
            let right = self.parse_and()?;
            left = Expr::BinOp {
                op: BinOp::Or,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_and(&mut self) -> Result<Expr, TemplateError> {
        let mut left = self.parse_equality()?;
        while *self.peek() == Token::And {
            self.advance();
            let right = self.parse_equality()?;
            left = Expr::BinOp {
                op: BinOp::And,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_equality(&mut self) -> Result<Expr, TemplateError> {
        let mut left = self.parse_comparison()?;
        loop {
            let op = match self.peek() {
                Token::Eq => BinOp::Eq,
                Token::NotEq => BinOp::NotEq,
                _ => break,
            };
            self.advance();
            let right = self.parse_comparison()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_comparison(&mut self) -> Result<Expr, TemplateError> {
        let mut left = self.parse_addition()?;
        loop {
            let op = match self.peek() {
                Token::Gt => BinOp::Gt,
                Token::Lt => BinOp::Lt,
                Token::GtEq => BinOp::GtEq,
                Token::LtEq => BinOp::LtEq,
                _ => break,
            };
            self.advance();
            let right = self.parse_addition()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_addition(&mut self) -> Result<Expr, TemplateError> {
        let mut left = self.parse_multiplication()?;
        loop {
            let op = match self.peek() {
                Token::Plus => BinOp::Add,
                Token::Minus => BinOp::Sub,
                _ => break,
            };
            self.advance();
            let right = self.parse_multiplication()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_multiplication(&mut self) -> Result<Expr, TemplateError> {
        let mut left = self.parse_power()?;
        loop {
            let op = match self.peek() {
                Token::Star => BinOp::Mul,
                Token::Slash => BinOp::Div,
                Token::Percent => BinOp::Mod,
                _ => break,
            };
            self.advance();
            let right = self.parse_power()?;
            left = Expr::BinOp {
                op,
                left: Box::new(left),
                right: Box::new(right),
            };
        }
        Ok(left)
    }

    fn parse_power(&mut self) -> Result<Expr, TemplateError> {
        let base = self.parse_unary()?;
        if *self.peek() == Token::Caret {
            self.advance();
            let exp = self.parse_power()?; // right-associative
            Ok(Expr::BinOp {
                op: BinOp::Pow,
                left: Box::new(base),
                right: Box::new(exp),
            })
        } else {
            Ok(base)
        }
    }

    fn parse_unary(&mut self) -> Result<Expr, TemplateError> {
        match self.peek() {
            Token::Not => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::UnaryOp {
                    op: UnaryOp::Not,
                    operand: Box::new(expr),
                })
            }
            Token::Minus => {
                self.advance();
                let expr = self.parse_unary()?;
                Ok(Expr::UnaryOp {
                    op: UnaryOp::Neg,
                    operand: Box::new(expr),
                })
            }
            _ => self.parse_postfix(),
        }
    }

    fn parse_postfix(&mut self) -> Result<Expr, TemplateError> {
        let mut expr = self.parse_primary()?;
        loop {
            match self.peek() {
                Token::Dot => {
                    self.advance();
                    if let Token::Ident(field) = self.advance() {
                        expr = Expr::Member {
                            object: Box::new(expr),
                            field,
                        };
                    } else {
                        return Err(TemplateError::ExprParse(
                            "Expected identifier after '.'".to_string(),
                        ));
                    }
                }
                Token::LBracket => {
                    self.advance();
                    let index = self.parse_expr()?;
                    self.expect(&Token::RBracket)?;
                    expr = Expr::Index {
                        object: Box::new(expr),
                        index: Box::new(index),
                    };
                }
                Token::LParen => {
                    // Only allow function calls on identifiers
                    if let Expr::Ident(name) = expr {
                        self.advance();
                        let mut args = Vec::new();
                        if *self.peek() != Token::RParen {
                            args.push(self.parse_expr()?);
                            while *self.peek() == Token::Comma {
                                self.advance();
                                args.push(self.parse_expr()?);
                            }
                        }
                        self.expect(&Token::RParen)?;
                        expr = Expr::Call { func: name, args };
                    } else {
                        break;
                    }
                }
                _ => break,
            }
        }
        Ok(expr)
    }

    fn parse_primary(&mut self) -> Result<Expr, TemplateError> {
        match self.advance() {
            Token::Number(n) => Ok(Expr::Literal(Value::Number(n))),
            Token::Bool(b) => Ok(Expr::Literal(Value::Bool(b))),
            Token::Str(s) => Ok(Expr::Literal(Value::String(s))),
            Token::Ident(name) => Ok(Expr::Ident(name)),
            Token::LParen => {
                let expr = self.parse_expr()?;
                self.expect(&Token::RParen)?;
                Ok(expr)
            }
            Token::LBracket => {
                // Array literal: [expr, expr, ...]
                let mut items = Vec::new();
                if *self.peek() != Token::RBracket {
                    items.push(self.parse_expr()?);
                    while *self.peek() == Token::Comma {
                        self.advance();
                        items.push(self.parse_expr()?);
                    }
                }
                self.expect(&Token::RBracket)?;
                Ok(Expr::ArrayLiteral(items))
            }
            tok => Err(TemplateError::ExprParse(format!(
                "Unexpected token in expression: {tok:?}"
            ))),
        }
    }
}

// ── Evaluation ──

pub fn parse_expr(input: &str) -> Result<Expr, TemplateError> {
    let mut tokenizer = Tokenizer::new(input);
    let tokens = tokenizer.tokenize()?;
    let mut parser = Parser::new(tokens);
    let expr = parser.parse_expr()?;
    if *parser.peek() != Token::Eof {
        return Err(TemplateError::ExprParse(format!(
            "Unexpected token after expression: {:?}",
            parser.peek()
        )));
    }
    Ok(expr)
}

pub fn eval_expr(expr: &Expr, ctx: &HashMap<String, Value>) -> Result<Value, TemplateError> {
    match expr {
        Expr::Literal(val) => Ok(val.clone()),
        Expr::Ident(name) => ctx
            .get(name)
            .cloned()
            .ok_or_else(|| TemplateError::ExprEval(format!("Undefined variable: '{name}'"))),
        Expr::ArrayLiteral(items) => {
            let values = items
                .iter()
                .map(|e| eval_expr(e, ctx))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Value::Array(values))
        }
        Expr::BinOp { op, left, right } => {
            let lv = eval_expr(left, ctx)?;
            let rv = eval_expr(right, ctx)?;
            eval_binop(*op, &lv, &rv)
        }
        Expr::UnaryOp { op, operand } => {
            let v = eval_expr(operand, ctx)?;
            match op {
                UnaryOp::Neg => Ok(Value::Number(-v.as_number()?)),
                UnaryOp::Not => Ok(Value::Bool(!v.as_bool()?)),
            }
        }
        Expr::Member { object, field } => {
            let obj = eval_expr(object, ctx)?;
            obj.member_access(field)
        }
        Expr::Index { object, index } => {
            let obj = eval_expr(object, ctx)?;
            let idx = eval_expr(index, ctx)?.as_number()? as usize;
            obj.index_access(idx)
        }
        Expr::Call { func, args } => {
            // Short-circuit evaluation for if()
            if func == "if" {
                if args.len() != 3 {
                    return Err(TemplateError::ExprEval(format!(
                        "if() expects 3 arguments, got {}",
                        args.len()
                    )));
                }
                let cond = eval_expr(&args[0], ctx)?;
                return if cond.as_bool()? {
                    eval_expr(&args[1], ctx)
                } else {
                    eval_expr(&args[2], ctx)
                };
            }
            let arg_values: Vec<Value> = args
                .iter()
                .map(|a| eval_expr(a, ctx))
                .collect::<Result<_, _>>()?;
            eval_function(func, &arg_values)
        }
    }
}

fn eval_binop(op: BinOp, lv: &Value, rv: &Value) -> Result<Value, TemplateError> {
    match op {
        BinOp::Add => {
            if matches!(lv, Value::String(_)) || matches!(rv, Value::String(_)) {
                Ok(Value::String(format!(
                    "{}{}",
                    lv.as_string()?,
                    rv.as_string()?
                )))
            } else {
                Ok(Value::Number(lv.as_number()? + rv.as_number()?))
            }
        }
        BinOp::Sub => Ok(Value::Number(lv.as_number()? - rv.as_number()?)),
        BinOp::Mul => Ok(Value::Number(lv.as_number()? * rv.as_number()?)),
        BinOp::Div => {
            let r = rv.as_number()?;
            if r == 0.0 {
                return Err(TemplateError::ExprEval("Division by zero".to_string()));
            }
            Ok(Value::Number(lv.as_number()? / r))
        }
        BinOp::Mod => Ok(Value::Number(lv.as_number()? % rv.as_number()?)),
        BinOp::Pow => Ok(Value::Number(lv.as_number()?.powf(rv.as_number()?))),
        BinOp::Eq => Ok(Value::Bool(values_equal(lv, rv))),
        BinOp::NotEq => Ok(Value::Bool(!values_equal(lv, rv))),
        BinOp::Gt => Ok(Value::Bool(lv.as_number()? > rv.as_number()?)),
        BinOp::Lt => Ok(Value::Bool(lv.as_number()? < rv.as_number()?)),
        BinOp::GtEq => Ok(Value::Bool(lv.as_number()? >= rv.as_number()?)),
        BinOp::LtEq => Ok(Value::Bool(lv.as_number()? <= rv.as_number()?)),
        BinOp::And => Ok(Value::Bool(lv.as_bool()? && rv.as_bool()?)),
        BinOp::Or => Ok(Value::Bool(lv.as_bool()? || rv.as_bool()?)),
    }
}

fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Number(a), Value::Number(b)) => (a - b).abs() < f64::EPSILON,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::String(a), Value::String(b)) => a == b,
        _ => false,
    }
}

fn eval_function(name: &str, args: &[Value]) -> Result<Value, TemplateError> {
    match name {
        "min" => {
            check_arg_count(name, args, 2)?;
            Ok(Value::Number(
                args[0].as_number()?.min(args[1].as_number()?),
            ))
        }
        "max" => {
            check_arg_count(name, args, 2)?;
            Ok(Value::Number(
                args[0].as_number()?.max(args[1].as_number()?),
            ))
        }
        "round" => {
            check_arg_count(name, args, 1)?;
            Ok(Value::Number(args[0].as_number()?.round()))
        }
        "floor" => {
            check_arg_count(name, args, 1)?;
            Ok(Value::Number(args[0].as_number()?.floor()))
        }
        "ceil" => {
            check_arg_count(name, args, 1)?;
            Ok(Value::Number(args[0].as_number()?.ceil()))
        }
        "abs" => {
            check_arg_count(name, args, 1)?;
            Ok(Value::Number(args[0].as_number()?.abs()))
        }
        "random" => {
            check_arg_count(name, args, 0)?;
            Ok(Value::Number(rand::rng().random()))
        }
        "date" => {
            check_arg_count(name, args, 1)?;
            let fmt = args[0].as_string()?;
            let now = chrono::Local::now();
            Ok(Value::String(now.format(&fmt).to_string()))
        }
        "has" => {
            check_arg_count(name, args, 2)?;
            let map = args[0].as_map()?;
            let key = args[1].as_string()?;
            Ok(Value::Bool(map.contains_key(&key)))
        }
        "contains" => {
            check_arg_count(name, args, 2)?;
            let arr = args[0].as_array()?;
            let needle = &args[1];
            Ok(Value::Bool(arr.iter().any(|v| values_equal(v, needle))))
        }
        _ => Err(TemplateError::ExprEval(format!(
            "Unknown function: '{name}'"
        ))),
    }
}

fn check_arg_count(name: &str, args: &[Value], expected: usize) -> Result<(), TemplateError> {
    if args.len() != expected {
        Err(TemplateError::ExprEval(format!(
            "{name}() expects {expected} arguments, got {}",
            args.len()
        )))
    } else {
        Ok(())
    }
}

// ── High-level evaluation helpers ──

/// Parse and evaluate a string as an expression.
pub fn eval_str(input: &str, ctx: &HashMap<String, Value>) -> Result<Value, TemplateError> {
    let expr = parse_expr(input)?;
    eval_expr(&expr, ctx)
}

/// Perform string interpolation: replace `{expr}` or `{expr:fmt}` inside a string.
///
/// Format specifiers (after `:`):
/// - `0N` — zero-pad to N digits, e.g. `{x:03}` → `"007"`
pub fn interpolate(template: &str, ctx: &HashMap<String, Value>) -> Result<String, TemplateError> {
    let mut result = String::new();
    let mut chars = template.chars().peekable();
    while let Some(ch) = chars.next() {
        if ch == '{' {
            let mut inner = String::new();
            let mut depth = 1;
            for c in chars.by_ref() {
                if c == '{' {
                    depth += 1;
                } else if c == '}' {
                    depth -= 1;
                    if depth == 0 {
                        break;
                    }
                }
                inner.push(c);
            }
            if depth != 0 {
                return Err(TemplateError::ExprParse(
                    "Unterminated '{' in string interpolation".to_string(),
                ));
            }

            // Split expr and optional format spec at the last top-level ':'
            let (expr_str, fmt_spec) = split_expr_format(&inner);

            let val = eval_str(expr_str, ctx)?;
            let formatted = apply_format_spec(&val.as_string()?, fmt_spec)?;
            result.push_str(&formatted);
        } else {
            result.push(ch);
        }
    }
    Ok(result)
}

/// Split `"expr:fmt"` at the last `:` that is not inside brackets/parens.
/// Returns `(expr, Some(fmt))` or `(input, None)`.
fn split_expr_format(input: &str) -> (&str, Option<&str>) {
    let mut depth = 0;
    let mut last_colon = None;
    for (i, c) in input.char_indices() {
        match c {
            '(' | '[' => depth += 1,
            ')' | ']' => depth -= 1,
            ':' if depth == 0 => last_colon = Some(i),
            _ => {}
        }
    }
    match last_colon {
        Some(i) => (&input[..i], Some(&input[i + 1..])),
        None => (input, None),
    }
}

/// Apply a simple format spec to a string value.
fn apply_format_spec(value: &str, spec: Option<&str>) -> Result<String, TemplateError> {
    let Some(spec) = spec else {
        return Ok(value.to_string());
    };
    if spec.is_empty() {
        return Ok(value.to_string());
    }

    if let Some(width_str) = spec.strip_prefix('0') {
        // Zero-pad: e.g. "03" → pad to 3 with zeros
        let width: usize = width_str
            .parse()
            .map_err(|_| TemplateError::ExprParse(format!("Invalid format spec: ':{spec}'")))?;
        Ok(format!("{value:0>width$}"))
    } else {
        Err(TemplateError::ExprParse(format!(
            "Unknown format spec: ':{spec}'"
        )))
    }
}

/// Evaluate a `yaml_serde::Value` as an expression-aware value.
/// Strings are treated as expressions, numbers/bools as literals, arrays recursively.
pub fn eval_yaml_value(
    value: &yaml_serde::Value,
    ctx: &HashMap<String, Value>,
) -> Result<Value, TemplateError> {
    match value {
        yaml_serde::Value::Number(n) => {
            Ok(Value::Number(n.as_f64().ok_or_else(|| {
                TemplateError::ExprEval("Invalid number".to_string())
            })?))
        }
        yaml_serde::Value::Bool(b) => Ok(Value::Bool(*b)),
        yaml_serde::Value::String(s) => eval_str(s, ctx),
        yaml_serde::Value::Sequence(arr) => {
            let values = arr
                .iter()
                .map(|v| eval_yaml_value(v, ctx))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Value::Array(values))
        }
        yaml_serde::Value::Null => Ok(Value::Number(0.0)),
        _ => Err(TemplateError::ExprEval(format!(
            "Unsupported YAML value: {value:?}"
        ))),
    }
}

/// Evaluate a `ValueExpr` to a `Value`.
pub fn eval_value_expr(
    expr: &crate::schema::ValueExpr,
    ctx: &HashMap<String, Value>,
) -> Result<Value, TemplateError> {
    match expr {
        crate::schema::ValueExpr::Number(n) => Ok(Value::Number(*n)),
        crate::schema::ValueExpr::Bool(b) => Ok(Value::Bool(*b)),
        crate::schema::ValueExpr::String(s) => eval_str(s, ctx),
        crate::schema::ValueExpr::Array(items) => {
            let values = items
                .iter()
                .map(|v| eval_value_expr(v, ctx))
                .collect::<Result<Vec<_>, _>>()?;
            Ok(Value::Array(values))
        }
    }
}

/// Evaluate a ValueExpr to [i32; 2] (for position/size).
pub fn eval_size(
    expr: &crate::schema::ValueExpr,
    ctx: &HashMap<String, Value>,
) -> Result<(i32, i32), TemplateError> {
    let val = eval_value_expr(expr, ctx)?;
    match val {
        Value::Array(ref a) if a.len() == 2 => Ok((a[0].as_i32()?, a[1].as_i32()?)),
        _ => Err(TemplateError::TypeError {
            expected: "[w, h] array",
            actual: format!("{val:?}"),
        }),
    }
}
