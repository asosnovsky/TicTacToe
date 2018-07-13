#!/usr/bin/python3.6

# =========================
# Version Check
# =========================
from sys import version_info
assert version_info > (3,6,4)

# =========================
# Define Board
# =========================
from typing import Dict, List
from enum import Enum
from random import choice, random

class CellValue(Enum):
    NA = " "
    X = "X"
    O = "O"

class WinState:
    def __init__(self):
        self.value = CellValue.NA
        self.count = 0
        self.lost = False

class Board:
    """This is the primary class for handling any interaction with the board
    """

    __size:int # number of colums or row (so the full size of the board is really __size ** 2)

    __win_state_row:Dict[int, WinState] = {} # track win_state for rows
    __win_state_col:Dict[int, WinState] = {} # track win_state for collumns
    __win_state_cross:Dict[str, WinState] = {
        'lr': WinState(), # track win_state for left diagnols
        'rl': WinState(), # track win_state for right diagnols
    }
    __winner:CellValue = None # This will store the symbol of the winning player

    _board:Dict[str, CellValue] = {} # this stores all the values in the board

    def __init__(self, size: int= 3):
        self.__size = size
        for row in range(size):
            self.__win_state_col[row] = WinState()
            self.__win_state_row[row] = WinState()
            for col in range(size):
                self._board[f"{row},{col}"] = CellValue.NA

    @property
    def winner(self) -> None or CellValue:
        return self.__winner

    def game_over(self) -> bool:
        """Determins if the game is done
        Returns:
            bool -- True for when over, False otherwise
        """

        if self.winner is not None or len(self.open_spots) == 0:
            return True
        else:
            return False

    @property
    def open_spots(self) -> List[str]:
        """Returns a list of spots that are empty

        Returns:
            List[str]
        """

        open_spots = []
        for key, v in self._board.items():
            if v == CellValue.NA:
                open_spots += [key]
        return open_spots

    def __update_win_state(self, idx:int or str, win_type: str, value: CellValue):
        """Updates a win state for a given type

        Arguments:
            idx {int or str} -- int for type='row' or 'col', str for 'cross'
            win_type {str} -- 'col', 'row', or 'cross'
            value {CellValue}
        """

        container:Dict[str or int, WinState]
        if win_type == "row":
            container = self.__win_state_row
        elif win_type == "col":
            container = self.__win_state_col
        elif win_type == "cross":
            container = self.__win_state_cross

        if not container.get(idx).lost:
            if container.get(idx).value == CellValue.NA:
                container.get(idx).value = value
                container.get(idx).count += 1
            elif container.get(idx).value == value:
                container.get(idx).count += 1
                if container.get(idx).count == self.__size:
                    self.__winner = value
            else:
                container.get(idx).lost = True

    def set(self, row:int, col:int, value:CellValue):
        """Sets a value on the board

        Arguments:
            row {int}
            col {int}
            value {CellValue}
        """

        self._board[f"{row},{col}"] = value

        self.__update_win_state(row, 'row', value)
        self.__update_win_state(col, 'col', value)

        if row == col:
            self.__update_win_state('lr', 'cross', value)

        if row + col == self.__size - 1:
            self.__update_win_state('rl', 'cross', value)


    def get(self, row:int, col:int) -> CellValue:
        """Gets a given value from the board

        Arguments:
            row {int}
            col {int}

        Returns:
            CellValue
        """

        return self._board[f"{row},{col}"]

    def random_set(self, value: CellValue):
        """Randomly select an open spot and assign a value to it

        Arguments:
            value {CellValue}
        """

        row, col = choice(self.open_spots).split(",")
        self.set(int(row), int(col), value)

    def __repr__(self) -> str :
        """Overwrite the "print" output

        Returns:
            str
        """

        sep = "-" * (self.__size*2 - 1)
        sep = "\n ] " + sep + "\n"
        print("  ", ",".join([
            str(idx+1) for idx in range(self.__size)
        ]))
        return sep.join([
            f"{row+1}] " + "|".join([
                self._board[f"{row},{col}"].value
                for col in range(self.__size)
            ])
            for row in range(self.__size)
        ])


# =========================
# Define Errors
# =========================
class BadPlayerMove(Exception):
    def __init__(self, why:str):
        self.why = why

# =========================
# Define utils for screen handling
# =========================
from os import system, name as os_name
from time import sleep
def clear_screen():
    system('cls' if os_name=='nt' else 'clear')

def ask_coord(current_player:str, role: CellValue) -> (int, int):
    """Asks the player for when to place their choice

    Arguments:
        current_player {str} -- player name
        role {CellValue} -- the player's symbol

    Raises:
        BadPlayerMove -- raised if the player did not input an expected format

    Returns:
        int -- x value
        int -- y value
    """

    user_input = input(f"{current_player} > ({role.value}) Enter where (row,col)> ")
    coords = user_input.split(",")
    if len(coords) != 2:
        raise BadPlayerMove("Invalid Input")
    x, y = coords
    try:
        x = int(x) - 1
        y = int(y) - 1
        return x, y
    except ValueError:
        raise BadPlayerMove("Invalid Input")

def ask_then_move_player(board:Board, current_player:str, current_role: CellValue) -> Board:
    """Asks the player for input and updates the board

    Arguments:
        board {Board} -- current active game board
        current_player {str} -- player name
        current_role {CellValue} -- player's symbol

    Raises:
        BadPlayerMove -- raised if the player did not input an expected format

    Returns:
        Board -- current active game board
    """

    x, y = ask_coord(current_player, current_role)
    try:
        if board.get(x,y) != CellValue.NA:
            raise BadPlayerMove("Spot taken")
        board.set(x, y, current_role)
        return board
    except KeyError:
        raise BadPlayerMove("Invalid Spot")

def decied_who_goes(player_1: str, player_2: str) -> Dict[str, str or CellValue]:
    """ 'Flips a coin' to see who goes first, and return a player state dict

    Arguments:
        player_1 {str} -- player's name
        player_2 {str} -- player's name

    Returns:
        Dict[str, str or CellValue] -- player's state
    """

    clear_screen()
    print("Flipping coin to see who goes first...")
    sleep(1)
    coin = choice(['heads', 'tails'])
    if coin == 'heads':
        print(f"its `heads` '{player_1}' goes first")
        players = {
            'current': player_1,
            player_1: CellValue.X,
            player_2: CellValue.O,
        }
    else:
        print(f"its `tails` '{player_2}' goes first")
        players = {
            'current': player_2,
            player_1: CellValue.O,
            player_2: CellValue.X,
        }
    return players

# =========================
# Define Game Types
# =========================
def new_two_player_game():
    """This will initilize multi-player mode
    """

    board = Board()
    player_1 = input("Please input name for player 1: ")
    player_2 = input("Please input name for player 2: ")
    if player_1 == player_2:
        player_2 += "_2"
    players = decied_who_goes(player_1, player_2)
    sleep(1)
    while not board.game_over():
        clear_screen()
        print(board)
        current_player = players.get('current')
        current_role = players.get( current_player )
        try:
            board = ask_then_move_player(board, current_player, current_role)
        except BadPlayerMove as e:
            print(e.why)
            sleep(0.5)
            continue

        if players.get('current') == player_1:
            players['current'] = player_2
        else:
            players['current'] = player_1
    clear_screen()
    print(board)
    if board.winner == players.get(player_1):
        print_baloons()
        print(f"Congragulations {player_1} has won!")
    else:
        print_baloons()
        print(f"Congragulations {player_2} has won!")

def new_single_player():
    """This will launch single player mode
    """

    board = Board()
    player_user = input("Please input name for player: ")
    player_pc = 'PC'
    if player_user == player_pc:
        player_pc += "_2"
    players = decied_who_goes(player_user, player_pc)
    sleep(1)
    while not board.game_over():
        clear_screen()
        print(board)
        current_player = players.get('current')
        current_role = players.get( current_player )
        if current_player == player_user:
            x, y = ask_coord(current_player, current_role)
            try:
                if board.get(x,y) != CellValue.NA:
                    print("Spot taken")
                    sleep(0.5)
                    continue
                board.set(x, y, current_role)
            except KeyError:
                print("Invalid Spot")
                sleep(0.5)
                continue
        else:
            print("Computer is thinking... 🤔")
            sleep(random() * 0.6 + 0.2)
            board.random_set(current_role)
        if players.get('current') == player_user:
            players['current'] = player_pc
        else:
            players['current'] = player_user
    clear_screen()
    print(board)
    if board.winner == players.get(player_user):
        print_baloons()
        print(f"Congragulations {player_user} has won!")
    else:
        print(f"Sorry... {player_user} has lost...")

def new_ai_game():
    """This is just for fun, see two "ai"'s compete with each other
    """

    board = Board()
    print(board)
    print('=====================')
    current_player = choice([CellValue.X, CellValue.O])
    while not board.game_over():
        board.random_set(current_player)
        clear_screen()
        print(board)
        print("🤔...")
        sleep(random() * 0.6 + 0.2)
        if current_player == CellValue.X:
            current_player = CellValue.O
        else:
            current_player = CellValue.X
    print_baloons()
    print(f"Congragulations {board.winner.value} has won!")

# =========================
# User Print Requests
# =========================
def print_logo():
    clear_screen()
    print("___________.__     ___________           ___________               ")
    print("\__    ___/|__| ___\__    ___/____    ___\__    ___/___   ____    ")
    print("  |    |   |  |/ ___\|    |  \__  \ _/ ___\|    | /  _ \_/ __ \    ")
    print("  |    |   |  \  \__ |    |   / __ \\  \___ |    |(  <_> )  ___/    ")
    print("  |____|   |__|\___  >____|  (____  /\___  >____| \____/ \___  >   ")
    print("                   \/             \/     \/                  \/    ")
    print("   _____        .__  ___________    .___.__  __  .__               ")
    print("  /  _  \_______|__| \_   _____/  __| _/|__|/  |_|__| ____   ____  ")
    print(" /  /_\  \_  __ \  |  |    __)_  / __ | |  \   __\  |/  _ \ /    \ ")
    print("/    |    \  | \/  |  |        \/ /_/ | |  ||  | |  (  <_> )   |  \\")
    print("\____|__  /__|  |__| /_______  /\____ | |__||__| |__|\____/|___|  /")
    print("        \/                   \/      \/                         \/ ")
    print("Welcom to Ari's TicTacToe")

def print_baloons():
    print("__$$$$$$$$$                  ")
    print("_$$________$$                ")
    print("$____________$               ")
    print("$____________$___$$$$$$$$    ")
    print("$____________$_$$________$$  ")
    print("$____________$$____________$ ")
    print("_$_________$$_$____________$ ")
    print("__$$$____$$$__$____________$ ")
    print("_____$$$$_____$____________$ ")
    print("_______$$______$_________$$  ")
    print("______$$________$$$____$$    ")
    print("_____$$____________$$$$      ")
    print("____$________________$$      ")
    print("____$_______________$$       ")
    print("____$$_____________$$        ")
    print("_____$$___________$          ")
    print("_______$$_________$          ")

def print_select_game_mode():
    print("Please select a game mode:")
    print(" (1) Single Player")
    print(" (2) Multi Player")
    print(" (3) Full AI Mode")
    print(" (4) Quit")
    game_type = input("Please choose a game type [1,2,3,4]: ")
    if game_type == '1':
        new_single_player()
    elif game_type == '2':
        new_two_player_game()
    elif game_type == '3':
        new_ai_game()
    elif game_type == '4':
        print("Good Bye!")
        return
    else:
        clear_screen()
        print("Invalid Input...")
        return print_select_game_mode()
    print("--------------------")
    again = input("Play again? (Y/n) ")
    if again[0].lower() == "y":
        clear_screen()
        print_select_game_mode()
    else:
        clear_screen()
        print("Good Bye!")

# =========================
# Where the magic happens
# =========================
clear_screen()
print_logo()
print_select_game_mode()
