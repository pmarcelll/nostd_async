pub fn main() {
    let runtime = nostd_async::Runtime::new();

    let mut t1 = nostd_async::Task::new(async { println!("Task 1") });
    let mut t2 = nostd_async::Task::new(async { println!("Task 2") });

    let h1 = t1.spawn(&runtime);
    let h2 = t2.spawn(&runtime);

    h1.join();
    h2.join();
}
