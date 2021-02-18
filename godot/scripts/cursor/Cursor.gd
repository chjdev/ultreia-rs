extends Node2D

signal hex_pressed
signal hex_enter
signal hex_exit

var HexGrid = preload("../../scripts/hex/HexGrid.gd").new()

onready var highlight = get_node("Highlight")
onready var hex = get_node("Highlight/Hex")
onready var area_coords = get_node("Highlight/AreaCoords")
onready var hex_coords = get_node("Highlight/HexCoords")

var current_hex = null

func _unhandled_input(event):
	if event is InputEventMouseMotion:
		var relative_pos = self.transform.affine_inverse() * get_local_mouse_position()
		var hex = HexGrid.get_hex_at(relative_pos)
		# Display the coords used
		if area_coords != null:
			area_coords.text = str(relative_pos)
		if hex_coords != null:
			hex_coords.text = str(hex.offset_coords)
		# Snap the highlight to the nearest grid cell
		if highlight != null:
			highlight.position = HexGrid.get_hex_center(hex)
		if self.current_hex == null or self.current_hex.cube_coords != hex.cube_coords:
			if self.current_hex != null:
				emit_signal("hex_exit", self.current_hex)
			self.current_hex = hex
			emit_signal("hex_enter", self.current_hex)
	if event.is_action("action_context_menu"):
		var relative_pos = self.transform.affine_inverse() * get_local_mouse_position()
		var hex = HexGrid.get_hex_at(relative_pos)
		emit_signal("hex_pressed", hex)
