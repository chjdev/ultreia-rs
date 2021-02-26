extends TileMap

var HexGrid = preload("../hex/HexGrid.gd").new()

func _ready():
	Buildings.connect("Created", self, "_on_building_created")
	Buildings.connect("Destroyed", self, "_on_building_destroyed")


# todo maybe it's own node?
func set_building_cell(i, j, building):
	self.set_cell(i, j, self.tile_set.find_tile_by_name(building))


func _on_building_created(coordinate, tile_name):
	var hex = HexGrid.from_cube_coords(coordinate)
	var offset = hex.offset_coords
	self.set_building_cell(offset.x, offset.y, tile_name)


func _on_building_destroyed(coordinate):
	var hex = HexGrid.from_cube_coords(coordinate)
	var offset = hex.offset_coords
	self.set_cell(offset.x, offset.y, "")
