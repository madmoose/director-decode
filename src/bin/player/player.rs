use std::{collections::HashMap, rc::Rc};

use director_decoder::{
    gfx,
    riff::{
        RiffFile, Tempo,
        cast_members::{self, CastMemberId},
    },
};
use winit::dpi::LogicalSize;

#[derive(Debug)]
pub enum DisplayObject {
    Bitmap {
        id: cast_members::CastMemberId,
        rect: gfx::Rect,
        image: Rc<gfx::IndexedImage>,
    },
}

pub type DisplayList = Vec<DisplayObject>;

#[allow(unused)]
pub enum PlayerEvent {
    Draw(DisplayList),
    EndOfMove,
}

pub struct Player<'a> {
    pub riff: RiffFile<'a>,
    tempo: Tempo,
    pub palette: gfx::Palette,

    current_frame_number: u16,
    next_frame_number: u16,

    current_frame_time: std::time::Instant,
    next_frame_time: std::time::Instant,

    cast_members: HashMap<CastMemberId, cast_members::CastMember>,
}

impl<'a> Player<'a> {
    pub fn new(riff: RiffFile<'a>) -> Self {
        Self {
            riff,
            tempo: Tempo::default(),
            palette: gfx::Palette::new(),

            current_frame_number: 0,
            next_frame_number: 0,

            current_frame_time: std::time::Instant::now(),
            next_frame_time: std::time::Instant::now(),

            cast_members: HashMap::new(),
        }
    }

    pub fn default_window_size(&self) -> LogicalSize<i32> {
        let width = self.riff.config().movie_right as i32 - self.riff.config().movie_left as i32;
        let height = self.riff.config().movie_bottom as i32 - self.riff.config().movie_top as i32;

        LogicalSize::new(width, height)
    }

    pub fn frame_duration(&self) -> Option<std::time::Duration> {
        match self.tempo {
            Tempo::FPS(fps) => Some(std::time::Duration::from_secs(1) / fps as u32),
            _ => None,
        }
    }

    pub fn time_for_new_frame(&self) -> bool {
        self.next_frame_time <= std::time::Instant::now()
    }

    pub fn step_frame(&mut self) -> DisplayList {
        self.current_frame_time = self.next_frame_time;
        self.current_frame_number = self.next_frame_number;

        self.preload_cast_members_for_frame(self.current_frame_number);

        let frame = self
            .riff
            .score()
            .get_frame(self.current_frame_number)
            .unwrap_or_else(|| panic!("Frame {} not found", self.current_frame_number));

        if let Some(tempo) = frame.tempo {
            self.tempo = tempo;
        }

        // If the current frame has a palette, update the global palette
        if let Some(palette_id) = frame.palette_id {
            if let Some(palette) = self.cast_members.get(&palette_id) {
                if let Some(palette) = palette.as_palette() {
                    if let Some(clut) = &palette.clut {
                        for (i, color) in clut.colors.iter().enumerate() {
                            let color = gfx::Rgb161616 {
                                r: color.r,
                                g: color.g,
                                b: color.b,
                            };
                            self.palette[i] = color;
                        }
                    }
                }
            }
        }

        let mut display_list = DisplayList::new();
        for (_, channel) in &frame.sprite_channels {
            if channel.sprite_type == 1 {
                if let Some(cast_member_id) = channel.cast_member_id {
                    let cast_member = self.cast_members.get(&cast_member_id).unwrap();
                    let bitmap_cast_member = cast_member.as_bitmap().unwrap();
                    let image = bitmap_cast_member.image().unwrap();

                    let rect = bitmap_cast_member
                        .info
                        .rect
                        .translate(-bitmap_cast_member.info.reg)
                        .translate(channel.position);

                    display_list.push(DisplayObject::Bitmap {
                        id: cast_member_id,
                        rect,
                        image: Rc::new(image),
                    });
                }
            }
        }

        self.next_frame_number += 1;
        let dt = self.frame_duration().unwrap();
        self.next_frame_time = self.current_frame_time + dt;

        display_list
    }

    pub fn preload_cast_members_for_frame(&mut self, frame_number: u16) {
        let frame = self
            .riff
            .score()
            .get_frame(frame_number)
            .expect("Frame not found");

        let mut cast_member_ids = Vec::with_capacity(frame.sprite_channels.len());

        if let Some(palette_id) = frame.palette_id {
            cast_member_ids.push(palette_id);
        }

        for (_, channel) in &frame.sprite_channels {
            if let Some(cast_member_id) = channel.cast_member_id {
                cast_member_ids.push(cast_member_id);
            }
        }

        for cast_member_id in cast_member_ids {
            self.preload_cast_member(cast_member_id)
        }
    }

    pub fn preload_cast_member(&mut self, cast_member_id: CastMemberId) {
        if self.cast_members.contains_key(&cast_member_id) {
            return;
        }

        let cast_member = self
            .riff
            .load_cast_member(cast_member_id)
            .expect("Failed to load cast member");

        self.cast_members.insert(cast_member_id, cast_member);
    }
}
