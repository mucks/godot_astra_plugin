extends Spatial
var joint_scene = preload("res://addons/godot_astra_plugin/scenes/Joint.tscn")

var joint_objects: Dictionary

func _ready():
    pass # Replace with function body.

#average 50usecs
func _on_AstraController_new_body_list(body_list):
    for body in body_list:
        var joints = body["joints"]
        for joint_type in joints:
            var joint = joints[joint_type]
            var world_pos = joint["world_position"]
            var normalized_world_pos = world_pos / 150
            normalized_world_pos.x *= -1

            if not joint_objects.has(joint_type):
                joint_objects[joint_type] = joint_scene.instance()
                joint_objects[joint_type].translation = normalized_world_pos
                add_child(joint_objects[joint_type])
            else:
                joint_objects[joint_type].translation = normalized_world_pos

