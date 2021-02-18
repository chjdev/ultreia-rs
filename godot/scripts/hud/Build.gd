extends MenuButton

func _ready():
	get_popup().add_separator("Civics")
	get_popup().add_item("City Center")
	get_popup().add_item("Neighbourhood")
	get_popup().add_item("Chapel")
	get_popup().add_separator("Industry")
	get_popup().add_item("Lumberjack")
	get_popup().connect("id_pressed", self, "_on_id_pressed")

func _on_id_pressed(id):
	match id:
		1:
			print("hello")
