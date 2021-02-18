extends Node

const configuration = {
	"columns": 650,
	"rows": 400,
	"island_noise": 6
}

func _ready():
	Game.start_game(configuration)
