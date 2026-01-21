# polLingua Completion

**polLingua Completion** is a highly configurable, user-friendly Unicode path completion tool with multi-language and multi-shell support.

The core has been rewritten in **Rust**, offering significantly improved performance. While the default configuration is "out-of-the-box" for most users, it remains deeply customizable for power users.

---

## Key Features

* **High Performance**: Powered by a Rust core for millisecond-level responses.
* **Multilingual**: Comprehensive support for CJK and various international scripts.
* **Multi-Shell Integration**: Native support for Bash, Zsh, and Fish.
* **Flexible Config**: Fully controlled via Environment Variables.

---

## Language Support

### CJK (Chinese, Japanese, Korean)
* **Chinese**:
    * Mandarin: Pinyin (Full, First Letter, Initials).
    * Zhuyin (Bopomofo): Preliminary support.
    * Cantonese: Not supported yet (Planned).
* **Japanese**: Supports Kanji/Kana to Romaji and Kanji to Kana.
* **Korean**: Hangeul support only (Hanja is not supported).

### Others
* **Latin variants**: e.g., Czech.
* **Cyrillic**: e.g., Russian.
* **Greek**.
* *Note: Support for Arabic, Hindi, Hebrew, and Thai may be limited. If you encounter issues, please contact the developer.*

---

## Shell Support & Setup

### Bash
1.  **Standard Users**: Supports **Native (Recommended)** and **fzf** completion modes.
2.  **ble.sh Users**: Use the dedicated `ble.sh` implementation.
3.  **Setup**: Source the specific `completer` file based on your needs.

### Zsh
* Utilizes the Zsh `compsys` system.
* In addition to `completer.zsh`, source `setup.zsh` for an out-of-the-box experience.

### Fish (Recommended)
* **abbr_fzf.fish (Recommended)**: Depends on `fzf`. This is currently the best practice.
* **abbr.fish**: Basic support without `fzf` dependency.
* **Usage**: To trigger decoding, wrap the target characters like `@foo@` or `::foo::` and press **Space**.
* **completer_fzf.fish**: For those who prefer traditional Tab completion (requires `fzf`).

---

## Configuration (Environment Variables)

Configure the tool by exporting the following variables in your shell profile:

### 1. Feature Toggles
| Variable | Description | Default |
| :--- | :--- | :--- |
| `POLINGUA_COMPLETION_CONVERTER_ENABLE_CHINESE` | Enable Chinese support | `true` |
| `POLINGUA_COMPLETION_CONVERTER_ENABLE_JAPANESE` | Enable Japanese support | `true` |
| `POLINGUA_COMPLETION_CONVERTER_ENABLE_KOREAN` | Enable Korean support | `true` |
| `POLINGUA_COMPLETION_CONVERTER_ENABLE_UNICODE_OTHER` | Enable other Unicode scripts | `true` |
| `POLINGUA_COMPLETION_CONVERTER_ENABLE_ASCII` | Process ASCII characters | `false` |
| `POLINGUA_COMPLETION_CONVERTER_ENABLE_IDENTITY` | Match original string | `true` |

### 2. Converter Chain Configuration
Customize specific logic for each language:
* `POLINGUA_COMPLETION_CONVERTER_CONFIG_CHINESE`
* `POLINGUA_COMPLETION_CONVERTER_CONFIG_JAPANESE`
* ...and so on.

**Global Override**:
* `POLINGUA_COMPLETION_CONVERTER_CONFIG`: If set, overrides all specific language configs.

---

## Converters Reference

| Converter | Description | Parameters |
| :--- | :--- | :--- |
| **identity** | No changes | None |
| **unicode** | Unicode to ASCII | None |
| **unicode_advanced** | Advanced Unicode conv | `anyascii`, `deunicode`, `unidecode` (bool) |
| **filter** | Script filter | `script`: (zh, jp, ko...), `mode`: (include, only, no) |
| **zh_hanzi** | Hanzi to Pinyin | `format`: (full, initials...), `heteronym`: (bool) |
| **jp_all** | Japanese to Romaji/Kana | `output`: (romaji, kana), `nbest`: (1-5) |
| **ko_hangeul** | Hangeul to ASCII/Jamo | `output`: (ascii, jamo), `capitalize`: (no, all...) |

---
