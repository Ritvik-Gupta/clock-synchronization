@echo off

cargo build --release --no-default-features --features master,persistence
copy .\target\release\native_demo.exe .\binary\master.exe

cargo build --release --no-default-features --features slave,dhat-profile
copy .\target\release\native_demo.exe .\binary\slave.exe
