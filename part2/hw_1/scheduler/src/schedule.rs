use job::Job;
use std::cmp::Ordering;

pub struct Schedule {
	pub jobs: Box<Vec<Job>>,
}

impl Schedule {
	pub fn sort_by_difference(&mut self) {
		// One way to access the inner contents of the jobs box is to take a reference to it.
		// This works:
		let ref mut jobs_ref = *self.jobs;
		jobs_ref.sort_by(compare_jobs_by_difference);

		// Note that it's necessary to take a reference because otherwise it would attempt to actually
		// borrow self.jobs. This would be okay if we owned self, but we don't! We've only borrowed self.
		// Fails to compile:
		//
		// let mut borrowed_jobs = *self.jobs;
		// borrowed_jobs.sort_by(compare_jobs_by_difference);
		//
		// error: cannot move out of borrowed content
		// 		let mut jobs_ref = *self.jobs;
		//                          ^~~~

	}

	pub fn sort_by_ratio(&mut self) {
		// Here's another way to avoid the borrow problem
		(*self.jobs).sort_by(|a, b| { b.ratio().partial_cmp(& a.ratio()).unwrap() });
	}

	pub fn total_weighted_completion_time(&self) -> i64 {
		let (weighted_time, _) = self.jobs.iter().fold((0,0), sum_completion_time);
		weighted_time
	}
}

fn compare_jobs_by_difference(job_a: &Job, job_b: &Job) -> Ordering {
	let order = job_b.difference().cmp(& job_a.difference());

	if (order == Ordering::Equal) { job_b.weight.cmp(& job_a.weight) }
	else { order }
}

fn sum_completion_time(accumulator: (i64, i64), job: &Job) -> (i64, i64) {
	let (weighted_sum, running_completion_time) = accumulator;
	let next_time = running_completion_time + job.duration as i64;

	(weighted_sum + next_time * (job.weight as i64), next_time)
}

#[test]
fn test_sort_by_difference() {
	let mut v = vec![
		Job {weight: 10, duration: 5},
		Job {weight: 2, duration: 3},
		Job {weight: 3, duration: 4} ];
	let mut sched = Schedule { jobs: Box::new(v) };

	sched.sort_by_difference();
	assert_eq!(sched.jobs.iter().map(|job| {job.weight}).collect::<Vec<i32>>(),
			   vec![10, 3, 2]);
}

#[test]
fn test_sort_by_ratio() {
	let mut v = vec![
		Job {weight: 10, duration: 5},
		Job {weight: 2, duration: 3},
		Job {weight: 3, duration: 4},
		Job {weight: 1, duration: 1}
	];
	let mut sched = Schedule { jobs: Box::new(v) };

	sched.sort_by_ratio();
	assert_eq!(sched.jobs.iter().map(|job| {job.weight}).collect::<Vec<i32>>(),
			   vec![10, 1, 3, 2]);
}

#[test]
fn total_weighted_completion_time() {
	let mut v = vec![
		Job {weight: 10, duration: 5},
		Job {weight: 2, duration: 3},
		Job {weight: 3, duration: 4}
	];
	let sched = Schedule { jobs: Box::new(v) };

	let total = sched.total_weighted_completion_time();
	assert_eq!(total, (10 * 5 + 2 * 8 + 3 * 12) as i64);
}