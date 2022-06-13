use crate::task_entity::TaskEntity;

pub struct TasksAggregatorEntity {
    tasks: Vec<TaskEntity>,
    selected_task_index: usize,
}

impl TasksAggregatorEntity {
    pub fn new() -> TasksAggregatorEntity {
        TasksAggregatorEntity {
            selected_task_index: 0,
            tasks: Vec::new(),
        } 
    }

    pub fn is_empty(&self) -> bool {
        self.tasks.is_empty()
    }

    pub fn get_selected_task(&mut self) -> Option<&mut TaskEntity> {
        self.tasks.get_mut(self.selected_task_index)
    }

    pub fn count_tasks(&self) -> usize {
        self.tasks.len()
    }

    pub fn selected_task_index(&self) -> usize {
        self.selected_task_index
    }

    pub fn list_tasks(&self) -> &Vec<TaskEntity> {
        &self.tasks
    }

    pub fn add_task(&mut self, value: TaskEntity) {
        self.tasks.push(value);
    }

    pub fn remove_task(&mut self, index: usize) {
        let last_index = self.count_tasks() - 1;

        if (last_index > 0) && (last_index == index) {
            self.select_task(self.selected_task_index - 1);
        }

        self.tasks.remove(index);
    }

    pub fn select_task(&mut self, index: usize) {
        if self.count_tasks() - 1 >= index {
            self.selected_task_index = index;
        }
    }
}
