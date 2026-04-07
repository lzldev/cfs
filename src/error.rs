use std::error::Error;

use anyhow::anyhow;
use seahorse::Context;

pub fn invalid(cause: &str) -> anyhow::Error {
	anyhow!(
		"invalid {}. get help by running `{} --help`",
		cause,
		env!("CARGO_PKG_NAME")
	)
}

pub struct WithActionResultClosure<'a> {
	f: Box<dyn Fn(&'a Context) -> anyhow::Result<()>>,
}

impl<'a> WithActionResultClosure<'a> {
	pub fn new(f: impl Fn(&'a Context) -> anyhow::Result<()> + 'static) -> Self {
		Self { f: Box::new(f) }
	}
}

impl<'a> Fn<(&'a Context,)> for WithActionResultClosure<'a> {
	extern "rust-call" fn call(&self, args: (&'a Context,)) -> Self::Output {
		return self.f.call(args).map_err(|err| err.into_boxed_dyn_error());
	}
}

impl<'a> FnMut<(&'a Context,)> for WithActionResultClosure<'a> {
	extern "rust-call" fn call_mut(&mut self, args: (&'a Context,)) -> Self::Output {
		return self.f.call(args).map_err(|err| err.into_boxed_dyn_error());
	}
}

impl<'a> FnOnce<(&'a Context,)> for WithActionResultClosure<'a> {
	type Output = Result<(), Box<dyn Error + Send + Sync>>;

	extern "rust-call" fn call_once(self, args: (&'a Context,)) -> Self::Output {
		return self.f.call(args).map_err(|err| err.into_boxed_dyn_error());
	}
}
