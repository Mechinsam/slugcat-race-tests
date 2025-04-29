import pygame
from fursona.colors import colors
from fursona.physics import Vector2
import json

#font = pygame.font.Font("DATA/fnt/BookAntiqua/BKANT.TTF", 24)

class Text:
	def __init__(self, text, fontname="BookAntiqua", folder="fursona/fnt", position=Vector2(0,0), type="Regular", size=24, color=colors["white"]) -> None:
		self.text = str(text)
		self.type = type
		self.fontname = fontname
		self.position = position
		self.folder = folder
		
		self.settings = json.loads(open(f"{self.folder}/{self.fontname}/{self.fontname}.json", "r").read())

		if self.type != "Regular" and self.type != "Bold" and self.type != "Italic" and self.type != "Bold-Italic":
			pygame.quit()
			raise ValueError(f"Text.type should be either: Regular, Bold, Italic, or Bold-Italic. Not '{self.type}'")
		self.fontfile = self.settings[self.type]
		self.font = pygame.font.Font(f"{self.folder}/{self.fontname}/{self.fontfile}", size)
		self.render = self.font.render(self.text, True, color)
	def Update(self, display) -> None:
		display.viewport.blit(self.render, (self.position.x, self.position.y))

class SimpleText:
	def __init__(self, text, fontfile="BookAntiqua", position=Vector2(0,0), type="Regular", size=24, color=colors["white"]) -> None:
		self.text = text
		self.type = type
		self.fontfile = fontfile
		self.position = position
		self.font = pygame.font.Font(self.fontfile, size)
		self.render = self.font.render(self.text, True, color)
	def Update(self, display) -> None:
		display.viewport.blit(self.render, (self.position.x, self.position.y))
