set botname=recapmachine
rmdir /s /q userVersion
mkdir userVersion
cd userVersion
pyinstaller ../engine/botcode/run.py --distpath . --onefile -n %botname% --hidden-import=skimage.feature._orb_descriptor_positions --add-binary="../engine/Tesseract-OCR/tesseract.exe;Tesseract-OCR"  --add-data="../engine/Tesseract-OCR/tessdata;Tesseract-OCR/tessdata"
rmdir /s /q build
del %botname%.spec
pause