extends Sprite

func _process(delta):
    var image = get_node("../../AstraController").get_depth_image()
    var imageTexture = ImageTexture.new()
    imageTexture.create_from_image(image)
    set_texture(imageTexture);  