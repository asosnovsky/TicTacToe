package board

// WinState is the state of a particular win type
type WinState struct {
	count int
	lost  bool
	value string
}

// WinStates a collection of states
type WinStates struct {
	row   map[int]WinState
	col   map[int]WinState
	diagL WinState
	diagR WinState
}
