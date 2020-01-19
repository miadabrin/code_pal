use crate::app::ui_component::{ListTextEditor, UIEventProcessor};
use crate::todo::todo::EditableStateItem;
use crate::todo::todo::TodoItem;
use crate::util::TabsState;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::cell::RefCell;
use std::rc::Rc;

pub enum CodePalAction {
    AddToDoItem,
    NoteEdit,
    DescriptionEdit,
    None,
}

pub struct AppState {
    pub todo_items: Rc<RefCell<Vec<TodoItem>>>,
}

impl AppState {
    pub fn new() -> AppState {
        AppState {
            todo_items: Rc::new(RefCell::new(vec![TodoItem::new(String::from(""))])),
        }
    }
}

pub struct App<'a> {
    pub app_state: AppState,
    pub todo_items: ListTextEditor<TodoItem>,
    pub current_action: CodePalAction,
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str, app_state: AppState) -> App<'a> {
        let mut a = App {
            app_state,
            title,
            todo_items: ListTextEditor::new(
                String::from("Todo Items"),
                None::<Rc<RefCell<Vec<TodoItem>>>>,
            ),
            current_action: CodePalAction::None,
            should_quit: false,
            tabs: TabsState::new(vec!["Notes"]),
        };
        a.init_state();
        a
    }

    pub fn init_state(&mut self) {
        self.todo_items.current_text = Some(self.app_state.todo_items.clone());
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
        self.current_action = CodePalAction::AddToDoItem;
        self.todo_items.on_activate();
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
            _ => None,
        }
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
