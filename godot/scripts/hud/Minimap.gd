extends Sprite

var HexGrid = preload("../../scripts/hex/HexGrid.gd").new()

export var margin = Vector2(10, 10)
export var minimap_scale = Vector2(0.33, 0.33)
onready var minimap_size = Vector2(get_node("/root/Main").configuration.columns, get_node("/root/Main").configuration.rows)
onready var minimap_display_size = self.minimap_scale * self.minimap_size

var terrain_enum = Terrain.terrain_enum()

func _ready():
	call_deferred("update_minimap")

func _input(event):
	if event is InputEventMouseButton and event.position.x < self.minimap_display_size.x and event.position.y < self.minimap_display_size.y:
		var click_pos = get_local_mouse_position()
		var offset = (click_pos / (self.minimap_display_size / 2)) * (self.minimap_size/2)
		var hex = HexGrid.get_hex_at(Vector3.ZERO)
		hex.offset_coords = offset
		var pos = HexGrid.get_hex_center(hex)
		get_node("/root/Main/WorldCamera").move_to(pos)

func update_minimap():
	var img = Image.new()
	img.create(self.minimap_size.x, self.minimap_size.y, false, Image.FORMAT_RGBA8)
	img.lock()
	var minimap = Terrain.minimap(int(self.minimap_size.x), int(self.minimap_size.y))
	for y in range(0, minimap_size.y):
		for x in range(0, minimap_size.x):
			img.set_pixel(x, y, self._find_tile_id(minimap[y * minimap_size.x + x]))
	img.unlock()
	img.resize(self.minimap_display_size.x, self.minimap_display_size.y, Image.INTERPOLATE_NEAREST)
	var tex = ImageTexture.new()
	tex.create_from_image(img)
	self.texture = tex
	self.position += self.margin + self.minimap_display_size / 2

func _find_tile_id(type):
	match type:
		terrain_enum.Bare:
			return Color.orange
		terrain_enum.Grassland:
			return Color.green
		terrain_enum.Ice:
			return Color.aquamarine
		terrain_enum.Marsh:
			return Color.greenyellow
		terrain_enum.Ocean:
			return Color.blue
		terrain_enum.Scorched:
			return Color.gray
		terrain_enum.Shrubland:
			return Color.darkgreen
		terrain_enum.Snow:
			return Color.snow
		terrain_enum.SubtropicalDesert:
			return Color.orangered
		terrain_enum.Taiga:
			return Color.lightseagreen
		terrain_enum.TemperateDeciduousForest:
			return Color.darkolivegreen
		terrain_enum.TemperateDesert:
			return Color.orangered
		terrain_enum.TemperateRainForest:
			return Color.darkolivegreen
		terrain_enum.TropicalRainForest:
			return Color.darkolivegreen
		terrain_enum.TropicalSeasonalForest:
			return Color.darkolivegreen
		terrain_enum.Tundra:
			return Color.greenyellow
		terrain_enum.TundraMarsh:
			return Color.olive
		terrain_enum.FreshWater:
			return Color.aqua
	return Color.transparent

var _camera_debounced = false
func _on_WorldCamera_change_debounced(rect: Rect2):
	var max_size = HexGrid.get_hex_at(Vector3(0, 0, 0))
	max_size.offset_coords = self.minimap_size/2
	var max_pixel = 2 * HexGrid.get_hex_center(max_size)
	var cursor_size = (rect.size / max_pixel) * self.minimap_display_size
	var cursor_pos = (rect.position / max_pixel) * self.minimap_display_size
	cursor_pos -= cursor_size / 2
	$Cursor.rect_position = cursor_pos
	$Cursor.rect_size = cursor_size
	_camera_debounced = false

func _on_WorldCamera_change(rect):
	if _camera_debounced == false:
		_camera_debounced = true
		call_deferred("_on_WorldCamera_change_debounced", rect)

