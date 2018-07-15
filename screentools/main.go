package screentools

import (
	"bufio"
	"fmt"
	"log"
	"os"
	"strconv"
	"strings"

	tm "github.com/buger/goterm"
)

// ClearScreen will empty the screen
func ClearScreen() {
	tm.Flush()
	tm.MoveCursor(1, 1)
	tm.Clear()
}

// ReadInput from user
func ReadInput(ask string) (string, error) {
	reader := bufio.NewReader(os.Stdin)
	fmt.Print(ask)
	s, err := reader.ReadString('\n')
	if err != nil {
		return s, err
	}
	return strings.TrimSpace(s), nil
}

// AskCoords will ask and format the coords
func AskCoords(playerName string, symbol string) (int, int, error) {
	coords, err := ReadInput(playerName + " > (" + symbol + ") Enter where (row,col)> ")
	if err != nil {
		return 0, 0, fmt.Errorf("Invalid Input")
	}
	idxs := strings.Split(coords, ",")
	if len(idxs) == 2 {
		x, xerr := strconv.Atoi(idxs[0])
		y, yerr := strconv.Atoi(idxs[1])
		if xerr == nil && yerr == nil {
			return x, y, nil
		}
	}
	return 0, 0, fmt.Errorf("Invalid Input")
}

// GameMode prints the mode selection screen
func GameMode() string {
	fmt.Println("Please select a game mode:")
	fmt.Println(" (1) Single Player")
	fmt.Println(" (2) Multi Player")
	fmt.Println(" (3) Human-less Mode")
	fmt.Println(" (4) Quit")
	mode, err := ReadInput("Please choose a game type [1,2,3,4]: ")
	if err != nil {
		log.Println(fmt.Errorf("Invalid"))
		return GameMode()
	}
	switch mode {
	case "1":
		return "single"
	case "2":
		return "multi"
	case "3":
		return "ai"
	case "4":
		return "quit"
	default:
		log.Println(fmt.Errorf("Invalid Input"))
		return GameMode()
	}
}

// Logo prints the Game logo
func Logo() {
	ClearScreen()
	fmt.Println("___________.__     ___________           ___________               ")
	fmt.Println("\\__    ___/|__| ___\\__    ___/____    ___\\__    ___/___   ____    ")
	fmt.Println("  |    |   |  |/ ___\\|    |  \\__  \\ _/ ___\\|    | /  _ \\_/ __ \\    ")
	fmt.Println("  |    |   |  \\  \\__ |    |   / __ \\\\  \\___ |    |(  <_> )  ___/    ")
	fmt.Println("  |____|   |__|\\___  >____|  (____  /\\___  >____| \\____/ \\___  >   ")
	fmt.Println("                   \\/             \\/     \\/                  \\/    ")
	fmt.Println("   _____        .__  ___________    .___.__  __  .__               ")
	fmt.Println("  /  _  \\_______|__| \\_   _____/  __| _/|__|/  |_|__| ____   ____  ")
	fmt.Println(" /  /_\\  \\_  __ \\  |  |    __)_  / __ | |  \\   __\\  |/  _ \\ /    \\ ")
	fmt.Println("/    |    \\  | \\/  |  |        \\/ /_/ | |  ||  | |  (  <_> )   |  \\")
	fmt.Println("\\____|__  /__|  |__| /_______  /\\____ | |__||__| |__|\\____/|___|  /")
	fmt.Println("        \\/                   \\/      \\/                         \\/ ")
	fmt.Println("Welcom to Ari's TicTacToe")
}

// Baloons prints baloons
func Baloons() {
	fmt.Println("__$$$$$$$$$                  ")
	fmt.Println("_$$________$$                ")
	fmt.Println("$____________$               ")
	fmt.Println("$____________$___$$$$$$$$    ")
	fmt.Println("$____________$_$$________$$  ")
	fmt.Println("$____________$$____________$ ")
	fmt.Println("_$_________$$_$____________$ ")
	fmt.Println("__$$$____$$$__$____________$ ")
	fmt.Println("_____$$$$_____$____________$ ")
	fmt.Println("_______$$______$_________$$  ")
	fmt.Println("______$$________$$$____$$    ")
	fmt.Println("_____$$____________$$$$      ")
	fmt.Println("____$________________$$      ")
	fmt.Println("____$_______________$$       ")
	fmt.Println("____$$_____________$$        ")
	fmt.Println("_____$$___________$          ")
	fmt.Println("_______$$_________$          ")
}
