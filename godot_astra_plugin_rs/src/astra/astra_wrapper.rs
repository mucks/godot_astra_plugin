use super::astra_bindings::*;
use std::ffi::CString;

pub unsafe fn init_sensor() -> *mut _astra_reader {
    astra_initialize();
    let mut sensor = Box::into_raw(Box::new(_astra_streamsetconnection { _unused: [] }))
        as *mut _astra_streamsetconnection;
    let path = CString::new("device/default").unwrap();
    astra_streamset_open(path.as_ptr(), &mut sensor as *mut _);

    let mut reader = Box::into_raw(Box::new(_astra_reader::default())) as *mut _astra_reader;
    astra_reader_create(sensor, &mut reader as *mut _);

    reader
}

pub unsafe fn start_masked_color_stream(reader: astra_reader_t) -> astra_streamconnection_t {
    let mut stream =
        Box::into_raw(Box::new(_astra_streamconnection::default())) as astra_streamconnection_t;
    astra_reader_get_maskedcolorstream(reader, &mut stream);
    astra_stream_start(stream);
    stream
}

pub unsafe fn start_body_stream(reader: astra_reader_t) -> *mut _astra_streamconnection {
    let mut stream =
        Box::into_raw(Box::new(_astra_streamconnection::default())) as *mut _astra_streamconnection;
    astra_reader_get_bodystream(reader, &mut stream as *mut _);
    astra_stream_start(stream);
    stream
}

pub unsafe fn start_color_stream(reader: astra_reader_t) -> *mut _astra_streamconnection {
    let mut stream =
        Box::into_raw(Box::new(_astra_streamconnection::default())) as *mut _astra_streamconnection;
    astra_reader_get_colorstream(reader, &mut stream as *mut _);
    astra_stream_start(stream);
    stream
}

pub unsafe fn update() {
    astra_update();
}

pub unsafe fn close_frame(frame: *mut *mut _astra_reader_frame) {
    astra_reader_close_frame(frame);
}

pub unsafe fn get_frame(reader: astra_reader_t) -> Option<astra_reader_frame_t> {
    let mut frame = Box::into_raw(Box::new(_astra_reader_frame::default())) as astra_reader_frame_t;
    let rc = astra_reader_open_frame(reader, 0, &mut frame);

    if rc == astra_status_t_ASTRA_STATUS_SUCCESS {
        Some(frame)
    } else {
        None
    }
}

pub unsafe fn get_body_frame(frame: astra_reader_frame_t) -> astra_bodyframe_t {
    let mut body_frame = Box::into_raw(Box::new(_astra_bodyframe::default())) as astra_bodyframe_t;
    astra_frame_get_bodyframe(frame, &mut body_frame);
    body_frame
}

pub unsafe fn get_color_frame(frame: astra_reader_frame_t) -> astra_colorframe_t {
    let mut color_frame =
        Box::into_raw(Box::new(_astra_imageframe::default())) as astra_colorframe_t;
    astra_frame_get_colorframe(frame, &mut color_frame);
    color_frame
}

pub unsafe fn get_masked_color_frame(frame: astra_reader_frame_t) -> astra_maskedcolorframe_t {
    let mut masked_color_frame =
        Box::into_raw(Box::new(_astra_imageframe::default())) as astra_colorframe_t;
    astra_frame_get_maskedcolorframe(frame, &mut masked_color_frame);
    masked_color_frame
}

pub unsafe fn get_masked_color_frame_index(masked_color_frame: astra_maskedcolorframe_t) -> i32 {
    let mut frame_index = 0_i32;
    astra_maskedcolorframe_get_frameindex(masked_color_frame, &mut frame_index);
    frame_index
}

pub unsafe fn get_body_frame_index(body_frame: *mut _astra_bodyframe) -> i32 {
    let mut frame_index = 0_i32;
    astra_bodyframe_get_frameindex(body_frame, &mut frame_index);
    frame_index
}

pub unsafe fn get_color_frame_index(color_frame: astra_colorframe_t) -> i32 {
    let mut frame_index = 0_i32;
    astra_colorframe_get_frameindex(color_frame, &mut frame_index);
    frame_index
}

pub unsafe fn get_color_bytes(color_frame: astra_colorframe_t) -> (u32, u32, Vec<u8>) {
    let mut metadata = astra_image_metadata_t::default();
    astra_colorframe_get_metadata(color_frame, &mut metadata);
    let mut byte_length = 0;
    astra_colorframe_get_data_byte_length(color_frame, &mut byte_length);

    let mut data: Vec<u8> = Vec::new();
    data.resize(byte_length as usize, 0);

    astra_colorframe_copy_data(color_frame, data.as_mut_ptr());
    (metadata.width, metadata.height, data)
}

pub unsafe fn get_body_list(body_frame: astra_bodyframe_t) -> _astra_body_list {
    let mut body_list = _astra_body_list::default();
    astra_bodyframe_body_list(body_frame, &mut body_list);
    body_list
}

pub unsafe fn terminate() {
    astra_terminate();
}
