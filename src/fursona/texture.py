import pygame
from fursona.physics import Vector2

class Texture:
	def __init__(self, filename, width=-1, height=-1, position=Vector2(0, 0), scalar=Vector2(1, 1)) -> None:
		self.filename = filename
		self.image = pygame.image.load(self.filename)
		
		self.width = width
		self.height = height
		self.position = position
		self.scalar = scalar
		
		if self.width == -1:
			self.width = self.image.get_width()
		if height == -1:
			self.height = self.image.get_height()

		if self.width < -1 or self.height < -1:
			raise ValueError("Texture.width or Texture.height cannot be below -1 (-1 = default dimensions)")

		self.texture = pygame.transform.scale(self.image, (self.width * self.scalar.x, self.height * self.scalar.y))
	
	def Update(self, display) -> None:
		display.viewport.blit(self.texture, (self.position.x, self.position.y))
