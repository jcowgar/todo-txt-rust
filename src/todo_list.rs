use crate::todo::Todo;

/// A list of Todo items
pub struct TodoList {
	pub items: Vec<Todo>,
}

impl TodoList {
	pub fn filter_by_priority(self, priority: char) -> TodoList {
		TodoList {
			items: self
				.items
				.into_iter()
				.filter(|t| t.priority.is_some() && t.priority.unwrap() <= priority)
				.collect(),
		}
	}

	pub fn filter_by_complete(self, complete: bool) -> TodoList {
		TodoList {
			items: self
				.items
				.into_iter()
				.filter(|t| t.is_complete == complete)
				.collect(),
		}
	}

	pub fn filter_by_past_due(self, is_past_due: bool) -> TodoList {
		TodoList {
			items: self
				.items
				.into_iter()
				.filter(|t| t.is_complete == false && t.is_past_due() == is_past_due)
				.collect(),
		}
	}

	pub fn filter_by_text(self, text: &str) -> TodoList {
		let mut search_text = text.to_string();
		let mut compare_result = true;

		if search_text.starts_with("-") {
			search_text = search_text.replace("-", "");
			compare_result = false;
		}

		TodoList {
			items: self
				.items
				.into_iter()
				.filter(|t| t.serialize().contains(&search_text) == compare_result)
				.collect(),
		}
	}

	pub fn sort(&mut self) {
		self.items.sort_by(|a, b| a.cmp(b));
	}

	pub fn sort_by_title(&mut self) {
		self.items.sort_by(|a, b| a.cmp_by_title(b));
	}

	pub fn sort_by_due_date(&mut self) {
		self.items.sort_by(|a, b| a.cmp_by_due_date(b));
	}
}
