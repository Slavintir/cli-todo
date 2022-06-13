mod task_entity;
mod tasks_aggregator_entity;
mod task_view;

use ncurses::*;
use task_view::*;
use task_entity::*;
use tasks_aggregator_entity::*;

fn main() {
    initscr();
    start_color();
    curs_set(CURSOR_VISIBILITY::CURSOR_INVISIBLE);
    let mut is_quit = false;

    init_pair(REGULAR_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIGH_LIGHT, COLOR_BLACK, COLOR_WHITE);

    let tasks_aggregator = &mut TasksAggregatorEntity::new();
    tasks_aggregator.add_task(TaskEntity::new("Wake up."));
    tasks_aggregator.add_task(TaskEntity::new("Make a cup of coffee."));
    tasks_aggregator.add_task(TaskEntity::new("Learn the rust."));

    while !is_quit {
        let mut tasks_view = TasksView { tasks_aggregator };
        help();
        tasks_view.show_tasks();
        refresh();

        match getch() as u8 as char {
            'q' => is_quit = true,
            'w' => tasks_view.move_up(),
            's' => tasks_view.move_down(),
            'x' => tasks_view.swish_task_status(),
            'r' => tasks_view.remove_selected_task(),
            'c' => tasks_view.create_task(TaskEntity::new("<what are you schedule?>")),
            'h' => help(),
            _ => (),
        }

        timeout(32);
        clear();
    }

    clear();
    endwin();
}

fn help() {
    static HELP_TEXT: [&str; 9] = [
        "Press button to:",
        "\n",
        "'q' - quit;",
        "'w' - move up;",
        "'s' - move down;",
        "'x' - swish task status;",
        "'r' - remove selected task;",
        "'c' - create task;",
        "\n\n",
    ];

    attron(COLOR_PAIR(REGULAR_PAIR));
    addstr(HELP_TEXT.join("\n").as_str());
}
