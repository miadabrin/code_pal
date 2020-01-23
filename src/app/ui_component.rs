use crate::app::{ActionPayload, Event};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::mpsc::Sender;
use tui::backend::Backend;
use tui::layout::Constraint;
use tui::layout::Rect;
use tui::style::{Color, Modifier, Style};
use tui::widgets::{Block, Borders, Row, SelectableList, Table, Widget};
use tui::Frame;

use crate::todo::todo::{EditableRowItem, EditableStateItem};

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
		sender: Sender<Event>,
	) -> TableEditor<T> {
		TableEditor {
			title,
			current_text: initial_text,
			current_selection: Option::None,
			current_header_selection: Option::None,
			headers,
			active: false,
			sender,
		}
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
	}
	fn on_event(&mut self, event: KeyEvent) {
		if let Some(_) = self.current_text {
			if self.active {
				match (event.code, event.modifiers) {
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
		if let Some(x) = self.current_text.as_ref() {
			let item_ref = (*x).clone();
			let mut borrowed_item = item_ref.borrow_mut();

			let items: Vec<_> = borrowed_item
				.iter_mut()
				.map(|s| s.get_content_vector())
				.collect();
			let rows = items
				.iter()
				.map(|s| Row::StyledData(s.into_iter(), Style::default()));
			let constraints: Vec<_> = (0..self.headers.len())
				.map(|_| Constraint::Length(15))
				.collect();
			Table::new(self.headers.iter(), rows)
				.block(Block::default().title(&self.title).borders(Borders::ALL))
				.header_style(Style::default().fg(Color::Yellow))
				.widths(&constraints)
				.render(f, area);
		}
	}
}
