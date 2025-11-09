# polLingua Completion
## 支持zsh和bash的多语言拉丁化补全，主要支持汉语拼音（普通话）
以前叫做`zsh-pinyin-completion-py`，见：https://github.com/CicadaSeventeen/zsh-pinyin-completion-p

## 特色：
支持部分其他非ascii语言如俄语的拉丁化补全

高度可自定义的配置和扩展

纯脚本语言写成，方便部署，无需编译

## 变更：
1、对bash的实验性支持（暂不支持ble.sh） (感谢https://github.com/AOSC-Dev/bash-pinyin-completion-rs)

2、重构部分抽象层，基本扫清添加新语言支持的障碍

3、重写了部分环境变量配置

## 使用方法：
1、将release中的压缩包放在你希望的位置

2、`source setup.zsh`或`source setup.zsh`

3、对`zsh`而言，`setup.zsh`只是一个参考，推荐根据个人需求自定义配置文件

##  我需要支持！
1、是否可能支持`fish`？

2、更好的`bash`补全生成脚本

3、其他

## zsh completer
推荐使用`_polLingua_smart` completer

其他的completer（通常不推荐使用）:

_polLingua_startswith

_polLingua_equal

_polLingua_file_startswith

_polLingua_file_equal

_polLingua_dir_startswith

_polLingua_dir_equal

推荐的顺序：
```
zstyle ':completion:*' completer _commands _polLingua_smart _complete _correct _approximate _list
```
`_polLingua_smart`考虑了fallback到`_files`的情况，因此通常不推荐用`_files`

## Daemon(turbo)模式
普通模式对cpu要求较大，daemon模式提供了加速，默认已经启用。

deamon模式优先采用Linux的`prctl`，在非Linux系统如macos或BSD仍然应当可用，但可能无法很优雅地正常关闭。若出现驻留的残余daemon，应当没有实质影响，但是会很丑陋

## 配置方法：环境变量，注意需要`export`
#### `COMPLETION_CONVERTER_LIST`
包括启用的各种字符串翻译/转换器(converter)的列表，较为复杂，非高级用户不建议编辑。

默认为
`{pypinyin_filtered,anyascii}:{pypinyin_filtered,anyascii}:{pypinyin_filtered,anyascii}:{pypinyin,anyascii}:{pypinyin,anyascii}:{pypinyin,anyascii}:{filter_no_hanzi,unidecode}:{filter_no_hanzi:anyascii}:identity`

其中:

`:` 类似于环境变量PATH中的冒号，分隔彼此平行的不同字符串转化器组。

`,` 连接同一组转换器组中先后发挥作用的多个转换器。

`{}` 包裹同一组别的转换器组，可以省略

这里之所以存在多个重复组，是为了和对应的参数列表对应。

##### 目前支持的转换器：

unicode通用：unidecode, anyascii

汉语拼音: pypinyin, pypinyin_filtered

过滤器(filter): filter_include_hanzi, filter_all_hanzi, fitler_no_hanzi, filter_include_unicode, filter_no_unicode, filter_all_ascii

基础: simplify, remove, first_letter

大小写: upper, lower, capitalize, capitalize_title

汉语声母大小写: initials_capitalize, initials_capitalize_title

#### `COMPLETION_CONVERTER_ARGUMENT_LIST`
转化器组对应的参数列表。较为复杂，非高级用户不建议编辑。

默认为
` {style=normal#filter=capitalize,none}:{style=first_letter#filter=capitalize,none}:{style=initials#filter=initials_capitalize,none}:{style=normal,none}:{style=first_letter,none}:{style=initials,none}:{none,none}:{none,none}:none`

其中：

`#` 连接了对同一个转化器起效的不同参数

多数转换器没有参数，请填入none

##### pypinyin的参数:

`style` 输出的拉丁字符串类型，支持`normal`, `first_letter`和`initials`

`heteronym` 是否开启多音字支持，默认`auto`，其他可选`on`, `off`, `all`

`strict` 严格模式，通常不需要启用

##### pypinyin_filtered的参数:

基本同上，但支持`filter`参数。支持启用一个filter类型的转换器，且只对于汉语拼音生效，用于处理汉字和拉丁字母混合的文件名。

#### `COMPLETION_FILENAME_MATCH_MODE`
是否支持部分补全，默认为`startswith`，可选`equal`

#### `COMPLETION_CASE_INSENSITIVE`
是否大小写不敏感。设置为`yes`则对一切输入进行大小写模糊匹配。默认为`no`

#### `COMPLETION_FILE_TYPE`
匹配目录还是非目录文件，默认`dir:file`，可以单独选择`dir`或`file`。通常不建议修改配置。

#### `COMPLETION_SHOW_HIDDEN`
是否匹配隐藏文件，默认为`yes`

#### `COMPLETION_STRING_QUOTE_MODE`
内部变量，通常不需要配置
