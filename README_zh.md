# polLingua Completion

**polLingua Completion** 是一个支持多语言、多 Shell、高度可配置且用户友好的 Unicode 路径补全工具。

对于大多数用户，默认配置即开箱即用；对于高级用户提供了充分的自定义空间。

v1版本使用 **Rust** 重写了核心逻辑，性能问题基本解决，并支持静态编译，提供Gnu/Linux静态链接二进制分发。

---

## 核心特性

* **广泛的语言支持**：涵盖中日韩（CJK）及其他语言。
* **多 Shell 适配**：支持 Bash, Zsh, 及 Fish。
* **灵活配置**：通过环境变量完全掌控补全行为。

---

## 语言支持情况

### CJK（中日韩）
* **汉语**：
    * 普通话：
         * 汉语拼音。
         *  *初步支持*注音（台湾）。
    * 粤语（香港）：**暂不**支持（计划中）。
* **日语**：支持汉字/假名转罗马字、汉字转假名。
* **韩语**：仅支持谚文，不支持韩国汉字。

### 其他语言
* **拉丁字母变体**：如捷克语等。
* **西里尔字母**：如俄语。
* **希腊字母**。
* *注：理论上对其他语言有初步支持，但缺乏测试和检验。如遇问题欢迎提交反馈。*

---

## Shell 支持与安装

安装需要：
* 将`pollingua-completion-core`放到PATH中并授予允许权限
* 根据你使用的shell环境，`source`下方对应的shell脚本文件

### Bash
*  **常规用户**：通常使用`bash-completion`进行补全，支持 **原生 (推荐)** 和 **fzf** 两种模式。
*   **ble.sh 用户**：上述模式不可用，有专门的 `ble.sh` 实现。

 ### Zsh
* 请使用 Zsh 的 `compsys` 系统。
* 除了 `completer.zsh` 外，若需开箱即用，同时需要加载`setup.zsh`。后者包含必要配置，建议用户根据需求自行修改。
* 请使用`menu-complete`而不是`expand-or-complete`以防止已知bug。

### Fish (推荐)
* **abbr_fzf.fish (推荐)**：依靠Fish的`abbr`纠错系统实现，依赖 `fzf`，这是目前的最佳实践。
* **abbr.fish**：依靠Fish的`abbr`纠错系统实现，不依赖 `fzf`，仅提供基础支持。
* **关于abbr**：`abbr`并不利用tab进行补全，而是半自动的。
	* **主动触发**：输入 `@foo@` 或 `::foo::` 后按下 **空格** 即可自动触发解码。（可修改具体行为，详情请自行查看对应Fish脚本）
	* **自动触发**：输入可补全的文本后，按下 **空格** 即可自动触发解码（对`abbr_fzf.fish`默认打开，对`abbr.fish`默认关闭，请自行斟酌是否开启）
* **completer_fzf.fish**：如果你希望使用 Tab 补全，请使用此版本（依赖 `fzf`）。

---

## 系统要求

* **测试环境**：目前主要在 Gnu/Linux 上进行测试。
* **兼容性**：理论支持所有遵循 XDG 标准的类 Unix 环境，如有问题请进行反馈。
* **二进制**：提供 Linux 静态链接二进制文件。BSD、macOS 和其他系统的用户请自行编译。

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
你可以通过以下变量自定义特定语言的转换链，否则将使用复杂的默认值。

* `POLINGUA_COMPLETION_CONVERTER_CONFIG_CHINESE`

* `POLINGUA_COMPLETION_CONVERTER_CONFIG_JAPANESE`

* `POLINGUA_COMPLETION_CONVERTER_CONFIG_KOREAN`

* `POLINGUA_COMPLETION_CONVERTER_CONFIG_UNICODE_OTHER`

* `POLINGUA_COMPLETION_CONVERTER_CONFIG_ASCII`


配置格式形如：
```
filter(script=zh,mode=include),zh_hanzi(heteronym=true),unicode:filter(script=zh,mode=include),zh_hanzi(heteronym=true,format=first_letter),unicode:identity
```
其中冒号分割平行逻辑，逗号连接先后逻辑，括号内为参数列表。

可选的转换器(Converters)和参数见下方章节。

**全局覆盖变量**：

* `POLINGUA_COMPLETION_CONVERTER_CONFIG`：默认不设置；如果设置，将覆盖上述所有特定语言的配置。

---

## 转换器 (Converters) 参数详解

| 转换器名称 | 描述 | 参数（粗体为默认值） | 说明/备注|
| :--- | :--- | :--- | :-- |
| **identity** | 不改变任何字符串 | -- |--|
| **unicode** | Unicode 转 ASCII | -- |--|
| **unicode_advanced** | 可配置的Unicode 转换 | `anyascii` (true/**false**) <br> `deunicode` (true/**false**)<br> `unidecode` (true/**false**) |选择是否开启对应后端|
| **filter** | 过滤符合规则的字符串 | `script`: (enum: zh, jp, ko, cjk, hanzi, kana, hangeul, latin, greek, cyrillic)<br>`mode`: (enum: **include**, only, no) |选择匹配的语言/文字类型<br>允许通过过滤的标准|
| **zh_hanzi** | 汉字转拼音 | `format`: (enum: **full**, first_letter, initials) <br>`capitalize`: (enum: **no**, all, first_letter, initials) <br>`heteronym`: (true/**false**) |输出的格式<br>是否输出为大写字母<br>多音字支持<br>别名**zh_hanzi_pinyin**|
| **zh_hanzi_zhuyin** | 汉字转注音 | `heteronym`: (true/**false**)<br>`tone`: (true/**false**) |多音字支持<br>是否带有声调<br>初步支持|
| **jp_all** | 日语转罗马字/假名 | `output`: (enum: **romaji**, ascii, kana)<br> `nbest`: (unsighed int, default as **1**) | 输出的格式（ascii比romaji更生硬）<br> 尝试的最佳匹配数（建议小于5）<br>别名**jp_kanji_and_kana**|
| **ko_hangeul** | 韩语谚文转 ASCII/韩语字母 | `output`: (enum: **ascii**, jamo)<br>`format`: (enum: **full**, first_letter, chosoeng<br>`capitalize`: (enum: **no**, all, choseong, first_letter) | 类似汉语拼音 |

---
