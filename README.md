# launcher

Simple launcher that supports long arguments.

In Windows, you can use a total of 8,192 characters, including the program's path and its arguments.<br>
However, the `.lnk` shortcut only allows 255 characters.<br>
This means you can't use Windows `.lnk` shortcut if you have a long argument.

That's why this program was created.

## Usage

```ini
[launcher]
program=
argument=
```

* `program`: The path to the program that will be run. It can be the absolute path or relative path of the `launcher.exe`.
* `argument`: Arguments that you want to pass to the `program`.

By default, `launcher.exe` will try to read `launcher.ini` in the same directory.<br>
If you change the name of the `launcher.exe` to something else like `shortcut.exe`, then it'll read `shortcut.ini`.<br>
Or you can run `launcher.exe` with the `-ini <ini path>` argument, like `launcher.exe -ini shortcut.ini`.

### Example Usage

Here is an example of the INI.

```ini
[launcher]
program=ungoogled_chromium_bin\chrome.exe
argument=--user-data-dir=..\ungoogled_chromium_profile --no-default-browser-check --disable-search-engine-collection --extension-mime-request-handling=always-prompt-for-install --disable-sharing-hub --remove-tabsearch-button --show-avatar-button=never --disable-top-sites --disable-logging --disable-breakpad --disable-features=PrintCompositorLPAC,RendererCodeIntegrity --enable-features=IncreaseIncognitoStorageQuota,DisableQRGenerator --disable-encryption --disable-machine-id
```

## Build

You need either one of these:

* Visual Studio 2026 (or corresponding Visual Studio Build Tool)
* [MinGW-w64](https://winlibs.com/)

Then, run `build_windows.cmd`.

> [!NOTE]
> Since I mainly use this launcher as a Chromium launcher, the default icon of the launcher is Chromium icon.<br>
> You can change the icon by replacing `resource/icon.ico` file.
