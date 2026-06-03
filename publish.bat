@echo off
REM Publish Nexus to GitHub
REM Usage: publish.bat <your-github-username>

if "%1"=="" (
    echo Usage: publish.bat ^<your-github-username^>
    echo.
    echo Example: publish.bat johndoe
    exit /b 1
)

set USER=%1
set REPO=nexus

echo Creating GitHub repository...
gh repo create %USER%/%REPO% --public --source=. --remote=origin --push
if %ERRORLEVEL% neq 0 (
    echo.
    echo Alternative: Create the repo manually at https://github.com/new
    echo Then run:
    echo   git remote add origin https://github.com/%USER%/%REPO%.git
    echo   git push -u origin main
)
