#include <windows.h>
#include <shellapi.h> // shellapi.h wants windows.h to be placed first
#include <string>
#include <vector>

static std::wstring GetExePath() {
  wchar_t buf[MAX_PATH];
  const DWORD n = GetModuleFileNameW(nullptr, buf, MAX_PATH);
  if (n == 0 || n == MAX_PATH) return std::wstring{};
  return std::wstring{buf, n};
}

static std::wstring MakeIniPathFromExe(const std::wstring &exePath) {
  if (exePath.empty()) return std::wstring{};
  const size_t posSlash = exePath.find_last_of(L"\\/");
  const size_t posDot = exePath.find_last_of(L'.');
  if (posDot == std::wstring::npos || (posSlash != std::wstring::npos && posDot < posSlash)) {
    return exePath + L".ini";
  }
  std::wstring out = exePath;
  out.replace(posDot, std::wstring::npos, L".ini");
  return out;
}

static bool FileExists(const std::wstring &path) {
  const DWORD attr = GetFileAttributesW(path.c_str());
  return attr != INVALID_FILE_ATTRIBUTES && !(attr & FILE_ATTRIBUTE_DIRECTORY);
}

static bool CreateDefaultIni(const std::wstring &iniPath) {
  constexpr char defaultContent[] = "[launcher]\r\nprogram=\r\nargument=\r\n";
  HANDLE hFile = CreateFileW(iniPath.c_str(), GENERIC_WRITE, 0, nullptr, CREATE_NEW, FILE_ATTRIBUTE_NORMAL, nullptr);
  if (hFile == INVALID_HANDLE_VALUE) {
    hFile = CreateFileW(iniPath.c_str(), GENERIC_WRITE, 0, nullptr, CREATE_ALWAYS, FILE_ATTRIBUTE_NORMAL, nullptr);
    if (hFile == INVALID_HANDLE_VALUE) return false;
  }

  DWORD written = 0;
  const BOOL ok = WriteFile(hFile, defaultContent, static_cast<DWORD>(sizeof(defaultContent)) - 1, &written, nullptr);
  CloseHandle(hFile);
  return ok != FALSE;
}

int WINAPI wWinMain(HINSTANCE, HINSTANCE, PWSTR, int) {
  int argc = 0;
  LPWSTR *argv = CommandLineToArgvW(GetCommandLineW(), &argc);
  std::wstring iniPath;

  if (argv) {
    for (int i = 1; i < argc; ++i) {
      if (lstrcmpiW(argv[i], L"-ini") == 0 && i + 1 < argc) {
        iniPath = argv[i + 1];
        break;
      }
    }
    LocalFree(argv);
  }

  if (iniPath.empty()) {
    std::wstring exePath = GetExePath();
    if (exePath.empty()) {
      MessageBoxW(nullptr, L"Failed to determine executable path.", L"ERROR", MB_OK | MB_ICONERROR);
      return 1;
    }
    iniPath = MakeIniPathFromExe(exePath);
  }

  if (!FileExists(iniPath)) {
    if (!CreateDefaultIni(iniPath)) {
      std::wstring msg = L"Failed to create INI file:\n" + iniPath;
      MessageBoxW(nullptr, msg.c_str(), L"ERROR", MB_OK | MB_ICONERROR);
      return 1;
    }
  }

  auto section = L"launcher";
  constexpr int BUF_SIZE = 4096;
  std::vector<wchar_t> bufProgram(BUF_SIZE);
  std::vector<wchar_t> bufArgument(BUF_SIZE);

  DWORD readProg = GetPrivateProfileStringW(section, L"program", L"", bufProgram.data(), BUF_SIZE, iniPath.c_str());
  DWORD readArg = GetPrivateProfileStringW(section, L"argument", L"", bufArgument.data(), BUF_SIZE, iniPath.c_str());

  std::wstring program(bufProgram.data(), readProg);
  std::wstring argument(bufArgument.data(), readArg);

  if (program.empty()) {
    MessageBoxW(nullptr, L"[launcher] program is empty in INI file", L"ERROR", MB_OK | MB_ICONERROR);
    return 1;
  }

  STARTUPINFOW si;
  PROCESS_INFORMATION pi;

  ZeroMemory(&si, sizeof(si));
  si.cb = sizeof(si);
  ZeroMemory(&pi, sizeof(pi));

  LPCWSTR lpApplicationName = nullptr;
  LPWSTR lpCommandLine = nullptr;
  std::wstring cmdLineStorage;

  if (argument.empty()) {
    lpApplicationName = program.c_str();
    lpCommandLine = nullptr;
  }
  else {
    cmdLineStorage = L"\"" + program + L"\" " + argument;
    lpApplicationName = nullptr;
    lpCommandLine = cmdLineStorage.empty() ? nullptr : &cmdLineStorage[0];
  }

  if (!CreateProcessW(lpApplicationName, lpCommandLine, nullptr, nullptr, FALSE, 0, nullptr, nullptr, &si, &pi)) {
    DWORD err = GetLastError();
    wchar_t errBuf[256];
    wsprintfW(errBuf, L"CreateProcessW failed (error %u).", err);
    std::wstring msg = errBuf;
    msg += L"\nProgram: " + program;
    if (!argument.empty()) msg += L"\nArgument: " + argument;
    MessageBoxW(nullptr, msg.c_str(), L"ERROR", MB_OK | MB_ICONERROR);
    return 1;
  }

  CloseHandle(pi.hProcess);
  CloseHandle(pi.hThread);

  return 0;
}
