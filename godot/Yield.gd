extends TileMap

var good_enum = Good.good_enum()

func _ready():
	pass

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
	var tile_id = self._find_tile_id(max_key)
	self.set_cell(i, j, tile_id)

func _find_tile_id(good):
	return self.tile_set.find_tile_by_name(good_enum[good])
