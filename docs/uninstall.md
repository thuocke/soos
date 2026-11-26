# Uninstall Soos

You have Soos installed. On Windows, Linux and Android, removing it deletes
your tabs and settings too; on macOS they are a separate folder.

## Windows

1. Quit Soos: right-click its tray icon, then click **Quit**.
2. Delete its folder. Your tabs and settings go with it: they are in its
   `data` folder.

If you put `soos-cli` on your `PATH`: remove that folder from `Path`.
If you installed the PowerToys plugin: delete
`%LOCALAPPDATA%\Microsoft\PowerToys\PowerToys Run\Plugins\Soos\`, and the
[`SOOS_EXE`](powertoys.md) variable if you set it.

## macOS

1. Quit Soos: right-click its menu bar icon, then click **Quit**.
2. Drag Soos from Applications to the Trash.
3. Delete `~/Library/Application Support/Soos/`, which holds your tabs and
   settings.

If you linked `soos-cli`: remove the link with
`sudo rm /usr/local/bin/soos-cli`.
If you installed with Homebrew: quit Soos and run
`brew uninstall --cask soos` in place of the steps. That keeps your tabs;
`brew uninstall --cask --zap soos` deletes them too.

## Linux

1. Close Soos.
2. Delete its folder. Your tabs and settings go with it: they are in its
   `data` folder.

If you added Soos to your applications menu: delete
`~/.local/share/applications/soos.desktop`.
If you linked `soos-cli`: delete `~/.local/bin/soos-cli`.

## Android

Uninstall it as you would any app. That deletes your tabs and settings too.

See also: [Install Soos](install-help.md)
