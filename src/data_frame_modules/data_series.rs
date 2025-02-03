use std::any::Any;
use std::collections::HashMap;
use std::fmt::Debug;

use super::series_index::SeriesIndex;

pub trait DebugAny {
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

pub struct DataSeries<T: Clone + Debug + Ord> {
    pub data: HashMap<usize, Box<dyn DebugAny>>,
    pub type_map: HashMap<usize, String>,
    pub index: Vec<SeriesIndex<T>>,
    pub next_key_value: usize,
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

    pub fn push<U: Any + Debug>(&mut self, label: T, value: U) {
        let key = self.next_key_value;
        self.index.push(SeriesIndex::new(label, key));
        self.data.insert(self.next_key_value, Box::new(value));
        self.type_map
            .insert(self.next_key_value, std::any::type_name::<U>().to_string());
        self.next_key_value += 1;
    }

    pub fn get<U: Any + Debug>(&self, index: usize) -> Option<&U> {
        if let Some(index_val) = self.index.get(index) {
            let key = &index_val.key;
            if let Some(value) = self.data.get(key) {
                return value.as_any().downcast_ref::<U>();
            }
        }
        None
    }

    pub fn get_mut<U: Any + Debug>(&mut self, index: usize) -> Option<&mut U> {
        if let Some(index_val) = self.index.get(index) {
            let key = &index_val.key;
            if let Some(value) = self.data.get_mut(key) {
                return value.as_any_mut().downcast_mut::<U>();
            }
        }
        None
    }

    pub fn update<U: Any + Debug>(&mut self, index: usize, new_value: U) {
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

    pub fn remove(&mut self, index: usize) -> Option<Box<(dyn DebugAny + 'static)>> {
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

    pub fn sort(&mut self) {
        self.index.sort_by(|a, b| a.label.cmp(&b.label));
    }

    pub fn print(&self) {
        for index_val in self.index.iter() {
            let key = &index_val.key;
            if let Some(value) = self.data.get(key) {
                if let Some(value_type) = self.type_map.get(key) {
                    println!(
                        "{:?}  {}: {}",
                        index_val.label,
                        value_type,
                        value.format_value()
                    );
                }
            }
        }
    }

    pub fn print_reverse(&self) {
        let mut reversed_index = self.index.clone();
        reversed_index.sort_by(|a, b| b.label.cmp(&a.label));
        for index_val in reversed_index.iter() {
            let key = &index_val.key;
            if let Some(value) = self.data.get(key) {
                if let Some(value_type) = self.type_map.get(key) {
                    println!(
                        "{:?}  {}: {}",
                        index_val.label,
                        value_type,
                        value.format_value()
                    );
                }
            }
        }
    }
}

impl Default for DataSeries<usize> {
    fn default() -> Self {
        Self::new()
    }
}
