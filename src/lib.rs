#[allow(dead_code)]
pub mod demo;
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
