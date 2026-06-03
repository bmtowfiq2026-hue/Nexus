@echo off
call "C:\Program Files\Microsoft Visual Studio\2022\BuildTools\VC\Auxiliary\Build\vcvars64.bat" > nul 2>&1
set PATH=%USERPROFILE%\.cargo\bin;%USERPROFILE%\Go\go\bin;%PATH%
cd /d "%~dp0"
cargo build %*
