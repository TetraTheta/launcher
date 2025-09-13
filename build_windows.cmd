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

cls

set "_SD=%~dp0"
set "SRC_DIR=%_SD:~0,-1%"
set "BUILD_DIR=%SRC_DIR%\build"

echo SRC_DIR is %SRC_DIR%
echo BUILD_DIR is %BUILD_DIR%

if exist "%BUILD_DIR%" rmdir /s /q "%BUILD_DIR%"

mkdir "%BUILD_DIR%"

cmake -S "%SRC_DIR%" -B "%BUILD_DIR%"
if errorlevel 1 (
  pause
  exit /b %errorlevel%
)
cmake --build "%BUILD_DIR%" --config Release
if errorlevel 1 (
  pause
  exit /b %errorlevel%
)
echo 'launcher.exe' is at 'build/Release'
pause
