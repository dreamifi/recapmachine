set botname=recapmachine
rmdir /s /q userVersion
mkdir userVersion
cd userVersion
pyinstaller ../engine/botcode/run.py --distpath . --onefile -n %botname% --hidden-import=skimage.feature._orb_descriptor_positions
rmdir /s /q build
del %botname%.spec
mkdir Tesseract-OCR
robocopy ../engine/Tesseract-OCR ./Tesseract-OCR /e
pause