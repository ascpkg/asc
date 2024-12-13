@echo off

winsdksetup.exe /quiet /norestart                                  ^
    /installpath "C:\toolsets\WindowsKits1903"                     ^
    /features "OptionId.DesktopCPPx64" "OptionId.DesktopCPParm64"

:wait_until_setup_finished
set "process_name=winsdksetup.exe"
tasklist /FI "IMAGENAME eq %process_name%" | find /I "%process_name%"
if %ERRORLEVEL%==0 (
    echo %process_name% is running. Waiting...
    timeout /t 5 > NUL
    goto wait_until_setup_finished
) else (
    echo %process_name% is not running. Exiting...
    exit /B
)
