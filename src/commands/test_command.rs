use crate::commands::command::Command;
use crate::commands::commons::CommonOpts;

pub struct TestCommand {
	list: bool,
	common: CommonOpts,
}

impl TestCommand {
	pub fn new(list: bool, common: CommonOpts) -> Self {
		Self { list, common }
	}
}

impl Command for TestCommand {
	fn execute(&self) {
		if self.list {
			println!("Listing test values...");
		}
		self.common.handle_common_opts();
	}
}
