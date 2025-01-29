use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

trait DebugAny {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn format_value(&self) -> String;
}

impl<T: Any + Debug> DebugAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn format_value(&self) -> String {
        format!("{:?}", &self)
    }
}

struct DataSeries {
    data: HashMap<usize, Box<dyn DebugAny>>,
    type_map: HashMap<usize, String>,
    index: Vec<usize>,
    last_index_id: usize,
}

impl DataSeries {
    pub fn new() -> DataSeries {
        DataSeries {
            data: HashMap::new(),
            type_map: HashMap::new(),
            index: Vec::new(),
            last_index_id: 0,
        }
    }

    fn push<T: Any + Debug>(&mut self, value: T) {
        self.index.push(self.last_index_id);
        self.data.insert(self.last_index_id, Box::new(value));
        self.type_map.insert(self.last_index_id, std::any::type_name::<T>().to_string());
        self.last_index_id += 1;
    }

    fn get<T: Any + Debug>(&self, index: usize) -> Option<&T> {
        if let Some(key) = self.index.get(index) {
            if let Some(value) = self.data.get(key) {
                return value.as_any().downcast_ref::<T>();
            }
        }
        None
    }

    fn get_mut<T: Any + Debug>(&mut self, index: usize) -> Option<&mut T> {
        if let Some(key) = self.index.get(index) {
            if let Some(value) = self.data.get_mut(key) {
                return value.as_any_mut().downcast_mut::<T>();
            }
        }
        None
    }

    fn update<T: Any + Debug>(&mut self, index: usize, new_value: T) {
        if let Some(key) = self.index.get(index) {
            if let Some(value) = self.data.get_mut(key) {
                let mut_ref = value.as_any_mut().downcast_mut::<T>().unwrap();
                *mut_ref = new_value;
            }

            if let Some(value) = self.type_map.get_mut(key) {
                *value = std::any::type_name::<T>().to_string();
            }
        }
    }

    fn remove(&mut self, index: usize) -> Option<Box<(dyn DebugAny + 'static)>> {
        if index < self.data.len() {
            let removed = if let Some(key) = self.index.get(index) {
                let _ = self.type_map.remove(key);
                self.data.remove(key)
            } else {
                None
            };
            let _ = self.index.remove(index);
            return removed;
        }
        None
    }

    fn print(&self) {
        for (i, key) in self.index.iter().enumerate() {
            if let Some(value) = self.data.get(key) {
                if let Some(value_type) = self.type_map.get (key) {
                    println!("{}  {}: {}", i, value_type, value.format_value());
                }
            }
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn new_data_series() {
        let series = DataSeries::new();

        assert_eq!(series.data.len(), 0);
        assert_eq!(series.index.len(), 0);
        assert_eq!(series.last_index_id, 0);
    }

    #[test]
    fn push_item_to_series() {
        let mut series = DataSeries::new();

        series.push(1);

        assert_eq!(series.data.len(), 1);
        assert_eq!(series.index.len(), 1);
        assert_eq!(series.last_index_id, 1);
        assert_eq!(series.get::<i32>(0), Some(&1));
        assert_eq!(series.get::<&str>(0), None);
    }

    #[test]
    fn delete_item_from_series() {
        let mut series = DataSeries::new();

        series.push(1);
        series.push("test");
        series.push([1.2, 3.4]);

        assert_eq!(series.data.len(), 3);
        assert_eq!(series.index.len(), 3);
        assert_eq!(series.last_index_id, 3);

        let removed_value = series.remove(1);
        assert!(removed_value.is_some());
        assert_eq!(
            *removed_value
                .unwrap()
                .as_any()
                .downcast_ref::<&str>()
                .unwrap(),
            "test"
        );

        assert_eq!(series.data.len(), 2);
        assert_eq!(series.index.len(), 2);
        assert_eq!(series.last_index_id, 3);
        assert_eq!(series.get::<&str>(1), None);
        assert_eq!(series.get::<[f64; 2]>(1), Some(&[1.2, 3.4]));

        series.push("test");
        assert_eq!(series.data.len(), 3);
        assert_eq!(series.index.len(), 3);
        assert_eq!(series.last_index_id, 4);
        assert_eq!(series.get::<&str>(2), Some(&"test"));
    }

    #[test]
    fn get_mut_series_value() {
        let mut series = DataSeries::new();

        series.push(1);

        assert_eq!(series.data.len(), 1);
        assert_eq!(series.index.len(), 1);
        assert_eq!(series.last_index_id, 1);
        assert_eq!(series.get::<i32>(0), Some(&1));

        let mut_ref = series.get_mut(0);

        assert!(mut_ref.is_some());

        *mut_ref.unwrap() = 100;

        assert_eq!(series.data.len(), 1);
        assert_eq!(series.index.len(), 1);
        assert_eq!(series.last_index_id, 1);
        assert_eq!(series.get::<i32>(0), Some(&100));
    }

    #[test]
    fn update_item_in_series() {
        let mut series = DataSeries::new();

        series.push(1);

        assert_eq!(series.data.len(), 1);
        assert_eq!(series.index.len(), 1);
        assert_eq!(series.last_index_id, 1);
        assert_eq!(series.get::<i32>(0), Some(&1));

        series.update::<i32>(0, 100);

        assert_eq!(series.data.len(), 1);
        assert_eq!(series.index.len(), 1);
        assert_eq!(series.last_index_id, 1);
        assert_eq!(series.get::<i32>(0), Some(&100));
    }

    #[ignore]
    #[test]
    fn print_test() {
        let mut series = DataSeries::new();

        series.push(1);
        series.push("test");
        series.push([1.2, 3.4]);
        series.push(vec![1, 3, 4, 5, 12, 30, 12]);

        series.print();

        println!("\n{:?}\n", series.get::<Vec<i32>>(3).unwrap());
    }
}
