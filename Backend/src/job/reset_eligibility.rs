// Modules
use crate::model::ModelManager;
use crate::state::AppState;
use crate::job::{Result, Error};
use crate::model::user_notification::{UserNotificationModelController, UserNotificationForCreateBulk};

use tokio_cron_scheduler::Job;
use tracing::{debug, error};

// Reset Eligibility Job that runs every day at 00:00:00 UTC (8am MYT)
pub fn job(app_state: AppState) -> Result<Job> {
    let job = Job::new_async("0 0 0 * * *", move |_uuid, _lock| {
        let model_manager = app_state.clone().model_manager;
        
        Box::pin(async move {
            debug!("{:<12} - reset_eligibility\n", "JOB");

            match update_user_eligibility(&model_manager).await {
                Ok(user_ids) => {
                    if let Err(e) = push_notification(&model_manager, user_ids).await {
                        error!("{:<12} - reset_eligibility failed: {:?}", "ERROR - JOB", e);
                    };
                }
                Err(e) => {
                    error!("{:<12} - reset_eligibility failed: {:?}", "ERROR - JOB", e);
                }
            }
        })
    }).map_err(|_| Error::JobBuildError("reset_eligibility".to_string()))?;

    Ok(job)
}

// Function that updates the eligibility of ineligible users who haven't donated in the last 90 days.
async fn update_user_eligibility(model_manager: &ModelManager) -> Result<Vec<i64>> {
    let db = model_manager.db();

    let affected_users = sqlx::query_scalar(
        "WITH latest_donations AS (
            SELECT user_id, MAX(created_at) AS latest_donation_date
            FROM donation_history
            GROUP BY user_id
        ),
        updated_users AS (
            UPDATE users
            SET eligibility = 'Eligible'
            FROM latest_donations
            WHERE users.id = latest_donations.user_id
              AND users.eligibility = 'Ineligible'
              AND latest_donations.latest_donation_date < NOW() - INTERVAL '90 days'
            RETURNING users.id
        )
        SELECT id FROM updated_users;"
    )
    .fetch_all(db)
    .await?;

    Ok(affected_users)
}

// Function that pushes notifications users who have their eligibility updates.
async fn push_notification(model_manager: &ModelManager, user_ids: Vec<i64>) -> Result<()> {
    let user_notifications = UserNotificationForCreateBulk {
        description: "You are now eligible to donate blood.".to_string(),
        redirect: Some("user-dashboard".to_string()),
        user_ids,
    }; 
    UserNotificationModelController::create_bulk(&model_manager, user_notifications).await?;

    Ok(())
}