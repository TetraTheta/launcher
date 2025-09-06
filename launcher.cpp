#include <string>
#include <windows.h>
#include "config.h"

int WINAPI wWinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, PWSTR lpCmdLine, int nShowCmd) {
  STARTUPINFO si;
  PROCESS_INFORMATION pi;

  ZeroMemory(&si, sizeof(si));
  si.cb = sizeof(si);
  ZeroMemory(&pi, sizeof(pi));

  const std::wstring program = LAUNCHER_PROGRAM;
  const std::wstring args = LAUNCHER_ARGUMENTS;

  LPCWSTR lpApplicationName = nullptr;
  LPWSTR lpCommandLine = nullptr;
  std::wstring cmdLineStorage;

  if (program.empty()) {
    MessageBoxW(nullptr, L"LAUNCHER_PROGRAM is empty", L"ERROR", MB_OK | MB_ICONERROR);
    return 1;
  }

  if (args.empty()) {
    lpApplicationName = program.c_str();
    lpCommandLine = nullptr;
  } else {
    cmdLineStorage = L"\"" + program + L"\" " + args;
    lpApplicationName = nullptr;
    lpCommandLine = cmdLineStorage.empty() ? nullptr : &cmdLineStorage[0];
  }

  if (!CreateProcessW(lpApplicationName, lpCommandLine, nullptr, nullptr, FALSE, 0, nullptr, nullptr, &si, &pi)) {
    MessageBoxW(nullptr, L"CreateProcessW Failed", L"ERROR", MB_OK | MB_ICONERROR);
    return 1;
  }

  CloseHandle(pi.hProcess);
  CloseHandle(pi.hThread);

  return 0;
}
