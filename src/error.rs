use core::{
	fmt::{self, Debug, Display, Formatter},
	result,
};

#[derive(PartialEq)]

pub enum Error {
	NoCommandGiven,
	ChildProcessTerminatedWithSignal,
}

impl Debug for Error {
	fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> {
		write!(
			f,
			"{}",
			match self {
				Self::NoCommandGiven => "no command given",
				Self::ChildProcessTerminatedWithSignal => "child process terminated with signal",
			}
		)
	}
}

impl Display for Error {
	fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> {
		Debug::fmt(self, f)
	}
}

impl std::error::Error for Error {}

pub type Result<T> = result::Result<T, Error>;

pub type ExecutionResult = Result<()>;
