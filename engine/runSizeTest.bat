cd ..
if not exist sizeTest (
mkdir sizeTest
)
cd sizeTest
for %%f in (.\..\engine\sizeTest\*) do (
if not exist %%~nf.exe (
pyinstaller %%~ff --distpath . --onefile -n %%~nf
rmdir /s /q build
del %%~nf.spec
)
)
pause