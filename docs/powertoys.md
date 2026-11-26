# Install the PowerToys Run plugin

You have Soos on Windows. The plugin lets you type `=` and a calculation in
PowerToys Run and read the answer as the app would show it; Enter copies
the plain value. There is no download: you build it from this repository.

1. Install PowerToys and the .NET 9 SDK, and clone this repository.
2. Put `soos-cli.exe` on your `PATH`, as in
   [Put soos-cli on your PATH](soos-cli-path.md#windows).
3. Build the plugin:

   ```
   cd integrations\powertoys
   dotnet build -c Release -p:Platform=x64
   ```

4. Quit PowerToys: click its tray icon, then **Exit**.
5. Copy the contents of
   `integrations\powertoys\bin\x64\Release\net9.0-windows10.0.26100.0\` to
   `%LOCALAPPDATA%\Microsoft\PowerToys\PowerToys Run\Plugins\Soos\`.
6. Start PowerToys again.
7. Press `Alt+Space`, then type `=20 inches in cm`. You see `50.8 cm`.

If you'd rather leave `PATH` alone: in step 2, point the plugin at
`soos-cli.exe` with an environment variable. In PowerShell, with its real
path:

```powershell
[Environment]::SetEnvironmentVariable("SOOS_EXE", "C:\Tools\Soos\soos-cli.exe", "User")
```

If you have an Arm device: build with `-p:Platform=ARM64` in step 3, and
copy from the `ARM64` folder, not `x64`, in step 5.
If it says "soos-cli.exe not found": PowerToys started before `PATH` or
`SOOS_EXE` was set. Restart PowerToys, or sign out and back in.
If PowerToys' own Calculator answers too: it also uses `=`. Give Soos
another activation keyword in PowerToys Settings, under **PowerToys Run**,
**Plugins**, **Soos**, or turn off the built-in Calculator plugin.
If a currency result says `no rates yet`: the rates couldn't be fetched and
none are saved. Open the app once while online.
If a converter from the app isn't recognized: open and close the app's
converters page (`settings`, then **Your converters**), which saves them
for `soos-cli`.

See also: [soos-cli](soos-cli.md)
