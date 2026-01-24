use std::collections::{BTreeMap, HashMap};
use std::env;
use serde::{Serialize, Serializer};
use regex::Regex;
use lazy_static::lazy_static;

// --- 数据结构定义 ---

// 参数值可以是字符串，也可以是布尔值(True)，使用 enum 或 serde_json::Value 存储
// 为了匹配 Python 逻辑：parts[2] if parts[1] else True
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum ArgValue {
    Str(String),
    Bool(bool),
}

// 自定义序列化逻辑，以匹配 Python 输出的格式
impl Serialize for ArgValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match self {
            ArgValue::Str(s) => serializer.serialize_str(s),
            ArgValue::Bool(b) => serializer.serialize_bool(*b),
        }
    }
}

// BTreeMap 自动按 Key 排序，这完美替代了 Python 中的 sorted(args.items())
// 这意味着 (func_name, args) 本身就可以直接作为 Map 的 Key，无需额外计算签名
type FunctionArgs = BTreeMap<String, ArgValue>;

// 构建时的节点结构 (使用 Map 方便去重)
struct BuilderNode {
    func: String,
    args: FunctionArgs,
    children: HashMap<(String, FunctionArgs), BuilderNode>,
    need_return: bool,
}

impl BuilderNode {
    fn new(func: String, args: FunctionArgs) -> Self {
        Self {
            func,
            args,
            children: HashMap::new(),
            need_return: false,
        }
    }
}

// 最终输出的节点结构 (使用 Vec 方便 JSON 序列化)
#[derive(Serialize)]
struct OutputNode {
    func: String,
    args: FunctionArgs,
    #[serde(rename = "ret")]
    need_return: bool,
    children: Vec<OutputNode>,
}

// --- 解析逻辑 ---

lazy_static! {
    // 用于解析 func(args) 结构
    static ref RE_FUNC: Regex = Regex::new(r"^([^(\s]+)\((.*)\)$").unwrap();
    // 用于分割参数内部的分隔符
    static ref RE_ARG_SPLIT: Regex = Regex::new(r"[,@#%^&]+").unwrap();
}

/// 解析单个函数字符串，例如 "filter(script=zh)" 或 "unicode"
fn parse_function_token(token: &str) -> (String, FunctionArgs) {
    let token = token.trim();
    if let Some(caps) = RE_FUNC.captures(token) {
        let func_name = caps.get(1).map_or("", |m| m.as_str()).to_string();
        let args_str = caps.get(2).map_or("", |m| m.as_str());

        let mut args = FunctionArgs::new();
        if !args_str.is_empty() {
            // 分割参数，例如 script=zh,mode=include
            for item in RE_ARG_SPLIT.split(args_str) {
                if item.trim().is_empty() { continue; }

                // 处理 key=value 或 flag
                match item.split_once('=') {
                    Some((k, v)) => {
                        args.insert(k.trim().to_string(), ArgValue::Str(v.trim().to_string()));
                    }
                    None => {
                        // 对应 Python: parts[2] if parts[1] else True
                        args.insert(item.trim().to_string(), ArgValue::Bool(true));
                    }
                }
            }
        }
        (func_name, args)
    } else {
        let func_name = token.to_string();
        let func_name = if func_name.is_empty() { "identity".to_string() } else { func_name };
        (func_name, FunctionArgs::new())
    }
}

/// 替代 Python 的 re.split(..., lookahead)
/// 手动遍历字符串，根据括号深度进行分割，确保不切分函数参数内部的逗号
fn split_pipeline_string(input: &str) -> Vec<String> {
    let mut parts = Vec::new();
    let mut current = String::new();
    let mut depth = 0;

    // Python 的分隔符集合: , @ # % ^ &
    let is_separator = |c: char| matches!(c, ',' | '@' | '#' | '%' | '^' | '&');

    for c in input.chars() {
        match c {
            '(' => {
                depth += 1;
                current.push(c);
            }
            ')' => {
                if depth > 0 { depth -= 1; }
                current.push(c);
            }
            c if is_separator(c) && depth == 0 => {
                if !current.trim().is_empty() {
                    parts.push(current.trim().to_string());
                }
                current.clear();
            }
            _ => current.push(c),
        }
    }
    if !current.trim().is_empty() {
        parts.push(current.trim().to_string());
    }
    parts
}

// --- 核心业务逻辑 ---

fn build_complex_tree(config_list: Vec<Vec<String>>) -> OutputNode {
    // 根节点
    let mut root = BuilderNode::new("root".to_string(), FunctionArgs::new());

    for path in config_list {
        let mut current_node = &mut root;

        for (i, func_str) in path.iter().enumerate() {
            // 1. 解析函数名和参数
            let (func_name, func_args) = parse_function_token(func_str);

            // 2. 获取签名 (在 Rust 中，(String, BTreeMap) 本身就是 Key)
            let signature = (func_name.clone(), func_args.clone());

            // 3. 检查并插入子节点
            // 利用 Entry API 避免重复查找
            current_node = current_node.children
                .entry(signature)
                .or_insert_with(|| BuilderNode::new(func_name, func_args));

            // 4. 标记终点
            if i == path.len() - 1 {
                current_node.need_return = true;
            }
        }
    }

    simplify_node(root)
}

/// 递归将 BuilderNode (Map children) 转换为 OutputNode (Vec children)
fn simplify_node(node: BuilderNode) -> OutputNode {
    let children: Vec<OutputNode> = node.children
        .into_values()
        .map(simplify_node)
        .collect();

    // 为了输出的确定性（方便测试或比对），可以对 children 进行排序，虽然 JSON 规范不强求
    // 这里暂时不做，为了保持与 Python 逻辑一致（Python 那边 .values() 顺序取决于哈希）

    OutputNode {
        func: node.func,
        args: node.args,
        need_return: node.need_return,
        children,
    }
}

fn get_config_list() -> Vec<Vec<String>> {
    let mut tmp = String::new();

    // 辅助闭包：获取 env 或默认值，拼接到 tmp
    let mut append_env = |key: &str, default_enable: &str, config_key: &str, default_config: &str| {
        let enable = env::var(key).unwrap_or_else(|_| default_enable.to_string());
        if enable.trim() == "true" {
            let config = env::var(config_key).unwrap_or_else(|_| default_config.to_string());
            tmp.push_str(config.trim());
            tmp.push(':');
        }
    };

    append_env("POLLINGUA_COMPLETION_CONVERTER_ENABLE_CHINESE", "true", "POLLINGUA_COMPLETION_CONVERTER_CONFIG_CHINESE", "filter(script=zh,mode=include),zh_hanzi(heteronym=true),filter(script=cjk,mode=no),unicode:filter(script=zh,mode=include),zh_hanzi(heteronym=true,format=first_letter),filter(script=cjk,mode=no),unicode:filter(script=zh,mode=include),zh_hanzi(heteronym=true,format=initials),filter(script=cjk,mode=no),unicode");
    append_env("POLLINGUA_COMPLETION_CONVERTER_ENABLE_JAPANESE", "true", "POLLINGUA_COMPLETION_CONVERTER_CONFIG_JAPANESE", "filter(script=jp,mode=include),jp_all(nbest=5),filter(script=cjk,mode=no),unicode");
    append_env("POLLINGUA_COMPLETION_CONVERTER_ENABLE_KOREAN", "true", "POLLINGUA_COMPLETION_CONVERTER_CONFIG_KOREAN", "filter(script=ko,mode=include),ko_hangeul,filter(script=cjk,mode=no),unicode:filter(script=ko,mode=include),ko_hangeul(format=first_letter),filter(script=cjk,mode=no),unicode::filter(script=ko,mode=include),ko_hangeul(format=choseong),filter(script=cjk,mode=no),unicode");
    append_env("POLLINGUA_COMPLETION_CONVERTER_ENABLE_UNICODE_OTHER", "true", "POLLINGUA_COMPLETION_CONVERTER_CONFIG_UNICODE_OTHER", "filter(script=unicode,mode=include),filter(script=cjk,mode=no),unicode");
    append_env("POLLINGUA_COMPLETION_CONVERTER_ENABLE_ASCII", "false", "POLLINGUA_COMPLETION_CONVERTER_CONFIG_ASCII", "filter(script=ascii,mode=only)");
    append_env("POLLINGUA_COMPLETION_CONVERTER_ENABLE_IDENTITY", "true", "POLLINGUA_COMPLETION_CONVERTER_CONFIG_IDENTITY", "identity");

    // 处理连续冒号
    let re_colons = Regex::new(r":+").unwrap();
    let tmp_cleaned = re_colons.replace_all(&tmp, ":");
    let tmp_final = if tmp_cleaned == ":" { "" } else { &tmp_cleaned };

    // 最终的环境变量覆盖
    let final_config_str = env::var("POLLINGUA_COMPLETION_CONVERTER_CONFIG")
        .unwrap_or_else(|_| tmp_final.to_string());

    let final_config_str = final_config_str.trim();

    if !final_config_str.is_empty() {
        final_config_str
            .split(':')
            .filter(|s| !s.is_empty())
            .map(|sublist_str| split_pipeline_string(sublist_str))
            .collect()
    } else {
        vec![
            vec!["identity".to_string()],
            vec!["filter(script=unicode,mode=include)".to_string(), "unicode".to_string()],
        ]
    }
}

pub fn build_json_config_from_env() -> String
{
    let config_list = get_config_list();
    let tree = build_complex_tree(config_list);
    let json_output = serde_json::to_string_pretty(&tree).unwrap();
    json_output
}
