package board

import (
	"fmt"
	"math/rand"
	"strconv"
	"strings"
)

// Board the main game board state
type Board struct {
	board     map[string]string
	winStates WinStates
	size      int
	Winner    string
}

// MakeBoard creates a new game board
func MakeBoard(size int) Board {
	board := make(map[string]string)
	winStates := WinStates{
		row:   make(map[int]WinState),
		col:   make(map[int]WinState),
		diagL: WinState{},
		diagR: WinState{},
	}

	for row := 1; row <= size; row++ {
		winStates.row[row] = WinState{}
		winStates.col[row] = WinState{}
		for col := 1; col <= size; col++ {
			key := strconv.Itoa(row) + "," + strconv.Itoa(col)
			board[key] = BoardCellNA
		}
	}
	return Board{
		board:     board,
		winStates: winStates,
		size:      size,
		Winner:    BoardCellNA,
	}
}

func (board Board) String() string {
	str := ""
	for row := 1; row <= board.size; row++ {
		for col := 1; col <= board.size; col++ {
			key := strconv.Itoa(row) + "," + strconv.Itoa(col)
			value := board.board[key]
			str += value
			if col < board.size {
				str += "|"
			}
		}
		str += "\n"
		if row < board.size {
			str += strings.Repeat("-", board.size*2-1)
			str += "\n"
		}
	}
	return str
}

func (board Board) updateWinState(winType string, idx int, symbol string) (Board, error) {
	var container WinState
	switch winType {
	case "row":
		container = board.winStates.row[idx]
		break
	case "col":
		container = board.winStates.col[idx]
		break
	case "diagL":
		container = board.winStates.diagL
		break
	case "diagR":
		container = board.winStates.diagR
		break
	default:
		return board, fmt.Errorf("Invalid winType")
	}
	if container.lost {
		return board, nil
	} else if container.value == "" {
		container.value = symbol
		container.count = 1
	} else if container.value == symbol {
		container.count++
		if container.count == board.size {
			board.Winner = symbol
		}
	} else if container.value != symbol {
		container.lost = true
	}

	switch winType {
	case "row":
		board.winStates.row[idx] = container
		break
	case "col":
		board.winStates.col[idx] = container
		break
	case "diagL":
		board.winStates.diagL = container
		break
	case "diagR":
		board.winStates.diagR = container
		break
	default:
		return board, fmt.Errorf("Invalid winType")
	}
	return board, nil
}

func (board Board) checkWinner() string {
	if board.winStates.diagL.count == board.size {
		return board.winStates.diagL.value
	} else if board.winStates.diagR.count == board.size {
		return board.winStates.diagR.value
	}
	for _, val := range board.winStates.col {
		if val.count == board.size {
			return val.value
		}
	}
	for _, val := range board.winStates.row {
		if val.count == board.size {
			return val.value
		}
	}
	return BoardCellNA
}

// GameIsOver will return true if there is a Winner or a tie
// Board.GameIsOver will return true if there is a Winner or a tie
func (board Board) GameIsOver() bool {
	board.Winner = board.checkWinner()
	if board.Winner == BoardCellNA && len(board.getOpenSpots()) > 0 {
		return false
	}
	return true
}

// Set sets the value on the boad
// Board.Set sets the value on the boad
func (board Board) Set(row, col int, value string) (Board, error) {
	key := strconv.Itoa(row) + "," + strconv.Itoa(col)
	if board.board[key] == BoardCellNA {
		board.board[key] = value
	} else {
		return board, fmt.Errorf("Spot Taken")
	}

	board, err := board.updateWinState("row", row, value)
	if err != nil {
		return board, err
	}
	board, err = board.updateWinState("col", col, value)
	if err != nil {
		return board, err
	}
	if row == col {
		board, err = board.updateWinState("diagL", row, value)
		if err != nil {
			return board, err
		}
	}
	if row+col-1 == board.size {
		board, err = board.updateWinState("diagR", board.size, value)
		if err != nil {
			return board, err
		}
	}
	return board, nil
}

// Get gets a value from the boad
// Board.Get gets a value from the boad
func (board Board) Get(row, col int) string {
	key := strconv.Itoa(row) + "," + strconv.Itoa(col)
	return board.board[key]
}

func (board Board) getOpenSpots() []string {
	var openSpots []string

	for key, value := range board.board {
		if value == BoardCellNA {
			openSpots = append(openSpots, key)
		}
	}
	return openSpots
}

// SetRandom sets the value on the board at random
// Board.SetRandom sets the value on the board at random
func (board Board) SetRandom(value string) (Board, error) {
	spots := board.getOpenSpots()
	idx := rand.Intn(len(spots))
	idxs := strings.Split(spots[idx], ",")
	if len(idxs) == 2 {
		x, xerr := strconv.Atoi(idxs[0])
		y, yerr := strconv.Atoi(idxs[1])
		if xerr == nil && yerr == nil {
			return board.Set(x, y, value)
		}
	}
	return board, fmt.Errorf("Invalid Coords")
}
