use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use tui::backend::Backend;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, SelectableList, Widget};
use tui::Frame;

pub trait UIEventProcessor {
	fn on_deactivate(&mut self) {}
	fn on_activate(&mut self) {}
	fn on_event(&mut self, _: KeyEvent) {}
}

pub trait UIComponent {
	fn draw<B>(&mut self, f: &mut Frame<B>, area: Rect)
	where
		B: Backend;
}

pub struct ListTextEditor {
	pub title: String,
	pub current_text: Vec<String>,
	pub current_selection: Option<usize>,
	pub active: bool,
}

impl ListTextEditor {
	pub fn new(title: String, initial_text: Vec<String>) -> ListTextEditor {
		ListTextEditor {
			title: title,
			current_text: initial_text,
			current_selection: Option::None,
			active: false,
		}
	}

	pub fn on_key(&mut self, c: char, _: KeyModifiers) {
		let selected_index = match self.current_selection {
			Some(selected_index) => selected_index,
			None => 0,
		};
		if let Some(elem) = self.current_text.get_mut(selected_index) {
			elem.push(c);
		}
	}

	pub fn on_up(&mut self) {
		self.current_selection = match self.current_selection {
			Some(x) if x > 0 => Some(x - 1),
			_ => Some(0),
		}
	}
	pub fn on_down(&mut self) {
		self.current_selection = match self.current_selection {
			Some(x) if x < self.current_text.len() - 1 => Some(x + 1),
			Some(x) if x == self.current_text.len() - 1 => {
				self.current_text.push(String::from(""));
				Some(self.current_text.len() - 1)
			}
			_ => Some(self.current_text.len()),
		}
	}
	pub fn on_backspace(&mut self) {
		match self.current_selection {
			Some(x) => {
				if let Some(elem) = self.current_text.get_mut(x) {
					if elem.len() > 0 {
						elem.pop();
					} else {
						if x >= 1 {
							self.current_text.remove(x);
							self.current_selection = Some(x - 1);
						}
					}
				}
			}
			_ => {}
		}
	}
	pub fn on_enter(&mut self) {
		if let Some(x) = self.current_selection {
			self.current_text.insert(x + 1, String::from(""));
			self.current_selection = Some(x + 1);
		}
	}
}

impl UIEventProcessor for ListTextEditor {
	fn on_deactivate(&mut self) {
		self.active = false
	}
	fn on_activate(&mut self) {
		if let None = self.current_selection {
			self.current_selection = Some(0);
		}
		self.active = true;
	}
	fn on_event(&mut self, event: KeyEvent) {
		match (event.code, event.modifiers) {
			(KeyCode::Char(c), _) => self.on_key(c, event.modifiers),
			(KeyCode::Up, _) => self.on_up(),
			(KeyCode::Down, _) => self.on_down(),
			(KeyCode::Backspace, _) => self.on_backspace(),
			(KeyCode::Enter, _) => self.on_enter(),
			(_, _) => {}
		}
	}
}
impl UIComponent for ListTextEditor {
	fn draw<B>(&mut self, f: &mut Frame<B>, area: Rect)
	where
		B: Backend,
	{
		let selection_symbol = match self.active {
			true => ">",
			false => "*",
		};
		SelectableList::default()
			.block(Block::default().borders(Borders::ALL).title("Todo List"))
			.items(&self.current_text)
			.select(self.current_selection)
			.highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
			.highlight_symbol(selection_symbol)
			.render(f, area);
	}
}
