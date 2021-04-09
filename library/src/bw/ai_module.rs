use crate::bw::player::Player;
use crate::bw::position::Position;
use crate::bw::unit::Unit;

pub trait AIModule {
    fn on_start(&mut self);
    fn on_frame(&mut self);
    fn on_end(&mut self, is_winner: bool);
    fn on_send_text(&mut self, text: String);
    fn on_receive_text(&mut self, player: &mut Player, text: String);
    fn on_player_left(&mut self, player: &mut Player);
    fn on_nuke_detect(&mut self, target: Position);
    fn on_unit_discover(&mut self, unit: &mut Unit);
    fn on_unit_evade(&mut self, unit: &mut Unit);
    fn on_unit_show(&mut self, unit: &mut Unit);
    fn on_unit_hide(&mut self, unit: &mut Unit);
    fn on_unit_create(&mut self, unit: &mut Unit);
    fn on_unit_destroy(&mut self, unit: &mut Unit);
    fn on_unit_morph(&mut self, unit: &mut Unit);
    fn on_unit_renegade(&mut self, unit: &mut Unit);
    fn on_save_game(&mut self, game_name: String);
    fn on_unit_complete(&mut self, unit: &mut Unit);
}
