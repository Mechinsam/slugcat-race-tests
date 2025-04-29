import pygame
from fursona.colors import colors
from fursona.physics import Vector2
import sys

def Update():
	pass

class Window:
	def __init__(self, title : str, width=500, height=500, fullscreen=False, displayPlatform=False, bgColor=colors["black"], fps=60) -> None:
		self.title = str(title)
		self.width = int(width)
		self.height = int(height)
		self.fpsClock = pygame.time.Clock()
		self.fullscreen = bool(fullscreen)
		self.bgColor = bgColor
		self.fps = int(fps)
		self.deltaTime = self.fpsClock.tick(self.fps) / 1000

		self.camera = Vector2()

		if self.fullscreen:
			self.viewport = pygame.display.set_mode((self.width, self.height), pygame.FULLSCREEN)
		else:
			self.viewport = pygame.display.set_mode((self.width, self.height))
		
		if displayPlatform:
			pygame.display.set_caption(f"{self.title} ({sys.platform})")
		else:
			pygame.display.set_caption(self.title)

	def Loop(self, update=Update) -> None:
		while True:
			#check for the user wanting to exit the game
			for event in pygame.event.get():
				if event.type == pygame.QUIT:
					pygame.quit()
					sys.exit()
			self.viewport.fill(colors["black"])
			update()
			
			pygame.display.update()
			self.deltaTime = self.fpsClock.tick(self.fps) / 1000

	def Exit(self) -> None:
		pygame.quit()
		sys.exit()
