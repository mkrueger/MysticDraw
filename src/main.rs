//! This example showcases an interactive `Canvas` for drawing Bézier curves.
use iced::widget::{button, column, text};
use iced::{Alignment, Element, Length, Sandbox, Settings};
use std::collections::HashMap;

use std::path::Path;
mod model;
use model::{TOOLS, Size, Tool, Editor};

pub fn main() -> iced::Result {
    Example::run(Settings {
        antialiasing: true,
        ..Settings::default()
    })
}

struct Example {
    bezier: bezier::State
}

#[derive(Debug, Clone, Copy)]
enum Message {
    AddCurve(bezier::Curve),
    Clear,
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {
        Example { bezier: bezier::State::new() }
    }

    fn title(&self) -> String {
        String::from("Mystic Draw - iCED")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::AddCurve(curve) => {
                self.bezier.request_redraw();
            }
            Message::Clear => {
                self.bezier = bezier::State::new();
            }
        }
    }

    fn view(&self) -> Element<Message> {
        column![
            self.bezier.view(&self.bezier).map(Message::AddCurve),
        ]
        .align_items(Alignment::Center)
        .into()
    }
}

mod bezier {
    use std::collections::HashMap;

    use iced::mouse;
    use iced::widget::canvas::event::{self, Event};
    use iced::widget::canvas::{
        self, Canvas, Cursor, Frame, Geometry, Path, Stroke,
    };
    use iced::{Element, Length, Point, Rectangle, Theme};
    use iced_native::Widget;
    use iced_native::image::Handle;

    use crate::model::Editor;
    pub struct State {
        cache: canvas::Cache,
        pub editor: Editor,
        pub chars: Vec<Vec<u8>>,
        pub hash : HashMap<(u16, u8), Handle>
    }
    
    impl State {
        pub fn new() -> Self {
            let buffer = crate::model::Buffer::load_buffer(std::path::Path::new("/home/mkrueger/Downloads/2m-blockfury_spaceinvaders2015.xb")).unwrap();
            let editor = Editor::new(0, buffer);
    
            let mut chars = Vec::new();
            let font_dimensions = editor.buf.get_font_dimensions();
            for color in 0..16 {
                let fg = editor.buf.palette.colors[color];
                for ch in 0..=255 {
                    let mut result = vec![0; font_dimensions.width as usize * font_dimensions.height as usize * 4];
                    let mut i = 0;
                    for y in 0..font_dimensions.height {
                        let line = editor.buf.get_font_scanline(ch, y as usize);
                        for x in 0..font_dimensions.width {
                            if (line & (128 >> x)) != 0 {
                                result[i] = 255;
                                i += 1;
                                result[i] = 255;
                                i += 1;
                                result[i] = 255;
                                i += 1;
                                result[i] = 255;
                                i += 1;
                            } else {
                                result[i] = 0;
                                i += 1;
                                result[i] = 0;
                                i += 1;
                                result[i] = 0;
                                i += 1;
                                result[i] = 0;
                                i += 1;
                            }
                        }
                    }
                    chars.push(result);    
                }
            }
            State {
                cache: canvas::Cache::default(),
                editor,
                chars,
                hash: HashMap::new()
            }

        }
        pub fn view<'a>(&'a self, state: &State) -> Element<'a, Curve> {

            let c = Canvas::new(Bezier {
                state: self
            })
                .width(Length::Units(state.editor.buf.width * 8))
                .height(Length::Units(state.editor.buf.height * 16));


            let scrollable = iced::widget::scrollable(c)
                .height(Length::Fill);

            scrollable.into()
        }

        pub fn request_redraw(&mut self) {
            self.cache.clear()
        }
    }

    struct Bezier<'a> {
        state: &'a State,
    }

    impl<'a> canvas::Program<Curve> for Bezier<'a> {
        type State = Option<Pending>;

        fn update(
            &self,
            state: &mut Self::State,
            event: Event,
            bounds: Rectangle,
            cursor: Cursor,
        ) -> (event::Status, Option<Curve>) {
            let cursor_position =
                if let Some(position) = cursor.position_in(&bounds) {
                    position
                } else {
                    return (event::Status::Ignored, None);
                };

            match event {
                Event::Mouse(mouse_event) => {
                    let message = match mouse_event {
                        mouse::Event::ButtonPressed(mouse::Button::Left) => {
                            match *state {
                                None => {
                                    *state = Some(Pending::One {
                                        from: cursor_position,
                                    });

                                    None
                                }
                                Some(Pending::One { from }) => {
                                    *state = Some(Pending::Two {
                                        from,
                                        to: cursor_position,
                                    });

                                    None
                                }
                                Some(Pending::Two { from, to }) => {
                                    *state = None;

                                    Some(Curve {
                                        from,
                                        to,
                                        control: cursor_position,
                                    })
                                }
                            }
                        }
                        _ => None,
                    };

                    (event::Status::Ignored, None)
                }
                _ => (event::Status::Ignored, None),
            }
        }

        fn draw(
            &self,
            state: &Self::State,
            _theme: &Theme,
            bounds: Rectangle,
            cursor: Cursor,
        ) -> Vec<Geometry> {
            let content =
            self.state.cache.draw(bounds.size(), |frame: &mut Frame| {
                let buffer = &self.state.editor.buf;
                let font_dimensions = buffer.get_font_dimensions();

                let x1 = (bounds.x as usize) / font_dimensions.width as usize;
                let x2 = ((bounds.x + bounds.width) as usize) / font_dimensions.width as usize + 1;
                let y1 = (bounds.y as usize) / font_dimensions.height as usize;
                let y2 = ((bounds.y + bounds.height) as usize) / font_dimensions.height as usize + 1;
                
                for y in y1..=y2 {
                    for x in x1..=x2 {
                        let rect  = Rectangle::new(
                            Point::new((x * font_dimensions.width as usize) as f32 + 0.5,  
                            (y * font_dimensions.height as usize) as f32 + 0.5), 
                            iced::Size::new(
                            ((x + 1) * font_dimensions.width as usize) as f32 + 0.5, 
                            ((y + 1) * font_dimensions.height as usize) as f32 + 0.5));
                            if let Some(ch) = buffer.get_char(crate::model::Position::from(x as i32, y as i32)) {
                                let bg = buffer.palette.colors[ch.attribute.get_background() as usize];
                                let (r, g, b) = bg.get_rgb_f32();

                                let color = iced::Color::new(r, g, b, 1.0);
                                frame.fill_rectangle(rect.position(), rect.size(), color);

                                let fg = buffer.palette.colors[ch.attribute.get_foreground() as usize];
                                let (r, g, b) = fg.get_rgb_f32();
                                let color = iced::Color::new(r, g, b, 1.0);
                                for y in 0..font_dimensions.height {
                                    let line = buffer.get_font_scanline(ch.char_code, y as usize);
                                    for x in 0..font_dimensions.width {
                                        if (line & (128 >> x)) != 0 {

                                            frame.fill_rectangle(Point::new(rect.x + x as f32, rect.y + y as f32), iced::Size::new(1_f32, 1_f32), color);

                                        //    let path = canvas::Path::rectangle(Point::new(rect.x + x as f32 + 0.5, rect.y + y as f32 + 0.5), iced::Size::new(1_f32, 1_f32));
                                          //  frame.stroke(&path, canvas::Stroke::default().with_line_cap(canvas::LineCap::Square).with_line_join(canvas::LineJoin::Miter).with_color(iced::Color::new(r, g, b, 1.0)));
                                        }
                                    }
                                }
                            }
                    }
                }
                });

            if let Some(pending) = state {
                let pending_curve = pending.draw(bounds, cursor);
                vec![content, pending_curve]
            } else {
                vec![content]
            }
        }

        fn mouse_interaction(
            &self,
            _state: &Self::State,
            bounds: Rectangle,
            cursor: Cursor,
        ) -> mouse::Interaction {
            if cursor.is_over(&bounds) {
                mouse::Interaction::Crosshair
            } else {
                mouse::Interaction::default()
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    pub struct Curve {
        from: Point,
        to: Point,
        control: Point,
    }

    impl Curve {
        fn draw_all( frame: &mut Frame) {/*
            let curves = Path::new(|p| {
                for curve in curves {
                    p.move_to(curve.from);
                    p.quadratic_curve_to(curve.control, curve.to);
                }
            });

            frame.stroke(&curves, Stroke::default().with_width(2.0));*/
        }
    }

    #[derive(Debug, Clone, Copy)]
    enum Pending {
        One { from: Point },
        Two { from: Point, to: Point },
    }

    impl Pending {
        fn draw(&self, bounds: Rectangle, cursor: Cursor) -> Geometry {
            let mut frame = Frame::new(bounds.size());

            if let Some(cursor_position) = cursor.position_in(&bounds) {
                match *self {
                    Pending::One { from } => {
                        let line = Path::line(from, cursor_position);
                        frame.stroke(&line, Stroke::default().with_width(2.0));
                    }
                    Pending::Two { from, to } => {
                        let curve = Curve {
                            from,
                            to,
                            control: cursor_position,
                        };

                        Curve::draw_all( &mut frame);
                    }
                };
            }

            frame.into_geometry()
        }
    }
}

/*

mod circle {
    use iced_native::layout::{self, Layout};
    use iced_native::{renderer, image};
    use iced_native::{Color, Element, Length, Point, Rectangle, Size, Widget};
    use crate::model::Editor;
    use iced_native::image::Handle;
    pub struct Circle {
        editor: Editor,
        chars: Vec<Vec<u8>>,
        hash : HashMap<(u16, u8), Handle>
    }

    impl Circle {
        pub fn new(editor: Editor) -> Self {
            

            Self {             
                editor, chars, hash: HashMap::new() }
            }
    }

    impl<Message, Renderer> Widget<Message, Renderer> for Circle
    where
        Renderer: renderer::Renderer,
    {
        fn width(&self) -> Length {
            Length::Shrink
        }

        fn height(&self) -> Length {
            Length::Shrink
        }

        fn layout(
            &self,
            _renderer: &Renderer,
            _limits: &layout::Limits,
        ) -> layout::Node {
            layout::Node::new(Size::new(self.editor.buf.width as f32 * 16.0, self.editor.buf.height as f32 * 8.0))
        }

        fn draw(
            &self,
            _state: &iced_native::widget::Tree,
            renderer: &mut Renderer,
            _theme: &Renderer::Theme,
            _style: &renderer::Style,
            layout: Layout<'_>,
            _cursor_position: Point,
            _viewport: &Rectangle,
        ) {
            
        }
    }

    impl<'a, Message, Renderer> Into<Element<'a, Message, Renderer>> for Circle
    where
        Renderer: renderer::Renderer
    {
        fn into(self) -> Element<'a, Message, Renderer> {
            Element::new(self)
        }
    }
}

pub fn main() -> iced::Result {
    Example::run(Settings::default())
}

struct Example {
    radius: f32,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    RadiusChanged(f32),
}

impl Sandbox for Example {
    type Message = Message;

    fn new() -> Self {

        Example {
            radius: 50.0
        }
    }

    fn title(&self) -> String {
        String::from("Custom widget - Iced")
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::RadiusChanged(radius) => {
                self.radius = radius;
            }
        }
    }

    fn view(&self) -> Element<Message> {
        println!("foo!!!");
        let buffer = model::Buffer::load_buffer(Path::new("/home/mkrueger/Dokumente/SAC0696A/ROY-COMI.ANS")).unwrap();

        let content = iced::widget::Column::new()
            .padding(20)
            .spacing(20)
            .max_width(500)
            .align_items(Alignment::Center)
            .push(Circle::new(Editor::new(0, buffer)))
          ;

        iced::widget::Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x()
            .center_y()
            .into()
    }
}
*/
pub struct AnsiSettings {
    font_path: Option<std::path::PathBuf>,
    console_font_path: Option<std::path::PathBuf>,
    tab_size: i32,
    outline_font_style: usize
}

#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Grid {
    Off,
    Raster4x2,
    Raster6x3,
    Raster8x4,
    Raster12x6,
    Raster16x8
}


#[repr(u32)]
#[derive(Copy, Clone, Debug)]
pub enum Guide {
    Off,
    Guide80x25,
    Guide80x40,
    Guide80x50,
    Guide44x22
}

pub struct Workspace {
    pub settings: AnsiSettings,
    selected_tool: usize,

    pub show_fg_color: bool,
    pub show_bg_color: bool,

    pub grid: Grid,
    pub guide: Guide
}

impl Workspace {
    pub fn cur_tool(&self) -> std::boxed::Box<&&'static mut dyn Tool> {
        unsafe {
            let t = &TOOLS[self.selected_tool];
            std::boxed::Box::new(t)
        }
    }

    pub fn get_grid_size(&self) -> Option<Size<u8>> {
        match self.grid {
            Grid::Off => None,
            Grid::Raster4x2 => Some(Size::from(4, 2)),
            Grid::Raster6x3 => Some(Size::from(6, 3)),
            Grid::Raster8x4 => Some(Size::from(8, 4)),
            Grid::Raster12x6 => Some(Size::from(12, 6)),
            Grid::Raster16x8 => Some(Size::from(16, 8)),
        }
    }

    pub fn get_guide_size(&self) -> Option<Size<u8>> {
        match self.guide {
            Guide::Off => None,
            Guide::Guide80x25 => Some(Size::from(80, 25)),
            Guide::Guide80x40 => Some(Size::from(80, 40)),
            Guide::Guide80x50 => Some(Size::from(80, 50)),
            Guide::Guide44x22 => Some(Size::from(44, 22)),
        }
    }
}

pub static mut WORKSPACE: Workspace = Workspace {
    settings: AnsiSettings { tab_size: 8, font_path: None, console_font_path: None, outline_font_style: 0},
    selected_tool: 0,
    show_fg_color: true,
    show_bg_color: true,
    grid: Grid::Off,
    guide: Guide::Off
};

const RESOURCES_BYTES:&[u8] = include_bytes!("../data/resources.gresource");
/* 
pub fn main() {

    if let Some(proj_dirs) = ProjectDirs::from("github.com", "mkrueger",  "Mystic Draw") {
        unsafe {
            WORKSPACE.settings.font_path = Some(proj_dirs.data_dir().to_path_buf().join("fonts/tdf"));
            WORKSPACE.settings.console_font_path = Some(proj_dirs.data_dir().to_path_buf().join("fonts/console"));

            if let Some(p) = &WORKSPACE.settings.font_path {
                fs::create_dir_all(p).expect("can't create tdf font path");
            }
            if let Some(p) = &WORKSPACE.settings.console_font_path {
                fs::create_dir_all(p).expect("can't create console font path");
            }
        }
    }
    init_tools();
    Counter::run(Settings::default());
}

#[cfg(test)]
#[allow(dead_code)]
mod tests {
    use crate::model::{Buffer, self, SaveOptions};

    fn is_hidden(entry: &walkdir::DirEntry) -> bool {
        entry.file_name()
             .to_str()
             .map_or(false, |s| s.starts_with('.'))
    }
    
    fn comp(buf1: &Buffer, buf2: &Buffer) {
        assert_eq!(buf1.width, buf2.width);
        //assert_eq!(buf1.height, buf2.height);
    
        assert_eq!(buf1.title, buf2.title);
        assert_eq!(buf1.group, buf2.group);
        assert_eq!(buf1.author, buf2.author);
        assert_eq!(buf1.comments, buf2.comments);
        
        assert_eq!(buf1.palette.colors[0..16], buf2.palette.colors[0..16]);
    
        for y in 0..buf1.height {
            for x in 0..buf1.width {
                let pos = model::Position::from(x as i32, y as i32);
                let ch1 = buf1.get_char(pos);
                let ch2 = buf2.get_char(pos);
                if ch1.is_none() && ch2.is_none() { continue; }
                let ch1 = ch1.unwrap_or_default();
                let ch2 = ch2.unwrap_or_default();
    
                if ch1.is_transparent() && ch2.is_transparent() { continue; }
                if (ch1.char_code == b' ' as u16 || ch1.char_code == 0) && (ch2.char_code== b' ' as u16 || ch2.char_code== 0) && ch2.attribute.get_background() == ch2.attribute.get_background() { continue; }
                assert_eq!(ch1, ch2);
            }
        }
    }
    
    // #[test]
    fn test_clear() {
        let walker = walkdir::WalkDir::new("/home/mkrueger/Dokumente/AnsiArt").into_iter();
        
        for entry in walker.filter_entry(|e| !is_hidden(e)) {
            let entry = entry.unwrap();
            let path = entry.path();

            if path.is_dir() {
                continue;
            }
            let extension = path.extension();
            if extension.is_none() { continue; }
            let extension = extension.unwrap().to_str();
            if extension.is_none() { continue; }
            let extension = extension.unwrap().to_lowercase();

            if extension == "ice" {
                let z = model::Buffer::load_buffer(path);
                if let Err(m) = z { 
                    eprintln!("Error loading file: {}", m);
                    continue;
                }
                let buf = z.unwrap();

                let mdf_bytes = model::convert_to_mdf(&buf).unwrap();
                let mut mdf_buffer = model::Buffer::new();
                model::read_mdf(&mut mdf_buffer, &mdf_bytes).unwrap();
                comp(&buf, &mdf_buffer);

                let adf_bytes = mdf_buffer.to_bytes(extension.as_str(), &SaveOptions::new()).unwrap();
                let buf2 = Buffer::from_bytes(&std::path::PathBuf::from(path), &adf_bytes).unwrap();
                comp(&buf, &buf2);
            }
        }
    }
}
*/