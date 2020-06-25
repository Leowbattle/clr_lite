use crate::metadata::tables::{MethodDefHandle, ParamHandle};
use crate::metadata::*;
use crate::vm::*;

use std::cell::RefCell;
use std::fmt;
use std::rc::{Rc, Weak};

#[derive(Clone)]
pub struct Method(pub(crate) Rc<MethodInternal>);

impl Method {
	pub(crate) fn load<'a>(
		clr: ClrLite,
		declaring_type: Type,
		i: usize,
		metadata: &'a Metadata<'a>,
	) -> Result<Method, String> {
		let def = &metadata.tables().method_def[MethodDefHandle(i)];

		let name = metadata.strings().get(def.name).unwrap().to_string();
		let signature = metadata
			.blob()
			.new_reader(def.signature)
			.map_err(|e| e.to_string())?
			.read_method_signature()
			.map_err(|e| e.to_string())?;

		let method = Method(Rc::new(MethodInternal {
			clr: Rc::downgrade(&clr.clone().0),
			name,
			declaring_type: Rc::downgrade(&declaring_type.0),
			return_type: Rc::downgrade(
				&Type::get_type_for_element_type(clr.clone(), metadata, &signature.return_type)?.0,
			),
			params: RefCell::new(vec![]),
			is_static: def.attributes.is_static,
			is_virtual: def.attributes.is_virtual,
			is_abstract: def.attributes.is_abstract,
		}));

		// Load method parameters
		{
			let param_start = def.param_list.0;
			let param_count = signature.params.len();
			let mut params = method.0.params.borrow_mut();
			params.reserve(param_count);

			for i in 0..param_count {
				let p_def = &metadata.tables().param[ParamHandle(param_start + i)];

				let p = Parameter(Rc::new(ParameterInternal {
					clr: Rc::downgrade(&clr.clone().0),
					method: Rc::downgrade(&method.clone().0),
					name: metadata.strings().get(p_def.name).unwrap().to_string(),
					index: p_def.index,
					param_type: Rc::downgrade(
						&Type::get_type_for_element_type(
							clr.clone(),
							metadata,
							&signature.params[i],
						)?
						.0,
					),
				}));

				params.push(p);
			}
		}

		Ok(method)
	}

	pub fn name<'a>(&'a self) -> &'a str {
		&self.0.name
	}

	pub fn declaring_type(&self) -> Option<Type> {
		Some(Type(self.0.declaring_type.upgrade()?))
	}

	pub fn return_type(&self) -> Option<Type> {
		Some(Type(self.0.return_type.upgrade()?))
	}

	pub fn parameters<'a>(&'a self) -> Parameters {
		Parameters {
			params: self.0.params.borrow(),
		}
	}

	pub fn is_static(&self) -> bool {
		self.0.is_static
	}

	pub fn is_virtual(&self) -> bool {
		self.0.is_virtual
	}

	pub fn is_abstract(&self) -> bool {
		self.0.is_abstract
	}
}

pub struct Parameters<'a> {
	params: Ref<'a, Vec<Parameter>>,
}

impl<'a> Deref for Parameters<'a> {
	type Target = [Parameter];

	fn deref(&self) -> &Self::Target {
		&self.params
	}
}

impl fmt::Display for Method {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{} {}(", self.return_type().unwrap().name(), self.name())?;
		let params = self.parameters();
		for i in 0..params.len() {
			write!(f, "{}", params[i].param_type().unwrap().name())?;
			if i != params.len() - 1 {
				write!(f, ", ")?;
			}
		}
		write!(f, ")")
	}
}

// TODO MethodBody
pub(crate) struct MethodInternal {
	clr: Weak<RefCell<ClrInternal>>,
	name: String,
	declaring_type: Weak<TypeInternal>,
	return_type: Weak<TypeInternal>,
	params: RefCell<Vec<Parameter>>,
	is_static: bool,
	is_virtual: bool,
	is_abstract: bool,
}
