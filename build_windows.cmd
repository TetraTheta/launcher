@echo off

@REM Check DevCmd
if exist "%ProgramFiles%\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat" (
  call "%ProgramFiles%\Microsoft Visual Studio\2022\Community\Common7\Tools\VsDevCmd.bat"
) else (
  if exist "%ProgramFiles%\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat" (
    call "%ProgramFiles%\Microsoft Visual Studio\2022\BuildTools\Common7\Tools\VsDevCmd.bat"
  ) else (
    echo Visual Studio or Build Tools not found. Please install one of them.
    exit /b 1
  )
)

cmake -S . -B build
cmake --build build --config Release
echo 'launcher.exe' is at 'build/Release'
pause
