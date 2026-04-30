#![windows_subsystem = "windows"]

use configparser::ini::Ini;
use std::env;
use std::ffi::OsStr;
use std::fs;
use std::os::windows::ffi::OsStrExt;
use std::path::{Path, PathBuf};
use std::ptr::{null, null_mut};
use windows_sys::Win32::Foundation::{CloseHandle, GetLastError, HWND};
use windows_sys::Win32::System::Environment::ExpandEnvironmentStringsW;
use windows_sys::Win32::System::Threading::{
  CreateProcessW, CREATE_UNICODE_ENVIRONMENT, PROCESS_INFORMATION, STARTUPINFOW,
};
use windows_sys::Win32::UI::WindowsAndMessaging::{MessageBoxW, MB_ICONERROR, MB_OK};

const DEFAULT_INI_CONTENT: &str =
    "[launcher]\r\nprogram=\r\nargument=\r\n[environment]\r\nENV_KEY_1=ENV_KEY_1_VALUE\r\n";

/// Rust 문자열(UTF-8)을 Win32 API 호출용 null-terminated UTF-16으로 변환한다.
fn to_wide(s: &str) -> Vec<u16> {
    OsStr::new(s).encode_wide().chain(Some(0)).collect()
}

/// 오류 메시지를 Windows MessageBox로 표시한다.
fn show_error(message: &str) {
    let title = to_wide("ERROR");
    let message_w = to_wide(message);
    unsafe {
        MessageBoxW(
            HWND::default(),
            message_w.as_ptr(),
            title.as_ptr(),
            MB_OK | MB_ICONERROR,
        );
    }
}

/// 실행 파일 경로에서 확장자만 `.ini`로 바꿔 기본 INI 경로를 만든다.
fn make_ini_path_from_exe(exe_path: &Path) -> PathBuf {
    exe_path.with_extension("ini")
}

/// 기본 템플릿 내용으로 INI 파일을 생성한다.
fn create_default_ini(path: &Path) -> Result<(), String> {
    fs::write(path, DEFAULT_INI_CONTENT).map_err(|e| format!("Failed to create INI file:\n{}\n{}", path.display(), e))
}

/// Windows 규칙(`%VAR%`)으로 환경 변수를 확장한다.
fn expand_env_windows(value: &str) -> Result<String, String> {
    let source = to_wide(value);
    let needed = unsafe { ExpandEnvironmentStringsW(source.as_ptr(), null_mut(), 0) };
    if needed == 0 {
        return Err(format!(
            "ExpandEnvironmentStringsW failed for value: {} (error {})",
            value,
            unsafe { GetLastError() }
        ));
    }

    let mut buffer = vec![0u16; needed as usize];
    let written = unsafe { ExpandEnvironmentStringsW(source.as_ptr(), buffer.as_mut_ptr(), needed) };
    if written == 0 {
        return Err(format!(
            "ExpandEnvironmentStringsW failed for value: {} (error {})",
            value,
            unsafe { GetLastError() }
        ));
    }

    if written > 0 {
        buffer.truncate((written - 1) as usize);
    }

    Ok(String::from_utf16_lossy(&buffer))
}

/// 명령줄에서 `-ini <path>` 인자를 찾아 INI 경로를 반환한다.
fn parse_ini_path_argument() -> Option<PathBuf> {
    let mut args = env::args_os().skip(1);
    while let Some(arg) = args.next() {
        if arg.to_string_lossy().eq_ignore_ascii_case("-ini") {
            if let Some(path) = args.next() {
                return Some(PathBuf::from(path));
            }
            break;
        }
    }
    None
}

/// 현재 프로세스 환경 + INI override를 합쳐 CreateProcessW용 Unicode 환경 블록을 만든다.
///
/// 반환 형식은 `key=value\0... \0\0` 구조이다.
fn build_unicode_environment_block(overrides: &[(String, String)]) -> Vec<u16> {
    let mut merged = std::collections::BTreeMap::<String, String>::new();

    for (k, v) in env::vars() {
        merged.insert(k, v);
    }

    for (k, v) in overrides {
        merged.insert(k.clone(), v.clone());
    }

    let mut block: Vec<u16> = Vec::new();
    for (k, v) in merged {
        let kv = format!("{k}={v}");
        block.extend(to_wide(&kv).into_iter().take_while(|c| *c != 0));
        block.push(0);
    }

    block.push(0);
    block
}

/// 런처의 메인 진입점.
///
/// 순서:
/// 1) INI 경로 결정 및 기본 INI 생성
/// 2) launcher/environment 섹션 로드
/// 3) `%VAR%` 확장
/// 4) 환경 블록 구성 후 CreateProcessW로 자식 프로세스 실행
fn main() {
    // `-ini`가 있으면 해당 경로를 사용하고, 없으면 exe 이름 기반 `.ini`를 사용한다.
    let ini_path = parse_ini_path_argument().unwrap_or_else(|| match env::current_exe() {
        Ok(exe) => make_ini_path_from_exe(&exe),
        Err(_) => {
            show_error("Failed to determine executable path.");
            std::process::exit(1);
        }
    });

    if !ini_path.exists() {
        if let Err(err) = create_default_ini(&ini_path) {
            show_error(&err);
            std::process::exit(1);
        }
    }

    let mut conf = Ini::new();
    if let Err(e) = conf.load(ini_path.to_string_lossy().as_ref()) {
        show_error(&format!("Failed to read INI file:\n{}\n{}", ini_path.display(), e));
        std::process::exit(1);
    }

    let raw_program = conf.get("launcher", "program").unwrap_or_default();
    let raw_argument = conf.get("launcher", "argument").unwrap_or_default();
    let raw_program = raw_program.trim().to_string();
    let raw_argument = raw_argument.trim().to_string();

    // 실행 대상은 필수 항목이다.
    if raw_program.is_empty() {
        show_error("[launcher] program is empty in INI file");
        std::process::exit(1);
    }

    let program = match expand_env_windows(&raw_program) {
        Ok(v) => v,
        Err(e) => {
            show_error(&e);
            std::process::exit(1);
        }
    };
    let argument = match expand_env_windows(&raw_argument) {
        Ok(v) => v,
        Err(e) => {
            show_error(&e);
            std::process::exit(1);
        }
    };

    let mut env_overrides: Vec<(String, String)> = Vec::new();
    if let Some(section) = conf.get_map_ref().get("environment") {
        // [environment]의 각 값을 확장한 뒤 override 목록으로 누적한다.
        for (key, value) in section {
            if key.trim().is_empty() {
                continue;
            }
            let raw_value = value.clone().unwrap_or_default();
            let expanded = match expand_env_windows(&raw_value) {
                Ok(v) => v,
                Err(e) => {
                    show_error(&e);
                    std::process::exit(1);
                }
            };
            env_overrides.push((key.to_string(), expanded));
        }
    }

    let env_block = build_unicode_environment_block(&env_overrides);

    // Win32 프로세스 생성 구조체 초기화
    let mut si: STARTUPINFOW = unsafe { std::mem::zeroed() };
    si.cb = size_of::<STARTUPINFOW>() as u32;
    let mut pi: PROCESS_INFORMATION = unsafe { std::mem::zeroed() };

    let cmd_line_storage = if argument.is_empty() {
        String::new()
    } else {
        format!("\"{program}\" {argument}")
    };

    let program_w = to_wide(&program);
    let mut cmd_line_w = to_wide(&cmd_line_storage);

    // argument가 없을 때는 application name만 전달하고,
    // argument가 있으면 전체 커맨드라인 문자열을 전달한다.
    let application_name = if argument.is_empty() {
        program_w.as_ptr()
    } else {
        null()
    };
    let command_line = if argument.is_empty() {
        null_mut()
    } else {
        cmd_line_w.as_mut_ptr()
    };

    let ok = unsafe {
        CreateProcessW(
            application_name,
            command_line,
            null(),
            null(),
            0,
            CREATE_UNICODE_ENVIRONMENT,
            env_block.as_ptr() as *const _,
            null(),
            &si,
            &mut pi,
        )
    };

    if ok == 0 {
        let err = unsafe { GetLastError() };
        let mut msg = format!("CreateProcessW failed (error {err}).\nProgram: {program}");
        if !argument.is_empty() {
            msg.push_str(&format!("\nArgument: {argument}"));
        }
        show_error(&msg);
        std::process::exit(1);
    }

    unsafe {
        CloseHandle(pi.hProcess);
        CloseHandle(pi.hThread);
    }
}
