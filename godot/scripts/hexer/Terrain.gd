extends TileMap

var TerrainTileFactory = preload("./TerrainTileFactory.gd")
var factory = TerrainTileFactory.new(self.get_tileset())

func _ready():
	pass

func set_terrain_cell(i, j, terrain):
	var tile_id = factory.from_terrain(terrain)
	self.set_cell(i, j, tile_id.id())

func focus(hex):
	var hex_coords = hex.offset_coords
	if self.get_cell(hex_coords.x, hex_coords.y) < 0:
		return
	var tile_id = factory.from_id(self.get_cell(hex_coords.x, hex_coords.y))
	self.set_cell(hex_coords.x, hex_coords.y, tile_id.highlighted())

func blur(hex):
	var hex_coords = hex.offset_coords
	if self.get_cell(hex_coords.x, hex_coords.y) < 0:
		return
	var tile_id = factory.from_id(self.get_cell(hex_coords.x, hex_coords.y))
	self.set_cell(hex_coords.x, hex_coords.y, tile_id.id())
