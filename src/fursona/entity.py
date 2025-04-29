import pygame
from fursona.physics import Vector2
from fursona.window import Window

class Entity(pygame.sprite.Sprite):
	def __init__(self, filename, position : Vector2, scalar : Vector2, width=-1, height=-1) -> None:
		super().__init__()

		self.filename = filename
		
		self.width = width
		self.height = height
		self.position = Vector2(0, 0)
		self.position = position
		self.scalar = scalar

		self.SetTexture(self.filename)

		self.rect = pygame.Rect(self.position.x, self.position.y, (self.width * self.scalar.x), (self.height * self.scalar.y))
	
	def SetTexture(self, filename) -> None:
		self.filename = filename
		self.image = pygame.image.load(self.filename)

		if self.width == -1:
			self.width = self.image.get_width()
		if self.height == -1:
			self.height = self.image.get_height()

		if self.width < -1 or self.height < -1:
			raise ValueError("Entity.width or Entity.height cannot be below -1 (-1 = default dimensions)")

		self.texture = pygame.transform.scale(self.image, (self.width * self.scalar.x, self.height * self.scalar.y))

	def IfCollide(self, obj, display) -> bool:
		if type(obj) != Entity:
			raise TypeError("Argument 'obj' must be type 'Entity'")

		temp_group = pygame.sprite.Group()
		temp_group.add(obj)

		self.rect = pygame.Rect(self.position.x, self.position.y, (self.width * self.scalar.x), (self.height * self.scalar.y))

		if pygame.sprite.spritecollide(self, temp_group, True):
			return True
		else:
			return False

	def Update(self, display : Window) -> None:
		self.texture = pygame.transform.scale(self.image, (self.width * self.scalar.x, self.height * self.scalar.y))

		if self.width < -1 or self.height < -1:
			raise ValueError("Entity.width or Entity.height cannot be below -1 (-1 = default dimensions)")

		dest = Vector2()
		dest.x = self.position.x - display.camera.x
		dest.y = self.position.y - display.camera.y

		display.viewport.blit(self.texture, (dest.x, dest.y))
