use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::rc::Rc;
use uuid::Uuid;

pub trait EditableStateItem {
	fn get_content_mut(&mut self) -> &mut String;
	fn get_identifier_mut(&mut self) -> &mut String;
	fn new(s: String) -> Self;
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TodoItem {
	pub identifier: String,
	pub title: String,
	#[serde(default)]
	pub project_identifier: String,
	pub description: Vec<String>,
	pub notes: Rc<RefCell<Vec<Note>>>,
}

impl EditableStateItem for TodoItem {
	fn get_content_mut(&mut self) -> &mut String {
		&mut self.title
	}
	fn get_identifier_mut(&mut self) -> &mut String {
		&mut self.identifier
	}
	fn new(s: String) -> Self {
		TodoItem {
			identifier: Uuid::new_v4().to_string(),
			title: s,
			description: vec![],
			notes: Rc::new(RefCell::new(vec![Note::new(String::from(""))])),
			project_identifier: String::from(""),
		}
	}
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Note {
	pub identifier: String,
	pub content: String,
}

impl EditableStateItem for Note {
	fn get_content_mut(&mut self) -> &mut String {
		&mut self.content
	}
	fn get_identifier_mut(&mut self) -> &mut String {
		&mut self.identifier
	}
	fn new(s: String) -> Self {
		Note {
			identifier: Uuid::new_v4().to_string(),
			content: s,
		}
	}
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Project {
	pub identifier: String,
	pub name: String,
	pub url: String,
	pub dir_location: String,
}

impl EditableRowItem for Project {
	fn get_content_vector(&mut self) -> Vec<&str> {
		vec![&self.name, &self.url, &self.dir_location]
	}
	fn get_content_mut(&mut self, index: usize) -> &mut String {
		match index {
			0 => &mut self.name,
			1 => &mut self.url,
			2 => &mut self.dir_location,
			_ => &mut self.name,
		}
	}
	fn get_identifier_mut(&mut self) -> &mut String {
		&mut self.identifier
	}
	fn new(s: Vec<String>) -> Self {
		let mut owned = s.to_owned();
		let dir_location = owned.pop().unwrap_or_default();
		let url = owned.pop().unwrap_or_default();
		let name = owned.pop().unwrap_or_default();

		Project {
			identifier: Uuid::new_v4().to_string(),
			name,
			url,
			dir_location,
		}
	}
}

pub trait EditableRowItem {
	fn get_content_vector(&mut self) -> Vec<&str>;
	fn get_content_mut(&mut self, index: usize) -> &mut String;
	fn get_identifier_mut(&mut self) -> &mut String;
	fn new(s: Vec<String>) -> Self;
}

pub trait SelectableItem {
	fn get_identifier(&mut self) -> String;
	fn get_name(&mut self) -> String;
}

impl SelectableItem for Project {
	fn get_identifier(&mut self) -> String {
		self.identifier.clone()
	}
	fn get_name(&mut self) -> String {
		self.name.clone()
	}
}
