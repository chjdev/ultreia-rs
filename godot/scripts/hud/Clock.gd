extends Label

export var start_year = 1400

func _ready():
	self._on_tock(0)
	Clock.connect("Tock", self, "_on_tock")

func _on_tock(clock: int):
	self.text = "A.D. " + str(start_year + clock)
