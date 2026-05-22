use rusty_tasks::{TaskFilter, TaskStore};

#[test]
fn adds_tasks_with_incrementing_ids() {
    let mut store = TaskStore::new();

    let first = store.add("learn ownership");
    let second = store.add("write tests");

    assert_eq!(first.id, 1);
    assert_eq!(second.id, 2);
    assert_eq!(store.tasks().len(), 2);
}

#[test]
fn completes_a_task_by_id() {
    let mut store = TaskStore::new();
    store.add("learn Result");

    let completed = store.complete(1).expect("task should exist");

    assert!(completed.done);
    assert!(store.tasks()[0].done);
}

#[test]
fn returns_none_when_completing_missing_task() {
    let mut store = TaskStore::new();

    assert!(store.complete(99).is_none());
}

#[test]
fn filters_open_and_done_tasks() {
    let mut store = TaskStore::new();
    store.add("open task");
    store.add("done task");
    store.complete(2);

    let open = store.filtered(TaskFilter::Open);
    let done = store.filtered(TaskFilter::Done);
    let all = store.filtered(TaskFilter::All);

    assert_eq!(open.len(), 1);
    assert_eq!(open[0].title, "open task");
    assert_eq!(done.len(), 1);
    assert_eq!(done[0].title, "done task");
    assert_eq!(all.len(), 2);
}

#[test]
fn serializes_and_loads_tasks_from_text() {
    let mut store = TaskStore::new();
    store.add("learn ownership");
    store.add("write tests");
    store.complete(2);

    let encoded = store.to_storage_text();
    let loaded = TaskStore::from_storage_text(&encoded).expect("storage text should parse");

    assert_eq!(loaded.tasks().len(), 2);
    assert_eq!(loaded.tasks()[0].title, "learn ownership");
    assert!(!loaded.tasks()[0].done);
    assert_eq!(loaded.tasks()[1].title, "write tests");
    assert!(loaded.tasks()[1].done);
}

#[test]
fn rejects_invalid_storage_lines() {
    let result = TaskStore::from_storage_text("not-a-valid-line");

    assert!(result.is_err());
}
