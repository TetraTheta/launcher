@echo off
chcp 65001
setlocal

set "USE_MSVC=0"
set "MINGW_ROOT=E:\MinGW64"

@REM Check DevCmd
if exist "%ProgramFiles(x86)%\Microsoft Visual Studio\18\Community\Common7\Tools\VsDevCmd.bat" (
  call "%ProgramFiles(x86)%\Microsoft Visual Studio\18\Community\Common7\Tools\VsDevCmd.bat"
  set "USE_MSVC=1"
) else if exist "%ProgramFiles(x86)%\Microsoft Visual Studio\18\BuildTools\Common7\Tools\VsDevCmd.bat" (
  call "%ProgramFiles(x86)%\Microsoft Visual Studio\18\BuildTools\Common7\Tools\VsDevCmd.bat"
  set "USE_MSVC=1"
)

if "%USE_MSVC%"=="0" (
  if not exist "%MINGW_ROOT%\bin\gcc.exe" (
    echo Neither MSVC nor MinGW64 found.
    exit /b 1
  )
  set "PATH=%MINGW_ROOT%\bin;%PATH%"
)

cls

set "_SD=%~dp0"
set "SRC_DIR=%_SD:~0,-1%"
set "BUILD_DIR=%SRC_DIR%\build"

echo SRC_DIR is %SRC_DIR%
echo BUILD_DIR is %BUILD_DIR%

if exist "%BUILD_DIR%" rmdir /s /q "%BUILD_DIR%"
mkdir "%BUILD_DIR%"

if "%USE_MSVC%"=="1" (
  cmake -S "%SRC_DIR%" -B "%BUILD_DIR%" -G "Visual Studio 18 2026"
) else (
  cmake -S "%SRC_DIR%" -B "%BUILD_DIR%" -G "MinGW Makefiles"
)
if errorlevel 1 (
  pause
  exit /b %errorlevel%
)

if "%USE_MSVC%"=="1" (
  cmake --build "%BUILD_DIR%" --config Release
) else (
  cmake --build "%BUILD_DIR%"
)
if errorlevel 1 (
  pause
  exit /b %errorlevel%
)

echo 'launcher.exe' is at 'build/Release'
pause
