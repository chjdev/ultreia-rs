extends TileMap

const highlighted_tile_offset = 1
const green_tile_offset = 2
const red_tile_offset = 3

var terrain_enum = Terrain.terrain_enum()

onready var base_tiles = self.get_tileset().get_tiles_ids()
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
	self.get_tileset().tile_set_region(to_tile_id, self.get_tileset().tile_get_region(from_tile_id))
	self.get_tileset().tile_set_texture(to_tile_id, self.get_tileset().tile_get_texture(from_tile_id))
	self.get_tileset().tile_set_texture_offset(to_tile_id, self.get_tileset().tile_get_texture_offset(from_tile_id))

func _ready():
	# highlight tiles
	var Brighten5 = load("./shaders/brighten.tres")
	Brighten5.set_shader_param("bright_amount", 1.5)
	for tile_id in base_tiles:
		var highlighted_tile_id = _highlighted_tile_id(tile_id)
		self.get_tileset().create_tile(highlighted_tile_id)
		self._copy_tile_to(tile_id, highlighted_tile_id)
		self.get_tileset().tile_set_material(highlighted_tile_id, Brighten5)
	# green tiles
	for tile_id in base_tiles:
		var green_tile_id = _green_tile_id(tile_id)
		self.get_tileset().create_tile(green_tile_id)
		self._copy_tile_to(tile_id, green_tile_id)
		self.get_tileset().tile_set_modulate(green_tile_id, Color.green)
	# red tiles
	for tile_id in base_tiles:
		var red_tile_id = _red_tile_id(tile_id)
		self.get_tileset().create_tile(red_tile_id)
		self._copy_tile_to(tile_id, red_tile_id)
		self.get_tileset().tile_set_modulate(red_tile_id, Color.red)
	Buildings.connect("Created", self, "_on_building_created")

func set_terrain_cell(i, j, terrain):
	var tile_id = self._find_tile_id(terrain)
	self.set_cell(i, j, tile_id)

func _find_tile_id(terrain):
	var tile_by_name = self.tile_set.find_tile_by_name(terrain_enum[terrain.terrain_type])
	return tile_by_name if tile_by_name >= 0 else self.tile_set.find_tile_by_name("None")

func focus(hex):
	var hex_coords = hex.offset_coords
	self.set_cell(hex_coords.x, hex_coords.y, self._highlighted_tile_id(self.get_cell(hex_coords.x, hex_coords.y)))

func blur(hex):
	var hex_coords = hex.offset_coords
	self.set_cell(hex_coords.x, hex_coords.y, self._base_tile_id(self.get_cell(hex_coords.x, hex_coords.y)))

func _on_building_created(coordinate, tile_name):
	self.set_cell(hex_coords.x, hex_coords.y, self._base_tile_id(self.get_cell(hex_coords.x, hex_coords.y)))
	print("wtf", coordinate, tile_name)
