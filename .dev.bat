@echo off

:: ===== EDIT THIS LINE =====
:: List your modules, separated by spaces.
set MODULES=core states terminal traits ui
:: ==========================

::Make sure main.rs exists
if not exist "src\main.rs" type nul > "src\main.rs"

::Run the setup steps once for each module in the list
for %%M in (%MODULES%) do call :setup %%M

echo Done!
goto :eof


:setup
set NAME=%1

:: Step 1: make the folder (only if missing)
if not exist "src\%NAME%" mkdir "src\%NAME%"

:: Step 2: make an empty mod.rs (only if missing)
if not exist "src\%NAME%\mod.rs" type nul > "src\%NAME%\mod.rs"

:: Step 3: add "mod name;" to main.rs (only if missing)
findstr /b /c:"mod %NAME%;" "src\main.rs" >nul
if errorlevel 1 (
    echo mod %NAME%;> "src\main.tmp"
    type "src\main.rs" >> "src\main.tmp"
    move /y "src\main.tmp" "src\main.rs" >nul
)
goto :eof
