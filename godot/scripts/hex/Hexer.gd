extends TileMap

enum Lense {
	DEFAULT,
	ELEVATION,
	MOISTURE
}

var current_rect: Rect2 = Rect2(0, 0, 0, 0)

const highlighted_tile_offset = 1
const green_tile_offset = 2
const red_tile_offset = 3

var terrain_enum = Terrain.terrain_enum()

var HexGrid = preload("./HexGrid.gd").new()

onready var base_tiles = get_tileset().get_tiles_ids()
onready var num_base_tiles = len(base_tiles)

func _base_tile_id(tile_id):
	return tile_id % num_base_tiles

func _offset_tile_id(offset, tile_id):
	return offset * num_base_tiles + self._base_tile_id(tile_id)

func _highlighted_tile_id(tile_id):
	return _offset_tile_id(highlighted_tile_offset, tile_id)

func _green_tile_id(tile_id):
	return _offset_tile_id(green_tile_offset, tile_id)

func _red_tile_id(tile_id):
	return _offset_tile_id(red_tile_offset, tile_id)

func _copy_tile_to(from_tile_id, to_tile_id):
	get_tileset().tile_set_region(to_tile_id, get_tileset().tile_get_region(from_tile_id))
	get_tileset().tile_set_texture(to_tile_id, get_tileset().tile_get_texture(from_tile_id))
	get_tileset().tile_set_texture_offset(to_tile_id, get_tileset().tile_get_texture_offset(from_tile_id))

func _ready():
	# highlight tiles
	var Brighten5 = load("./shaders/brighten.tres")
	Brighten5.set_shader_param("bright_amount", 1.5)
	for tile_id in base_tiles:
		var highlighted_tile_id = _highlighted_tile_id(tile_id)
		get_tileset().create_tile(highlighted_tile_id)
		self._copy_tile_to(tile_id, highlighted_tile_id)
		get_tileset().tile_set_material(highlighted_tile_id, Brighten5)
	# green tiles
	for tile_id in base_tiles:
		var green_tile_id = _green_tile_id(tile_id)
		get_tileset().create_tile(green_tile_id)
		self._copy_tile_to(tile_id, green_tile_id)
		get_tileset().tile_set_modulate(green_tile_id, Color.green)
	# red tiles
	for tile_id in base_tiles:
		var red_tile_id = _red_tile_id(tile_id)
		get_tileset().create_tile(red_tile_id)
		self._copy_tile_to(tile_id, red_tile_id)
		get_tileset().tile_set_modulate(red_tile_id, Color.red)
	Game.connect("start_game", self, "_on_game_start")

var last_start_coords = Vector2.ZERO
var last_stop_coords = Vector2.ZERO

func update_hexes(rect: Rect2, force: bool = false):
	self.current_rect = rect
	var view_size = rect.size * Vector2(1.2, 1.2)
	var current_center = rect.position
	var start = current_center - (view_size / 2)
	var stop = start + view_size
	var start_coords = HexGrid.get_hex_at(start).offset_coords
	var stop_coords = HexGrid.get_hex_at(stop).offset_coords
	var used_rect = get_used_rect()
	# clear out last rect
	for j in range(used_rect.position.y, used_rect.position.y + used_rect.size.y):
		var i = used_rect.position.x
		while i < used_rect.position.x + used_rect.size.x:
			if j >= last_start_coords.y and j < last_stop_coords.y and i >= last_start_coords.x and i < last_stop_coords.x:
				i = last_stop_coords.x
				# nothing to do, still visible
				continue
			self.set_cell(i, j, -1)
			i += 1
	if force:
		self.clear()
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
			var terrain = Terrain.at(coord.cube_coords)
			var tile_id = self._find_tile_id(terrain)
			self.set_cell(i, j, tile_id)
			i += 1
	last_start_coords = start_coords
	last_stop_coords = stop_coords

func _find_tile_id(terrain):
	var tile_by_name = self.tile_set.find_tile_by_name(terrain_enum[terrain.terrain_type])
	return tile_by_name if tile_by_name >= 0 else self.tile_set.find_tile_by_name("None")

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
	var hex_coords = hex.offset_coords
	self.set_cell(hex_coords.x, hex_coords.y, self._highlighted_tile_id(self.get_cell(hex_coords.x, hex_coords.y)))

func _on_Cursor_hex_exit(hex):
	var hex_coords = hex.offset_coords
	self.set_cell(hex_coords.x, hex_coords.y, self._base_tile_id(self.get_cell(hex_coords.x, hex_coords.y)))
