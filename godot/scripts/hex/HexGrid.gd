"""
	Copyright 2018 Mel Collins
	Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
	The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
	THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

	A converter between hex and Godot-space coordinate systems.
	
	The hex grid uses +x => NE and +y => N, whereas
	the projection to Godot-space uses +x => E, +y => S.
	
	We map hex coordinates to Godot-space with +y flipped to be the down vector
	so that it maps neatly to both Godot's 2D coordinate system, and also to
	x,z planes in 3D space.
	
	
	## Usage:
	
	#### var hex_scale = Vector2(...)

		If you want your hexes to display larger than the default 1 x 0.866 units,
		then you can customise the scale of the hexes using this property.
	
	#### func get_hex_center(hex)
	
		Returns the Godot-space Vector2 of the center of the given hex.
		
		The coordinates can be given as either a HexCell instance; a Vector3 cube
		coordinate, or a Vector2 axial coordinate.
	
	#### func get_hex_at(coords)
	
		Returns HexCell whose grid position contains the given Godot-space coordinates.
		
		The given value can either be a Vector2 on the grid's plane
		or a Vector3, in which case its (x, z) coordinates will be used.
	
"""
extends Reference

var HexCell = preload("./HexCell.gd")

# Allow the user to scale the hex for fake perspective or somesuch
#export(Vector2) var hex_scale = Vector2(1, 1) setget set_hex_scale
export(Vector2) var hex_scale = Vector2(32, 32) setget set_hex_scale

var base_hex_size = Vector2(1, sqrt(3)/2)
var hex_size
var hex_transform
var hex_transform_inv

func _init():
	set_hex_scale(hex_scale)

func set_hex_scale(scale):
	# We need to recalculate some stuff when projection scale changes
	hex_scale = scale
	# round so we actually get pixels
	hex_size = (base_hex_size * hex_scale).round()
	hex_transform = Transform2D(
		Vector2(hex_size.x * 3/4, -hex_size.y / 2),
		Vector2(0, -hex_size.y),
		Vector2(0, 0)
	)
	hex_transform_inv = hex_transform.affine_inverse()
	

func get_zero_hex():
	return HexCell.new(Vector3(0, 0, 0))

"""
	Converting between hex-grid and 2D spatial coordinates
"""
func get_hex_center(hex):
	# Returns hex's centre position on the projection plane
	hex = HexCell.new(hex)
	return hex_transform * hex.axial_coords
	
func get_hex_at(coords):
	# Returns a HexCell at the given Vector2/3 on the projection plane
	# If the given value is a Vector3, its x,z coords will be used
	if typeof(coords) == TYPE_VECTOR3:
		coords = Vector2(coords.x, coords.z)
	return HexCell.new(hex_transform_inv * coords)
