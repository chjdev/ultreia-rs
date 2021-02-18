extends Camera2D

signal change

export var move_speed = 30
export var max_zoom = 1.5
export var min_zoom = 0.33

# Called when the node enters the scene tree for the first time.
func _ready():
	_emit_change()

func _unhandled_input(event):
	if event is InputEventMagnifyGesture:
		zoom_at_point(event.factor, event.position)
		_emit_change()
	if event is InputEventPanGesture:
		move_by(event.delta)
		_emit_change()

func get_rect():
	return Rect2(self.get_camera_screen_center(), self.get_viewport_rect().size * self.zoom)

func _emit_change():
	emit_signal("change", get_rect())

func move_by(delta):
	self.translate(delta * move_speed * zoom)

func move_to(pos):
	self.position = pos
	_emit_change()

# https://godotengine.org/qa/25983/camera2d-zoom-position-towards-the-mouse
func zoom_at_point(zoom_change, point):
	var c0 = global_position # camera position
	var v0 = get_viewport().size # vieport size
	var c1 # next camera position
	var z0 = zoom # current zoom value
	var z1 = (z0 * (1 / zoom_change))# next zoom value
	z1 = Vector2(clamp(z1.x, min_zoom, max_zoom), clamp(z1.y, min_zoom, max_zoom))

	c1 = c0 + (-0.5*v0 + point)*(z0 - z1)
	zoom = z1
	global_position = c1
