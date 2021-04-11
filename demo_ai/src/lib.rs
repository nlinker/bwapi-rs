use library::bw::register_ai_module;
use library::bw::ai_module::AIModule;
use library::bw::player::Player;
use library::bw::position::Position;
use library::bw::unit::Unit;
use std::ptr::null_mut;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn newAIModule() -> *mut std::ffi::c_void {
    println!("newAIModule called!");
    null_mut()
}


struct DemoAI {
    name: String,
}

impl AIModule for DemoAI {
    fn on_start(&mut self) {
        println!("`on_start` called for {}", self.name);
    }
    fn on_frame(&mut self) {
        println!("`on_frame` called for {}", self.name);
    }
    fn on_end(&mut self, is_winner: bool) {
        println!("`on_end({})` called for {}", is_winner, self.name);
    }
    fn on_send_text(&mut self, _text: String) {}
    fn on_receive_text(&mut self, _player: &Player, _text: String) {}
    fn on_player_left(&mut self, _player: &Player) {}
    fn on_nuke_detect(&mut self, _target: Position) {}
    fn on_unit_discover(&mut self, _unit: &Unit) {}
    fn on_unit_evade(&mut self, _unit: &Unit) {}
    fn on_unit_show(&mut self, _unit: &Unit) {}
    fn on_unit_hide(&mut self, _unit: &Unit) {}
    fn on_unit_create(&mut self, _unit: &Unit) {}
    fn on_unit_destroy(&mut self, _unit: &Unit) {}
    fn on_unit_morph(&mut self, _unit: &Unit) {}
    fn on_unit_renegade(&mut self, _unit: &Unit) {}
    fn on_save_game(&mut self, _game_name: String) {}
    fn on_unit_complete(&mut self, _unit: &Unit) {}
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
