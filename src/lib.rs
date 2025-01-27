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
    order: Vec<usize>,
    last_order_id: usize,
}

impl DataSeries {
    pub fn new() -> DataSeries {
        DataSeries {
            data: HashMap::new(),
            order: Vec::new(),
            last_order_id: 0,
        }
    }

    fn push<T: Any + Debug>(&mut self, value: T) {
        self.order.push(self.last_order_id);
        self.data.insert(self.last_order_id, Box::new(value));
        self.last_order_id += 1;
    }

    fn get<T: Any + Debug>(&self, index: usize) -> Option<&T> {
        if let Some(key) = self.order.get(index) {
            if let Some(value) = self.data.get(key) {
                return value.as_any().downcast_ref::<T>();
            }
        } 
        None
    }

    fn get_mut<T: Any + Debug>(&mut self, index: usize) -> Option<&mut T> {
        if let Some(key) = self.order.get(index) {
            if let Some(value) = self.data.get_mut(key) {
                return value.as_any_mut().downcast_mut::<T>();
            }
        }
        None
    }

    fn update<T: Any + Debug>(&mut self, index: usize, new_value: T) {
        if let Some(key) = self.order.get(index) {
            if let Some(value) = self.data.get_mut(key) {
                let mut_ref = value.as_any_mut().downcast_mut::<T>().unwrap();
                *mut_ref = new_value;
            }
        }
    }

    fn remove(&mut self, index: usize) -> Option<Box<(dyn DebugAny + 'static)>> {
        if let Some(key) = self.order.get(index) {
            let _ = self.order.remove(index);
            return self.data.remove(&index);
        }
        None
    }

    fn print(&self) {
        for key in self.order.iter() {
            if let Some(value) = self.data.get(key) {
                println!("{}", value.format_value());
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
        assert_eq!(series.order.len(), 0);
        assert_eq!(series.last_order_id, 0);
    }

    #[test]
    fn push_item_to_series() {
        let mut series = DataSeries::new();

        series.push(1);

        assert_eq!(series.data.len(), 1);
        assert_eq!(series.order.len(), 1);
        assert_eq!(series.last_order_id, 1);
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
        assert_eq!(series.order.len(), 3);
        assert_eq!(series.last_order_id, 3);

        let removed_value = series.remove(1);
        assert!(removed_value.is_some());
        assert_eq!(*removed_value.unwrap().as_any().downcast_ref::<&str>().unwrap(), "test");

        assert_eq!(series.data.len(), 2);
        assert_eq!(series.order.len(), 2);
        assert_eq!(series.last_order_id, 3);
        assert_eq!(series.get::<&str>(1), None);
        assert_eq!(series.get::<[f64; 2]>(1), Some(&[1.2, 3.4]));
    }

    #[test]
    fn get_mut_series_value() {
        let mut series = DataSeries::new();

        series.push(1);

        assert_eq!(series.data.len(), 1);
        assert_eq!(series.order.len(), 1);
        assert_eq!(series.last_order_id, 1);
        assert_eq!(series.get::<i32>(0), Some(&1));

        let mut_ref = series.get_mut(0);

        assert!(mut_ref.is_some());

        *mut_ref.unwrap() = 100;

        assert_eq!(series.data.len(), 1);
        assert_eq!(series.order.len(), 1);
        assert_eq!(series.last_order_id, 1);
        assert_eq!(series.get::<i32>(0), Some(&100));
    }

    #[test]
    fn update_item_in_series() {
        let mut series = DataSeries::new();

        series.push(1);

        assert_eq!(series.data.len(), 1);
        assert_eq!(series.order.len(), 1);
        assert_eq!(series.last_order_id, 1);
        assert_eq!(series.get::<i32>(0), Some(&1));

        series.update::<i32>(0, 100);

        assert_eq!(series.data.len(), 1);
        assert_eq!(series.order.len(), 1);
        assert_eq!(series.last_order_id, 1);
        assert_eq!(series.get::<i32>(0), Some(&100));
    }
}