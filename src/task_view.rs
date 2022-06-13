use ncurses::*;

use crate::tasks_aggregator_entity::*;
use crate::task_entity::*;


pub static REGULAR_PAIR: i16 = 1;
pub static HIGH_LIGHT: i16 = 2;

pub struct TasksView<'a> {
    pub(crate) tasks_aggregator: &'a mut TasksAggregatorEntity,
}

impl TasksView<'_> {
    pub fn create_task(&mut self, task: TaskEntity) {
        self.tasks_aggregator.add_task(task);   
    }

    pub fn show_tasks(&self) {
        if self.tasks_aggregator.is_empty() {
            return; 
        }

        self.tasks_aggregator
            .list_tasks()
            .iter()
            .enumerate()
            .for_each(|(index, task)| {
                let color_pair = self.get_color_pair(index);
                self.show_task(task, color_pair);
            });
    }

    pub fn swish_task_status(&mut self) {
        if self.tasks_aggregator.is_empty() {
            return; 
        }

        let selected_task = self.tasks_aggregator.get_selected_task();

        match selected_task {
            Some(task) if task.is_compleat() => task.un_compleat(),
            Some(task) if !task.is_compleat() => task.compleat(),
            _ => (),
        }
    }

    pub fn remove_selected_task(&mut self) {
        if self.tasks_aggregator.is_empty() {
            return; 
        }

        let selected_task = self.tasks_aggregator.selected_task_index();
        self.tasks_aggregator.remove_task(selected_task);
    }

    pub fn move_down(&mut self) {
        if self.tasks_aggregator.is_empty() {
            return;
        }

        let last_index = self.tasks_aggregator.count_tasks() - 1;
        let next_index = self.tasks_aggregator.selected_task_index() + 1;

        if next_index <= last_index {
            self.tasks_aggregator.select_task(next_index);
        }
    }

    pub fn move_up(&mut self) {
        if self.tasks_aggregator.is_empty() {
            return; 
        }

        if self.tasks_aggregator.selected_task_index() > 0 {
            let next_index = self.tasks_aggregator.selected_task_index() - 1;
            self.tasks_aggregator.select_task(next_index);
        }
    }

    fn show_task(&self, task: &TaskEntity, color_pair: u32) {
        let task_tittle = format!("{}\n", task.to_string());
        attron(color_pair);
        addstr(task_tittle.as_str());
    }

    fn get_color_pair(&self, index: usize) -> u32 {
        let selected_index = self.tasks_aggregator.selected_task_index();

        if index == selected_index {
            return COLOR_PAIR(HIGH_LIGHT);
        }
        
        return COLOR_PAIR(REGULAR_PAIR);
    }
}
