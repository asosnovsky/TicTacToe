package game

import (
	"fmt"
	"log"
	"math/rand"
	"time"

	st "../screentools"
	tc "./board"
)

// Game all game state data is stored here
type Game struct {
	board         tc.Board
	player1       Player
	player2       Player
	currentPlayer *Player
}

// NewGame creates a new Game stuct
func NewGame(player1Name string, player2Name string, isPC [2]bool) Game {
	if player1Name == player2Name {
		player2Name += " 2"
	}
	game := Game{
		board:   tc.MakeBoard(3),
		player1: Player{name: player1Name, isPC: isPC[0]},
		player2: Player{name: player2Name, isPC: isPC[1]},
	}
	if rand.Intn(2) < 1 {
		game.player1.symbol = tc.BoardCellX
		game.player2.symbol = tc.BoardCellO
	} else {
		game.player2.symbol = tc.BoardCellX
		game.player1.symbol = tc.BoardCellO
	}
	return game
}

// Start will initilize the game
func (game Game) Start() (string, error) {
	fmt.Println("Flipping coin to see who goes first...")
	time.Sleep(time.Duration(rand.Intn(500)+200) * time.Millisecond)
	var coin string
	if rand.Intn(2) < 1 {
		coin = "heads"
		game.currentPlayer = &game.player1
	} else {
		coin = "tails"
		game.currentPlayer = &game.player2
	}

	fmt.Println(" > its `", coin, "`", game.currentPlayer.name, "goes first")
	time.Sleep(time.Second)
	winner, game, err := game.restart()
	if err != nil {
		return winner, err
	}
	if winner == "tie" {
		fmt.Println("There are no winners...")
	} else {
		st.Baloons()
		fmt.Println(winner, "("+game.board.Winner+")", "has won the game!")
	}
	return winner, nil
}

func (game Game) restart() (string, Game, error) {
	st.ClearScreen()
	fmt.Println(game.board)
	game, err := game.next()
	if err != nil {
		log.Println(err)
		return game.restart()
	}
	if game.board.Winner == tc.BoardCellNA {
		return "tie", game, nil
	} else if game.board.Winner == game.player1.symbol {
		return game.player1.name, game, nil
	}
	return game.player2.name, game, nil
}

func (game Game) next() (Game, error) {
	if game.currentPlayer.isPC {
		fmt.Println("Computer is thinking... 🤔")
		time.Sleep(time.Millisecond * time.Duration(rand.Intn(600)+200))
		board, err := game.board.SetRandom(game.currentPlayer.symbol)
		if err != nil {
			return game, err
		}
		game.board = board
		return game.endOrSwitchPlayers()
	}
	x, y, err := st.AskCoords(game.currentPlayer.name, game.currentPlayer.symbol)
	if err != nil {
		return game, err
	}
	board, err := game.board.Set(x, y, game.currentPlayer.symbol)
	if err != nil {
		return game, err
	}
	game.board = board
	return game.endOrSwitchPlayers()
}

func (game Game) endOrSwitchPlayers() (Game, error) {
	st.ClearScreen()
	fmt.Println(game.board)
	if game.board.GameIsOver() {
		return game, nil
	}
	if game.currentPlayer.symbol == game.player1.symbol {
		game.currentPlayer = &game.player2
	} else {
		game.currentPlayer = &game.player1
	}
	return game.next()
}
