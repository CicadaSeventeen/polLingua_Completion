# polLingua Completion

[简体中文](https://github.com/CicadaSeventeen/polLingua_Completion/blob/main/README_zh.md)

**polLingua Completion** is a multi-language, multi-shell, highly configurable, and user-friendly Unicode path completion tool.

While the default configuration works out-of-the-box for most users, it provides extensive customization space for power users.

The v1 release features a core logic rewritten in **Rust**, effectively resolving performance issues. It supports static compilation and provides statically-linked binary distributions for Gnu/Linux.

---

## Core Features

* **Broad Language Support**: Covers CJK (Chinese, Japanese, Korean) and many other languages.
* **Multi-Shell Compatibility**: Supports Bash, Zsh, and Fish.
* **Flexible Configuration**: Full control over completion behavior via environment variables.

---

## Language Support Status

### CJK (Chinese, Japanese, Korean)
* **Chinese**:
    * Mandarin:
        * Putonghua Pinyin.
        * *Initial support* for Zhuyin/Bopomofo (Taiwan).
    * Cantonese (Hong Kong): **Not** currently supported (planned).
* **Japanese**: Supports Kanji/Kana to Romaji and Kanji to Kana conversion.
* **Korean**: Supports Hangul only; Hanja is not supported.

### Other Languages
* **Latin Script Variants**: Such as Czech, etc.
* **Cyrillic Script**: Such as Russian.
* **Greek Script**.
* *Note: Preliminary support for other languages exists in theory but lacks extensive testing. Feedback is welcome if you encounter issues.*

---

## Shell Support & Installation

Requirements:

* Place `pollingua-completion-core` in your PATH and grant execution permissions.

* `source` the corresponding shell script file based on your environment.

### Bash
* **Regular Users**: Typically uses `bash-completion`. Supports both **Native (Recommended)** and **fzf** modes.
* **ble.sh Users**: The above modes are unavailable; a dedicated `ble.sh` implementation is provided.
   * **Notice**: `ble.sh` users should use `ble-import -C 'source /path/to/completer_ble.sh' core-complete` instead of normal source. 

### Zsh
* Please use Zsh's `compsys` system.
* In addition to `completer.zsh`, you need to load `setup.zsh` for an out-of-the-box experience. The latter contains necessary configurations which users are encouraged to modify as needed.
* Please use **`menu-complete`** instead of `expand-or-complete` to avoid a known bug.

### Fish (Recommended)
* **abbr_fzf.fish (Recommended)**: Implemented using Fish's `abbr` (abbreviation) error-correction system and relies on `fzf`. This is currently the best practice.
* **abbr.fish**: Uses the `abbr` system without `fzf` dependency; provides basic support only.
   * **About abbr**: `abbr` does not use the Tab key for completion; it is semi-automatic.
      * **Manual Trigger**: Type `@foo@` or `::foo::` and press **Space** to automatically trigger decoding. (Behavior can be modified in the Fish scripts).
      * **Auto Trigger**: Type completable text and press **Space** to trigger decoding (Enabled by default for `abbr_fzf.fish`, disabled for `abbr.fish`).
* **completer_fzf.fish**: Use this version if you prefer traditional **Tab** completion (requires `fzf`).

---

## System Requirements

* **Test Environment**: Primarily tested on Gnu/Linux.
* **Compatibility**: Theoretically supports all Unix-like environments following XDG standards. Please report any issues.
* **Binaries**: Linux statically-linked binaries are provided. BSD, macOS, and other system users should compile from source.

---

## Configuration (Environment Variables)

Configuration is handled via environment variables. Remember to `export` them in your shell profile.

### 1. Feature Toggles
| Environment Variable | Description | Default |
| :--- | :--- | :--- |
| `POLINGUA_COMPLETION_CONVERTER_ENABLE_CHINESE` | Enable Chinese support | `true` |
| `POLINGUA_COMPLETION_CONVERTER_ENABLE_JAPANESE` | Enable Japanese support | `true` |
| `POLINGUA_COMPLETION_CONVERTER_ENABLE_KOREAN` | Enable Korean support | `true` |
| `POLINGUA_COMPLETION_CONVERTER_ENABLE_UNICODE_OTHER` | Enable other Unicode languages | `true` |
| `POLINGUA_COMPLETION_CONVERTER_ENABLE_ASCII` | Process ASCII characters | `false` |
| `POLINGUA_COMPLETION_CONVERTER_ENABLE_IDENTITY` | Match any string against itself | `true` |

### 2. Converter Details (Converter Config)
You can customize the conversion chain for specific languages using the following variables; otherwise, complex default values are used.

* `POLINGUA_COMPLETION_CONVERTER_CONFIG_CHINESE`

* `POLINGUA_COMPLETION_CONVERTER_CONFIG_JAPANESE`

* `POLINGUA_COMPLETION_CONVERTER_CONFIG_KOREAN`

* `POLINGUA_COMPLETION_CONVERTER_CONFIG_UNICODE_OTHER`

* `POLINGUA_COMPLETION_CONVERTER_CONFIG_ASCII`

**Configuration Format Example**:
```
filter(script=zh,mode=include),zh_hanzi(heteronym=true),unicode:filter(script=zh,mode=include),zh_hanzi(heteronym=true,format=first_letter),unicode:identity
```

* Colons (`:`) separate parallel logic.

* Commas (`,`) connect sequential logic.

* Parentheses `()` contain parameter lists.

**Global Override**:
* `POLINGUA_COMPLETION_CONVERTER_CONFIG`: Not set by default; if set, it overrides all language-specific configurations listed above.

---

## Converter Parameter Details

| Converter Name | Description | Parameters (**Bold** denotes default) | Notes |
| :--- | :--- | :--- | :-- |
| **identity** | Returns the string unchanged | -- | -- |
| **unicode** | Unicode to ASCII conversion | -- | -- |
| **unicode_advanced** | Configurable Unicode conversion | `anyascii` (true/**false**) <br> `deunicode` (true/**false**)<br> `unidecode` (true/**false**) | * Select backend toggle |
| **filter** | Filters strings matching rules | `script`: (enum: zh, jp, ko, cjk, hanzi, kana, hangeul, latin, greek, cyrillic)<br>`mode`: (enum: **include**, only, no) | * Select language/script type <br> * filtering criteria |
| **zh_hanzi** | Hanzi to Pinyin | `format`: (enum: **full**, first_letter, initials) <br>`capitalize`: (enum: **no**, all, first_letter, initials) <br>`heteronym`: (true/**false**) | Alias: **zh_hanzi_pinyin** |
| **zh_hanzi_zhuyin** | Hanzi to Zhuyin | `heteronym`: (true/**false**)<br>`tone`: (true/**false**) | *Initial support* |
| **jp_all** | Japanese to Romaji/Kana | `output`: (enum: **romaji**, ascii, kana)<br> `nbest`: (unsigned int, **1**) | * Output format (ascii is more raw)<br>* Best matches count (suggest < 5)<br> Alias: **jp_kanji_and_kana** |
| **ko_hangeul** | Hangul to ASCII/Jamo |`output`: (enum: **ascii**, jamo)<br>  `format`: (enum: **full**, first_letter, choseong) <br>`capitalize`: (enum: **no**, all, choseong, first_letter) | Similar to Chinese Pinyin converter |
