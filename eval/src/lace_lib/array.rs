use crate::object::Object;

pub fn append(obj: Object, appendable: Object) -> Object {
    let mut arr = match obj {
        Object::Array(arr) => arr,
        _ => {
            return Object::Error(format!("Expected an array, Got {}", obj.kind()));
        }
    };

    if let Object::Array(x) = appendable {
        arr.extend(x);
    } else {
        arr.push(appendable);
    }

    Object::Array(arr)
}
