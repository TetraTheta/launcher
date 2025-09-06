cmake_minimum_required (VERSION 3.10)
project ("launcher")

# MSVC Edit-and-Continue policy
if (POLICY CMP0141)
  cmake_policy(SET CMP0141 NEW)
  set(CMAKE_MSVC_DEBUG_INFORMATION_FORMAT "$<IF:$<AND:$<C_COMPILER_ID:MSVC>,$<CXX_COMPILER_ID:MSVC>>,$<$<CONFIG:Debug,RelWithDebInfo>:EditAndContinue>,$<$<CONFIG:Debug,RelWithDebInfo>:ProgramDatabase>>")
endif()

# Read launcher settings: prefer -D overrides, then environment, then defaults
if(DEFINED LAUNCHER_PROGRAM)
  set(LAUNCHER_PROGRAM "${LAUNCHER_PROGRAM}")
elseif(DEFINED ENV{LAUNCHER_PROGRAM})
  set(LAUNCHER_PROGRAM "$ENV{LAUNCHER_PROGRAM}")
else()
  set(LAUNCHER_PROGRAM "Firefox\\FirefoxPortable.exe")
endif()
if(DEFINED LAUNCHER_ARGUMENTS)
  set(LAUNCHER_ARGUMENTS "${LAUNCHER_ARGUMENTS}")
elseif(DEFINED ENV{LAUNCHER_ARGUMENTS})
  set(LAUNCHER_ARGUMENTS "$ENV{LAUNCHER_ARGUMENTS}")
else()
  set(LAUNCHER_ARGUMENTS "")
endif()

message(STATUS "LAUNCHER_PROGRAM = ${LAUNCHER_PROGRAM}")
message(STATUS "LAUNCHER_ARGUMENTS = ${LAUNCHER_ARGUMENTS}")

configure_file("${CMAKE_SOURCE_DIR}/config.h.in" "${CMAKE_CURRENT_BINARY_DIR}/config.h" @ONLY)

add_executable (launcher WIN32 "launcher.cpp")

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET launcher PROPERTY CXX_STANDARD 20)
endif()

target_include_directories(launcher PRIVATE "${CMAKE_CURRENT_BINARY_DIR}")

target_compile_definitions(launcher PRIVATE UNICODE _UNICODE)

target_link_libraries(launcher PRIVATE user32)

target_compile_options(launcher PRIVATE "$<$<AND:$<CONFIG:Release>,$<C_COMPILER_ID:MSVC>>:/O2>" "$<$<AND:$<CONFIG:Release>,$<NOT:$<C_COMPILER_ID:MSVC>>>:-O3>")
