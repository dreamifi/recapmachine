echo off
setlocal EnableDelayedExpansion
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
    cd rust\%%~nd 
	echo on
	cargo build --release || pause && exit
	echo off
	cd..
	cd..
    robocopy .\rust\%%~nd\target\i686-pc-windows-msvc\release\ .\botcode\ %%~nd.dll
    cd botcode
	del %%~nd.pyd
	rename %%~nd.dll %%~nd.pyd
	cd..
  )
)
echo on