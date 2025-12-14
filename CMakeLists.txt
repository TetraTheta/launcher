cmake_minimum_required(VERSION 3.10)

# Default to 'Release'
if (NOT CMAKE_BUILD_TYPE)
  set(CMAKE_BUILD_TYPE Release CACHE STRING "" FORCE)
endif ()

project(launcher)

# MSVC-specific configuration
if (MSVC)
  # Set toolset version manually
  set(CMAKE_GENERATOR_TOOLSET "v145" CACHE STRING "Platform Toolset to use for MSVC build." FORCE)

  # MSVC Edit-and-Continue policy
  if (POLICY CMP0141)
    cmake_policy(SET CMP0141 NEW)
    set(CMAKE_MSVC_DEBUG_INFORMATION_FORMAT "$<IF:$<AND:$<C_COMPILER_ID:MSVC>,$<CXX_COMPILER_ID:MSVC>>,$<$<CONFIG:Debug,RelWithDebInfo>:EditAndContinue>,$<$<CONFIG:Debug,RelWithDebInfo>:ProgramDatabase>>")
  endif ()
endif ()

set(SRC launcher.cpp)
set(ICON_FILE "${CMAKE_SOURCE_DIR}/resource/icon.ico")
set(MANIFEST_FILE "${CMAKE_SOURCE_DIR}/resource/app.manifest")

if (EXISTS "${ICON_FILE}" OR EXISTS "${MANIFEST_FILE}")
  set(RESOURCE_RC "${CMAKE_CURRENT_BINARY_DIR}/launcher.rc")
  file(WRITE "${RESOURCE_RC}" "/* auto-generated resource file */\n")

  if (EXISTS "${ICON_FILE}")
    file(APPEND "${RESOURCE_RC}" "1 ICON \"${ICON_FILE}\"\n")
    message(STATUS "Embedding icon: ${ICON_FILE}")
  endif ()

  if(EXISTS "${MANIFEST_FILE}")
    file(APPEND "${RESOURCE_RC}" "1 RT_MANIFEST \"${MANIFEST_FILE}\"\n")
    message(STATUS "Embedding manifest: ${MANIFEST_FILE}")
  endif()

  list(APPEND SRC "${RESOURCE_RC}")
else ()
  message(STATUS "No 'icon.ico' or 'app.manifest' found in source dir. building without extra resources.")
endif ()

add_executable(launcher WIN32 ${SRC})

if (CMAKE_VERSION VERSION_GREATER 3.12)
  set_property(TARGET launcher PROPERTY CXX_STANDARD 20)
endif ()

if (MSVC)
  target_compile_definitions(launcher PRIVATE UNICODE _UNICODE)
else ()
  target_compile_definitions(launcher PRIVATE WIN32)
endif ()

target_compile_options(launcher PRIVATE
        # MSVC (cl.exe) options
        "$<$<C_COMPILER_ID:MSVC>:/W4>"
        "$<$<C_COMPILER_ID:MSVC>:/utf-8>"
        "$<$<AND:$<CONFIG:Release>,$<C_COMPILER_ID:MSVC>>:/O2>"
        # MinGW (gcc.exe) options
        "$<$<C_COMPILER_ID:GNU>:-Wall>"
        "$<$<AND:$<CONFIG:Release>,$<C_COMPILER_ID:GNU>>:-O3>"
)

target_link_libraries(launcher PRIVATE user32)
