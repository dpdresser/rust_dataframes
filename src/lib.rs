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

#[derive(Debug, Clone)]
struct SeriesIndex<T: Clone + Debug + Ord> {
    label: T,
    key: usize,
}

impl<T: Clone + Debug + Ord> SeriesIndex<T> {
    pub fn new(label: T, key: usize) -> SeriesIndex<T> {
        SeriesIndex { label, key }
    }
}

struct DataSeries<T: Clone + Debug + Ord> {
    data: HashMap<usize, Box<dyn DebugAny>>,
    type_map: HashMap<usize, String>,
    index: Vec<SeriesIndex<T>>,
    next_key_value: usize,
}

impl<T: Clone + Debug + Ord> DataSeries<T> {
    pub fn new() -> DataSeries<T> {
        DataSeries {
            data: HashMap::new(),
            type_map: HashMap::new(),
            index: Vec::new(),
            next_key_value: 0,
        }
    }

    fn push<U: Any + Debug>(&mut self, label: T, value: U) {
        let key = self.next_key_value;
        self.index.push(SeriesIndex::new(label, key));
        self.data.insert(self.next_key_value, Box::new(value));
        self.type_map.insert(self.next_key_value, std::any::type_name::<U>().to_string());
        self.next_key_value += 1;
    }

    fn get<U: Any + Debug>(&self, index: usize) -> Option<&U> {
        if let Some(index_val) = self.index.get(index) {
            let key = &index_val.key;
            if let Some(value) = self.data.get(key) {
                return value.as_any().downcast_ref::<U>();
            }
        }
        None
    }

    fn get_mut<U: Any + Debug>(&mut self, index: usize) -> Option<&mut U> {
        if let Some(index_val) = self.index.get(index) {
            let key = &index_val.key; 
            if let Some(value) = self.data.get_mut(key) {
                return value.as_any_mut().downcast_mut::<U>();
            }
        }
        None
    }

    fn update<U: Any + Debug>(&mut self, index: usize, new_value: U) {
        if let Some(index_val) = self.index.get(index) {
            let key = &index_val.key;
            if let Some(value) = self.data.get_mut(key) {
                let mut_ref = value.as_any_mut().downcast_mut::<U>().unwrap();
                *mut_ref = new_value;
            }

            if let Some(value) = self.type_map.get_mut(key) {
                *value = std::any::type_name::<T>().to_string();
            }
        }
    }

    fn remove(&mut self, index: usize) -> Option<Box<(dyn DebugAny + 'static)>> {
        if index < self.data.len() {
            let removed = if let Some(index_val) = self.index.get(index) {
                let key = &index_val.key;
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

    fn sort(&mut self) {
        self.index.sort_by(|a, b| a.label.cmp(&b.label));
    }

    fn print(&self) {
        for index_val in self.index.iter() {
            let key = &index_val.key;
            if let Some(value) = self.data.get(key) {
                if let Some(value_type) = self.type_map.get(key) {
                    println!("{:?}  {}: {}", index_val.label, value_type, value.format_value());
                }
            }
        }
    }

    fn print_reverse(&self) {
        let mut reversed_index = self.index.clone();
        reversed_index.sort_by(|a, b| b.label.cmp(&a.label));
        for index_val in reversed_index.iter() {
            let key = &index_val.key;
            if let Some(value) = self.data.get(key) {
                if let Some(value_type) = self.type_map.get (key) {
                    println!("{:?}  {}: {}", index_val.label, value_type, value.format_value());
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
        let series = DataSeries::<String>::new();

        assert_eq!(series.data.len(), 0);
        assert_eq!(series.index.len(), 0);
        assert_eq!(series.next_key_value, 0);
    }

    #[test]
    fn push_item_to_series() {
        let mut series = DataSeries::<String>::new();

        series.push("A".to_string(), 1);

        assert_eq!(series.data.len(), 1);
        assert_eq!(series.index.len(), 1);
        assert_eq!(series.next_key_value, 1);
        assert_eq!(series.get::<i32>(0), Some(&1));
        assert_eq!(series.get::<&str>(0), None);
    }

    #[test]
    fn delete_item_from_series() {
        let mut series = DataSeries::<String>::new();

        series.push("Item 1".to_string(), 1);
        series.push("Item 2".to_string(), "test");
        series.push("Item 3".to_string(), [1.2, 3.4]);

        assert_eq!(series.data.len(), 3);
        assert_eq!(series.index.len(), 3);
        assert_eq!(series.next_key_value, 3);

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
        assert_eq!(series.next_key_value, 3);
        assert_eq!(series.get::<&str>(1), None);
        assert_eq!(series.get::<[f64; 2]>(1), Some(&[1.2, 3.4]));

        series.push("New Item".to_string(), "test");
        assert_eq!(series.data.len(), 3);
        assert_eq!(series.index.len(), 3);
        assert_eq!(series.next_key_value, 4);
        assert_eq!(series.get::<&str>(2), Some(&"test"));
    }

    #[test]
    fn get_mut_series_value() {
        let mut series = DataSeries::<String>::new();

        series.push("Item 1".to_string(), 1);

        assert_eq!(series.data.len(), 1);
        assert_eq!(series.index.len(), 1);
        assert_eq!(series.next_key_value, 1);
        assert_eq!(series.get::<i32>(0), Some(&1));

        let mut_ref = series.get_mut(0);

        assert!(mut_ref.is_some());

        *mut_ref.unwrap() = 100;

        assert_eq!(series.data.len(), 1);
        assert_eq!(series.index.len(), 1);
        assert_eq!(series.next_key_value, 1);
        assert_eq!(series.get::<i32>(0), Some(&100));
    }

    #[test]
    fn update_item_in_series() {
        let mut series = DataSeries::<String>::new();

        series.push("Item 1".to_string(), 1);

        assert_eq!(series.data.len(), 1);
        assert_eq!(series.index.len(), 1);
        assert_eq!(series.next_key_value, 1);
        assert_eq!(series.get::<i32>(0), Some(&1));

        series.update::<i32>(0, 100);

        assert_eq!(series.data.len(), 1);
        assert_eq!(series.index.len(), 1);
        assert_eq!(series.next_key_value, 1);
        assert_eq!(series.get::<i32>(0), Some(&100));
    }

    #[test]
    #[should_panic]
    fn update_item_in_series_with_new_type() {
        let mut series = DataSeries::<String>::new();

        series.push("Item 1".to_string(), 1);

        assert_eq!(series.data.len(), 1);
        assert_eq!(series.index.len(), 1);
        assert_eq!(series.next_key_value, 1);
        assert_eq!(series.get::<i32>(0), Some(&1));

        series.update::<f32>(0, 1.1);
    }

    #[ignore]
    #[test]
    fn print_test() {
        let mut series = DataSeries::<String>::new();

        series.push("Item 1".to_string(), 1);
        series.push("Item 2".to_string(), "test");
        series.push("Item 3".to_string(), [1.2, 3.4]);
        series.push("Item 4".to_string(), vec![1, 3, 4, 5, 12, 30, 12]);

        series.print();

        println!("\n{:?}\n", series.get::<Vec<i32>>(3).unwrap());
    }

    #[ignore]
    #[test]
    fn print_reverse_test() {
        let mut series = DataSeries::<String>::new();

        series.push("Item 1".to_string(), 1);
        series.push("Item 2".to_string(), "test");
        series.push("Item 3".to_string(), [1.2, 3.4]);
        series.push("Item 4".to_string(), vec![1, 3, 4, 5, 12, 30, 12]);

        series.print_reverse();

        println!("\n{:?}\n", series.get::<Vec<i32>>(3).unwrap());
    }

    #[test]
    fn test_sort_by_index_label() {
        let mut series = DataSeries::<usize>::new();

        series.push(4, 1);
        series.push(1, "test");
        series.push(3, [1.2, 3.4]);
        series.push(10, vec![1, 3, 4, 5, 12, 30, 12]);

        assert_eq!(series.get::<i32>(0), Some(&1));

        series.sort();

        assert_eq!(series.get::<&str>(0), Some(&"test"));
        assert_eq!(series.get::<[f64; 2]>(1), Some(&[1.2, 3.4]));
        assert_eq!(series.get::<i32>(2), Some(&1));
        assert_eq!(series.get::<Vec<i32>>(3), Some(&vec!(1, 3, 4, 5, 12, 30, 12)))
    }
}
