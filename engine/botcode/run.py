#local imports
import robotocr
import start

#standard library imports
from os import path

robotocr.init(\
tesseractPath= path.join('..', 'Tesseract-OCR', 'tesseract'), \
logFolderPath= path.join('..', '..', 'latestRun'))

start.bloodbowl()