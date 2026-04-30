# launcher

Simple launcher that supports long arguments and per-launch environment variables.

In Windows, you can use a total of 8,192 characters, including the program's path and its arguments.<br>
However, the `.lnk` shortcut only allows 255 characters.<br>
This means you can't use Windows `.lnk` shortcut if you have a long argument.

That's why this program was created.

## Usage

```ini
[launcher]
program=
argument=
[environment]
ENV_KEY_1=ENV_KEY_1_VALUE
ENV_KEY_2=ENV_KEY_2_VALUE
ENV_KEY_3=ENV_KEY_3_VALUE
```

* `program`: Path to the target executable.
* `argument`: Raw argument string to pass to the target executable.
* `[environment]`: Environment variables to inject into the child process.

`program`, `argument`, and every value in `[environment]` are expanded with Windows-style environment variables (`%VAR%`) before launch.

Example:

* `program=%AppData%\Program\VSCode\code.exe`
* `TEMP_DIR=%TEMP%\my_launcher_temp`

`[environment]` values override existing environment variables with the same key (INI section has higher priority).

By default, `launcher.exe` will try to read `launcher.ini` in the same directory.<br>
If you change the name of the `launcher.exe` to something else like `shortcut.exe`, then it'll read `shortcut.ini`.<br>
Or you can run `launcher.exe` with the `-ini <ini path>` argument, like `launcher.exe -ini shortcut.ini`.

### Example Usage

Here is an example of the INI.

```ini
[launcher]
program=ungoogled_chromium_bin\chrome.exe
argument=--user-data-dir=..\ungoogled_chromium_profile --no-default-browser-check --disable-search-engine-collection --extension-mime-request-handling=always-prompt-for-install --disable-sharing-hub --remove-tabsearch-button --show-avatar-button=never --disable-top-sites --disable-logging --disable-breakpad --disable-features=PrintCompositorLPAC,RendererCodeIntegrity --enable-features=IncreaseIncognitoStorageQuota,DisableQRGenerator --disable-encryption --disable-machine-id
[environment]
CHROME_LOG_FILE=%TEMP%\launcher_chrome.log
```

## Build

You need Rust toolchain (`cargo`) installed.

```
cargo build --release
```