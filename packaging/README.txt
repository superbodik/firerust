firerust
========

A small browser written from scratch in Rust: its own page engine, no Chromium
and no Firefox inside. It aims to use far less memory than a big browser on
simple and medium sites.

Run
---
Double-click firerust.exe, or start it from a terminal:

    firerust.exe
    firerust.exe https://example.com
    firerust.exe https://example.com https://example.org

Several addresses open as several tabs. A path to an HTML file works too.

Keys
----
Ctrl+T new tab            Ctrl+W close tab        Ctrl+Tab next tab
Ctrl+L or F6 address bar  Ctrl+D bookmark page    Ctrl+B bookmarks
Ctrl+S save typed password  Ctrl+P passwords      Ctrl+M memory panel
Ctrl+Plus / Minus / 0 zoom                        Alt+Left / Alt+Right history
F5 or Ctrl+R reload       Ctrl+click or middle click opens a link in a new tab

The menu button (three dots) opens about:menu with settings, console,
bookmarks, passwords and import from other browsers.

Where your data lives
---------------------
%APPDATA%\firerust\bookmarks.json   bookmarks
%APPDATA%\firerust\passwords.bin    saved passwords, encrypted for your Windows account
%APPDATA%\firerust\settings.json    settings
Downloads go to your Downloads folder. Set FIRERUST_DOWNLOADS to change it.
Set FIRERUST_THEME to light or dark to override the theme setting.

Please read SUPPORTED.txt before relying on it. Do not use firerust for banking,
payments or mail yet.
