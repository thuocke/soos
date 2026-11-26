# Put soos-cli on your PATH

You have [installed Soos](install-help.md). These steps make the `soos-cli`
that came with it run from any terminal without copying it: a copy keeps
its own data, as [Data folder](soos-cli.md#data-folder) explains.

## macOS

`soos-cli` is inside the app, at
`/Applications/Soos.app/Contents/MacOS/soos-cli`.

1. Clear the quarantine mark, if you haven't: run
   `xattr -cr /Applications/Soos.app`, as in
   [Install Soos](install-help.md#macos). macOS refuses to run the unsigned
   `soos-cli` otherwise.
2. Link it, so it stays current when you update the app:

   ```
   sudo mkdir -p /usr/local/bin
   sudo ln -sf /Applications/Soos.app/Contents/MacOS/soos-cli /usr/local/bin/soos-cli
   ```

3. Open a new terminal and try it:

   ```
   $ soos-cli '20 inches in cm'
   50.8 cm
   ```

If you installed Soos with Homebrew: it is already linked, so start at
step 3.

## Linux

`soos-cli` is beside `soos-app` in the folder you extracted.

1. Link it into a folder on your `PATH`:

   ```
   mkdir -p ~/.local/bin
   ln -sf ~/.local/opt/soos-linux-x86_64/soos-cli ~/.local/bin/soos-cli
   ```

2. Open a new terminal and try it:

   ```
   $ soos-cli '20 inches in cm'
   50.8 cm
   ```

If you extracted Soos somewhere other than `~/.local/opt`: use that path in
step 1.
If `soos-cli` isn't found: your distribution doesn't put `~/.local/bin` on
`PATH`. Add `export PATH="$HOME/.local/bin:$PATH"` to your `~/.bashrc` or
`~/.zshrc`, and open a new terminal.

## Windows

`soos-cli.exe` is beside `soos-app.exe` in the folder you extracted.

1. Press Start, type "Edit environment variables for your account", and
   open it.
2. Select `Path`, click **Edit**, then **New**, and paste the path of the
   folder that holds `soos-cli.exe`.
3. Click **OK** in each window.
4. Open a new terminal and try it:

   ```
   soos-cli "20 inches in cm"
   ```

   It prints `50.8 cm`.

If you pipe results into PowerShell: run this first, or symbols such as `€`
and `≈` come out garbled.

```powershell
[Console]::OutputEncoding = [Text.Encoding]::UTF8
```

See also: [soos-cli](soos-cli.md)
