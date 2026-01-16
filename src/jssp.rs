use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct Job {
    pub id: usize,
    pub operations: Vec<Operation>,
}

#[derive(Debug, Clone)]
pub struct Operation {
    pub job_id: usize,
    pub operation_id: usize,
    pub machine_id: usize,
    pub duration: f64,
}

#[derive(Debug, Clone)]
pub struct ScheduledOperation {
    pub job_id: usize,
    pub operation_id: usize,
    pub machine_id: usize,
    pub start_time: f64,
    pub end_time: f64,
    pub duration: f64,
}

pub struct JsspSolver {
    pub jobs: Vec<Job>,
    pub num_machines: usize,
}

impl JsspSolver {
    pub fn new(jobs: Vec<Job>, num_machines: usize) -> Self {
        Self { jobs, num_machines }
    }

    /// Greedy algorithm: Schedule operations based on earliest available time
    pub fn solve_greedy(&self) -> Vec<ScheduledOperation> {
        let mut schedule = Vec::new();
        let mut machine_available_time: HashMap<usize, f64> = HashMap::new();
        let mut job_completion_time: HashMap<usize, f64> = HashMap::new();

        // Initialize machine and job availability
        for i in 0..self.num_machines {
            machine_available_time.insert(i, 0.0);
        }
        for job in &self.jobs {
            job_completion_time.insert(job.id, 0.0);
        }

        // Create a list of all operations with their dependencies
        let mut pending_operations: Vec<(usize, usize, &Operation)> = Vec::new();
        for job in &self.jobs {
            for (op_idx, op) in job.operations.iter().enumerate() {
                pending_operations.push((job.id, op_idx, op));
            }
        }

        // Schedule operations in order for each job
        for job in &self.jobs {
            for (op_idx, operation) in job.operations.iter().enumerate() {
                let machine_time = *machine_available_time.get(&operation.machine_id).unwrap_or(&0.0);
                let job_time = *job_completion_time.get(&job.id).unwrap_or(&0.0);
                
                // Operation can start when both the machine and previous job operation are done
                let start_time = machine_time.max(job_time);
                let end_time = start_time + operation.duration;

                schedule.push(ScheduledOperation {
                    job_id: job.id,
                    operation_id: op_idx,
                    machine_id: operation.machine_id,
                    start_time,
                    end_time,
                    duration: operation.duration,
                });

                // Update availability times
                machine_available_time.insert(operation.machine_id, end_time);
                job_completion_time.insert(job.id, end_time);
            }
        }

        schedule
    }

    pub fn calculate_makespan(&self, schedule: &[ScheduledOperation]) -> f64 {
        schedule.iter()
            .map(|op| op.end_time)
            .fold(0.0, f64::max)
    }
}

/// Generate a random JSSP instance
pub fn generate_random_instance(num_jobs: usize, num_machines: usize, min_duration: f64, max_duration: f64) -> Vec<Job> {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    // Ensure valid duration range
    let min_dur = min_duration.max(1.0);
    let max_dur = max_duration.max(min_dur + 0.1);
    
    let mut jobs = Vec::new();
    
    for job_id in 0..num_jobs {
        let mut machines: Vec<usize> = (0..num_machines).collect();
        // Shuffle machines for random order
        for i in (1..machines.len()).rev() {
            let j = rng.gen_range(0..=i);
            machines.swap(i, j);
        }
        
        let operations: Vec<Operation> = machines.iter().enumerate()
            .map(|(op_id, &machine_id)| {
                let duration = rng.gen_range(min_dur..=max_dur);
                Operation {
                    job_id,
                    operation_id: op_id,
                    machine_id,
                    duration,
                }
            })
            .collect();
        
        jobs.push(Job {
            id: job_id,
            operations,
        });
    }
    
    jobs
}
