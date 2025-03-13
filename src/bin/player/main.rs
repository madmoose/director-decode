mod player;

use std::{
    fs::File,
    io::{Read, Seek},
    num::NonZeroU32,
    ops::DerefMut,
    rc::Rc,
};

use clap::Parser;

use anyhow::Result;
use director_decoder::{
    gfx,
    reader::Reader,
    riff::{Projector, RiffFile, tags},
};
use player::{DisplayList, Player};
use winit::{
    application::ApplicationHandler,
    event::WindowEvent,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
    window::{Window, WindowId},
};

#[derive(Debug, Parser)]
struct Cli {
    filename: String,
}

struct App<'a> {
    player: Player<'a>,

    window: Option<Rc<Window>>,
    surface: Option<softbuffer::Surface<Rc<Window>, Rc<Window>>>,

    display_list: DisplayList,
}

impl ApplicationHandler for App<'_> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window_attributes = Window::default_attributes()
            .with_title("madmoose's Assistant Director")
            .with_inner_size(self.player.default_window_size());

        let window = Rc::new(event_loop.create_window(window_attributes).unwrap());
        let context = softbuffer::Context::new(window.clone()).unwrap();
        let surface = softbuffer::Surface::new(&context, window.clone()).unwrap();

        self.window = Some(window);
        self.surface = Some(surface);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _id: WindowId, event: WindowEvent) {
        match event {
            WindowEvent::CloseRequested => {
                println!("The close button was pressed; stopping");
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                let (Some(window), Some(surface)) = (self.window.as_ref(), self.surface.as_mut())
                else {
                    return;
                };

                let size = window.inner_size();
                surface
                    .resize(
                        NonZeroU32::new(size.width).unwrap(),
                        NonZeroU32::new(size.height).unwrap(),
                    )
                    .unwrap();

                if self.player.time_for_new_frame() {
                    self.display_list = self.player.step_frame();
                }

                let mut buffer = surface.buffer_mut().unwrap();
                {
                    let buffer_bytes = buffer.deref_mut();
                    let window_size = window.inner_size();
                    for item in &self.display_list {
                        match item {
                            player::DisplayObject::Bitmap {
                                id,
                                rect,
                                image,
                                draw_mode,
                            } => {
                                let _ = id;
                                let destination = &mut gfx::ImageBuffer::<&mut [u32]>::new(
                                    window_size.width as usize,
                                    window_size.height as usize,
                                    buffer_bytes,
                                );
                                let destination_rect = rect.scale(2.0);
                                let source = image.as_ref();
                                let source_rect = gfx::Rect {
                                    y0: 0,
                                    x0: 0,
                                    y1: source.height() as i16,
                                    x1: source.width() as i16,
                                };
                                let palette = &self.player.palette;

                                let transparent_color_index = match draw_mode {
                                    player::DrawMode::Copy => None,
                                    player::DrawMode::TransparentColorIndex(index) => Some(*index),
                                };

                                // source
                                //     .save_to_png(palette, &format!("out/{}.png", id.id()))
                                //     .unwrap();

                                gfx::blit(
                                    destination,
                                    destination_rect,
                                    source,
                                    source_rect,
                                    palette,
                                    transparent_color_index,
                                );
                            }
                        };
                    }
                }

                buffer.present().unwrap();

                self.window.as_ref().unwrap().request_redraw();
            }
            _ => (),
        }
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let filename = std::path::Path::new(&cli.filename);

    let mut buf = Vec::new();
    let mut file = File::open(filename)?;
    file.read_to_end(&mut buf)?;
    let mut reader = Reader::new(&buf);

    let mut riff = if filename
        .extension()
        .is_some_and(|ext| ext.eq_ignore_ascii_case("exe"))
    {
        let mut projector = Projector::read(reader.clone())?;

        projector.display_header();

        let mut riff = projector.read_initial_riff()?;

        if riff.type_tag() == tags::TAG_APPL {
            if let Some(file) = riff.mmap().first_entry_with_tag(tags::TAG_File) {
                reader.seek(std::io::SeekFrom::Start(file.pos() as u64))?;
                riff = RiffFile::new(reader)?;
            }
        }

        riff.read_key_table()?;
        if let Err(err) = riff.read_config() {
            println!("{err}");
        }

        riff
    } else {
        let mut riff = RiffFile::new(reader)?;

        riff.read_key_table()?;
        if let Err(err) = riff.read_config() {
            println!("{err}");
        }

        riff
    };

    riff.read_file_info()?;

    riff.read_cast_table()?;

    match riff.read_frame_labels() {
        Err(err) if err.kind() == std::io::ErrorKind::NotFound => {
            println!("No frame labels found");
            Ok(())
        }
        v => v,
    }?;

    riff.read_score()?;

    let event_loop = EventLoop::new().expect("Failed to create event loop");
    event_loop.set_control_flow(ControlFlow::Poll);

    let player = Player::new(riff);

    let mut app = App {
        display_list: DisplayList::new(),
        player,
        window: None,
        surface: None,
    };
    event_loop.run_app(&mut app)?;

    Ok(())
}
