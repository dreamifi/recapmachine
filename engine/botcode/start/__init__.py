
#local imports
import robotocr
from robotocr.cut import *
from robotocr.processing import *

#third party imports
import pyautogui
import pydirectinput

#standard library imports
import time

def bloodbowl():
	pyautogui.press('winleft')
	pyautogui.write('Steam')
	pyautogui.press('enter')
	
	robotocr.clickSentence('LIBRARY', \
	processing=(RgbToGray(), Floor(0.4), AutoLevel())) 
	
	pyautogui.hotkey('alt', 'space', interval=0.1)
	pyautogui.press('x')
	pyautogui.hotkey('ctrl', 'f', interval=0.1)
	pyautogui.write('Blood Bowl 2')
	
	robotocr.waitSentence('SEARCH RESULTS', \
	cut=(top, left, left),\
	processing=(RgbToGray(), Floor(0.3), LocalMaxFloor(), AutoLevel(), Rescale(2)))
	
	robotocr.doubleClickSentence('Blood Bowl 2', \
	cut=(topHalf, left, left),\
	processing=(RgbToGray(), Floor(0.3), LocalMaxFloor(), AutoLevel(), Rescale(2)),\
	ratio=1, skip=1)
	
	robotocr.clickSentence('Play Blood Bowl 2', \
	cut=(wideMiddle, wideMiddleHalf, tallMiddle),\
	processing=(RgbToGray(), AutoLevel(), Rescale(2)))
	
	pyautogui.press('enter')
	
	time.sleep(25)
	
	robotocr.waitSentence('Press Enter', \
	cut=(wideMiddle, tallMiddle, bottom),\
	processing=(RgbToGray(), MaxFloor(), AutoLevel(100), WidenTo16by10()))
	
	pydirectinput.press('enter')