use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

// Task Trait
trait Task {
    fn execute(&self) -> usize;
}

// SumTask struct
struct SumTask {
    n1: usize,
    n2: usize,
}

impl SumTask {
    fn new(n1: usize, n2: usize) -> Self {
        Self { n1, n2 }
    }
}

impl Task for SumTask {
    fn execute(&self) -> usize {
        self.n1 + self.n2
    }
}

// LenTask struct
struct LenTask {
    s: String,
}

impl LenTask {
    fn new(s: String) -> Self {
        Self { s }
    }
}

impl Task for LenTask {
    fn execute(&self) -> usize {
        self.s.len()
    }
}

// Shared task queue type
type TaskQueue = Arc<Mutex<VecDeque<Box<dyn Task>>>>;

// Tasker struct
struct Tasker {
    queue: TaskQueue,
}

impl Tasker {
    // Create a new Tasker
    fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    // Schedule a task
    fn schedule_task(&mut self, task: Box<dyn Task>) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(task);
    }

    // Create a new Tasker linked to the current queue
    fn get_tasker(&self) -> Tasker {
        Tasker {
            queue: Arc::clone(&self.queue),
        }
    }

    // Create an Executer linked to the current queue
    fn get_executer(&self) -> Executer {
        Executer {
            queue: Arc::clone(&self.queue),
        }
    }
}

// Executer struct
struct Executer {
    queue: TaskQueue,
}

impl Executer {
    // Execute the next task in the queue
    fn execute_task(&mut self) -> Option<usize> {
        let mut queue = self.queue.lock().unwrap();
        queue.pop_front().map(|task| task.execute())
    }
}

fn main() {
    macro_rules! sum_task {
        (let $task: ident = $n1: literal + $n2: literal) => {
            let $task: Box<dyn Task> = Box::new(SumTask::new($n1, $n2));
        };
    }

    sum_task!(let t1 = 1+1);
    sum_task!(let t2 = 2+2);
    sum_task!(let t3 = 3+3);
    sum_task!(let t4 = 4+4);
    sum_task!(let t5 = 5+5);
    sum_task!(let t6 = 6+6);
    sum_task!(let t7 = 7+7);

    let mut tasker = Tasker::new();
    let mut executer = tasker.get_executer();

    println!("{:?}", executer.execute_task()); // Output: None

    tasker.schedule_task(t1);
    tasker.schedule_task(t2);

    println!("{:?}", executer.execute_task()); // Output: Some(2)

    tasker.schedule_task(t3);
    tasker.schedule_task(t4);
    tasker.schedule_task(t5);
    tasker.schedule_task(t6);
    tasker.schedule_task(t7);

    println!("{:?}", executer.execute_task()); // Output: Some(4)
    println!("{:?}", executer.execute_task()); // Output: Some(6)
    println!("{:?}", executer.execute_task()); // Output: Some(8)
    println!("{:?}", executer.execute_task()); // Output: Some(10)
    println!("{:?}", executer.execute_task()); // Output: Some(12)
    println!("{:?}", executer.execute_task()); // Output: Some(14)
    println!("{:?}", executer.execute_task()); // Output: None
}
