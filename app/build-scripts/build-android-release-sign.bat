@echo off
setlocal

rem Get a safe timestamp using PowerShell (yyyy-MM-dd--HH-mm-ss)
for /f "usebackq delims=" %%A in (`powershell -NoProfile -Command "Get-Date -Format 'yyyy-MM-dd--HH-mm-ss'"`) do set "fullstamp=%%A"

rem Set up the paths
set "original_dir=%CD%"
set "out_folder=%original_dir%\builds\android\release\%fullstamp%"

rem Create the output folder (unless it exists for some reason)
if not exist "%out_folder%" (
    mkdir "%out_folder%"
)

rem Build the release APK
cargo android apk build --release

rem Copy the built APK to the builds folder
copy "%original_dir%\gen\android\app\build\outputs\apk\universal\release\app-universal-release-unsigned.apk" "%out_folder%"

rem Align and sign
pushd "%out_folder%"
if exist "app-universal-release-unsigned.apk" (
    ren "app-universal-release-unsigned.apk" "app-universal-release-unsigned-unaligned.apk"
    zipalign -p 4 "app-universal-release-unsigned-unaligned.apk" "app-universal-release-unsigned-aligned.apk"
) else (
    echo ERROR: Unsigned apk not found in %out_folder%, can not align it
    popd
    goto :eof
)
popd

rem Sign the APK
call apksigner sign --ks "%original_dir%\stellar-guesser.keystore" "%out_folder%\app-universal-release-unsigned-aligned.apk"

pushd "%out_folder%"
if exist "app-universal-release-unsigned-aligned.apk" (
    ren "app-universal-release-unsigned-aligned.apk" "app-universal-release-signed-aligned.apk"
) else (
    echo ERROR: Aligned and signed apk not found
    popd
    goto :eof
)
popd

rem Copy signed apk back to project root and rename
copy "%out_folder%\app-universal-release-signed-aligned.apk" "%original_dir%\"
if exist "%original_dir%\stellar-guesser.apk" del "%original_dir%\stellar-guesser.apk"
if exist "%original_dir%\app-universal-release-signed-aligned.apk" ren "%original_dir%\app-universal-release-signed-aligned.apk" "stellar-guesser.apk"

rem Remove the old 'latest' folder
if exist "%original_dir%\builds\android\release\latest" (
    rmdir /S /Q "%original_dir%\builds\android\release\latest"
)
xcopy /E /I "%out_folder%" "%original_dir%\builds\android\release\latest\"

rem Add timestamp to copies inside latest
pushd "%original_dir%\builds\android\release\latest"
if exist "app-universal-release-unsigned-unaligned.apk" ren "app-universal-release-unsigned-unaligned.apk" "app-universal-release-unsigned-unaligned---%fullstamp%.apk"
if exist "app-universal-release-signed-aligned.apk" ren "app-universal-release-signed-aligned.apk" "app-universal-release-signed-aligned---%fullstamp%.apk"
if exist "app-universal-release-unsigned-aligned.apk.idsig" ren "app-universal-release-unsigned-aligned.apk.idsig" "app-universal-release-unsigned-aligned---%fullstamp%.apk.idsig"
popd

endlocal