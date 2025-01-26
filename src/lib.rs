use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::fmt::{self, Debug};

trait NamedAny: Any + Debug {
    fn type_name(&self) -> &'static str;
    fn as_any(&self) -> &dyn Any;
    fn print_value(&self);
}

impl<T: Any + Debug> NamedAny for T {
    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn print_value(&self) {
        println!("{:?}", self);
    }
}

#[derive(Debug, Eq, PartialEq)]
struct TypeKey {
    type_id: TypeId,
    name: String,
}

impl TypeKey {
    fn new(type_id: TypeId, name: String) -> TypeKey {
        TypeKey { type_id, name }
    }
}

impl Hash for TypeKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.type_id.hash(state);
        self.name.hash(state);
    }
}

#[derive(Debug)]
struct DataSeries {
    data: HashMap<TypeKey, Box<dyn NamedAny>>,
}

impl DataSeries {
    fn new() -> DataSeries {
        DataSeries {
            data: HashMap::new(),
        }
    }

    fn push<T: 'static + Debug>(&mut self, value: T) {
        let key = TypeKey::new(
            TypeId::of::<T>(),
            String::from(format!("col_{}", self.data.len())),
        );
        self.data.insert(key, Box::new(value));
    }

    fn print_all(&self) {
        for (key, value) in &self.data {
            println!("Column: {}", key.name);
            value.print_value();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_hvec() {
        let mut series = DataSeries::new();
        series.push(1);
        series.push(vec!["a", "b", "c"]);
        series.push([4.32, 1.79]);
        series.push(5);

        series.print_all();
    }
}
