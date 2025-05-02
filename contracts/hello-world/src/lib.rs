#![no_std]

use soroban_sdk::{contract, contractimpl, contracttype, Address, Env, String, Vec, log};

#[contracttype]
#[derive(Clone)]
pub struct JobListing {
    pub id: u64,
    pub employer: Address,
    pub title: String,
    pub description: String,
    pub location: String,
    pub timestamp: u64,
}

#[contracttype]
pub enum DataKey {
    JobById(u64),
    JobCount,
}

#[contract]
pub struct JobBoard;

#[contractimpl]
impl JobBoard {
    pub fn post_job(env: Env, employer: Address, title: String, description: String, location: String, timestamp: u64) {
        let mut job_count: u64 = env.storage().instance().get(&DataKey::JobCount).unwrap_or(0);
        job_count += 1;

        let job = JobListing {
            id: job_count,
            employer,
            title,
            description,
            location,
            timestamp,
        };

        env.storage().instance().set(&DataKey::JobById(job_count), &job);
        env.storage().instance().set(&DataKey::JobCount, &job_count);

        log!(&env, "Job #{} posted", job_count);
    }

    pub fn get_job(env: Env, job_id: u64) -> JobListing {
        env.storage()
            .instance()
            .get(&DataKey::JobById(job_id))
            .expect("Job not found")
    }

    pub fn total_jobs(env: Env) -> u64 {
        env.storage().instance().get(&DataKey::JobCount).unwrap_or(0)
    }
}
