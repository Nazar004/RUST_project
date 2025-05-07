@echo off
REM
SET PGPASSWORD=2004

REM 
psql -U postgres -d postgres -f create_db_and_user.sql

IF %ERRORLEVEL% NEQ 0 (
    echo 🔴
) ELSE (
    echo ✅
)

pause