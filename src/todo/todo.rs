use uuid::Uuid;

#[derive(Debug)]
pub struct TodoItem {
	pub identifier: String,
	pub title: String,
	pub description: Vec<String>,
	pub notes: Vec<String>,
}

pub trait EditableStateItem {
	fn get_content_mut(&mut self) -> &mut String;
	fn get_identifier_mut(&mut self) -> &mut String;
	fn get_content_ref(&mut self) -> &str;
	fn new(s: String) -> Self;
}

impl EditableStateItem for TodoItem {
	fn get_content_mut(&mut self) -> &mut String {
		&mut self.title
	}
	fn get_content_ref(&mut self) -> &str {
		&self.title
	}
	fn get_identifier_mut(&mut self) -> &mut String {
		&mut self.identifier
	}
	fn new(s: String) -> Self {
		TodoItem {
			identifier: Uuid::new_v4().to_string(),
			title: s,
			description: vec![],
			notes: vec![],
		}
	}
}
