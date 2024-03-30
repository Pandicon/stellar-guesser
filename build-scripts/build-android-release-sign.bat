@echo off
for /f "tokens=2 delims==" %%a in ('wmic OS Get localdatetime /value') do set "dt=%%a"
set "Year=%dt:~0,4%" & set "Mon=%dt:~4,2%" & set "Day=%dt:~6,2%"
set "Hour=%dt:~8,2%" & set "Min=%dt:~10,2%" & set "Sec=%dt:~12,2%"

set "fullstamp=%Year%-%Mon%-%Day%--%Hour%-%Min%-%Sec%"


set "out_folder=.\builds\android\release\%fullstamp%"
set "original_dir=%CD%"
mkdir %out_folder%

cargo android apk build --release
copy .\gen\android\app\build\outputs\apk\universal\release\app-universal-release-unsigned.apk %out_folder%
cd %out_folder%
ren app-universal-release-unsigned.apk app-universal-release-unsigned-unaligned.apk
zipalign -p 4 app-universal-release-unsigned-unaligned.apk app-universal-release-unsigned-aligned.apk
cd %original_dir%
start /wait cmd /c apksigner sign --ks stellar-guesser.keystore %out_folder%\app-universal-release-unsigned-aligned.apk
cd %out_folder%
ren app-universal-release-unsigned-aligned.apk app-universal-release-signed-aligned.apk
cd %original_dir%

copy %out_folder%\app-universal-release-signed-aligned.apk .\
ren app-universal-release-signed-aligned.apk stellar-guesser.apk

del /Q .\builds\android\release\latest
xcopy /E /I "%out_folder%" ".\builds\android\release\latest\"

cd .\builds\android\release\latest
ren app-universal-release-unsigned-unaligned.apk app-universal-release-unsigned-unaligned---%fullstamp%.apk
ren app-universal-release-signed-aligned.apk app-universal-release-signed-aligned---%fullstamp%.apk
ren app-universal-release-unsigned-aligned.apk.idsig app-universal-release-unsigned-aligned---%fullstamp%.apk.idsig
cd %original_dir%