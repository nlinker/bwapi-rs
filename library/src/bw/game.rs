use crate::bw::color::{Color, TextSize};
use crate::bw::coordinate_type::CoordinateType;
use crate::bw::playerset::Playerset;
use crate::bw::position::{Position, TilePosition};
use crate::bw::unit_filter::UnitFilter;
use crate::bw::unitset::Unitset;
use crate::ffi;
use std::pin::Pin;
use cxx::UniquePtr;

#[derive(Debug)]
pub struct Game {
    pub(crate) raw: *mut ffi::Game,
}

/// Game object doesn't contain any self-references
impl Unpin for Game {}

/// Safety: https://stackoverflow.com/a/60295465/5066426
unsafe impl Send for Game {}

impl Game {
    pub fn debug(&self) {
        unsafe { ffi::_game_debug(&*self.raw) };
    }

    pub fn allies(&self) -> Playerset {
        let game: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        let set: Pin<&mut ffi::Playerset> = game.allies();
        Playerset {
            iter: ffi::createPlayersetIterator(&*set),
        }
    }
    pub fn send_text(&self, text: &str) {
        ffi::_game_sendText(unsafe { Pin::new_unchecked(&mut *self.raw) }, text)
    }
    pub fn get_frame_count(&self) -> i32 {
        unsafe { (*self.raw).getFrameCount() }
    }
    pub fn get_all_units(&self) -> Unitset {
        let game: &ffi::Game = unsafe { &*self.raw };
        let set: &ffi::Unitset = game.getAllUnits();
        // let set: UniquePtr<ffi::Unitset> = ffi::createUnitsetIteratorRef(set);
        todo!()
    }
    pub fn get_units_in_radius(&self, position: Position, radius: i32, pred: UnitFilter) -> Unitset {
        let game: &ffi::Game = unsafe { &*self.raw };
        let iter: UniquePtr<ffi::Unitset> = ffi::_game_getUnitsInRadius(game, position, radius, pred);
        // Unitset { iter }
        todo!()
    }

    pub fn get_nuke_dots(&self) -> Vec<Position> {
        let game: &ffi::Game = unsafe { &*self.raw };
        let xs = ffi::_game_getNukeDots(game);
        // https://github.com/dtolnay/cxx/issues/855
        xs.into_iter().map(|p| Position { x: p.x, y: p.y }).collect()
    }

    pub fn get_start_locations(&self) -> Vec<TilePosition> {
        let game: &ffi::Game = unsafe { &*self.raw };
        let xs = ffi::_game_getStartLocations(game);
        // https://github.com/dtolnay/cxx/issues/855
        xs.into_iter().map(|p| TilePosition { x: p.x, y: p.y }).collect()
    }

    // let ctype = ctype.unwrap_or(CoordinateType::Map);
    pub fn set_text_size(&self, size: TextSize) {
        let game: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        game.setTextSize(size);
    }
    pub fn draw_text(&self, ctype: CoordinateType, x: i32, y: i32, text: &str) {
        let game: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        ffi::_game_drawText(game, ctype, x, y, text);
    }
    pub fn draw_text_map(&self, x: i32, y: i32, text: &str) {
        self.draw_text(CoordinateType::Map, x, y, text);
    }
    pub fn draw_box(&self, ctype: CoordinateType, left: i32, top: i32, right: i32, bottom: i32, color: Color, is_solid: bool) {
        let game: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        game.drawBox(ctype, left, top, right, bottom, color, is_solid);
    }
    pub fn draw_box_map(&self, left: i32, top: i32, right: i32, bottom: i32, color: Color, is_solid: bool) {
        self.draw_box(CoordinateType::Map, left, top, right, bottom, color, is_solid);
    }
    pub fn draw_triangle(&self, ctype: CoordinateType, ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32, color: Color, is_solid: bool) {
        let game: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        game.drawTriangle(ctype, ax, ay, bx, by, cx, cy, color, is_solid);
    }
    pub fn draw_triangle_map(&self, ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32, color: Color, is_solid: bool) {
        self.draw_triangle(CoordinateType::Map, ax, ay, bx, by, cx, cy, color, is_solid);
    }
    pub fn draw_circle(&self, ctype: CoordinateType, x: i32, y: i32, radius: i32, color: Color, is_solid: bool) {
        let game: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        game.drawCircle(ctype, x, y, radius, color, is_solid);
    }
    pub fn draw_circle_map(&self, x: i32, y: i32, radius: i32, color: Color, is_solid: bool) {
        self.draw_circle(CoordinateType::Map, x, y, radius, color, is_solid);
    }
    pub fn draw_ellipse(&self, ctype: CoordinateType, x: i32, y: i32, xrad: i32, yrad: i32, color: Color, is_solid: bool) {
        let game: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        game.drawEllipse(ctype, x, y, xrad, yrad, color, is_solid);
    }
    pub fn draw_ellipse_map(&self, x: i32, y: i32, xrad: i32, yrad: i32, color: Color, is_solid: bool) {
        self.draw_ellipse(CoordinateType::Map, x, y, xrad, yrad, color, is_solid);
    }
    pub fn draw_dot(&self, ctype: CoordinateType, x: i32, y: i32, color: Color) {
        let game: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        game.drawDot(ctype, x, y, color);
    }
    pub fn draw_dot_map(&self, x: i32, y: i32, color: Color) {
        self.draw_dot(CoordinateType::Map, x, y, color);
    }
    pub fn draw_line(&self, ctype: CoordinateType, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
        let game: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        game.drawLine(ctype, x1, y1, x2, y2, color);
    }
    pub fn draw_line_map(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
        self.draw_line(CoordinateType::Map, x1, y1, x2, y2, color);
    }
}
