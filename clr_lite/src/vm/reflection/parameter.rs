use crate::vm::*;

use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Parameter(pub(crate) Rc<ParameterInternal>);

impl Parameter {
	pub fn name<'a>(&'a self) -> &'a str {
		&self.0.name
	}

	pub fn method(&self) -> Option<Method> {
		Some(Method(self.0.method.upgrade()?))
	}

	pub fn index(&self) -> usize {
		self.0.index
	}

	pub fn param_type(&self) -> Option<Type> {
		Some(Type(self.0.param_type.upgrade()?))
	}
}

impl fmt::Display for Parameter {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}", self.param_type().unwrap().name(), self.name())
	}
}

pub(crate) struct ParameterInternal {
	#[allow(dead_code)]
	pub(super) clr: Weak<RefCell<ClrInternal>>,
	pub(super) method: Weak<MethodInternal>,
	pub(super) name: String,
	pub(super) index: usize,
	pub(super) param_type: Weak<TypeInternal>,
}
