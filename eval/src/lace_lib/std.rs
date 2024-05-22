use crate::object::Object;

pub fn len(obj: Object) -> Object {
    let l = match obj {
        Object::Str(s) => s.len(),
        Object::Array(arr) => arr.len(),
        _ => {
            return Object::Error(format!(
                "{} does not have any associated function len()",
                obj.kind()
            ));
        }
    };

    Object::Integer(l as i64)
}
