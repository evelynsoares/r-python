use crate::ir::ast::{EnvValue, Expression};

pub fn str_upper(args: Vec<EnvValue>) -> EnvValue {
    if args.len() != 1 {
        panic!("str_upper expects exactly one argument");
    }

    if let EnvValue::Exp(Expression::CStr(s)) = &args[0] {
        EnvValue::Exp(Expression::CStr(s.to_uppercase()))
    } else {
        panic!("str_upper expects a string argument");
    }
}    

pub fn str_lower(args: Vec<EnvValue>) -> EnvValue {
    if args.len() != 1 {
        panic!("str_lower expects exactly one argument");
    }
    if let EnvValue::Exp(Expression::CStr(s)) = &args[0] {
        EnvValue::Exp(Expression::CStr(s.to_lowercase()))
    } else {
        panic!("str_lower expects a string argument");
    }
}

//tamanho da string
pub fn str_length(args: Vec<EnvValue>) -> EnvValue {
    if args.len() != 1 {
        panic!("str_length expects exactly one argument");
    }
    if let EnvValue::Exp(Expression::CStr(s)) = &args[0] {
        EnvValue::Exp(Expression::CInt(s.chars().count() as i32))
    } else {
        panic!("str_length expects a string argument");
    }
}

//string reversa
pub fn str_reverse(args: Vec<EnvValue>) -> EnvValue {
    if args.len() != 1 {
        panic!("str_reverse expects exactly one argument");
    }
    if let EnvValue::Exp(Expression::CStr(s)) = &args[0] {
        EnvValue::Exp(Expression::CStr(s.chars().rev().collect()))
    } else {
        panic!("str_reverse expects a string argument");
    }
}

//conta quantas vezes aparece um char especifico
pub fn cont_chars(args: Vec<EnvValue>) -> EnvValue {
    if args.len() != 2 {
        panic!("cont_chars expects exactly two arguments");
    }
    if let (EnvValue::Exp(Expression::CStr(s)), EnvValue::Exp(Expression::CStr(c))) = (&args[0], &args[1]) {
        if c.len() != 1 {
            panic!("cont_chars expects a single character as the second argument");
        }
        let target = c.chars().next().unwrap();
        EnvValue::Exp(Expression::CInt(s.chars().filter(|&ch| ch == target).count() as i32))
    } else {
        panic!("cont_chars expects a string and a character as arguments");
    }
}

//retira todas as aparicoes de um char especifico
pub fn filter_out_char(args: Vec<EnvValue>) -> EnvValue {
    if args.len() != 2 {
        panic!("filter_out_char expects exactly two arguments");
    }
    if let (EnvValue::Exp(Expression::CStr(s)), EnvValue::Exp(Expression::CStr(c))) = (&args[0], &args[1]) {
        if c.len() != 1 {
            panic!("filter_out_char expects a single character as the second argument");
        }
        let target = c.chars().next().unwrap();
        EnvValue::Exp(Expression::CStr(s.chars().filter(|&ch| ch != target).collect()))
    } else {
        panic!("filter_out_char expects a string and a character as arguments");
    }
}

//centraliza a string e preenche o resto com um char
pub fn center(args: Vec<EnvValue>) -> EnvValue {
    if args.len() != 3 {
        panic!("center expects exactly three arguments");
    }
    if let (
        EnvValue::Exp(Expression::CStr(s)),
        EnvValue::Exp(Expression::CInt(width)),
        EnvValue::Exp(Expression::CStr(fillchar)),
    ) = (&args[0], &args[1], &args[2])
    {
        if fillchar.len() != 1 {
            panic!("center expects a single character as the fill character");
        }
        let fill = fillchar.chars().next().unwrap();
        let pad = (width as usize).saturating_sub(s.len());
        let left = pad / 2;
        let right = pad - left;
        EnvValue::Exp(Expression::CStr(format!(
            "{}{}{}",
            fill.to_string().repeat(left),
            s,
            fill.to_string().repeat(right)
        )))
    } else {
        panic!("center expects a string, an integer width, and a character as arguments");
    }
}

//acha uma substring
pub fn find(args: Vec<EnvValue>) -> EnvValue {
    if args.len() < 2 || args.len() > 4 {
        panic!("find expects between 2 and 4 arguments");
    }
    if let (EnvValue::Exp(Expression::CStr(s)), EnvValue::Exp(Expression::CStr(sub))) = (&args[0], &args[1]) {
        let start = if args.len() > 2 {
            if let EnvValue::Exp(Expression::CInt(n)) = &args[2] {
                *n as usize
            } else {
                panic!("find expects an integer as the third argument (start index)");
            }
        } else {
            0
        };

        let end = if args.len() > 3 {
            if let EnvValue::Exp(Expression::CInt(n)) = &args[3] {
                Some(*n as usize)
            } else {
                panic!("find expects an integer as the fourth argument (end index)");
            }
        } else {
            None
        };

        let end = end.unwrap_or(s.len());
        let result = s.get(start..end)?.find(sub).map(|i| i + start);

        match result {
            Some(index) => EnvValue::Exp(Expression::CInt(index as i32)),
            None => EnvValue::Exp(Expression::CInt(-1)),
        }
    } else {
        panic!("find expects two strings as first arguments");
    }
}

//recebe uma lista de strings e junta elas em uma string so
pub fn join(args: Vec<EnvValue>) -> EnvValue {
    if args.len() != 2 {
        panic!("join expects exactly two arguments");
    }
    if let (EnvValue::Exp(Expression::CStr(sep)), EnvValue::Exp(Expression::CList(iter))) = (&args[0], &args[1]) {
        let strings: Vec<String> = iter.iter().map(|val| {
            if let EnvValue::Exp(Expression::CStr(s)) = val {
                s.clone()
            } else {
                panic!("join expects a list of strings as the second argument");
            }
        }).collect();

        EnvValue::Exp(Expression::CStr(strings.join(sep)))
    } else {
        panic!("join expects a string and a list of strings as arguments");
    }
}

//partition usando "world"
pub fn partition(args: Vec<EnvValue>) -> EnvValue {
    if args.len() != 2 {
        panic!("partition expects exactly two arguments");
    }
    if let (EnvValue::Exp(Expression::CStr(s)), EnvValue::Exp(Expression::CStr(sep))) = (&args[0], &args[1]) {
        match s.find(sep) {
            Some(index) => EnvValue::Exp(Expression::CTuple(vec![
                EnvValue::Exp(Expression::CStr(s[..index].to_string())),
                EnvValue::Exp(Expression::CStr(sep.clone())),
                EnvValue::Exp(Expression::CStr(s[index + sep.len()..].to_string())),
            ])),
            None => EnvValue::Exp(Expression::CTuple(vec![
                EnvValue::Exp(Expression::CStr(s.clone())),
                EnvValue::Exp(Expression::CStr(String::new())),
                EnvValue::Exp(Expression::CStr(String::new())),
            ])),
        }
    } else {
        panic!("partition expects two string arguments");
    }
}

// substitui os N primeiros caracter "old" por um novo caracter "new" 
pub fn replace(args: Vec<EnvValue>) -> EnvValue {
    if args.len() < 3 || args.len() > 4 {
        panic!("replace expects between 3 and 4 arguments");
    }
    if let (EnvValue::Exp(Expression::CStr(s)), EnvValue::Exp(Expression::CStr(old)), EnvValue::Exp(Expression::CStr(new))) = (&args[0], &args[1], &args[2]) {
        let count = if args.len() == 4 {
            if let EnvValue::Exp(Expression::CInt(n)) = &args[3] {
                *n
            } else {
                panic!("replace expects an integer as the fourth argument (count)");
            }
        } else {
            -1
        };

        let result = if count < 0 {
            s.replace(old, new)
        } else {
            let mut result = s.clone();
            let mut occurrences = 0;
            while occurrences < count {
                if let Some(pos) = result.find(old) {
                    result = format!("{}{}{}", &result[..pos], new, &result[pos + old.len()..]);
                    occurrences += 1;
                } else {
                    break;
                }
            }
            result
        };

        EnvValue::Exp(Expression::CStr(result))
    } else {
        panic!("replace expects three string arguments and an optional integer");
    }
}

#[cfg(test)]
mod tests {
    use crate::ir::ast::{EnvValue, Expression};
    use crate::util::*; 

    #[test]
    #[should_panic(expected = "Incorrect result for str_upper()")]
    fn test_str_upper_valid_strings() {
        let result = str_upper(vec![EnvValue::Exp(Expression::CStr(String::from("hello")))]);
        match result {
            EnvValue::Exp(Expression::CStr(res_value)) => assert_eq!(res_value, "HELLO"),
            _ => panic!("Incorrect result for str_upper('hello')"),
        }
    }
    

    #[test]
    #[should_panic(expected = "str_upper expects exactly one argument")]
    fn test_str_upper_invalid_number_of_arguments() {
        str_upper(vec![]);
    }

    #[test]
    #[should_panic(expected = "str_upper expects exactly one argument")]
    fn test_str_upper_invalid_number_of_arguments_multiple() {
        str_upper(vec![
            EnvValue::Exp(Expression::CStr(String::from("hello"))),
            EnvValue::Exp(Expression::CStr(String::from("world"))),
        ]);
    }

    #[test]
    #[should_panic(expected = "str_upper expects a string argument")]
    fn test_str_upper_invalid_argument_type() {
        str_upper(vec![EnvValue::Exp(Expression::CInt(42))]);
    }

