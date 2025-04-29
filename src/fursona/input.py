import pygame

class Input:
	def GetKeyPressed(key : str) -> bool:
		getKeyboard = pygame.key.get_pressed()

		try:
			if getKeyboard[ord(key)]:
				return True
			else:
				return False
		except TypeError:
			if key.lower() == "f12" and getKeyboard[pygame.K_F12]:
				return True
			else:
				return False
