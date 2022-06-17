@echo off
cd wkpkg
cargo build --release
cd ..
copy .\wkpkg\target\release\wkpkg.exe wkpkg.exe /y
if %errorlevel% NEQ 0 (pause)