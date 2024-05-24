use crate::object::Object;

pub fn split(obj: Object, delimeter: Object) -> Object {
    if let Object::Str(s) = obj {
        let delim = match delimeter {
            Object::Str(s) => s,
            Object::Char(c) => c.to_string(),
            _ => {
                return Object::Error(format!(
                    "Expected String or character, got {}",
                    delimeter.kind()
                ));
            }
        };
        let split_str = s.split(&delim).map(|sec| Object::Str(sec.into())).collect();
        Object::Array(split_str)
    } else {
        Object::Error(format!(
            "{} does not have any associated function first()",
            obj.kind()
        ))
    }
}

pub fn chars(obj: Object) -> Object {
    if let Object::Str(s) = obj {
        let chs = s.chars().map(|ch| Object::Char(ch)).collect();
        Object::Array(chs)
    } else {
        Object::Error(format!(
            "{} does not have any associated function chars()",
            obj.kind()
        ))
    }
}
