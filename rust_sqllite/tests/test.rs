use rust_sqllite::{extract, query, load};

#[test]
fn test_extract() {
    let file_path = extract();
    assert_eq!(file_path.unwrap(), "data/Behaviors.csv");
}

#[test]
fn test_load() {
    let dataset = "data/Behaviors.csv";
    let result = load(dataset);

    assert_eq!(result.unwrap(), "Behavior.db");
}

#[test]
fn test_query() {
    let select_query = "SELECT * FROM Behaviors WHERE id = 3;";
    let result = query(select_query);

    assert!(result.is_ok());
}