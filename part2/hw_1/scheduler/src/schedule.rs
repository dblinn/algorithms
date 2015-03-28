use job::Job;
use std::cmp::Ordering;

pub struct Schedule<'a> {
	pub jobs: &'a mut Vec<Job>,
}

impl<'a> Schedule<'a> {
	fn sort_by_difference(&mut self) {
		self.jobs.sort_by(|a, b| {
			let order = b.difference().cmp(& a.difference());

			if (order == Ordering::Equal) {
				b.weight.cmp(& a.weight)
			}
			else {
				order
			}
		});
	}

	fn sort_by_ratio(&mut self) {
		self.jobs.sort_by(|a, b| { b.ratio().partial_cmp(& a.ratio()).unwrap() });
	}

	fn total_weighted_completion_time(&self) -> i32 {
		let (weighted_time, accum_time) = self.jobs.iter().fold((0,0), sum_completion_time);
		weighted_time
	}

}

fn sum_completion_time(accumulator: (i32, i32), job: &Job) -> (i32, i32) {
	let (weighted_sum, running_completion_time) = accumulator;
	let next_time = running_completion_time + job.duration;

	(weighted_sum + next_time * job.weight, next_time)
}

#[test]
fn test_sort_by_difference() {
	let mut v = vec![
		Job {weight: 10, duration: 5},
		Job {weight: 2, duration: 3},
		Job {weight: 3, duration: 4} ];
	let mut sched = Schedule { jobs: &mut v };

	sched.sort_by_difference();
	assert_eq!(sched.jobs.iter().map(|job| {job.weight}).collect::<Vec<i32>>(), vec![10, 3, 2]);
}

#[test]
fn test_sort_by_ratio() {
	let mut v = vec![
		Job {weight: 10, duration: 5},
		Job {weight: 2, duration: 3},
		Job {weight: 3, duration: 4},
		Job {weight: 1, duration: 1}
	];
	let mut sched = Schedule { jobs: &mut v };

	sched.sort_by_ratio();
	assert_eq!(sched.jobs.iter().map(|job| {job.weight}).collect::<Vec<i32>>(), vec![10, 1, 3, 2]);
}

#[test]
fn total_weighted_completion_time() {
	let mut v = vec![
		Job {weight: 10, duration: 5},
		Job {weight: 2, duration: 3},
		Job {weight: 3, duration: 4}
	];
	let sched = Schedule { jobs: &mut v };

	let total = sched.total_weighted_completion_time();
	assert_eq!(total, 10 * 5 + 2 * 8 + 3 * 12);
}