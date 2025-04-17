@echo off
REM Установим пароль для psql
SET PGPASSWORD=2004

REM Выполним SQL-скрипт от имени postgres
psql -U postgres -d postgres -f create_db_and_user.sql

IF %ERRORLEVEL% NEQ 0 (
    echo 🔴 Произошла ошибка при выполнении SQL-скрипта.
) ELSE (
    echo ✅ Скрипт успешно выполнен.
)

pause