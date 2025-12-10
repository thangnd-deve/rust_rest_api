use crate::database;

pub struct UserRepository;

impl UserRepository {
    pub async fn create_user(
        db: &database::Database,
        username: &str,
        email: &str,
        password_hash: &str,
    ) -> Result<i32, Box<dyn std::error::Error>> {
        let rec = db.pool.get().await?
            .query_one(
                "INSERT INTO users (username, email, password) VALUES ($1, $2, $3) RETURNING id",
                &[&username, &email, &password_hash],
            )
            .await?;
        Ok(rec.get("id"))
    }
}