use rust_dataframes::data_frame_modules::data_series::DataSeries;

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
    assert_eq!(
        series.get::<Vec<i32>>(3),
        Some(&vec!(1, 3, 4, 5, 12, 30, 12))
    )
}
