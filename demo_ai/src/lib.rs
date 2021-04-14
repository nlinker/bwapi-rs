use library::bw::ai_module::AIModule;
use library::{ffi, AimBox};
use library::bw::player::Player;
use library::bw::position::Position;
use library::bw::unit::Unit;
use cxx::UniquePtr;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn newAIModule() -> *mut ffi::AIModuleWrapper {
    println!("newAIModule called!");
    let r = DemoAI { name: "RustAIModule here".to_string(), counter: 0 };
    let mut b = AimBox(Box::new(r));
    let ai: UniquePtr<ffi::AIModuleWrapper> = ffi::create_ai_module_wrapper(&mut b);
    ai.into_raw()
}

#[derive(Debug, Clone)]
pub struct DemoAI {
    name: String,
    counter: u32,
}

impl AIModule for DemoAI {
    fn on_start(&mut self) {
        println!("fn on_start()");
    }

    fn on_end(&mut self, is_winner: bool) {
        println!("fn on_end(is_winner: {})", is_winner);
    }

    fn on_frame(&mut self) {
        // too much of them
        // println!("fn on_frame()");
    }

    fn on_send_text(&mut self, text: String) {
        println!("fn on_send_text(text: {})", text);
    }

    fn on_receive_text(&mut self, player: Player, text: String) {
        println!("fn on_receive_text(player: {:?}, text: {})", player, text)
    }

    fn on_player_left(&mut self, player: Player) {
        println!("fn on_player_left(player: {:?})", player);
    }

    fn on_nuke_detect(&mut self, target: Position) {
        println!("fn on_nuke_detect(target: {:?})", target);
    }

    fn on_unit_discover(&mut self, unit: Unit) {
        println!("fn on_unit_discover(unit: {:?})", unit);
    }

    fn on_unit_evade(&mut self, unit: Unit) {
        println!("fn on_unit_evade(unit: {:?})", unit);
    }

    fn on_unit_show(&mut self, unit: Unit) {
        println!("fn on_unit_show(unit: {:?})", unit);
    }

    fn on_unit_hide(&mut self, unit: Unit) {
        println!("fn on_unit_hide(unit: {:?})", unit);
    }

    fn on_unit_create(&mut self, unit: Unit) {
        println!("fn on_unit_create(unit: {:?})", unit);
    }

    fn on_unit_destroy(&mut self, unit: Unit) {
        println!("fn on_unit_destroy(unit: {:?})", unit);
    }

    fn on_unit_morph(&mut self, unit: Unit) {
        println!("fn on_unit_morph(unit: {:?})", unit);
    }

    fn on_unit_renegade(&mut self, unit: Unit) {
        println!("fn on_unit_renegade(unit: {:?})", unit);
    }

    fn on_save_game(&mut self, game_name: String) {
        println!("fn on_save_game(game_name: {})", game_name);
    }

    fn on_unit_complete(&mut self, unit: Unit) {
        println!("fn on_unit_complete(unit: {:?})", unit);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
