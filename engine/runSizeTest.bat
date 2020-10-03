cd ..
rmdir /s /q sizeTest
mkdir sizeTest
cd sizeTest
for %%f in (.\..\engine\sizeTest\*) do (
pyinstaller %%~ff --distpath . --onefile -n %%~nf
rmdir /s /q build
del %%~nf.spec
)
pause