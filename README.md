# polLingua Completion
## Multi-language Latinization Completion Supporting zsh and bash, Primarily Supporting Chinese Pinyin (Mandarin)
Formerly known as `zsh-pinyin-completion-py`, see: https://github.com/CicadaSeventeen/zsh-pinyin-completion-py

[中文版本](https://github.com/CicadaSeventeen/polLingua_Completion/blob/main/README_zh.md)

## Features:
1. Supports Latinization completion for some other non-ASCII languages like Russian.
2. Highly customizable configuration and extensions.
3. Written purely in a scripting language, making deployment easy without compilation.

## Changes:
1. Experimental support for **bash** (ble.sh not supported for now) (Thanks to https://github.com/AOSC-Dev/bash-pinyin-completion-rs).
2. Refactored part of the abstraction layer, basically clearing the obstacles for adding new language support.
3. Rewrote some environment variable configurations.

## Usage:
1. Place the compressed package from the release in your desired location.
2. `source setup.zsh` or `source setup.sh`.
3. For **zsh**, `setup.zsh` is just a reference; customizing the configuration file according to personal needs is recommended.

## I Need Support!
1. Is it possible to support **fish**?
2. Better **bash** completion generation scripts.
3. Others.

## zsh completer
It is recommended to use the `_polLingua_smart` completer.

Other completers (generally not recommended):

 `_polLingua_startswith`

 `_polLingua_equal`

 `_polLingua_file_startswith`

 `_polLingua_file_equal`

 `_polLingua_dir_startswith`

 `_polLingua_dir_equal`

Recommended order:
```
zstyle ':completion:*' completer _commands _polLingua_smart _complete _correct _approximate _list
```
`_polLingua_smart` considers falling back to `_files`, so using `_files` is generally not recommended.

## Daemon (turbo) Mode
The normal mode is quite demanding on the CPU; daemon mode provides acceleration and is enabled by default.

The daemon mode primarily utilizes Linux's `prctl`. It should still be available on non-Linux systems like macOS or BSD, but shutting it down gracefully might not be possible. If residual daemons remain, they should have no substantial impact, although they might look unsightly.

## Configuration Method: Environment Variables (Note: Requires `export`)
#### `COMPLETION_CONVERTER_LIST`
A list of enabled string translation/conversion (converter) mechanisms. This is rather complex; non-advanced users are advised not to edit it.

Default value is:
`{pypinyin_filtered,anyascii}:{pypinyin_filtered,anyascii}:{pypinyin_filtered,anyascii}:{pypinyin,anyascii}:{pypinyin,anyascii}:{pypinyin,anyascii}:{filter_no_hanzi,unidecode}:{filter_no_hanzi:anyascii}:identity`

Where:

**`:`** acts like the colon separator in the environment variable PATH, separating parallel, distinct string converter groups.

**`,`** connects multiple converters that take effect sequentially within the same converter group.

**`{}`** wraps the converters in the same group; it can be omitted.

 The existence of multiple repeating groups is to correspond to the associated parameter list.

##### Currently Supported Converters:

 **Unicode General**: `unidecode`, `anyascii`

 **Chinese Pinyin**: `pypinyin`, `pypinyin_filtered`

 **Filter**: `filter_include_hanzi`, `filter_all_hanzi`, `filter_no_hanzi`, `filter_include_unicode`, `filter_no_unicode`, `filter_all_ascii`

 **Basic**: `simplify`, `remove`, `first_letter`

 **Case**: `upper`, `lower`, `capitalize`, `capitalize_title`

 **Chinese Initial Case**: `initials_capitalize`, `initials_capitalize_title`

#### `COMPLETION_CONVERTER_ARGUMENT_LIST`
The parameter list corresponding to the converter groups. This is quite complex; non-advanced users are advised not to edit it.

Default value is:
` {style=normal#filter=capitalize,none}:{style=first_letter#filter=capitalize,none}:{style=initials#filter=initials_capitalize,none}:{style=normal,none}:{style=first_letter,none}:{style=initials,none}:{none,none}:{none,none}:none`

Where:

**`#`** connects different parameters taking effect on the same converter.

Most converters have no parameters; fill in `none`.

##### `pypinyin` Parameters:

**`style`**: The type of Latin string output. Supports `normal`, `first_letter`, and `initials`.

**`heteronym`**: Whether to enable support for polyphones (multiple pronunciations). Default is `auto`; other options are `on`, `off`, `all`.

**`strict`**: Strict mode, usually not required to be enabled.

##### `pypinyin_filtered` Parameters:
Basically the same as above, but supports the `filter` parameter. It supports enabling a filter-type converter that only acts on Chinese pinyin, used to handle filenames mixed with Chinese characters and Latin letters.

#### `COMPLETION_FILENAME_MATCH_MODE`
Whether to support partial completion. Defaults to `startswith`; can be set to `equal`.

#### `COMPLETION_CASE_INSENSITIVE`
Whether completion is case-insensitive. Setting it to `yes` performs case-insensitive fuzzy matching for all input. Defaults to `no`.

#### `COMPLETION_FILE_TYPE`
Matches directories or non-directory files. Defaults to `dir:file`; you can select `dir` or `file` separately. Configuration modification is generally not recommended.

#### `COMPLETION_SHOW_HIDDEN`
Whether to match hidden files. Defaults to `yes`.

#### `COMPLETION_STRING_QUOTE_MODE`
Internal variable, usually does not require configuration.
