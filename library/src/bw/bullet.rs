use crate::bw::bullet_type::BulletType;
use crate::bw::player::Player;
use crate::bw::position::Position;
use crate::bw::unit::Unit;
use crate::{ffi, FromRaw};
use std::ptr::null_mut;

pub struct Bullet {
    pub(crate) raw: *const ffi::BulletInterface,
}

impl FromRaw<ffi::BulletInterface> for Bullet {
    unsafe fn from_raw(raw: *const ffi::BulletInterface) -> Self {
        assert!(!raw.is_null());
        Self { raw }
    }
}

impl Bullet {
    pub fn exists(&self) -> bool {
        unsafe { (&*self.raw).exists() }
    }
    pub fn get_angle(&self) -> f64 {
        unsafe { (&*self.raw).getAngle() }
    }
    pub fn get_id(&self) -> i32 {
        unsafe { (&*self.raw).getID() }
    }
    pub fn get_player(&self) -> Player {
        unsafe { Player::from_raw((&*self.raw).getPlayer()) }
    }
    pub fn get_position(&self) -> Position {
        unsafe { (&*self.raw).getPosition() }
    }
    pub fn get_remove_timer(&self) -> i32 {
        unsafe { (&*self.raw).getRemoveTimer() }
    }
    pub fn get_source(&self) -> Unit {
        unsafe { Unit::from_raw((&*self.raw).getSource()) }
    }
    pub fn get_target(&self) -> Unit {
        unsafe { Unit::from_raw((&*self.raw).getTarget()) }
    }
    pub fn get_target_position(&self) -> Position {
        unsafe { (&*self.raw).getTargetPosition() }
    }
    pub fn get_type(&self) -> BulletType {
        unsafe { (&*self.raw).getType() }
    }
    pub fn get_velocity_x(&self) -> f64 {
        unsafe { (&*self.raw).getVelocityX() }
    }
    pub fn get_velocity_y(&self) -> f64 {
        unsafe { (&*self.raw).getVelocityY() }
    }
    pub fn is_visible(&self) -> bool {
        unsafe { (&*self.raw).isVisible(null_mut()) }
    }
    pub fn is_visible_to_player(&self, player: Player) -> bool {
        unsafe { (&*self.raw).isVisible(player.raw as *mut _) }
    }
}
