use crate::ir::ast::{EnvValue, Expression};
pub fn str_upper(args: Vec<EnvValue>) -> EnvValue {
    if args.len() != 1 {
        panic!("str_upper expects exactly one argument");
    }

    if let EnvValue::Exp(Expression::CString(s)) = &args[0] {
        EnvValue::Exp(Expression::CString(s.to_uppercase()))
    } else {
        panic!("str_upper expects a string argument");
    }
}    

pub fn str_lower(args: Vec<EnvValue>) -> EnvValue {
    if args.len() != 1 {
        panic!("str_lower expects exactly one argument");
    }
    if let EnvValue::Exp(Expression::CString(s)) = &args[0] {
        EnvValue::Exp(Expression::CString(s.to_lowercase()))
    } else {
        panic!("str_lower expects a string argument");
    }
}

pub fn str_length(args: Vec<EnvValue>) -> EnvValue {
    if args.len() != 1 {
        panic!("str_length expects exactly one argument");
    }
    if let EnvValue::Exp(Expression::CString(s)) = &args[0] {
        EnvValue::Exp(Expression::CInt(s.chars().count() as i32))
    } else {
        panic!("str_length expects a string argument");
    }
}

pub fn str_reverse(args: Vec<EnvValue>) -> EnvValue {
    if args.len() != 1 {
        panic!("str_reverse expects exactly one argument");
    }
    if let EnvValue::Exp(Expression::CString(s)) = &args[0] {
        EnvValue::Exp(Expression::CString(s.chars().rev().collect()))
    } else {
        panic!("str_reverse expects a string argument");
    }
}

//conta quantas vezes aparece um char especifico
pub fn cont_chars(args: Vec<EnvValue>) -> EnvValue {
    if args.len() != 2 {
        panic!("cont_chars expects exactly two arguments");
    }
    if let (EnvValue::Exp(Expression::CString(s)), EnvValue::Exp(Expression::CString(c))) = (&args[0], &args[1]) {
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
    if let (EnvValue::Exp(Expression::CString(s)), EnvValue::Exp(Expression::CString(c))) = (&args[0], &args[1]) {
        if c.len() != 1 {
            panic!("filter_out_char expects a single character as the second argument");
        }
        let target = c.chars().next().unwrap();
        EnvValue::Exp(Expression::CString(s.chars().filter(|&ch| ch != target).collect()))
    } else {
        panic!("filter_out_char expects a string and a character as arguments");
    }
}

// substitui os N primeiros caracteres "old" por um novo caracter "new" 
pub fn replace(args: Vec<EnvValue>) -> EnvValue {
    if args.len() < 3 || args.len() > 4 {
        panic!("replace expects between 3 and 4 arguments");
    }
    if let (EnvValue::Exp(Expression::CString(s)), EnvValue::Exp(Expression::CString(old)), EnvValue::Exp(Expression::CString(new))) = (&args[0], &args[1], &args[2]) {
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

        EnvValue::Exp(Expression::CString(result))
    } else {
        panic!("replace expects three string arguments and an optional integer");
    }
}

#[cfg(test)]
mod tests {
    use crate::ir::ast::{EnvValue, Expression};
    use crate::util::*; 

   #[test]
    fn test_str_upper_valid_strings() {
        let result = str_upper(vec![EnvValue::Exp(Expression::CString(String::from("hello")))]);
        match result {
            EnvValue::Exp(Expression::CString(res_value)) => assert_eq!(res_value, "HELLO"),
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
            EnvValue::Exp(Expression::CString(String::from("hello"))),
            EnvValue::Exp(Expression::CString(String::from("world"))),
        ]);
    }

    #[test]
    #[should_panic(expected = "str_upper expects a string argument")]
    fn test_str_upper_invalid_argument_type() {
        str_upper(vec![EnvValue::Exp(Expression::CInt(42))]);
    }
}
