extends PopupPanel

func _on_Cursor_hex_pressed(hex):
	var anchor = get_global_mouse_position() / get_viewport_rect().size
	self.anchor_left = anchor.x
	self.anchor_top = anchor.y
	self.anchor_right = anchor.x
	self.anchor_bottom = anchor.y
	$InfoContainer/CoordinatesGroup/Coordinates.text = str(hex.cube_coords)
	self.popup()
	# todo: this is just to trigger a render pass to resize the layout containers
	# remove when size is more robust
	self.popup()
