from datetime import datetime
import os
os.environ["PYGAME_HIDE_SUPPORT_PROMPT"] = "hide"

initalTime = datetime.now()

PYGAME_HIDE_SUPPORT_PROMPT = 1

import pygame
import importlib.util
from os import path
pygame.init()

from fursona.colors import colors
from fursona.entity import *
from fursona.input import *
from fursona.physics import *
from fursona.text import *
from fursona.window import *

VERSION = "1.0.0" # MAJOR.MINOR.PATCH
CODENAME = "Velvet"

importTime = datetime.now() - initalTime

print(f"""
    ________  ______  _____ ____  _   _____ 
   / ____/ / / / __ \/ ___// __ \/ | / /   |
  / /_  / / / / /_/ /\__ \/ / / /  |/ / /| |
 / __/ / /_/ / _, _/___/ / /_/ / /|  / ___ |
/_/    \____/_/ |_|/____/\____/_/ |_/_/  |_|
-------------------ENGINE-------------------
{VERSION} {CODENAME} (Mechinsam)\nImported in {importTime}""")
