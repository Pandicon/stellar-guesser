cargo build
copy .\target\x86_64-pc-windows-msvc\debug\stellar-guesser-desktop.exe .\
del stellar-guesser-debug.exe
ren stellar-guesser-desktop.exe stellar-guesser-debug.exe
.\stellar-guesser-debug.exe