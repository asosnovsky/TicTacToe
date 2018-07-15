package game

// Player data regarding a player
type Player struct {
	name   string
	symbol string
	isPC   bool
}

func (player Player) String() string {
	str := "<Player name='"
	str += player.name
	str += "' symbol='"
	str += player.symbol
	str += "'"
	if player.isPC {
		str += " PC/>"
	} else {
		str += "/>"
	}
	return str
}
