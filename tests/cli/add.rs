use todors::cli::Add;
use todors::handlers::handle_add;
use todors::storage::TaskStorage;

#[test]
fn it_add_the_new_task() {
    let todo_file =
        tempfile::NamedTempFile::new().expect("Failed to create temporary file for the test");
    let todo_file_name = todo_file.as_ref().to_path_buf();

    let storage = TaskStorage::new(todo_file_name.clone());

    let params = Add::new(
        vec![
            "this".to_string(),
            "is".to_string(),
            "a".to_string(),
            "test".to_string(),
        ],
        None,
    );

    handle_add(params, storage).unwrap();

    let result_file = std::fs::read_to_string(todo_file_name).unwrap();

    assert!(result_file.contains("this is a test"));
}

#[test]
fn it_add_the_new_task_with_priority_pass_as_arg() {
    let todo_file =
        tempfile::NamedTempFile::new().expect("Failed to create temporary file for the test");
    let todo_file_name = todo_file.as_ref().to_path_buf();

    let storage = TaskStorage::new(todo_file_name.clone());

    let params = Add::new(
        vec![
            "this".to_string(),
            "is".to_string(),
            "a".to_string(),
            "test".to_string(),
        ],
        Some('A'),
    );

    handle_add(params, storage).unwrap();

    let result_file = std::fs::read_to_string(todo_file_name).unwrap();

    assert!(result_file.starts_with("(A)"));
}
