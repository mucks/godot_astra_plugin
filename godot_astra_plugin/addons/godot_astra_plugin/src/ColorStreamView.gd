extends Sprite

func _process(delta):
    var image = get_node("../../AstraController").get_color_image()
    var image_texture = ImageTexture.new()
    image_texture.create_from_image(image)
    set_texture(image_texture)
