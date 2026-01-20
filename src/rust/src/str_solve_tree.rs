use serde::{Deserialize, Serialize};
use serde_aux::prelude::*;
use rayon::prelude::*;
use std::collections::HashSet;
use std::collections::HashMap;

use crate::str_solve_func::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct TreeNode {
	#[serde(flatten)]
	func: StrSolveFunc,
	#[serde(deserialize_with = "deserialize_bool_from_anything")]
	ret: bool,
	children: Vec<TreeNode>,
}

impl TreeNode{
	fn exec(&self, str_list: &[String]) -> Vec<String>
	{
		let solved_str_list: Vec<String> = str_list.iter().flat_map(|x| self.func.exec(x)).collect();
		let children_output: Vec<String> = self.children
		//.par_iter()
		.iter()
		.flat_map(|child| child.exec( &solved_str_list.clone()))
		.collect();
		let output = if self.ret {
			children_output.into_iter().chain(solved_str_list).collect()
		} else {
			children_output
		};
		output
	}

	fn exec_par(&self, str_list: &[String]) -> Vec<String>
	{
		let solved_str_list: Vec<String> = str_list.par_iter().flat_map(|x| self.func.exec(x)).collect();
		let children_output: Vec<String> = self.children
		.par_iter()
		//.iter()
		.flat_map(|child| child.exec_par( &solved_str_list.clone()))
		.collect();
		let output = if self.ret {
			children_output.into_iter().chain(solved_str_list).collect()
		} else {
			children_output
		};
		output
	}
}

#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ExecMode {
	Parallel,
	Single,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct StrSolveTree {
	root: TreeNode,
}

impl StrSolveTree {
	fn exec_for_string(&self, input: &str,  execmode: ExecMode) -> HashSet<String>
	{
		let str_vec = match execmode {
			ExecMode::Parallel => {
				self.root.exec_par(&vec![input.to_string()])
			},
			ExecMode::Single => {
				self.root.exec(&vec![input.to_string()])
			},
		};
		let mut str_set = str_vec.into_iter().collect::<HashSet<String>>();
		str_set.remove("");
		let str_set = str_set;
		return str_set;
	}

	pub fn exec(&self, str_list: &[String],  execmode: ExecMode) -> HashMap<String	, HashSet<String>>
	{
		match execmode {
			ExecMode::Parallel => {
				let result: HashMap<String, HashSet<String>> = str_list
				.par_iter()
				.map(|x| {
					let key = x.clone();
					let value = self.exec_for_string(x, execmode.clone());
					(key, value)
				})
				.collect();
				result
			},
			ExecMode::Single => {
				let result: HashMap<String, HashSet<String>> = str_list
				.iter()
				.map(|x| {
					let key = x.clone();
					let value = self.exec_for_string(x, execmode.clone());
					(key, value)
				})
				.collect();
				result
			},
		}
	}

	pub fn get_completion(&self, str_list: &[String], execmode: ExecMode, input: &str) -> Vec<String>
	{
		let ret_map = self.exec(str_list, execmode);
		let mut ret_list = Vec::new();
		for ( source, list ) in ret_map.iter() {
			if list.into_iter().any(|s| s.starts_with(input)) {
				ret_list.push(source.clone());
			}
		}
		return ret_list;
	}

	pub fn from_json_str(json_str: &str) -> Result<StrSolveTree,serde_json::Error>
	{
		let root_node_result =  serde_json::from_str(json_str);
		match root_node_result {
			Ok(root_node) => {
				let tree = StrSolveTree {
					root: root_node,
					};
				Ok(tree)
			},
			Err(e) => {
				Err(e)
			}
		}
	}
}

