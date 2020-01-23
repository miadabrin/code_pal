use crate::app::ui_component::{ListTextEditor, TableEditor, UIEventProcessor};
use crate::app::{ActionPayload, Event};
use crate::todo::todo::{EditableRowItem, EditableStateItem};
use crate::todo::todo::{Note, Project, TodoItem};
use crate::util::TabsState;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::error::Error;
use std::fs::File;
use std::io::prelude::*;
use std::rc::Rc;

struct Person {
    name: String,
    age: u8,
    phones: Vec<String>,
}

use std::sync::mpsc::Sender;

#[derive(Debug)]
pub enum CodePalAction {
    AddToDoItem,
    AddNote,
    AddProject,
    None,
}

#[derive(Serialize, Deserialize)]
pub struct AppState {
    pub todo_items: Rc<RefCell<Vec<TodoItem>>>,
    #[serde(default = "default_projects")]
    pub projects: Rc<RefCell<Vec<Project>>>,
}

fn default_projects() -> Rc<RefCell<Vec<Project>>> {
    Rc::new(RefCell::new(vec![Project::new(vec![
        String::from(""),
        String::from(""),
        String::from(""),
    ])]))
}

impl AppState {
    pub fn new() -> AppState {
        let mut pathbuf = dirs::home_dir().unwrap();
        pathbuf.push("code_pal.json");
        let path = pathbuf.as_path();
        if path.exists() {
            let json_file = File::open(path).expect("file not found");
            let app_state: AppState =
                serde_json::from_reader(json_file).expect("error while reading json");
            return app_state;
        }
        AppState {
            todo_items: Rc::new(RefCell::new(vec![TodoItem::new(String::from(""))])),
            projects: Rc::new(RefCell::new(vec![Project::new(vec![
                String::from(""),
                String::from(""),
                String::from(""),
            ])])),
        }
    }
    pub fn save(&mut self) {
        let serialized = serde_json::to_string(&self).unwrap();

        let mut pathbuf = dirs::home_dir().unwrap();
        pathbuf.push("code_pal.json");
        let path = pathbuf.as_path();
        let display = path.display();

        // Open a file in write-only mode, returns `io::Result<File>`
        let mut file = match File::create(&path) {
            Err(why) => panic!("couldn't create {}: {}", display, why.description()),
            Ok(file) => file,
        };

        // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
        match file.write_all(serialized.as_bytes()) {
            Err(why) => panic!("couldn't write to {}: {}", display, why.description()),
            Ok(_) => {}
        }
    }
}

pub struct App<'a> {
    pub app_state: AppState,
    pub todo_items: ListTextEditor<TodoItem>,
    pub notes: ListTextEditor<Note>,
    pub projects: TableEditor<Project>,
    pub current_action: CodePalAction,
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, app_state: AppState, sender: Sender<Event>) -> App<'a> {
        let mut a = App {
            app_state,
            title,
            todo_items: ListTextEditor::new(
                String::from("Todo Items"),
                None::<Rc<RefCell<Vec<TodoItem>>>>,
                Sender::clone(&sender),
            ),
            notes: ListTextEditor::new(
                String::from("Notes"),
                None::<Rc<RefCell<Vec<Note>>>>,
                Sender::clone(&sender),
            ),
            projects: TableEditor::new(
                String::from("Projects"),
                None::<Rc<RefCell<Vec<Project>>>>,
                vec![
                    String::from("Name"),
                    String::from("url"),
                    String::from("Directory"),
                ],
                vec![20, 50, 50],
                Sender::clone(&sender),
            ),
            current_action: CodePalAction::None,
            should_quit: false,
            tabs: TabsState::new(vec!["Notes", "Projects"]),
        };
        a.init_state();
        a
    }

    pub fn init_state(&mut self) {
        self.todo_items.current_text = Some(self.app_state.todo_items.clone());
        self.projects.current_text = Some(self.app_state.projects.clone());
        self.set_notes();
    }

    pub fn on_up(&mut self) {}

    pub fn on_down(&mut self) {}

    pub fn on_right(&mut self) {
        self.tabs.next();
    }

    pub fn on_left(&mut self) {
        self.tabs.previous();
    }

    pub fn on_add_todo(&mut self) {
        self.on_stop_action();
        self.current_action = CodePalAction::AddToDoItem;
        self.todo_items.on_activate();
    }

    pub fn on_add_note(&mut self) {
        self.on_stop_action();
        self.current_action = CodePalAction::AddNote;
        self.notes.on_activate();
    }

    pub fn on_add_project(&mut self) {
        self.on_stop_action();
        self.current_action = CodePalAction::AddProject;
        self.projects.on_activate();
    }

    pub fn on_stop_action(&mut self) {
        if let Some(x) = self.current_active_item() {
            x.on_deactivate();
            self.current_action = CodePalAction::None;
        }
    }

    pub fn current_active_item(&mut self) -> Option<&mut dyn UIEventProcessor> {
        match self.current_action {
            CodePalAction::AddToDoItem => Some(&mut self.todo_items),
            CodePalAction::AddNote => Some(&mut self.notes),
            CodePalAction::AddProject => Some(&mut self.projects),
            _ => None,
        }
    }

    pub fn on_key(&mut self, event: KeyEvent) {
        match (event.code, event.modifiers) {
            (KeyCode::Char('q'), KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            (KeyCode::Char('a'), KeyModifiers::CONTROL) => self.on_add_todo(),
            (KeyCode::Char('n'), KeyModifiers::CONTROL) => self.on_add_note(),
            (KeyCode::Char('p'), KeyModifiers::CONTROL) => self.on_add_project(),
            (KeyCode::Char('s'), KeyModifiers::CONTROL) => self.on_save(),
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

    pub fn on_enter(&mut self) {}

    pub fn on_tick(&mut self) {}

    pub fn set_notes(&mut self) {
        let item_ref = self.app_state.todo_items.clone();
        let selected_index = self.todo_items.current_selection;
        match selected_index {
            Some(index) => {
                let mut borrowed = item_ref.borrow_mut();
                let todo = borrowed.get_mut(index).unwrap();
                self.notes.current_text = Some(todo.notes.clone());
                self.notes.select_item(0);
            }
            None => {
                self.notes.current_text = None;
            }
        };
    }

    pub fn on_action(&mut self, action: ActionPayload) {
        match action {
            ActionPayload::Selection(sender, _) => {
                if sender == "Todo Items" {
                    self.set_notes();
                }
            }
        }
    }

    pub fn on_save(&mut self) {
        self.app_state.save();
    }
}
