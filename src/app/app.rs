use crate::util::TabsState;
use crossterm::event::KeyModifiers;
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

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub tasks: ListState<String>,
    pub servers: Vec<Server<'a>>,
    pub todo_add_activate: bool,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(vec!["Tab0", "Tab1"]),
            tasks: ListState::new(vec![]),
            servers: vec![
                Server {
                    name: "NorthAmerica-1",
                    location: "New York City",
                    status: "Up",
                },
                Server {
                    name: "Europe-1",
                    location: "Paris",
                    status: "Failure",
                },
                Server {
                    name: "SouthAmerica-1",
                    location: "São Paulo",
                    status: "Up",
                },
                Server {
                    name: "Asia-1",
                    location: "Singapore",
                    status: "Up",
                },
            ],
            todo_add_activate: false,
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

    pub fn on_start_add_todo(&mut self) {
        self.todo_add_activate = true
    }

    pub fn on_stop_action(&mut self) {
        self.todo_add_activate = false
    }

    pub fn on_key(&mut self, c: char, m: KeyModifiers) {
        match (c, m) {
            ('q', KeyModifiers::CONTROL) => {
                self.should_quit = true;
            }
            _ => {
                //println!("Hello, world! {:#?}", c);
            }
        }
    }

    pub fn on_tick(&mut self) {
        // Update self values
    }
}
