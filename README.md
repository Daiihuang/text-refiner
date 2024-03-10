
## Introduction
<img src="src-tauri/icons/128x128.png" width=128 height=128>

***Text Refiner***: A powerful AI-powered application designed for refining and translating text.

Features:
 - [x] Translation: Translate your text to any target language.
 - [x] Local LLM: Use Ollama as a local LLM (available in offline mode).
 - [x] Providers: Multiple providers including OpenAI, Ollama, Claude (WIP).
 - [x] Hot key: Quickly translate selected text using shortcut key (`Cmd + E` at present)
 - [] Linux support.
 - [] Windows support.
 - [WIP] Custom prompt: Fine-tune the style of translations or text refinement.

***Note***: Please bear in mind that this application is currently under active development.

## Screenshots:
- Home:
![Refiner Home](./docs/images/home_sc.png)
- Settings:
![Refiner settings](./docs/images/setting_sc.png)


## Installation
1. Download the released file (.dmg for MacOS users) from the [Released page](https://github.com/Daiihuang/text-refiner/releases)
2. Install the app.
3. Set up your API key (required for OpenAI provider).
   - Click the Setting button conveniently located at the bottom-right corner of the app.
   - Select OpenAI as your provider and pick a model.
   - Enter your OpenAI's API key
   - Click `Save` to store the settings.
5. If you opt for the `Hot key` feature to quickly translate selected text by pressing `Cmd + E`, you will have to enable Accessibility permissions for the Text Refiner application. Here are the steps for doing so on a Mac:
    - Open System Preferences.
    - Locate and click on "Security & Privacy".
    - Switch to the "Privacy" tab.
    - Scroll down to "Accessibility" on the left-hand menu.
    - Click the lock icon located at the bottom-left corner and enter your password.
    - Add the app to the list by hitting the '+' button if it's not already there.
    - Check the box next to the Text Refiner app to grant Accessibility permissions.
    - Lock the settings again to prevent any further changes.

> Remember to grant Accessibility permissions only to apps you trust. They will have greater control over your system.

> Attention MacOS Users: If you encounter a problem stating "Refiner.app" is damaged, open terminal and execute: `xattr -c /Applications/Refiner.app`