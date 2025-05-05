	// I wouldn't try to touch this if i were you..... i have no clue HOW this works.... but it works
	pub fn update(
		&mut self,
		screen_width: i32,
		screen_height: i32,
		delta_time: f32,
		map_mask: &[bool],
		mask_width: i32,
		mask_height: i32,
		other_slugcats: &Vec<CollisionData>,
	) {
		let mut rng: rand::prelude::ThreadRng = rand::rng();
		let next_x = self.position.x + self.speed.x * delta_time;
		let next_y = self.position.y + self.speed.y * delta_time;
	
		// ─── HORIZONTAL TEST ────────────────────────────────────────────────────────
		let mut collided_x = false;
		'horizontal: for y in 0..self.height {
			for x in 0..self.width {
				let idx = (y * self.width + x) as usize;
				if !self.mask[idx] { continue; }
	
				let wx = next_x + x as f32;
				let wy = self.position.y + y as f32;
	
				// 1) map collision
				if wx >= 0.0 && wy >= 0.0
				   && wx < mask_width  as f32
				   && wy < mask_height as f32
				   && map_mask[(wy as i32 * mask_width + wx as i32) as usize]
				{
					collided_x = true;
					break 'horizontal;
				}
	
				// 2) entity–entity collision
				for other in other_slugcats.iter() {
					if other.name == self.name { continue; }
	
					let rel_x = (wx - other.position.x).floor() as i32;
					let rel_y = (wy - other.position.y).floor() as i32;
					if rel_x < 0 || rel_y < 0 || rel_x >= other.width || rel_y >= other.height {
						continue;
					}
					let other_idx = (rel_y * other.width + rel_x) as usize;
					if other.mask[other_idx] {
						collided_x = true;
						break 'horizontal;
					}
				}
			}
		}
		if collided_x {
			self.speed.x = -self.speed.x.signum() * rng.random_range(MIN_SPEED..MAX_SPEED) as f32;
		} else {
			self.position.x = next_x;
		}
	
		// ─── VERTICAL TEST ──────────────────────────────────────────────────────────
		let mut collided_y = false;
		'vertical: for y in 0..self.height {
			for x in 0..self.width {
				let idx = (y * self.width + x) as usize;
				if !self.mask[idx] { continue; }
	
				let wx = self.position.x + x as f32;
				let wy = next_y + y as f32;
	
				// 1) map collision
				if wx >= 0.0 && wy >= 0.0
				   && wx < mask_width  as f32
				   && wy < mask_height as f32
				   && map_mask[(wy as i32 * mask_width + wx as i32) as usize]
				{
					collided_y = true;
					break 'vertical;
				}
	
				// 2) entity–entity collision
				for other in other_slugcats.iter() {
					if other.name == self.name { continue; }
	
					let rel_x = (wx - other.position.x).floor() as i32;
					let rel_y = (wy - other.position.y).floor() as i32;
					if rel_x < 0 || rel_y < 0 || rel_x >= other.width || rel_y >= other.height {
						continue;
					}
					let other_idx = (rel_y * other.width + rel_x) as usize;
					if other.mask[other_idx] {
						collided_y = true;
						break 'vertical;
					}
				}
			}
		}
		if collided_y {
			self.speed.y = -self.speed.y.signum() * rng.random_range(MIN_SPEED..MAX_SPEED) as f32;
		} else {
			self.position.y = next_y;
		}
	
		// ─── SCREEN BOUNDS ──────────────────────────────────────────────────────────
		if self.position.x < 0.0 || self.position.x + self.width as f32 > screen_width as f32 {
			self.speed.x = -self.speed.x;
		}
		if self.position.y < 0.0 || self.position.y + self.height as f32 > screen_height as f32 {
			self.speed.y = -self.speed.y;
		}
	}