from fursona import *
from sys import platform

SCR_W = 1024
SCR_H = 768

speed = Vector2(2, 2)

display = Window(f"sgr", 1024, 768, displayPlatform=True, fps=75)
slugcat1 = Entity("data/slugcat1.png", Vector2(), Vector2(1, 1))

def Update():
	if Input.GetKeyPressed("q"):
		display.Exit()

	slugcat1.rect = slugcat1.rect.move(speed.x, speed.y)
	if slugcat1.rect.left < 0 or slugcat1.rect.right > SCR_W:
		speed.x = -speed.x
	if slugcat1.rect.top < 0 or slugcat1.rect.bottom > SCR_H:
		speed.y = -speed.y
	
	#slugcat1.texture = pygame.transform.scale(slugcat1.image, (slugcat1.width * slugcat1.scalar.x, slugcat1.height * slugcat1.scalar.y))

	if slugcat1.width < -1 or slugcat1.height < -1:
		raise ValueError("Entity.width or Entity.height cannot be below -1 (-1 = default dimensions)")
	"""dest = Vector2()
	dest.x = slugcat1.position.x - display.camera.x
	dest.y = slugcat1.position.y - display.camera.y"""
	display.viewport.blit(slugcat1.texture, slugcat1.rect)

display.Loop(Update)
