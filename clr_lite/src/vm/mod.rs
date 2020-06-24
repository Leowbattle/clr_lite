pub mod reflection;
use reflection::*;

use std::cell::RefCell;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::PathBuf;
use std::rc::Rc;

#[derive(Clone)]
pub struct ClrLite(pub(crate) Rc<RefCell<ClrInternal>>);

impl ClrLite {
	pub fn new_runtime() -> Result<ClrLite, String> {
		let mut rt = ClrLite(Rc::new(RefCell::new(ClrInternal::new_runtime()?)));
		rt.load_default_assemblies();
		Ok(rt)
	}

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

		let data = fs::read(path).map_err(|e| e.to_string())?;
		let a = Assembly::load(self.clone(), &data)?;
		self.0.borrow_mut().assemblies.push(a.clone());
		Ok(a)
	}

	fn load_default_assemblies(&mut self) -> Result<(), String> {
		let libs = ["System.Runtime"];
		for lib in libs.iter() {
			self.load_assembly(lib)?;
		}
		Ok(())
	}
}

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
