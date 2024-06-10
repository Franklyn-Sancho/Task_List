/* #[cfg(test)]
mod tests {
    use crate::todo_list::{TodoList, Priority};

    use chrono::NaiveTime;

    #[test]
    fn test_add_task() {
        let mut todo_list = TodoList::new();
        let task = "Teste".to_string();
        let time = NaiveTime::from_hms_opt(12, 0, 0).expect("invalid time");
        /* et priority = Priority::Alta; */
        assert!(todo_list.add_task(task, time, Priority::Alta));
        assert_eq!(todo_list.tasks.len(), 1);
        assert_eq!(todo_list.tasks[0], ("Teste".to_string(), time, Priority::Alta));
    }
} */