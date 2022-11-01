# Auspex, for reading AUR
Simple commandline tool for Arch Linux to list packages installed from AUR that have updates ready. Useful helper for people who prefer to manually operate AUR rather than use a frontend, but don't want to have to manually _check_ AUR for updates.

## Usage
Run auspex from the terminal without any parameters like so:
```bash
$ auspex
```
If any AUR-installed packages have a newer version in AUR than on your local machine, you will receive a list of them such as this example:
```bash
$ auspex
gdm-settings 1.0-1 has new version 1.1-1 available: https://aur.archlinux.org/packages/gdm-settings
unityhub 3.2.9-1 has new version 3.3.0-1 available: https://aur.archlinux.org/packages/unityhub
$
```