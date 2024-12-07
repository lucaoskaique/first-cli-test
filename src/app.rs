#[derive(Default)]
pub struct App {
    pub tasks: Vec<String>,
    pub input: String,
    pub selected_index: usize,
}

impl App {
    pub fn new() -> App {
        App {
            tasks: vec![
                "Complete project".to_string(),
                "Write documentation".to_string(),
            ],
            input: String::new(),
            selected_index: 0,
        }
    }

    pub fn add_task(&mut self) {
        if !self.input.is_empty() {
            self.tasks.push(self.input.clone());
            self.input.clear();
        }
    }

    pub fn delete_task(&mut self) {
        if !self.tasks.is_empty() {
            self.tasks.remove(self.selected_index);
            if self.selected_index >= self.tasks.len() && !self.tasks.is_empty() {
                self.selected_index = self.tasks.len() - 1;
            }
        }
    }

    pub fn move_selection_up(&mut self) {
        if !self.tasks.is_empty() {
            self.selected_index = self.selected_index.saturating_sub(1);
        }
    }

    pub fn move_selection_down(&mut self) {
        if !self.tasks.is_empty() {
            self.selected_index = (self.selected_index + 1).min(self.tasks.len() - 1);
        }
    }
}
