mod logger;

fn add_homework(
    db: &DatabaseConnection,
    class_id: i32,
    subject_id: i32,
    title: String,
    description: Option<String>,
    due_date: Date,
    creator_id: Uuid,
) {
}

fn get_homework_for_class(db: &DatabaseConnection, class_id: i32) {}

fn update_homework(
    db: &DatabaseConnection,
    id: i32,
    title: String,
    description: Option<String>,
    due_date: Date,
    creator_id: Uuid,
) {
}

fn delete_homework(db: &DatabaseConnection, id: i32) {}

fn mark_homework_comleted(
    db: &DatabaseConnection,
    user_id: Uuid,
    homework_id: i32,
    is_completed: bool,
) {
}

fn get_completed_homework_ids(db: &DatabaseConnection, user_id: Uuid) {}
