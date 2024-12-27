// Modules
use crate::job::reset_eligibility;
use crate::{state::AppState, Error, Result};
use tokio_cron_scheduler::JobScheduler;

pub struct CronJobScheduler;

impl CronJobScheduler {
    pub async fn run(app_state: AppState) -> Result<()> {
        // Creates a new CRON job scheduler
        let scheduler = JobScheduler::new()
            .await
            .map_err(|_| Error::SchedulerError)?;

        // Sets up the jobs to be added to the scheduler
        let reset_eligibility_job = reset_eligibility::job(app_state.clone()).map_err(|_| Error::SchedulerError)?;

        // Adds the jobs to the scheduler
        scheduler.add(reset_eligibility_job).await.map_err(|_| Error::SchedulerError)?;

        // Runs the scheduler
        scheduler
            .start()
            .await
            .map_err(|_| Error::SchedulerError)?;

        Ok(())
    }
}
