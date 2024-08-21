use task_manager::task::{get_tasks, perform_action};

fn main() {
    let mut tasks = get_tasks();
    perform_action(&mut tasks);
}
