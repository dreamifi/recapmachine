#local imports
import robotocr

#standard library imports
from os import path

robotocr.init(\
tesseractPath= path.join('..', 'Tesseract-OCR', 'tesseract'), \
logFolderPath= path.join('..', '..', 'latestRun'))

# Insert robot code here