extends Resource

enum {
	highlighted_tile_offset = 1
	green_tile_offset
	red_tile_offset
	territory_id_tile_offset
}

var _tile_id: int
var _num_base_tiles: int

func _init(tile_id: int, num_base_tiles: int):
	self._num_base_tiles = num_base_tiles
	self._tile_id = _base_tile_id(tile_id, num_base_tiles)

static func _base_tile_id(tile_id, num_base_tiles):
	return tile_id % num_base_tiles

static func _offset_tile_id(offset, tile_id, num_base_tiles):
	return offset * num_base_tiles + _base_tile_id(tile_id, num_base_tiles)

func id():
	return self._tile_id

static func highlighted_from(tile_id, num_base_tiles):
	return _offset_tile_id(highlighted_tile_offset, tile_id, num_base_tiles)
	
func highlighted():
	return highlighted_from(self._tile_id, self._num_base_tiles)

static func green_from(tile_id, num_base_tiles):
	return _offset_tile_id(green_tile_offset, tile_id, num_base_tiles)

func green():
	return green_from(self._tile_id, self._num_base_tiles)

static func red_from(tile_id, num_base_tiles):
	return _offset_tile_id(red_tile_offset, tile_id, num_base_tiles)

func red():
	return red_from(self._tile_id, self._num_base_tiles)

static func territory_from(territory_id, tile_id, num_base_tiles):
	return _offset_tile_id(territory_id_tile_offset + territory_id * num_base_tiles, tile_id, num_base_tiles)

func territory(territory_id):
	return territory_from(territory_id, self._tile_id, self._num_base_tiles)
