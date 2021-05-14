use crate::bw::bullet_type::BulletType;
use crate::bw::player::Player;
use crate::bw::position::Position;
use crate::bw::unit::Unit;
use crate::{ffi, FromRaw};
use std::ptr::{null_mut, NonNull};

pub struct Bullet {
    pub(crate) raw: NonNull<ffi::BulletInterface>,
}

impl FromRaw<ffi::BulletInterface> for Bullet {
    unsafe fn from_raw(raw: *mut ffi::BulletInterface) -> Self {
        assert!(!raw.is_null());
        Self {
            raw: NonNull::new_unchecked(raw),
        }
    }
}

impl Bullet {
    pub fn exists(&self) -> bool {
        unsafe { self.raw.as_ref().exists() }
    }
    pub fn get_angle(&self) -> f64 {
        unsafe { self.raw.as_ref().getAngle() }
    }
    pub fn get_id(&self) -> i32 {
        unsafe { self.raw.as_ref().getID() }
    }
    pub fn get_player(&self) -> Player {
        unsafe { Player::from_raw(self.raw.as_ref().getPlayer()) }
    }
    pub fn get_position(&self) -> Position {
        unsafe { self.raw.as_ref().getPosition() }
    }
    pub fn get_remove_timer(&self) -> i32 {
        unsafe { self.raw.as_ref().getRemoveTimer() }
    }
    pub fn get_source(&self) -> Unit {
        unsafe { Unit::from_raw(self.raw.as_ref().getSource()) }
    }
    pub fn get_target(&self) -> Unit {
        unsafe { Unit::from_raw(self.raw.as_ref().getTarget()) }
    }
    pub fn get_target_position(&self) -> Position {
        unsafe { self.raw.as_ref().getTargetPosition() }
    }
    pub fn get_type(&self) -> BulletType {
        unsafe { self.raw.as_ref().getType() }
    }
    pub fn get_velocity_x(&self) -> f64 {
        unsafe { self.raw.as_ref().getVelocityX() }
    }
    pub fn get_velocity_y(&self) -> f64 {
        unsafe { self.raw.as_ref().getVelocityY() }
    }
    pub fn is_visible(&self) -> bool {
        unsafe { self.raw.as_ref().isVisible(null_mut()) }
    }
    pub fn is_visible_to_player(&self, player: Player) -> bool {
        unsafe { self.raw.as_ref().isVisible(player.raw.as_ptr()) }
    }
}
