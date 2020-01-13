use crate::app::ui_component::{ListTextEditor, UIComponent};
use crate::util::TabsState;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::convert::TryFrom;

pub struct ListState<I> {
    pub items: Vec<I>,
    pub selected: usize,
}

impl<I> ListState<I> {
    fn new(items: Vec<I>) -> ListState<I> {
        ListState { items, selected: 0 }
    }
    fn select_previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }
    fn select_next(&mut self) {
        if i64::try_from(self.selected).unwrap() < i64::try_from(self.items.len()).unwrap() - 1 {
            self.selected += 1
        }
    }
}

pub struct Server<'a> {
    pub name: &'a str,
    pub location: &'a str,
    pub status: &'a str,
}

pub enum CodePalAction {
    AddToDoItem,
    NoteEdit,
    DescriptionEdit,
    None,
}

pub struct App<'a> {
    pub todo_items: ListTextEditor,
    pub current_action: CodePalAction,
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub tasks: ListState<String>,
    pub servers: Vec<Server<'a>>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            todo_items: ListTextEditor::new(String::from("Todo Items"), vec![String::from("")]),
            current_action: CodePalAction::None,
            should_quit: false,
            tabs: TabsState::new(vec!["Notes", "Description"]),
            tasks: ListState::new(vec![]),
            servers: vec![Server {
                name: "NorthAmerica-1",
                location: "New York City",
                status: "Up",
            }],
        }
    }

    pub fn on_up(&mut self) {
        self.tasks.select_previous();
    }

    pub fn on_down(&mut self) {
        self.tasks.select_next();
    }

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_add_todo(&mut self) {
        self.current_action = CodePalAction::AddToDoItem;
    }

    pub fn on_stop_action(&mut self) {
        self.current_action = CodePalAction::None;
    }

    pub fn current_active_item(&mut self) -> Option<&mut impl UIComponent> {
        Some(&mut self.todo_items)
    }

    pub fn on_key(&mut self, event: KeyEvent) {
        match (event.code, event.modifiers) {
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            (KeyCode::Char('a'), KeyModifiers::CONTROL) => self.on_add_todo(),
            (KeyCode::Esc, _) => self.on_stop_action(),
            _ => {
                if let Some(x) = self.current_active_item() {
                    x.on_event(event);
                } else {
                    match (event.code, event.modifiers) {
                        (KeyCode::Left, _) => self.on_left(),
                        (KeyCode::Up, _) => self.on_up(),
                        (KeyCode::Right, _) => self.on_right(),
                        (KeyCode::Down, _) => self.on_down(),
                        (KeyCode::Enter, _) => self.on_enter(),
                        (_, _) => {}
                    }
                }
            }
        }
    }

    pub fn on_enter(&mut self) {
        match self.tabs.index {
            0 => {
                self.current_action = CodePalAction::NoteEdit;
            }
            1 => {
                self.current_action = CodePalAction::DescriptionEdit;
            }
            _ => {}
        }
    }

    pub fn on_tick(&mut self) {
        // Update self values
    }
}
