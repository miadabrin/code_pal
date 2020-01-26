use crate::app::{ActionPayload, Event};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use tui::backend::Backend;
use tui::layout::{Constraint, Layout, Rect};
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Paragraph, Row, SelectableList, Table, Text, Widget};
use tui::Frame;

use crate::todo::todo::{EditableRowItem, EditableStateItem, SelectableItem};

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

pub struct ListTextEditor<T>
where
	T: EditableStateItem,
{
	pub title: String,
	pub current_text: Option<Rc<RefCell<Vec<T>>>>,
	pub current_selection: Option<usize>,
	pub active: bool,
	pub sender: Sender<Event>,
}

impl<T> ListTextEditor<T>
where
	T: EditableStateItem,
{
	pub fn new(
		title: String,
		initial_text: Option<Rc<RefCell<Vec<T>>>>,
		sender: Sender<Event>,
	) -> ListTextEditor<T> {
		ListTextEditor {
			title,
			current_text: initial_text,
			current_selection: Option::None,
			active: false,
			sender,
		}
	}

	pub fn on_key(&mut self, c: char, _: KeyModifiers) {
		let selected_index = match self.current_selection {
			Some(selected_index) => selected_index,
			None => 0,
		};
		let item_ref = (*self.current_text.as_ref().unwrap()).clone();
		if let Some(elem) = item_ref.borrow_mut().get_mut(selected_index) {
			let content = elem.get_content_mut();
			content.push(c);
		};
	}

	pub fn on_paste(&mut self) {
		let selected_index = match self.current_selection {
			Some(selected_index) => selected_index,
			None => 0,
		};
		let item_ref = (*self.current_text.as_ref().unwrap()).clone();
		if let Some(elem) = item_ref.borrow_mut().get_mut(selected_index) {
			let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
			let content = elem.get_content_mut();
			if let Ok(received) = ctx.get_contents() {
				content.push_str(&received);
			}
		};
	}

	pub fn on_up(&mut self) {
		self.select_item(match self.current_selection {
			Some(x) if x > 0 => x - 1,
			_ => 0,
		});
	}
	pub fn on_down(&mut self) {
		let item_ref = (*self.current_text.as_ref().unwrap()).clone();
		let mut borrowed_item = item_ref.borrow_mut();
		self.select_item(match self.current_selection {
			Some(x) if x < borrowed_item.len() - 1 => x + 1,
			Some(x) if x == borrowed_item.len() - 1 => {
				let t = T::new(String::from(""));
				borrowed_item.push(t);
				borrowed_item.len() - 1
			}
			_ => borrowed_item.len(),
		});
	}
	pub fn on_backspace(&mut self) {
		let item_ref = (*self.current_text.as_ref().unwrap()).clone();
		match self.current_selection {
			Some(x) => {
				let mut borrowed_item = item_ref.borrow_mut();
				if let Some(elem) = borrowed_item.get_mut(x) {
					let content = elem.get_content_mut();
					if content.len() > 0 {
						content.pop();
					} else {
						if x >= 1 {
							borrowed_item.remove(x);
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
			let t = T::new(String::from(""));
			let item_ref = (*self.current_text.as_ref().unwrap()).clone();
			item_ref.borrow_mut().insert(x + 1, t);
			self.select_item(x + 1);
		}
	}

	pub fn select_item(&mut self, index: usize) {
		self.current_selection = Some(index);
		self.broadcast_selection();
	}

	pub fn broadcast_selection(&mut self) {
		self.sender
			.send(Event::Action(ActionPayload::Selection(
				self.title.clone(),
				self.current_selection,
			)))
			.unwrap_or_default();
	}

	pub fn unselect(&mut self) {
		self.current_selection = None;
		self.broadcast_selection();
	}
}

impl<T> UIEventProcessor for ListTextEditor<T>
where
	T: EditableStateItem,
{
	fn on_deactivate(&mut self) {
		self.active = false
	}
	fn on_activate(&mut self) {
		if let None = self.current_selection {
			self.select_item(0);
		}
		self.active = true;
	}
	fn on_event(&mut self, event: KeyEvent) {
		if let Some(_) = self.current_text {
			if self.active {
				match (event.code, event.modifiers) {
					(KeyCode::Char('v'), KeyModifiers::CONTROL) => self.on_paste(),
					(KeyCode::Char(c), _) => self.on_key(c, event.modifiers),
					(KeyCode::Up, _) => self.on_up(),
					(KeyCode::Down, _) => self.on_down(),
					(KeyCode::Backspace, _) => self.on_backspace(),
					(KeyCode::Enter, _) => self.on_enter(),
					(_, _) => {}
				}
			}
		}
	}
}
impl<T> UIComponent for ListTextEditor<T>
where
	T: EditableStateItem,
{
	fn draw<B>(&mut self, f: &mut Frame<B>, area: Rect)
	where
		B: Backend,
	{
		let selection_symbol = match self.active {
			true => ">",
			false => "*",
		};
		if let Some(x) = self.current_text.as_ref() {
			let item_ref = (*x).clone();
			let mut borrowed_item = item_ref.borrow_mut();

			let items: Vec<_> = borrowed_item
				.iter_mut()
				.map(|x| (x.get_content_mut()))
				.collect();

			SelectableList::default()
				.block(Block::default().borders(Borders::ALL).title(&self.title))
				.items(&items)
				.select(self.current_selection)
				.highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
				.highlight_symbol(selection_symbol)
				.render(f, area);
		}
	}
}

pub struct TableEditor<T>
where
	T: EditableRowItem,
{
	pub title: String,
	pub current_text: Option<Rc<RefCell<Vec<T>>>>,
	pub current_selection: Option<usize>,
	pub current_header_selection: Option<usize>,
	pub column_lengths: Vec<u16>,
	pub headers: Vec<String>,
	pub active: bool,
	pub sender: Sender<Event>,
}

impl<T> TableEditor<T>
where
	T: EditableRowItem,
{
	pub fn new(
		title: String,
		initial_text: Option<Rc<RefCell<Vec<T>>>>,
		headers: Vec<String>,
		column_lengths: Vec<u16>,
		sender: Sender<Event>,
	) -> TableEditor<T> {
		TableEditor {
			title,
			current_text: initial_text,
			current_selection: Option::None,
			current_header_selection: Option::None,
			column_lengths,
			headers,
			active: false,
			sender,
		}
	}
	pub fn select_item(&mut self, index: usize) {
		self.current_selection = Some(index);
	}
	pub fn select_header(&mut self, index: usize) {
		self.current_header_selection = Some(index);
	}
	pub fn on_key(&mut self, c: char, _: KeyModifiers) {
		let selected_index = match self.current_selection {
			Some(selected_index) => selected_index,
			None => 0,
		};
		let selected_header = match self.current_header_selection {
			Some(selected_header) => selected_header,
			None => 0,
		};
		let item_ref = (*self.current_text.as_ref().unwrap()).clone();
		if let Some(elem) = item_ref.borrow_mut().get_mut(selected_index) {
			let content = elem.get_content_mut(selected_header);
			content.push(c);
		};
	}
	pub fn on_backspace(&mut self) {
		let selected_index = match self.current_selection {
			Some(selected_index) => selected_index,
			None => 0,
		};
		let selected_header = match self.current_header_selection {
			Some(selected_header) => selected_header,
			None => 0,
		};
		let item_ref = (*self.current_text.as_ref().unwrap()).clone();
		let mut borrowed_item = item_ref.borrow_mut();
		if let Some(elem) = borrowed_item.get_mut(selected_index) {
			let content = elem.get_content_mut(selected_header);
			if content.len() > 0 {
				content.pop();
			} else {
				if selected_header == 0 {
					if selected_index >= 1 {
						borrowed_item.remove(selected_index);
						self.current_selection = Some(selected_index - 1);
					}
				}
			}
		};
	}
	pub fn on_up(&mut self) {
		self.select_item(match self.current_selection {
			Some(x) if x > 0 => x - 1,
			_ => 0,
		});
	}
	pub fn on_down(&mut self) {
		let item_ref = (*self.current_text.as_ref().unwrap()).clone();
		let mut borrowed_item = item_ref.borrow_mut();
		self.select_item(match self.current_selection {
			Some(x) if x < borrowed_item.len() - 1 => x + 1,
			Some(x) if x == borrowed_item.len() - 1 => {
				let t = T::new(vec![]);
				borrowed_item.push(t);
				borrowed_item.len() - 1
			}
			_ => borrowed_item.len(),
		});
	}

	pub fn on_left(&mut self) {
		let selected_index = match self.current_selection {
			Some(selected_index) => selected_index,
			None => 0,
		};
		let selected_header = match self.current_header_selection {
			Some(selected_header) => selected_header,
			None => 0,
		};
		let item_ref = (*self.current_text.as_ref().unwrap()).clone();
		let mut borrowed_item = item_ref.borrow_mut();
		if let Some(_) = borrowed_item.get_mut(selected_index) {
			if selected_header > 0 {
				self.select_header(selected_header - 1);
			}
		};
	}
	pub fn on_right(&mut self) {
		let selected_index = match self.current_selection {
			Some(selected_index) => selected_index,
			None => 0,
		};
		let selected_header = match self.current_header_selection {
			Some(selected_header) => selected_header,
			None => 0,
		};
		let item_ref = (*self.current_text.as_ref().unwrap()).clone();
		let mut borrowed_item = item_ref.borrow_mut();
		if let Some(elem) = borrowed_item.get_mut(selected_index) {
			if selected_header < elem.get_content_vector().len() - 1 {
				self.select_header(selected_header + 1);
			}
		};
	}
	pub fn on_paste(&mut self) {
		let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
		let selected_index = match self.current_selection {
			Some(selected_index) => selected_index,
			None => 0,
		};
		let selected_header = match self.current_header_selection {
			Some(selected_header) => selected_header,
			None => 0,
		};
		let item_ref = (*self.current_text.as_ref().unwrap()).clone();
		if let Some(elem) = item_ref.borrow_mut().get_mut(selected_index) {
			let content = elem.get_content_mut(selected_header);
			if let Ok(received) = ctx.get_contents() {
				content.push_str(&received);
			}
		};
	}
}

impl<T> UIEventProcessor for TableEditor<T>
where
	T: EditableRowItem,
{
	fn on_deactivate(&mut self) {
		self.active = false
	}
	fn on_activate(&mut self) {
		self.active = true;
		self.select_item(0);
		self.select_header(0);
	}
	fn on_event(&mut self, event: KeyEvent) {
		if let Some(_) = self.current_text {
			if self.active {
				match (event.code, event.modifiers) {
					(KeyCode::Char('v'), KeyModifiers::CONTROL) => self.on_paste(),
					(KeyCode::Char(c), _) => self.on_key(c, event.modifiers),
					(KeyCode::Backspace, _) => self.on_backspace(),
					(KeyCode::Up, _) => self.on_up(),
					(KeyCode::Down, _) => self.on_down(),
					(KeyCode::Right, _) => self.on_right(),
					(KeyCode::Left, _) => self.on_left(),
					(_, _) => {}
				}
			}
		}
	}
}
impl<T> UIComponent for TableEditor<T>
where
	T: EditableRowItem,
{
	fn draw<B>(&mut self, f: &mut Frame<B>, area: Rect)
	where
		B: Backend,
	{
		let selected_index = match self.current_selection {
			Some(selected_index) => selected_index,
			None => 0,
		};
		let selected_header = match self.current_header_selection {
			Some(selected_header) => selected_header,
			None => 0,
		};
		if let Some(x) = self.current_text.as_ref() {
			let item_ref = (*x).clone();
			let mut borrowed_item = item_ref.borrow_mut();
			let rows = borrowed_item.iter_mut().enumerate().map(|(i, elem)| {
				let style = match selected_index {
					x if x == i => Style::default().fg(Color::Yellow),
					_ => Style::default(),
				};
				let content_to_show: Vec<_> = elem
					.get_content_vector()
					.into_iter()
					.enumerate()
					.map(|(ii, s)| match (selected_index, selected_header) {
						(x, y) if x == i && y == ii => format!(">{}", s),
						_ => s.to_string(),
					})
					.collect();
				Row::StyledData(content_to_show.into_iter(), style)
			});
			let constraints: Vec<_> = self
				.column_lengths
				.iter()
				.map(|l| Constraint::Length(*l))
				.collect();
			Table::new(self.headers.iter(), rows)
				.block(Block::default().title(&self.title).borders(Borders::ALL))
				.header_style(Style::default().fg(Color::Yellow))
				.widths(&constraints)
				.render(f, area);
		}
	}
}

pub struct AutoCompleteEditor<T>
where
	T: SelectableItem,
{
	pub active: bool,
	pub title: String,
	pub text: String,
	pub current_suggestions: Vec<T>,
	pub current_selection: Option<usize>,
	pub current_chosen: Option<String>,
	pub sender: Sender<Event>,
}

impl<T> AutoCompleteEditor<T>
where
	T: SelectableItem,
{
	pub fn new(
		title: String,
		text: String,
		current_suggestions: Vec<T>,
		sender: Sender<Event>,
	) -> AutoCompleteEditor<T> {
		AutoCompleteEditor {
			active: false,
			title,
			text,
			current_suggestions,
			current_selection: None,
			current_chosen: None,
			sender,
		}
	}

	pub fn on_key(&mut self, c: char, _: KeyModifiers) {
		self.text.push(c);
		self.broadcast_text();
	}

	pub fn on_backspace(&mut self) {
		if self.text.len() > 0 {
			self.text.pop();
			self.broadcast_text();
		} else {
			if let Some(x) = &self.current_chosen {
				if x.len() > 0 {
					self.current_chosen = Some(String::from(""));
					self.text = String::from("");
					self.broadcast_text();
				}
			}
		}
	}

	pub fn broadcast_text(&mut self) {
		self.sender
			.send(Event::Action(ActionPayload::Text(self.text.clone())))
			.unwrap_or_default();
	}

	pub fn broadcast_selection(&mut self) {
		match self.current_selection {
			Some(x) => {
				self.sender
					.send(Event::Action(ActionPayload::TextSelection(
						self.title.clone(),
						self.current_suggestions
							.get_mut(x)
							.unwrap()
							.get_identifier(),
					)))
					.unwrap_or_default();
			}
			None => {}
		}
	}

	pub fn select_suggestion(&mut self, index: Option<usize>) {
		self.current_selection = index;
	}

	pub fn set_text_to_choice(&mut self) {
		match self.current_selection {
			Some(x) => {
				self.text = self.current_suggestions.get_mut(x).unwrap().get_name();
			}
			_ => {}
		}
	}

	pub fn on_up(&mut self) {
		self.select_suggestion(match self.current_selection {
			Some(x) if x > 0 => Some(x - 1),
			_ => Some(0),
		});
		self.set_text_to_choice();
	}
	pub fn on_down(&mut self) {
		self.select_suggestion(match self.current_selection {
			Some(x) if x < self.current_suggestions.len() - 1 => Some(x + 1),
			Some(x) => Some(x),
			None => None,
		});
		self.set_text_to_choice();
	}
	pub fn on_enter(&mut self) {
		match self.current_selection {
			Some(x) => {
				self.text = self.current_suggestions.get_mut(x).unwrap().get_name();
			}
			None => {}
		}
		self.broadcast_selection();
	}
}

impl<T> UIComponent for AutoCompleteEditor<T>
where
	T: SelectableItem,
{
	fn draw<B>(&mut self, f: &mut Frame<B>, area: Rect)
	where
		B: Backend,
	{
		let chunks = Layout::default()
			.constraints([Constraint::Length(3), Constraint::Min(1)].as_ref())
			.split(area);
		let items: Vec<_> = self
			.current_suggestions
			.iter_mut()
			.map(|x| (x.get_name()))
			.collect();
		let text = [Text::raw(self.text.clone())];

		Paragraph::new(text.iter())
			.block(
				Block::default()
					.borders(Borders::ALL)
					.title("Find ...")
					.title_style(Style::default()),
			)
			.wrap(true)
			.render(f, chunks[0]);

		SelectableList::default()
			.block(Block::default().borders(Borders::ALL).title(&self.title))
			.items(&items)
			.select(self.current_selection)
			.highlight_style(Style::default().fg(Color::Yellow).modifier(Modifier::BOLD))
			.highlight_symbol(">")
			.render(f, chunks[1]);
	}
}

impl<T> UIEventProcessor for AutoCompleteEditor<T>
where
	T: SelectableItem,
{
	fn on_deactivate(&mut self) {
		self.active = false
	}
	fn on_activate(&mut self) {
		self.active = true;
	}
	fn on_event(&mut self, event: KeyEvent) {
		if self.active {
			match (event.code, event.modifiers) {
				(KeyCode::Char(c), _) => self.on_key(c, event.modifiers),
				(KeyCode::Backspace, _) => self.on_backspace(),
				(KeyCode::Up, _) => self.on_up(),
				(KeyCode::Down, _) => self.on_down(),
				(KeyCode::Enter, _) => self.on_enter(),
				(_, _) => {}
			}
		}
	}
}
