pub mod reflection;
use reflection::*;

pub mod gc;
use gc::*;

pub mod interpreter;
use interpreter::*;

use std::cell::{Ref, RefCell};
use std::collections::HashMap;
use std::env;
use std::fs;
use std::ops::Deref;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Clone)]
pub struct ClrLite(pub(crate) Rc<RefCell<ClrInternal>>);

impl ClrLite {
	pub fn new_runtime() -> Result<ClrLite, String> {
		let mut clr = ClrLite(Rc::new(RefCell::new(ClrInternal::new_runtime()?)));
		clr.load_builtin_assemblies()?;
		Ok(clr)
	}

	pub fn assemblies<'a>(&'a self) -> Assemblies<'a> {
		Assemblies {
			clr: self.0.borrow(),
		}
	}

	pub fn types<'a>(&'a self) -> Types<'a> {
		Types {
			clr: self.0.borrow(),
		}
	}

	pub fn load_assembly(&mut self, name: &str) -> Result<Assembly, String> {
		let path = self
			.0
			.borrow()
			.resolve_assembly_name(name)
			.ok_or_else(|| format!("Could not locate assembly with name {}", name))?;
		self.load_assembly_from_path(path)
	}

	pub fn load_assembly_from_path(&mut self, path: impl AsRef<Path>) -> Result<Assembly, String> {
		let data = fs::read(path).map_err(|e| e.to_string())?;
		self.load_assembly_from_data(&data)
	}

	pub fn load_assembly_from_data(&mut self, data: &[u8]) -> Result<Assembly, String> {
		Assembly::load(self.clone(), &data)
	}

	fn load_builtin_assemblies(&mut self) -> Result<(), String> {
		#[cfg(debug_assertions)]
		macro_rules! builtin_assemblies {
			($($name:expr),*) => {
				$(
					self.load_assembly_from_data(
						include_bytes!(concat!("../../../libraries/", $name, "/bin/Debug/netcoreapp3.1/", $name, ".dll"))
					)?;
				)*
			};
		}

		#[cfg(not(debug_assertions))]
		macro_rules! builtin_assemblies {
			($($name:expr),*) => {
				$(
					self.load_assembly_from_data(
						include_bytes!(concat!("../../../libraries/", $name, "/bin/Debug/netcoreapp3.1/", $name, ".dll"))
					)?;
				)*
			};
		}

		builtin_assemblies! {
			"System.Runtime",
			"System.Console"
		}

		// Load primitive types
		let primitives = PrimitiveTypes {
			sbyte: self.get_type("System.SByte").unwrap(),
			byte: self.get_type("System.Byte").unwrap(),
			short: self.get_type("System.Int16").unwrap(),
			ushort: self.get_type("System.UInt16").unwrap(),
			int: self.get_type("System.Int32").unwrap(),
			uint: self.get_type("System.UInt32").unwrap(),
			long: self.get_type("System.Int64").unwrap(),
			ulong: self.get_type("System.UInt64").unwrap(),
			float: self.get_type("System.Single").unwrap(),
			double: self.get_type("System.Double").unwrap(),

			object: self.get_type("System.Object").unwrap(),
		};
		self.0.borrow_mut().primitive_types.replace(primitives);

		Ok(())
	}

	pub fn get_type(&self, name: &str) -> Option<Type> {
		Some(self.0.borrow().type_map.get(name)?.clone())
	}

	pub fn execute(&mut self, m: Method, params: &mut [Value]) -> RunResult {
		// TODO Check arguments for validity

		if !m.is_static() {
			return Err(format!(
				"Cannot use non-static method {}.{} as entry point",
				m.declaring_type().unwrap(),
				m.name()
			));
		}
		Interpreter::new(self.clone()).execute(m, params)
	}

	pub(crate) fn next_type_id(&mut self) -> TypeID {
		let mut internal = self.0.borrow_mut();
		let id = internal.next_type_id;
		internal.next_type_id += 1;
		id
	}

	pub(crate) fn internal<'a>(&'a self) -> Ref<'a, ClrInternal> {
		self.0.borrow()
	}
}

pub struct Assemblies<'a> {
	clr: Ref<'a, ClrInternal>,
}

impl<'a> Deref for Assemblies<'a> {
	type Target = [Assembly];

	fn deref(&self) -> &Self::Target {
		&self.clr.assemblies
	}
}

pub struct Types<'a> {
	clr: Ref<'a, ClrInternal>,
}

impl<'a> Deref for Types<'a> {
	type Target = [Type];

	fn deref(&self) -> &Self::Target {
		&self.clr.types
	}
}

pub(crate) struct ClrInternal {
	assembly_load_paths: Vec<PathBuf>,
	assemblies: Vec<Assembly>,
	types: Vec<Type>,
	type_map: HashMap<String, Type>,

	next_type_id: TypeID,
	primitive_types: Option<PrimitiveTypes>,
}

pub(crate) struct PrimitiveTypes {
	pub sbyte: Type,
	pub byte: Type,
	pub short: Type,
	pub ushort: Type,
	pub int: Type,
	pub uint: Type,
	pub long: Type,
	pub ulong: Type,
	pub float: Type,
	pub double: Type,

	pub object: Type,
}

impl ClrInternal {
	fn new_runtime() -> Result<ClrInternal, String> {
		let mut exe_dir = env::current_exe().map_err(|e| e.to_string())?;
		exe_dir.pop();

		let mut lib_dir = exe_dir.clone();
		lib_dir.push("libraries");

		Ok(ClrInternal {
			assembly_load_paths: vec![exe_dir, lib_dir],
			assemblies: vec![],
			types: vec![],
			type_map: HashMap::new(),

			next_type_id: 0,
			primitive_types: None,
		})
	}

	/// Get the file path for a named assembly
	fn resolve_assembly_name(&self, name: &str) -> Option<PathBuf> {
		let dll = format!("{}.dll", name);
		let exe = format!("{}.exe", name);

		for path in self.assembly_load_paths.iter() {
			let mut path = PathBuf::from(path);
			path.push(&dll);
			if path.exists() {
				return Some(path);
			}
			path.pop();
			path.push(&exe);
			if path.exists() {
				return Some(path);
			}
		}
		None
	}

	pub(crate) fn add_type(&mut self, t: Type) {
		self.types.push(t.clone());
		self.type_map.insert(t.full_name().to_string(), t);
	}

	pub(crate) fn primitives<'a>(&'a self) -> &'a PrimitiveTypes {
		self.primitive_types.as_ref().unwrap()
	}
}
