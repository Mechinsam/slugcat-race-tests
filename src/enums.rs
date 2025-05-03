#[derive(PartialEq)] // this allows comparisons
pub enum GameState
{
	InRace,
	Win
}

// No this is not the same as GameState. GameState is used for keeping track of "scenes"
// GameEvent is for keeping track of events that have happened (clears every frame)
#[derive(PartialEq)]
pub enum GameEvent
{
	None,
	RaceWon,
	UnleashSlugcats
}
