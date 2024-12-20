use crate::job::reset_eligibility;
use crate::{state::AppState, Error, Result};
use tokio_cron_scheduler::JobScheduler;

pub struct CronJobScheduler;

impl CronJobScheduler {
    pub async fn run(app_state: AppState) -> Result<()> {
        let scheduler = JobScheduler::new()
            .await
            .map_err(|_| Error::SchedulerError)?;

        let reset_eligibility_job = reset_eligibility::job(app_state.clone()).map_err(|_| Error::SchedulerError)?;

        scheduler.add(reset_eligibility_job).await.map_err(|_| Error::SchedulerError)?;

        scheduler
            .start()
            .await
            .map_err(|_| Error::SchedulerError)?;

        Ok(())
    }
}
