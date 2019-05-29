@echo off

for %%F in (stdin\*.txt) do (
  echo ----------------------------------------
  echo [ Test No. %%~nF ]
  echo.
  echo input:
  type %%F
  echo.
  echo output:
  type %%F | target\debug\rust-practice.exe
  echo.
)
