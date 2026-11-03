mod logger;

fn add_user(
    db: &DatabaseConnection,
    email: String,
    username: String,
    class_id: i32,
    token: String,
    token_expires_at: DateTime,
) {
}

fn find_user_by_token(db: &DatabaseConnection, token: String) {}

fn update_token(db: &DatabaseConnection, user_id: Uuid, token: String, expires_at: DateTime) {}

fn set_banned(db: &DatabaseConnection, user_id: Uuid, banned: bool) {}
