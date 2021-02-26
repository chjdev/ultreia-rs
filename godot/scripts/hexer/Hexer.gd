extends Node2D

export var oversample = Vector2(1.2, 1.2)
var current_rect: Rect2 = Rect2(0, 0, 0, 0)

var HexGrid = preload("../hex/HexGrid.gd").new()

func _ready():
	Game.connect("GameStart", self, "_on_game_start")
	FOW.connect("Uncover", self, "_on_uncover")

var last_start_coords = Vector2.ZERO
var last_stop_coords = Vector2.ZERO

func update_hexes(rect: Rect2, force: bool = false):
	self.current_rect = rect
	var view_size = rect.size * oversample
	var current_center = rect.position
	var start = current_center - (view_size / 2)
	var stop = start + view_size
	var start_coords = HexGrid.get_hex_at(start).offset_coords
	var stop_coords = HexGrid.get_hex_at(stop).offset_coords
	var used_rect = $Terrain.get_used_rect()
	# clear out last rect
	for j in range(used_rect.position.y, used_rect.position.y + used_rect.size.y):
		var i = used_rect.position.x
		while i < used_rect.position.x + used_rect.size.x:
			if j >= last_start_coords.y and j < last_stop_coords.y and i >= last_start_coords.x and i < last_stop_coords.x:
				i = last_stop_coords.x
				# nothing to do, still visible
				continue
			$Terrain.set_cell(i, j, -1)
			$Yield.set_cell(i, j, -1)
			i += 1
	if force:
		$Terrain.clear()
		$Yield.clear()
	# fill in new rect where necessary
	for j in range(floor(start_coords.y), ceil(stop_coords.y)):
		var i = floor(start_coords.x)
		while i < ceil(stop_coords.x):
			if not force and j >= last_start_coords.y and j < last_stop_coords.y and i >= last_start_coords.x and i < last_stop_coords.x:
				i = last_stop_coords.x
				# nothing to do, still visible
				continue
			var coord = HexGrid.get_zero_hex()
			coord.offset_coords = Vector2(i, j)
			# todo smth more sane
			if FOW.at(coord.cube_coords):
				var terrain = Terrain.at(coord.cube_coords)
				$Terrain.set_terrain_cell(i, j, terrain)
				$Yield.show_majority_yield(i, j, terrain)
			i += 1
	last_start_coords = start_coords
	last_stop_coords = stop_coords

var _camera_debounced = false
func _on_WorldCamera_change_debounced(rect: Rect2):
	update_hexes(rect)
	_camera_debounced = false

func _on_WorldCamera_change(rect: Rect2):
	if _camera_debounced == false:
		_camera_debounced = true
		call_deferred("_on_WorldCamera_change_debounced", rect)

func _on_game_start():
	# deferred because `Game` needs time to populate
	call_deferred("update_hexes", get_parent().get_node("WorldCamera").get_rect())

func _on_Cursor_hex_enter(hex):
	$Terrain.focus(hex)

func _on_Cursor_hex_exit(hex):
	$Terrain.blur(hex)

func _on_Cursor_hex_pressed(hex):
	print(Buildings.try_construct(hex.cube_coords, "Warehouse"))

func _on_uncover(hex_coords):
	var hex = HexGrid.get_zero_hex()
	for hex_coord in hex_coords:
		hex.cube_coords = hex_coord
		var offset = hex.offset_coords
		if offset > last_start_coords and offset <= last_stop_coords:
			var terrain = Terrain.at(hex_coord)
			$Terrain.set_terrain_cell(offset.x, offset.y, terrain)
			$Yield.show_majority_yield(offset.x, offset.y, terrain)
