# polLingua Completion

**polLingua Completion** 是一个支持多语言、多 Shell、高度可配置且用户友好的 Unicode 路径补全工具。

新版本使用 **Rust** 重写了核心逻辑，性能得到了质的飞跃。对于大多数用户，默认配置即可“开箱即用”；对于高级用户，它提供了极强的自定义空间。

---

## 核心特性

* **极速体验**：Rust 核心驱动，毫秒级响应。
* **广泛的语言支持**：涵盖中日韩（CJK）及多种拉丁、西里尔、希腊语系。
* **多 Shell 适配**：支持 Bash, Zsh, 及 Fish。
* **灵活配置**：通过环境变量完全掌控补全行为。

---

## 语言支持情况

### CJK（中日韩）
* **汉语**：
    * 普通话：支持全拼、首字母、声母。
    * 注音（台湾）：初步支持。
    * 粤语（香港）：暂不支持（计划中）。
* **日语**：支持汉字/假名转罗马字（Romaji）、汉字转假名。
* **韩语**：仅支持谚文（Hangeul），暂不支持韩国汉字（Hanja）。

### 其他语言
* **拉丁字母变体**：如捷克语等。
* **西里尔字母**：如俄语。
* **希腊字母**。
* *注：理论上对其他语言有初步支持，但对阿拉伯语、印地语、希伯来语、泰语支持较弱。如遇问题欢迎提交反馈。*

---

## Shell 支持与安装

### Bash
1.  **常规用户**：如果你使用了 `bash-completion`，支持 **原生 (推荐)** 和 **fzf** 两种模式。
2.  **ble.sh 用户**：上述模式不可用，有专门的 `ble.sh` 实现。
3.  **配置**：根据需求 `source` 对应的 `completer` 文件。

### Zsh
* 使用 Zsh 的 `compsys` 系统。
* 除了 `completer.zsh` 外，若需开箱即用，同时需要 `source setup.zsh`。后者包含必要配置，建议用户根据需求自行修改。

### Fish (推荐)
* **abbr_fzf.fish (推荐)**：依赖 `fzf`，这是目前的最佳实践。
* **abbr.fish**：不依赖 `fzf`，仅提供基础支持。
* **触发方式**：在 Fish 中使用 `abbr` API，输入 `@foo@` 或 `::foo::` 后按下 **空格** 即可自动触发解码。
* **completer_fzf.fish**：如果你习惯使用 Tab 补全，请使用此版本（依赖 `fzf`）。

---

## 系统要求

* **测试环境**：目前主要在 Gnu/Linux 上进行测试。
* **兼容性**：理论支持所有遵循 XDG 标准的类 Unix 环境。
* **二进制**：提供 Linux 静态链接二进制文件。BSD、macOS 用户请自行编译。

---

## 配置 (Environment Variables)

使用环境变量进行配置，请记得在 shell 配置文件中 `export` 它们。

### 1. 功能开关
| 环境变量 | 描述 | 默认值 |
| :--- | :--- | :--- |
| `POLINGUA_COMPLETION_CONVERTER_ENABLE_CHINESE` | 是否启用中文支持 | `true` |
| `POLINGUA_COMPLETION_CONVERTER_ENABLE_JAPANESE` | 是否启用日语支持 | `true` |
| `POLINGUA_COMPLETION_CONVERTER_ENABLE_KOREAN` | 是否启用韩语支持 | `true` |
| `POLINGUA_COMPLETION_CONVERTER_ENABLE_UNICODE_OTHER` | 是否启用其他 Unicode 语言 | `true` |
| `POLINGUA_COMPLETION_CONVERTER_ENABLE_ASCII` | 是否处理 ASCII 字符 | `false` |
| `POLINGUA_COMPLETION_CONVERTER_ENABLE_IDENTITY` | 是否对任意字符串匹配自身 | `true` |

### 2. 转换器详细配置 (Converter Config)
你可以通过以下变量自定义特定语言的转换链（格式见下方 Converter 章节）：
* `POLINGUA_COMPLETION_CONVERTER_CONFIG_CHINESE`
* `POLINGUA_COMPLETION_CONVERTER_CONFIG_JAPANESE`
* `POLINGUA_COMPLETION_CONVERTER_CONFIG_KOREAN`
* `POLINGUA_COMPLETION_CONVERTER_CONFIG_UNICODE_OTHER`
* `POLINGUA_COMPLETION_CONVERTER_CONFIG_ASCII`

**全局覆盖变量**：
* `POLINGUA_COMPLETION_CONVERTER_CONFIG`：如果设置，将覆盖上述所有特定语言的配置。

---

## 转换器 (Converters) 参数详解

| 转换器名称 | 描述 | 参数 |
| :--- | :--- | :--- |
| **identity** | 不改变任何字符串 | 无 |
| **unicode** | Unicode 转 ASCII | 无 |
| **unicode_advanced** | 高级 Unicode 转换 | `anyascii`, `deunicode`, `unidecode` (均为 bool) |
| **filter** | 过滤器 | `script`: (zh, jp, ko, cjk, latin...), `mode`: (include, only, no) |
| **zh_hanzi** | 汉字转拼音 | `format`: (full, first_letter, initials), `heteronym`: (bool) |
| **zh_hanzi_zhuyin** | 汉字转注音 | `heteronym`: (bool), `tone`: (bool) |
| **jp_all** | 日语转罗马字/假名 | `output`: (romaji, ascii, kana), `nbest`: (1-5) |
| **ko_hangeul** | 韩语转 ASCII/字母 | `output`: (ascii, jamo), `capitalize`: (no, all, choseong) |

---
