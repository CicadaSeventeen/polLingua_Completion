#!/usr/bin/env python3
import json
import re
import os

def get_node_signature(name,args):
    """
    支持字典参数的签名构建
    """
    if isinstance(args, str):
        return (name,args)
    # 如果 args 是字典，将其转换为排序后的元组元组
    if isinstance(args, dict):
        # 排序是为了保证 {"a":1, "b":2} 和 {"b":2, "a":1} 生成相同的签名
        hashable_args = tuple(sorted(args.items()))
    elif isinstance(args, list):
        hashable_args = tuple(args)
    else:
        hashable_args = args
    return (name, hashable_args)

def build_complex_tree(config_list):
	# 根节点
	tree = {"func": "root", "args": {}, "children": {}, "need_return": False}

	for path in config_list:
		current_node = tree

		for i, func_str in enumerate(path):
			# 0. 获取参数列表，解码括号中保护的内容
			match = re.match(r"^([^(\s]+)\((.*)\)$", func_str.strip())
			if match:
				(func_name, func_argv_str) = match.groups()
				func_argv = {
					# 解析参数列表，分割参数名和参数值，并构建参数字典
					parts[0]: (parts[2] if parts[1] else True)
					for item_str in re.split(r'[,@#%^&]+',func_argv_str)
					if (parts := item_str.partition("="))
				}
			else:
				func_name = func_str.strip()
				func_argv = dict()
			if not func_name:
				func_name = "identity"
			# 由于名称的函数，若参数列表不同，则结果不同，因此必须判断函数名+参数列表的全等来判断是否要建立新节点
			# 1. 获取签名
			sig = get_node_signature(func_name, func_argv)
			# 2. 检查是否已存在全等的节点
			if sig not in current_node["children"]:
				# 创建新节点
				name, args = sig
				current_node["children"][sig] = {
					"func": func_name,
					"args": func_argv,
					"children": {},
					"need_return": False
				}

			# 3. 移动指针
			current_node = current_node["children"][sig]

			# 4. 标记终点
			# 只要有一条路径在这里结束，它就是终点。终点标签对于决定是否返回结果是必要的。
			if i == len(path) - 1:
				current_node["need_return"] = True
	#print(tree)
	return simplify_node(tree)

def simplify_node(node):
	"""
	递归将构建时的字典结构转换为最终的列表结构 (JSON friendly)
	"""
	return {
		"func": node["func"],
		"args": node["args"],  # 由于serde aux，字符串格式的数字和bool都可以不处理
		# 下面这行代码弃用了，根据测试字典好于列表
		#"args": [(key,"y" if (isinstance(node["args"][key],bool) and node["args"][key] == True) else node["args"][key]) for key in node["args"] ],
		"ret": True if node["need_return"] else False,
		# 这里把 children 的 values 取出来，丢弃掉用于去重的 Key (Signature)
		"children": [simplify_node(child) for child in node["children"].values()]
	}

def get_config_list():
	#从环境变量和默认值编码这段复杂的字符串变量
	tmp = ""
	if os.environ.get("POLINGUA_COMPLETION_CONVERTER_ENABLE_CHINESE","true").strip() == "true" :
		tmp = tmp + os.environ.get("POLINGUA_COMPLETION_CONVERTER_CONFIG_CHINESE",'filter(script=zh,mode=include),zh_hanzi(heteronym=true),unicode').strip() + ":"
	if os.environ.get("POLINGUA_COMPLETION_CONVERTER_ENABLE_JAPANESE","true").strip() == "true" :
		tmp = tmp + os.environ.get("POLINGUA_COMPLETION_CONVERTER_CONFIG_JAPANESE",'filter(script=jp,mode=include),jp_all(nbest=5),filter(script=hanzi,mode=no),unicode').strip() + ":"
	if os.environ.get("POLINGUA_COMPLETION_CONVERTER_ENABLE_KOREAN","true").strip() == "true" :
		tmp = tmp + os.environ.get("POLINGUA_COMPLETION_CONVERTER_CONFIG_KOREAN",'filter(script=ko,mode=include),ko_hangeul,filter(script=hanzi,mode=no),unicode').strip() + ":"
	if os.environ.get("POLINGUA_COMPLETION_CONVERTER_ENABLE_UNICODE_OTHER","true").strip() == "true":
		tmp = tmp + os.environ.get("POLINGUA_COMPLETION_CONVERTER_CONFIG_UNICODE_OTHER",'filter(script=unicode,mode=include),filter(script=cjk,mode=no),unicode').strip() + ":"
	if os.environ.get("POLINGUA_COMPLETION_CONVERTER_ENABLE_ASCII","false").strip() == "true" :
		tmp = tmp + os.environ.get("POLINGUA_COMPLETION_CONVERTER_CONFIG_ASCII",'filter(script=ascii,mode=only)').strip() + ":"
	if os.environ.get("POLINGUA_COMPLETION_CONVERTER_ENABLE_IDENTITY","true").strip() == "true" :
		tmp = tmp + os.environ.get("POLINGUA_COMPLETION_CONVERTER_CONFIG_IDENTITY",'identity').strip() + ":"
	tmp = re.sub(r":+", ":", tmp)
	if tmp == ":":
		tmp = ""
	tmp = os.environ.get("POLINGUA_COMPLETION_CONVERTER_CONFIG",tmp).strip()
	if tmp:
		config_list = []
		# 解码字符串变量到列表格式
		# 规则：
		# 1、以冒号作为分割，不同的冒号代表平行/并行的不同字符串处理逻辑
		# 2、以逗号（兼容@#%#^&）分割同一个串行的字符串处理逻辑中，先后执行的不同函数
		# 3、 成对括号用来保护参数列表，里面的逗号等分隔符暂时不被拆分
		for sublist_str in tmp.split(":"): #这里似乎没有解决连续冒号的问题，这是可能存在危险的地方。
			config_list.append(re.split(r'[,@#%\^&]+(?![^()]*\))', sublist_str))
	else:
		config_list = [
			["identity"],
			["filter(script=unicode,mode=include)","unicode"]
		]
	return config_list

def main():
	config_list = get_config_list()
	tree_config = build_complex_tree(config_list)
	#print(json.dumps(tree_config, indent=2))
	config_file_path = (os.environ.get("POLINGUA_COMPLETION_RUNTIME_TMPFILE_PATH")
		or os.path.join(os.environ.get("XDG_RUNTIME_DIR", "/tmp"), ".polingua.config.tmp")
		or "/tmp/.polingua.config.tmp"
	).strip()
	with open(config_file_path, "w", encoding="utf-8") as f:
		json.dump(tree_config, f)

if __name__ == "__main__":
    main()
