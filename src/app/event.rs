use crossterm::event::KeyEvent;

#[derive(Debug)]
pub enum ActionPayload {
	Selection(String, Option<usize>),
}

pub enum Event {
	Input(KeyEvent),
	Tick,
	Action(ActionPayload),
}
