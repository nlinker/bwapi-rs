use library::ffi::Player;
use library::ffi::Position;
use library::ffi::Unit;
use std::ptr::null_mut;
use library::bw::ai_module::AIMod;

// #[no_mangle]
// #[allow(non_snake_case)]
// pub unsafe extern "C" fn newAIModule() -> *mut std::ffi::c_void {
//     println!("newAIModule called!");
//     Box::new(DemoAI {
//         name: "Hello, this is DemoAI".into()
//     }).as_mut_ptr()
// }

struct DemoAI {
    name: String,
}

impl AIMod for DemoAI {
    fn on_start(&mut self) {
        println!("`on_start` called for {}", self.name);
    }
    fn on_end(&mut self, is_winner: bool) {
        println!("`on_end({})` called for {}", is_winner, self.name);
    }
    fn on_frame(&mut self) {
        println!("`on_frame` called for {}", self.name);
    }

    fn on_send_text(&mut self, text: String) {
        todo!()
    }

    fn on_receive_text(&mut self, player: &Player, text: String) {
        todo!()
    }

    fn on_player_left(&mut self, player: &Player) {
        todo!()
    }

    fn on_nuke_detect(&mut self, target: Position) {
        todo!()
    }

    fn on_unit_discover(&mut self, unit: &Unit) {
        todo!()
    }

    fn on_unit_evade(&mut self, unit: &Unit) {
        todo!()
    }

    fn on_unit_show(&mut self, unit: &Unit) {
        todo!()
    }

    fn on_unit_hide(&mut self, unit: &Unit) {
        todo!()
    }

    fn on_unit_create(&mut self, unit: &Unit) {
        todo!()
    }

    fn on_unit_destroy(&mut self, unit: &Unit) {
        todo!()
    }

    fn on_unit_morph(&mut self, unit: &Unit) {
        todo!()
    }

    fn on_unit_renegade(&mut self, unit: &Unit) {
        todo!()
    }

    fn on_save_game(&mut self, game_name: String) {
        todo!()
    }

    fn on_unit_complete(&mut self, unit: &Unit) {
        todo!()
    }
    // fn on_send_text(&mut self, _text: String) {}
    // fn on_receive_text(&mut self, _player: &Player, _text: String) {}
    // fn on_player_left(&mut self, _player: &Player) {}
    // fn on_nuke_detect(&mut self, _target: Position) {}
    // fn on_unit_discover(&mut self, _unit: &Unit) {}
    // fn on_unit_evade(&mut self, _unit: &Unit) {}
    // fn on_unit_show(&mut self, _unit: &Unit) {}
    // fn on_unit_hide(&mut self, _unit: &Unit) {}
    // fn on_unit_create(&mut self, _unit: &Unit) {}
    // fn on_unit_destroy(&mut self, _unit: &Unit) {}
    // fn on_unit_morph(&mut self, _unit: &Unit) {}
    // fn on_unit_renegade(&mut self, _unit: &Unit) {}
    // fn on_save_game(&mut self, _game_name: String) {}
    // fn on_unit_complete(&mut self, _unit: &Unit) {}
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
