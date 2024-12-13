@echo off

VisualStudioSetup.exe --quiet --norestart                     ^
    --productId Microsoft.VisualStudio.Product.Community      ^
    --installPath C:\toolsets\vs2022community                 ^
    --add Microsoft.VisualStudio.Component.VC.Tools.x86.x64   ^
    --add Microsoft.VisualStudio.Component.VC.Tools.ARM64

:wait_until_setup_finished
set "process_name=setup.exe"
tasklist /FI "IMAGENAME eq %process_name%" | find /I "%process_name%"
if %ERRORLEVEL%==0 (
    echo %process_name% is running. Waiting...
    timeout /t 5 > NUL
    goto wait_until_setup_finished
) else (
    echo %process_name% is not running. Exiting...
    exit /B
)
