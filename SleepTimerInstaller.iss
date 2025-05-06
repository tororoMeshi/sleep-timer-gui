[Setup]
AppName=Sleep Timer with Pause
AppVersion=1.0
DefaultDirName={localappdata}\SleepTimer
DefaultGroupName=Sleep Timer
OutputDir=.
OutputBaseFilename=SleepTimerInstaller
Compression=lzma
SolidCompression=yes

[Files]
Source: "target\release\sleep_timer_gui.exe"; DestDir: "{app}"; Flags: ignoreversion
Source: "icon.png"; DestDir: "{app}"; Flags: ignoreversion

[Icons]
Name: "{group}\Sleep Timer"; Filename: "{app}\sleep_timer_gui.exe"; IconFilename: "{app}\icon.png"
Name: "{group}\Uninstall Sleep Timer"; Filename: "{uninstallexe}"

[Run]
Filename: "{app}\sleep_timer_gui.exe"; Description: "Sleep Timer を起動"; Flags: nowait postinstall skipifsilent
