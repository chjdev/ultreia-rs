"""
	Copyright 2018 Mel Collins
	Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:
	The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.
	THE SOFTWARE IS PROVIDED 'AS IS', WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.

	A single cell of a hexagonal grid.
	
	There are many ways to orient a hex grid, this library was written
	with the following assumptions:
	
	* The hexes use a flat-topped orientation;
	* Axial coordinates use +x => NE; +y => N;
	* Offset coords have odd rows shifted up half a step.
	
	Using x,y instead of the reference's preferred x,z for axial coords makes
	following along with the reference a little more tricky, but is less confusing
	when using Godot's Vector2(x, y) objects.
	
	
	## Usage:
	
	#### var cube_coords; var axial_coords

		Cube coordinates are used internally as the canonical representation, but
		both axial and offset coordinates can be read and modified through these
		properties.

"""
extends Resource
#warning-ignore-all:unused_class_variable

# We use unit-size flat-topped hexes
const size = Vector2(1, sqrt(3)/2)

# Cube coords are canonical
var cube_coords = Vector3(0, 0, 0) setget set_cube_coords, get_cube_coords
var axial_coords setget set_axial_coords, get_axial_coords
# -1, -1
#   ...
#      1, 1
var offset_coords setget set_offset_coords, get_offset_coords

func _init(coords=null):
	# HexCells can be created with coordinates
	if coords:
		self.cube_coords = obj_to_coords(coords)

func new_hex(coords):
	# Returns a new HexCell instance
	return get_script().new(coords)

func copy():
	return new_hex(self.cube_coords)

"""
	Handle coordinate access and conversion
"""
func obj_to_coords(val):
	# Returns suitable cube coordinates for the given object
	# The given object can an be one of:
	# * Vector3 of standard cube coords;
	# * Vector2 of axial coords;
	# * HexCell instance
	# Any other type of value will return null
	#
	# NB that offset coords are NOT supported, as they are
	# indistinguishable from axial coords.
	
	if typeof(val) == TYPE_VECTOR3:
		return val
	elif typeof(val) == TYPE_VECTOR2:
		return _axial_to_cube_coords(val)
	elif typeof(val) == TYPE_OBJECT and val.has_method("get_cube_coords"):
		return val.get_cube_coords()
	# Fall through to nothing
	return

func _axial_to_cube_coords(val):
	# Returns the Vector3 cube coordinates for an axial Vector2
	var x = val.x
	var y = val.y
	return Vector3(x, y, -x - y)

func get_cube_coords():
	# Returns a Vector3 of the cube coordinates
	return cube_coords
	
func set_cube_coords(val):
	# Sets the position from a Vector3 of cube coordinates
	if abs(val.x + val.y + val.z) > 0.0001:
		print("WARNING: Invalid cube coordinates for hex (x+y+z!=0): ", val)
		return
	cube_coords = _round_coords(val)

func _round_coords(val):
	# Rounds floaty coordinate to the nearest whole number cube coords
	if typeof(val) == TYPE_VECTOR2:
		val = _axial_to_cube_coords(val)
	
	# Straight round them
	var rounded = Vector3(round(val.x), round(val.y), round(val.z))
	
	# But recalculate the one with the largest diff so that x+y+z=0
	var diffs = (rounded - val).abs()
	if diffs.x > diffs.y and diffs.x > diffs.z:
		rounded.x = -rounded.y - rounded.z
	elif diffs.y > diffs.z:
		rounded.y = -rounded.x - rounded.z
	else:
		rounded.z = -rounded.x - rounded.y
	
	return rounded

func get_axial_coords():
	# Returns a Vector2 of the axial coordinates
	return Vector2(cube_coords.x, cube_coords.y)
	
func set_axial_coords(val):
	# Sets position from a Vector2 of axial coordinates
	self.set_cube_coords(_axial_to_cube_coords(val))

func get_offset_coords():
	# Returns a Vector2 of the offset coordinates
	var x = int(cube_coords.x)
	var z = int(cube_coords.z)
	var col = x
	var row = z + (x + (x&1)) / 2
	return Vector2(col, row)

func set_offset_coords(val):
	# Sets position from a Vector2 of offset coordinates
	var col = int(val.x)
	var row = int(val.y)
	var x = col
	var z = row - (col + (col&1)) / 2
	var y = -x-z
	self.set_cube_coords(Vector3(x, y, z))
