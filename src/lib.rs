#[allow(dead_code)]
pub mod app;
#[allow(dead_code)]
pub mod util;

pub mod todo {
	#[derive(Debug)]
	pub struct TodoItem {
		pub title: String,
		pub description: Vec<String>,
		pub notes: Vec<String>,
	}
}
