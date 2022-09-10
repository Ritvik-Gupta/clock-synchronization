@echo off

echo:
if "%1" == "master" goto correct_system_arch
if "%1" == "slave"  goto correct_system_arch

echo Invalid System Architecture "%1"
goto program_end

:correct_system_arch

set SYSTEM_ARCH=%1
shift

echo Building for System Architecture "%SYSTEM_ARCH%"
echo:

set BUILD_SCRIPT=cargo build --release --no-default-features --features %SYSTEM_ARCH%

:do_while
    if (%1) == () goto end_while
    
    set BUILD_SCRIPT=%BUILD_SCRIPT%,%1
    shift
    
    goto do_while
:end_while

echo Running command:
echo:
echo "%BUILD_SCRIPT%"
echo:

%BUILD_SCRIPT%
copy .\target\release\native_demo.exe .\binary\%SYSTEM_ARCH%.exe

:program_end
