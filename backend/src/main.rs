use llm_convos::data::models::groupchat_1::GROUPCHAT_1;
use llm_convos::services::scheduler_service::SchedulerService;

#[tokio::main]
pub async fn main() {
    dotenvy::dotenv().expect("Failed to load .env file"); // Loads .env into std::env
    println!("Hello, world!");

    let pool = sqlx::SqlitePool::connect("sqlite::memory:").await.unwrap();

    // Run migrations to create tables
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");
    println!("Migrations complete");

    let scheduler = SchedulerService::new(GROUPCHAT_1.clone(), pool);
    scheduler.schedule_groupchat().await.unwrap();
}
