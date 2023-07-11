
cargo build --release --manifest-path=%~dp0Cargo.toml
copy %~dp0target\release\stellar-guesser.exe %~dp0
powershell "$s=(New-Object -COM WScript.Shell).CreateShortcut('%userprofile%\Desktop\Stellar Guesser.lnk');$s.TargetPath='%~dp0stellar-guesser.exe';$s.WorkingDirectory = '%~dp0';$s.Save()"
