extends Spatial
var joint_scene = preload("res://addons/godot_astra_plugin/scenes/Joint.tscn")

var joint_objects: Dictionary

func _ready():
    pass # Replace with function body.

func get_bodies():
    var bodies = get_node("../AstraController").get_bodies()
    for body in bodies:
        for joint_type in body.joints:
            var joint = body.joints[joint_type]

            if joint.status == 2:
                var world_pos = joint.world_position
                var normalized_world_pos = world_pos / 150
                normalized_world_pos.x *= -1

                if not joint_objects.has(joint_type):
                    joint_objects[joint_type] = joint_scene.instance()
                    joint_objects[joint_type].translation = normalized_world_pos
                    add_child(joint_objects[joint_type])
                else:
                    joint_objects[joint_type].translation = normalized_world_pos
            else:
                if joint_objects.has(joint_type):
                    remove_child(joint_objects[joint_type])
                    joint_objects.erase(joint_type)



#average 50usecs
func _process(delta):
    get_bodies()
