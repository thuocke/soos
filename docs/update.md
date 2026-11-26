# Update Soos

You have Soos installed and the new download for your system. Soos saves
your tabs when you close it, so each set of steps starts by quitting it.
The downloads hold no `data` folder, so the new copy keeps your tabs.

## Windows

1. Quit Soos: right-click its tray icon, then click **Quit**. Closing the
   window only hides it.
2. Extract the new `.zip` into the same folder as before, and replace the
   files when Windows asks.
3. Start Soos.

If you'd rather keep the old folder: extract the new `.zip` somewhere new,
and move the old `data` folder into the new `soos-windows-x86_64` folder.

## macOS

1. Quit Soos: right-click its menu bar icon, then click **Quit**.
2. Open the new `.dmg` and drag Soos to Applications, replacing the old one.
   Your tabs are in `~/Library/Application Support/Soos/`, not in the app,
   so they stay.
3. Run `xattr -cr /Applications/Soos.app` again, as in
   [Install Soos](install-help.md#macos): the new copy is unsigned too.
4. Open Soos.

If you installed with Homebrew: quit Soos and run
`brew upgrade --cask soos` in place of steps 2 to 4.

## Linux

1. Close Soos.
2. Extract the new archive over the old folder:

   ```
   tar xzf soos-linux-x86_64.tar.gz -C ~/.local/opt
   ```

3. Start Soos.

If you installed Soos somewhere other than `~/.local/opt`, the folder
[Install Soos](install-help.md#linux) uses: put that folder after `-C`.

## Android

1. Download the new `soos-android.apk`.
2. Open it. Android installs it over the old Soos and keeps your tabs.

Every release is signed with the same key, which is how Android knows it is
the same app.

See also: [Install Soos](install-help.md) · [Uninstall Soos](uninstall.md)
