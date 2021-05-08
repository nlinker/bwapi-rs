use crate::bw::color::{Color, TextSize};
use crate::bw::coordinate_type::CoordinateType;
use crate::bw::playerset::Playerset;
use crate::bw::position::{Position, TilePosition};
use crate::bw::unit_filter::UnitFilter;
use crate::bw::unitset::Unitset;
use crate::bw::Handle;
use crate::ffi;
use cxx::UniquePtr;
use std::pin::Pin;
use crate::bw::player::Player;
use crate::bw::forceset::Forceset;

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
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        let set: Pin<&mut ffi::Playerset> = g.allies();
        Playerset { raw: Handle::BorrowedMut(set) }
    }
    pub fn set_alliance(&self, player: Player, allied: bool, allied_victory: bool) -> bool {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        unsafe { g.setAlliance(player.raw as *mut _, allied, allied_victory) }
    }

    pub fn get_forces(&self) -> Forceset {
        let g: &ffi::Game = unsafe { &*self.raw };
        Forceset { raw: Handle::Borrowed(g.getForces()) }
    }

    pub fn send_text(&self, text: &str) {
        ffi::_game_sendText(unsafe { Pin::new_unchecked(&mut *self.raw) }, text)
    }
    pub fn get_frame_count(&self) -> i32 {
        unsafe { (*self.raw).getFrameCount() }
    }
    pub fn get_all_units(&self) -> Unitset {
        let g: &ffi::Game = unsafe { &*self.raw };
        let set: &ffi::Unitset = g.getAllUnits();
        Unitset { raw: Handle::Borrowed(set) }
    }
    pub fn get_units_in_radius(&self, position: Position, radius: i32, pred: UnitFilter) -> Unitset {
        let g: &ffi::Game = unsafe { &*self.raw };
        let set: UniquePtr<ffi::Unitset> = ffi::_game_getUnitsInRadius(g, position, radius, pred);
        Unitset { raw: Handle::Owned(set) }
    }

    pub fn get_nuke_dots(&self) -> Vec<Position> {
        let g: &ffi::Game = unsafe { &*self.raw };
        ffi::_game_getNukeDots(g)
    }

    pub fn get_start_locations(&self) -> Vec<TilePosition> {
        let g: &ffi::Game = unsafe { &*self.raw };
        ffi::_game_getStartLocations(g)
    }

    // let ctype = ctype.unwrap_or(CoordinateType::Map);
    pub fn set_text_size(&self, size: TextSize) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        g.setTextSize(size);
    }
    pub fn draw_text(&self, ctype: CoordinateType, x: i32, y: i32, text: &str) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        ffi::_game_drawText(g, ctype, x, y, text);
    }
    pub fn draw_text_map(&self, x: i32, y: i32, text: &str) {
        self.draw_text(CoordinateType::Map, x, y, text);
    }
    pub fn draw_box(&self, ctype: CoordinateType, left: i32, top: i32, right: i32, bottom: i32, color: Color, is_solid: bool) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        g.drawBox(ctype, left, top, right, bottom, color, is_solid);
    }
    pub fn draw_box_map(&self, left: i32, top: i32, right: i32, bottom: i32, color: Color, is_solid: bool) {
        self.draw_box(CoordinateType::Map, left, top, right, bottom, color, is_solid);
    }
    pub fn draw_triangle(&self, ctype: CoordinateType, ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32, color: Color, is_solid: bool) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        g.drawTriangle(ctype, ax, ay, bx, by, cx, cy, color, is_solid);
    }
    pub fn draw_triangle_map(&self, ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32, color: Color, is_solid: bool) {
        self.draw_triangle(CoordinateType::Map, ax, ay, bx, by, cx, cy, color, is_solid);
    }
    pub fn draw_circle(&self, ctype: CoordinateType, x: i32, y: i32, radius: i32, color: Color, is_solid: bool) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        g.drawCircle(ctype, x, y, radius, color, is_solid);
    }
    pub fn draw_circle_map(&self, x: i32, y: i32, radius: i32, color: Color, is_solid: bool) {
        self.draw_circle(CoordinateType::Map, x, y, radius, color, is_solid);
    }
    pub fn draw_ellipse(&self, ctype: CoordinateType, x: i32, y: i32, xrad: i32, yrad: i32, color: Color, is_solid: bool) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        g.drawEllipse(ctype, x, y, xrad, yrad, color, is_solid);
    }
    pub fn draw_ellipse_map(&self, x: i32, y: i32, xrad: i32, yrad: i32, color: Color, is_solid: bool) {
        self.draw_ellipse(CoordinateType::Map, x, y, xrad, yrad, color, is_solid);
    }
    pub fn draw_dot(&self, ctype: CoordinateType, x: i32, y: i32, color: Color) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        g.drawDot(ctype, x, y, color);
    }
    pub fn draw_dot_map(&self, x: i32, y: i32, color: Color) {
        self.draw_dot(CoordinateType::Map, x, y, color);
    }
    pub fn draw_line(&self, ctype: CoordinateType, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
        let g: Pin<&mut ffi::Game> = unsafe { Pin::new_unchecked(&mut *self.raw) };
        g.drawLine(ctype, x1, y1, x2, y2, color);
    }
    pub fn draw_line_map(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: Color) {
        self.draw_line(CoordinateType::Map, x1, y1, x2, y2, color);
    }
}
