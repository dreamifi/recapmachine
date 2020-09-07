#local imports
import robotocr
import start

#standard library imports
from os import path
import sys

if getattr(sys, 'frozen', False) and hasattr(sys, '_MEIPASS'):
    # running in a PyInstaller bundle (User Version)
	robotocr.init(\
	tesseractPath= path.join('Tesseract-OCR', 'tesseract'), \
	logFolderPath= 'latestRun')
else:
    #running in a normal Python process (Dev Version)
	robotocr.init(\
	tesseractPath= path.join('..', 'Tesseract-OCR', 'tesseract'), \
	logFolderPath= path.join('..', '..', 'latestRun'))

start.bloodbowl()