use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum Token {
    Num(i32),
    NumFloat(f32),
    Str(String),
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

fn tokenlizer(json: String) -> Result<Vec<Token>, failure::Error> {
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
            let mut is_float = false;
            while chars[j].is_numeric() || chars[j] == '.' {
                j += 1;
                if chars[j] == '.' {
                    is_float = true;
                }
            }

            let s: String = chars.iter().skip(i).take(j - i).collect();
            if is_float {
                tokens.push(Token::NumFloat(s.parse()?));
            } else {
                tokens.push(Token::Num(s.parse()?));
            }

            i = j;
            continue;
        }

        if chars[i] == '"' || chars[i] == '\'' {
            let mut j = i + 1;
            while chars[j] != chars[i] {
                j += 1;
            }
            let s: String = chars.iter().skip(i + 1).take(j - i - 1).collect();
            tokens.push(Token::Str(s));

            i = j;
        }

        if chars[i].is_ascii_alphabetic() {
            let mut j = i + 1;

            while chars[j].is_ascii_alphabetic() {
                j += 1;
            }

            let s: String = chars.iter().skip(i).take(j - i).collect();
            tokens.push(Token::Str(s));

            i = j;
            continue;
        }

        i += 1;
    }

    Ok(tokens)
}

#[derive(Debug, PartialEq)]
enum JsonNode {
    Object(HashMap<String, JsonNode>),
    String(String),
    Num(i32),
    NumFloat(f32),
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
                    .map(|(k, v)| format!(r#""{}":{}"#, *k, v.to_json_string()))
                    .collect::<Vec<_>>()
                    .join(",");
                s.push_str(inner.as_str());
                s.push('}');
                s
            }
            JsonNode::String(s) => format!(r#""{}""#, s.replace(r#"""#, r#"\""#)),
            JsonNode::Num(num) => (*num).to_string(),
            JsonNode::NumFloat(num) => (*num).to_string(),
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

        if let Token::Num(num) = &tokens[i] {
            return (i + 1, JsonNode::Num(*num));
        }

        if let Token::NumFloat(num) = &tokens[i] {
            return (i + 1, JsonNode::NumFloat(*num));
        }

        if let Token::Str(s) = &tokens[i] {
            return (i + 1, JsonNode::String(s.clone()));
        }

        if tokens[i] == Token::LBig {
            i += 1;
            let mut map = HashMap::new();
            loop {
                if let Token::Str(name) = &tokens[i] {
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

fn parse(json: String) -> Result<JsonNode, failure::Error> {
    println!("{:?}", json);

    let tokens: Vec<Token> = tokenlizer(json)?;
    dbg!(&tokens);
    Ok(generate_ast(tokens))
}

fn main() -> Result<(), failure::Error> {
    let res = parse(
        r#"
{
    c: -11.1,
    d: [
        1,
        {
            f: 'ggga',
            g: {
                c: -11,
                d: [
                    1,
                    {
                        f: 'gg"ga'
                    }
                ]
            }
        }
    ]
}
"#
        .to_string(),
    )?;
    dbg!(&res);
    println!("{}", res.to_json_string());

    Ok(())
}
