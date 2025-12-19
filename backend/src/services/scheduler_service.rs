use crate::models::groupchat::Groupchat;
use crate::services::groupchat_service::GroupchatService;
/**
 * Service for scheduling tasks
 */
use sqlx::SqlitePool;
use std::sync::Arc;
use tokio_cron_scheduler::{Job, JobScheduler, JobSchedulerError};

pub struct SchedulerService {
    groupchat: Groupchat,
    pool: SqlitePool,
}

impl SchedulerService {
    pub fn new(groupchat: Groupchat, pool: SqlitePool) -> Self {
        Self { groupchat, pool }
    }

    pub async fn schedule_groupchat(&self) -> Result<(), JobSchedulerError> {
        let sched = JobScheduler::new().await?;

        let groupchat_service = Arc::new(GroupchatService::new(
            self.groupchat.clone(),
            self.pool.clone(),
        ));

        sched
            .add(Job::new_async("0 * * * * *", move |_uuid, _l| {
                let service = Arc::clone(&groupchat_service);
                Box::pin(async move {
                    match service.chat().await {
                        Ok(_) => {}
                        Err(e) => eprintln!("Chat error: {}", e),
                    }
                })
            })?)
            .await?;

        println!("Scheduled groupchat");

        // Start the scheduler
        sched.start().await?;

        // Keep the program running
        loop {
            tokio::time::sleep(tokio::time::Duration::from_secs(60)).await;
        }
    }
}
