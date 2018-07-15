package main

import (
	"fmt"
	"log"
	"strings"
	"time"

	"./game"
	st "./screentools"
)

func begin() {
	var g game.Game
	gameMode := st.GameMode()
	switch gameMode {
	case "single":
		playerName, err := st.ReadInput("Please input name for player: ")
		if err != nil {
			log.Fatal(err)
		}
		g = game.NewGame(playerName, "PC", [2]bool{false, true})
		break
	case "multi":
		playerName1, err := st.ReadInput("Please input name for player 1: ")
		if err != nil {
			log.Fatal(err)
		}
		playerName2, err := st.ReadInput("Please input name for player 2: ")
		if err != nil {
			log.Fatal(err)
		}
		g = game.NewGame(playerName1, playerName2, [2]bool{false, false})
		break
	case "ai":
		g = game.NewGame("PC", "PC", [2]bool{true, true})
		break
	default:
		fmt.Println("Goodbye!")
		return
	}
	fmt.Println("Starting an", strings.ToTitle(gameMode[0:1])+strings.ToLower(gameMode[1:len(gameMode)]), "Player game ...")
	time.Sleep(time.Second)
	_, err := g.Start()
	if err != nil {
		log.Fatal(err)
	}
}

func main() {
	st.Logo()
	for true {
		begin()
		answ, err := st.ReadInput("Play again? (Y/n) ")
		if err != nil {
			log.Fatal(err)
		}
		if strings.ToUpper(answ)[0:1] == "N" {
			fmt.Println("Goodbye!")
			return
		}
	}
}
