use std::any::{Any, TypeId};
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::fmt::Debug;

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

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct TypeKey {
    type_id: TypeId,
    counter_id: usize,
}

impl TypeKey {
    fn new(type_id: TypeId, counter_id: usize) -> TypeKey {
        TypeKey { type_id, counter_id }
    }
}

impl Hash for TypeKey {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.type_id.hash(state);
        self.counter_id.hash(state);
    }
}

#[derive(Debug)]
struct DataSeries {
    data: HashMap<TypeKey, Box<dyn NamedAny>>,
    order: Vec<TypeKey>,
    last_counter_id: usize,
}

impl DataSeries {
    fn new() -> DataSeries {
        DataSeries {
            data: HashMap::new(),
            order: Vec::new(),
            last_counter_id: 0,
        }
    }

    fn push<T: 'static + Debug>(&mut self, value: T) {
        let key = TypeKey::new(
            TypeId::of::<T>(),
            self.last_counter_id,
        );
        self.order.push(key);
        self.data.insert(key, Box::new(value));
        self.last_counter_id += 1;
    }

    fn print_all(&self) {
        for key in &self.order {
            if let Some(value) = self.data.get(key) {
                value.print_value();
            }
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
