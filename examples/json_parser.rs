use std::collections::HashMap;

#[derive(Debug)]
struct Json {}

#[derive(Debug, Eq, PartialEq)]
enum Token {
    NUM(i32),
    STR(String),
    /// 左大括号
    LBig,
    /// 右大括号
    RBig,
    /// 左方括号
    LSquare,
    /// 右方括号
    RSquare,
    /// 冒号
    COLON,
    /// 逗号
    COMMA,
}

fn tokenlizer(json: String) -> Vec<Token> {
    let mut tokens = vec![];

    let chars: Vec<_> = json.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        if chars[i].is_whitespace() {
            i += 1;
            continue;
        }
        if chars[i] == '{' {
            tokens.push(Token::LBig);
            i += 1;
            continue;
        }
        if chars[i] == '}' {
            tokens.push(Token::RBig);
            i += 1;
            continue;
        }
        if chars[i] == '[' {
            tokens.push(Token::LSquare);
            i += 1;
            continue;
        }
        if chars[i] == ':' {
            tokens.push(Token::COLON);
            i += 1;
            continue;
        }
        if chars[i] == ',' {
            tokens.push(Token::COMMA);
            i += 1;
            continue;
        }
        if chars[i] == ']' {
            tokens.push(Token::RSquare);
            i += 1;
            continue;
        }

        if chars[i].is_numeric() || chars[i] == '-' {
            let mut j = i + 1;

            while chars[j].is_numeric() {
                j += 1;
            }

            let s: String = chars.iter().skip(i).take(j - i).collect();
            tokens.push(Token::NUM(s.parse().unwrap()));

            i = j;
            continue;
        }

        if chars[i] == '"' || chars[i] == '\'' {
            let mut j = i + 1;
            while chars[j] != chars[i] {
                j += 1;
            }
            let s: String = chars.iter().skip(i + 1).take(j - i - 1).collect();
            tokens.push(Token::STR(s));

            i = j;
        }

        if chars[i].is_ascii_alphabetic() {
            let mut j = i + 1;

            while chars[j].is_ascii_alphabetic() {
                j += 1;
            }

            let s: String = chars.iter().skip(i).take(j - i).collect();
            tokens.push(Token::STR(s));

            i = j;
            continue;
        }

        i += 1;
    }

    tokens
}

#[derive(Debug, Eq, PartialEq)]
enum JsonNode {
    Object(HashMap<String, JsonNode>),
    String(String),
    Num(i32),
    Array(Vec<JsonNode>),
}

impl JsonNode {
    fn to_json_string(&self) -> String {
        match self {
            JsonNode::Object(map) => {
                let mut s = String::new();
                s.push('{');
                let inner = map
                    .iter()
                    .map(|(k, v)| format!("\"{}\":{}", *k, v.to_json_string()))
                    .collect::<Vec<_>>()
                    .join(",");
                s.push_str(inner.as_str());
                s.push('}');
                s
            }
            JsonNode::String(s) => format!("\"{}\"", s),
            JsonNode::Num(num) => (*num).to_string(),
            JsonNode::Array(arr) => format!(
                "[{}]",
                arr.iter()
                    .map(|it| it.to_json_string())
                    .collect::<Vec<_>>()
                    .join(",")
            ),
        }
    }
}

fn generate_ast(tokens: Vec<Token>) -> JsonNode {
    fn get_node(mut i: usize, tokens: &Vec<Token>) -> (usize, JsonNode) {
        if tokens[i] == Token::COMMA {
            i += 1;
        }

        if let Token::NUM(num) = &tokens[i] {
            return (i + 1, JsonNode::Num(*num));
        }

        if let Token::STR(s) = &tokens[i] {
            return (i + 1, JsonNode::String(s.clone()));
        }

        if tokens[i] == Token::LBig {
            i += 1;
            let mut map = HashMap::new();
            loop {
                if let Token::STR(name) = &tokens[i] {
                    i += 2;
                    let (idx, node) = get_node(i, tokens);
                    map.insert(name.clone(), node);
                    i = idx;
                    if tokens[i] == Token::COMMA {
                        i += 1;
                    }
                } else {
                    break;
                }
            }
            return (i + 1, JsonNode::Object(map));
        }

        if tokens[i] == Token::LSquare {
            i += 1;
            let mut arr = vec![];

            while tokens[i] != Token::RSquare {
                let (idx, node) = get_node(i, tokens);
                arr.push(node);
                i = idx;
                if tokens[i] == Token::COMMA {
                    i += 1;
                }
            }
            i += 1;
            return (i, JsonNode::Array(arr));
        }

        unreachable!()
    }

    get_node(0, &tokens).1
}

fn parse(json: String) -> Result<Json, &'static str> {
    println!("{:?}", json);

    let tokens: Vec<Token> = tokenlizer(json);
    dbg!(&tokens);
    let ast = generate_ast(tokens);
    dbg!(&ast);
    let json = ast.to_json_string();
    println!("{}", json);
    Err("待完成")
}

fn main() {
    let res = parse(
        r#"
{
    c: -11,
    d: [
        1,
        {
            f: 'ggga',
            g: {
                c: -11,
                d: [
                    1,
                    {
                        f: 'ggga'
                    }
                ]
            }
        }
    ]
}
"#
            .to_string(),
    );
    dbg!(res.unwrap_err());
}
