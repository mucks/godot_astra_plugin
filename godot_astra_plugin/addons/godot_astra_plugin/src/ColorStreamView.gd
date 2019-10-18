extends Sprite

func _on_AstraController_new_color_byte_array(width, height, color_base64):
    var colors = Marshalls.base64_to_raw(color_base64)
    var img = Image.new()
    img.create_from_data(width, height, false, Image.FORMAT_RGB8, colors)

    var imageTexture = ImageTexture.new()
    imageTexture.create_from_image(img, 7)

    set_texture(imageTexture);  
    