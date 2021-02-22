extends TileMap

var good_enum = Good.good_enum()

const red_tile_offset = 1
const yellow_tile_offset = 2
const light_green_tile_offset = 3
const green_tile_offset = 4

onready var base_tiles = self.get_tileset().get_tiles_ids()
onready var num_base_tiles = len(base_tiles)

func _base_tile_id(tile_id):
	return tile_id % num_base_tiles

func _offset_tile_id(offset, tile_id):
	return offset * num_base_tiles + self._base_tile_id(tile_id)

func _yellow_tile_id(tile_id):
	return _offset_tile_id(yellow_tile_offset, tile_id)

func _light_green_tile_id(tile_id):
	return _offset_tile_id(light_green_tile_offset, tile_id)

func _green_tile_id(tile_id):
	return _offset_tile_id(green_tile_offset, tile_id)

func _red_tile_id(tile_id):
	return _offset_tile_id(red_tile_offset, tile_id)

func _copy_tile_to(from_tile_id, to_tile_id):
	self.get_tileset().tile_set_region(to_tile_id, self.get_tileset().tile_get_region(from_tile_id))
	self.get_tileset().tile_set_texture(to_tile_id, self.get_tileset().tile_get_texture(from_tile_id))
	self.get_tileset().tile_set_texture_offset(to_tile_id, self.get_tileset().tile_get_texture_offset(from_tile_id))

func _ready():
	var Red = load("./shaders/grayscale.tres")
	Red.set_shader_param("mode", Vector3(1, 0, 0))
	# red tiles
	for tile_id in base_tiles:
		var red_tile_id = _red_tile_id(tile_id)
		self.get_tileset().create_tile(red_tile_id)
		self._copy_tile_to(tile_id, red_tile_id)
		self.get_tileset().tile_set_material(red_tile_id, Red)
	var Yellow = Red.duplicate()
	Yellow.set_shader_param("mode", Vector3(1.2, 1.2, 0))
	# yellow tiles
	for tile_id in base_tiles:
		var yellow_tile_id = _yellow_tile_id(tile_id)
		self.get_tileset().create_tile(yellow_tile_id)
		self._copy_tile_to(tile_id, yellow_tile_id)
		self.get_tileset().tile_set_material(yellow_tile_id, Yellow)
	var LightGreen = Red.duplicate()
	LightGreen.set_shader_param("mode", Vector3(0.4, 1, 0.4))
	# light green tiles
	for tile_id in base_tiles:
		var light_green_tile_id = _light_green_tile_id(tile_id)
		self.get_tileset().create_tile(light_green_tile_id)
		self._copy_tile_to(tile_id, light_green_tile_id)
		self.get_tileset().tile_set_material(light_green_tile_id, LightGreen)
	var Green = Red.duplicate()
	Green.set_shader_param("mode", Vector3(0, 1, 0.8))
	# green tiles
	for tile_id in base_tiles:
		var green_tile_id = _green_tile_id(tile_id)
		self.get_tileset().create_tile(green_tile_id)
		self._copy_tile_to(tile_id, green_tile_id)
		self.get_tileset().tile_set_material(green_tile_id, Green)

func show_majority_yield(i, j, terrain):
	var max_value = 0
	var max_key
	for key in terrain.yields.keys():
		var value = terrain.yields[key]
		if value > max_value:
			max_value = value
			max_key = key
	if max_key == null:
		return
	var tile_id = self._find_tile_id_with_yield(max_key, max_value)
	self.set_cell(i, j, tile_id)

func _find_tile_id_with_yield(good, yields, threshold = 0.33):
	if yields < threshold:
		return -1
	var tile_id = self.tile_set.find_tile_by_name(good_enum[good])
	if yields < 0.5:
		return _red_tile_id(tile_id)
	if yields < 0.65:
		return _yellow_tile_id(tile_id)
	if yields < 0.8:
		return _light_green_tile_id(tile_id)
	return _green_tile_id(tile_id)
