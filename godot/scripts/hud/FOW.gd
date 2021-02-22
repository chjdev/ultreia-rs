extends Sprite

var HexGrid = preload("../hex/HexGrid.gd").new()
onready var minimap = get_parent()
var img
var max_pos

func _ready():
	FOW.connect("Uncover", self, "_on_uncover_all")

func update_fow():
	if img == null:
		img = Image.new()
		img.create(minimap.minimap_size.x, minimap.minimap_size.y, false, Image.FORMAT_RGBA8)
		img.lock()
		var fow = FOW.minimap(int(minimap.minimap_size.x), int(minimap.minimap_size.y))
		for y in range(0, minimap.minimap_size.y):
			for x in range(0, minimap.minimap_size.x):
				img.set_pixel(x, y, Color.transparent if fow[y * minimap.minimap_size.x + x] else Color.black)
		img.unlock()
	var tex = ImageTexture.new()
	tex.create_from_image(img)
	self.texture = tex

var _update_debounced = false
func _update_fow_debounced():
	update_fow()
	_update_debounced = false

func _update_fow():
	if _update_debounced == false:
		_update_debounced = true
		call_deferred("_update_fow_debounced")

func uncover_all(hexes):
	if img == null:
		return
	# not ready in _ready because it depends on nodes up in the tree
	if max_pos == null:
		var max_offset = minimap.minimap_size
		var max_hex = HexGrid.get_zero_hex()
		max_hex.offset_coords = Vector2(max_offset.x, max_offset.y)
		max_pos = HexGrid.get_hex_center(max_hex)
	img.lock()
	var should_update = false
	for hex in hexes:
		var pos = HexGrid.get_hex_center(HexGrid.from_cube_coords(hex))
		var normalized_pos = pos / max_pos
		var minimap_pos = normalized_pos * minimap.minimap_size + minimap.minimap_size / 2
		if img.get_pixelv(minimap_pos) != Color.transparent:
			img.set_pixelv(minimap_pos, Color.transparent)
			should_update = true
	self._update_fow()
	img.unlock()

func _on_uncover_all(coordinates):
	call_deferred("uncover_all", coordinates)

