setlocal EnableDelayedExpansion
cd engine
for /d %%d in (.\rust\*) do (
  set latestBuild=
  for %%f in (.\botcode\%%~nd.pyd) do (
    set latestBuild=%%~tf
  )
  if defined latestBuild (
	set shouldBuild=
	for /r %%f in (.\rust\%%~nd\src\*) do (
      if %%~tf gtr !latestBuild! (
        set shouldBuild=yes
      )
    )
  ) else (
	set shouldBuild=yes
  )
  if defined shouldBuild (
    cd rust\%%~nd && cargo build --release && cd.. && cd..
    if %errorlevel% equ 0 (
	  robocopy .\rust\%%~nd\target\i686-pc-windows-msvc\release\ .\botcode\ %%~nd.dll
      cd botcode
	  del %%~nd.pyd
	  rename %%~nd.dll %%~nd.pyd
	  cd..
    ) else (
	  pause
	  exit
	)
  )
)
cd botcode && run.py && exit
pause