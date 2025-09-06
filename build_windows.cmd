@echo off
call "%ProgramFiles%\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat"

set "LAUNCHER_PROGRAM=chrome\chrome.exe"
set "LAUNCHER_ARGUMENTS=--user-data-dir=..\profile --no-default-browser-check --disable-logging --disable-breakpad --no-report-upload --disable-features=PrintCompositorLPAC,RendererCodeIntegrity --disable-machine-id --disable-encryption-win"
cmake -S . -B build
cmake --build build --config Release
echo 'launcher.exe' is at 'build/Release'
pause
