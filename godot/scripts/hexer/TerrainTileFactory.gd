extends Reference

var terrain_enum = Terrain.terrain_enum()

var TerrainTileId = preload("./TerrainTileId.gd")

var _tile_set

var _base_tiles
var _num_base_tiles

func _init(tile_set):
	self._tile_set = tile_set
	#todo is this a copy ??
	self._base_tiles = self.get_tileset().get_tiles_ids()
	self._num_base_tiles = len(self._base_tiles)
	_new_batch(funcref(TerrainTileId, "highlighted_from"), funcref(self, "_init_highlighted"))
	_new_batch(funcref(TerrainTileId, "green_from"), funcref(self, "_init_green"))
	_new_batch(funcref(TerrainTileId, "red_from"), funcref(self, "_init_red"))

func get_tileset():
	return self._tile_set

func _copy_tile_to(from_tile_id, to_tile_id):
	var tile_set = self.get_tileset()
	tile_set.tile_set_region(to_tile_id, tile_set.tile_get_region(from_tile_id))
	tile_set.tile_set_texture(to_tile_id, tile_set.tile_get_texture(from_tile_id))
	tile_set.tile_set_texture_offset(to_tile_id, tile_set.tile_get_texture_offset(from_tile_id))

func _new_batch(id: FuncRef, init: FuncRef):
	for tile_id in _base_tiles:
		var new_tile_id = id.call_func(tile_id, self._num_base_tiles)
		self.get_tileset().create_tile(new_tile_id)
		self._copy_tile_to(tile_id, new_tile_id)
		init.call_func(new_tile_id)

var Brighten5 = load("./shaders/brighten.tres")
func _init_highlighted(tile_id):
	Brighten5.set_shader_param("bright_amount", 1.5)
	self.get_tileset().tile_set_material(tile_id, Brighten5)

func _init_green(tile_id):
	self.get_tileset().tile_set_modulate(tile_id, Color.green)

func _init_red(tile_id):
	self.get_tileset().tile_set_modulate(tile_id, Color.red)

func from_terrain(terrain):
	var tile_by_name = self.get_tileset().find_tile_by_name(terrain_enum[terrain.terrain_type])
	var tile_by_name_or_none = tile_by_name if tile_by_name >= 0 else self.get_tileset().find_tile_by_name("None")
	return TerrainTileId.new(tile_by_name_or_none, self._num_base_tiles)

func from_id(tile_id):
	return TerrainTileId.new(tile_id, self._num_base_tiles)
