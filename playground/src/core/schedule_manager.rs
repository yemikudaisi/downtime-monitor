use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use super::types::{ServiceParameters, ServiceVerificationResult};
use tokio::signal;
use tokio_cron_scheduler::{Job, JobScheduler};
use uuid::Uuid;

#[derive(Clone)]
pub struct JobSchedulerManager {
    scheduler: JobScheduler,
    job_map: Arc<Mutex<HashMap<Uuid, Option<i64>>>>, // Maps UUID to ServiceParameter ID
}

impl JobSchedulerManager {
    pub async fn new() -> Self {
        JobSchedulerManager {
            scheduler: JobScheduler::new().await.unwrap(),
            job_map: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    #[allow(unused)]
    pub async fn add_service(
        &self,
        service_verifier: fn(ServiceParameters) -> ServiceVerificationResult,
        service_parameters: ServiceParameters,
        notifier: fn(ServiceVerificationResult),
    ) -> Result<Uuid, Box<dyn std::error::Error>> {
        let seconds = service_parameters.interval.unwrap_or(60);
        let job_schedule = seconds_to_cron_exp(seconds);

        let job_id = Uuid::new_v4();
        let sp_clone = service_parameters.clone();
        let job_map = Arc::clone(&self.job_map);

        self.scheduler
            .add(Job::new(job_schedule.as_str(), move |_uuid, _l| {
                let result = service_verifier(sp_clone.clone());
                notifier(result)
            })?)
            .await?;

        job_map
            .lock()
            .unwrap()
            .insert(job_id, service_parameters.id);
        Ok(job_id)
    }

    #[allow(unused)]
    pub async fn start(&self) -> Result<(), Box<dyn std::error::Error>> {
        self.scheduler.start().await?;
        Ok(())
    }

    #[allow(unused)]
    pub async fn shutdown_on_ctrl_c(&mut self) {
        signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
        println!("Received Ctrl+C, shutting down...");
        self.scheduler.shutdown().await.unwrap();
    }

    #[allow(unused)]
    pub async fn stop_job_by_uuid(&self, job_id: &Uuid) -> Result<(), Box<dyn std::error::Error>> {
        self.scheduler.remove(job_id).await?;
        self.job_map.lock().unwrap().remove(&job_id);
        Ok(())
    }

    #[allow(unused)]
    pub async fn stop_job_by_service_id(
        &self,
        service_id: i64,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let job_id = {
            let job_map = self.job_map.lock().unwrap();
            job_map
                .iter()
                .find(|&(_, &id)| id == Some(service_id))
                .map(|(&job_id, _)| job_id)
        };
        if let Some(job_id) = job_id {
            self.scheduler.remove(&job_id).await?;
            self.job_map.lock().unwrap().remove(&job_id);
        }
        Ok(())
    }

    #[allow(unused)]
    pub async fn stop_all_jobs(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.scheduler.shutdown().await.unwrap();
        self.job_map.lock().unwrap().clear();
        Ok(())
    }

    #[allow(unused)]
    pub fn clear_tracking_list(&self) {
        self.job_map.lock().unwrap().clear();
    }
}

pub fn seconds_to_cron_exp(seconds: u32) -> String {
    if seconds == 0 {
        return String::from("* * * * * *");
    }

    let hours = seconds / 3600;
    let minutes = (seconds % 3600) / 60;

    if hours == 0 && minutes == 0 {
        format!("*/{} * * * * *", seconds)
    } else if hours == 0 {
        format!("{} * * * * *", minutes)
    } else {
        format!("0 {} * * * *", hours)
    }
}

#[test]
fn test_seconds_to_cron() {
    assert_eq!(seconds_to_cron_exp(10), "1/10 * * * * *")
}
