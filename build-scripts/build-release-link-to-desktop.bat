cargo build --release --manifest-path=%~dp0Cargo.toml
copy %~dp0target\x86_64-pc-windows-msvc\release\stellar-guesser-desktop.exe %~dp0
del %~dp0stellar-guesser.exe
ren %~dp0stellar-guesser-desktop.exe %~dp0stellar-guesser.exe
powershell "$s=(New-Object -COM WScript.Shell).CreateShortcut('%userprofile%\Desktop\Stellar Guesser.lnk');$s.TargetPath='%~dp0stellar-guesser-desktop.exe';$s.WorkingDirectory = '%~dp0';$s.Save()"
