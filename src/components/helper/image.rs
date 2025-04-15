use ratatui::{backend::Backend, layout::Rect, Frame, Terminal};
use ratatui_image::{
    picker::{Picker, ProtocolType},
    protocol::StatefulProtocol,
    Image, StatefulImage,
};

pub fn view(frame: &mut Frame, path: String, area: Rect) {
    let mut picker = Picker::from_query_stdio().unwrap();
    // let mut picker = Picker::from_fontsize((10, 10));
    // picker.set_protocol_type(ProtocolType::Sixel);

    let dyn_image = image::ImageReader::open(path)
        .unwrap()
        .decode()
        .expect("dont know");

    let mut image = picker.new_resize_protocol(dyn_image);

    frame.render_stateful_widget(StatefulImage::default(), area, &mut image);
}
