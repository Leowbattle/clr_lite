pub mod reflection;
use reflection::*;

use std::cell::RefCell;
use std::env;
use std::fs;
use std::path::{Path, PathBuf};
use std::rc::Rc;

#[derive(Clone)]
pub struct ClrLite(pub(crate) Rc<RefCell<ClrInternal>>);

impl ClrLite {
	pub fn new_runtime() -> Result<ClrLite, String> {
		Ok(ClrLite(Rc::new(RefCell::new(ClrInternal::new_runtime()?))))
	}

	/// Iterate over all loaded assemblies
	pub fn assemblies(&self) -> impl Iterator<Item = Assembly> {
		Assemblies {
			clr: self.clone(),
			current: 0,
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
		Assembly::load(self.clone(), &data)
	}

	pub fn get_type(&self, name: &str) -> Option<Type> {
		Some(
			self.0
				.borrow()
				.types
				.iter()
				.find(|t| t.full_name() == name)?
				.clone(),
		)
	}
}

/// Iterator over all loaded assemblies
struct Assemblies {
	clr: ClrLite,
	current: usize,
}

impl Iterator for Assemblies {
	type Item = Assembly;

	fn next(&mut self) -> Option<Self::Item> {
		let next = self.clr.0.borrow().assemblies.get(self.current)?.clone();
		self.current += 1;
		Some(next)
	}
}

pub(crate) struct ClrInternal {
	assembly_load_paths: Vec<PathBuf>,
	assemblies: Vec<Assembly>,
	types: Vec<Type>,
}

impl ClrInternal {
	fn new_runtime() -> Result<ClrInternal, String> {
		let mut exe_dir = env::current_exe().map_err(|e| e.to_string())?;
		exe_dir.pop();

		let mut lib_dir = exe_dir.clone();
		lib_dir.push("libraries");

		Ok(ClrInternal {
			assembly_load_paths: vec![exe_dir, lib_dir],
			types: vec![],
			assemblies: vec![],
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
}
