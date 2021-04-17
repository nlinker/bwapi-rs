use library::prelude::*;
use library::ffi;
use cxx::UniquePtr;
use std::borrow::Borrow;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn newAIModule() -> *mut ffi::AIModuleWrapper {
    println!("newAIModule called!");
    let demo = DemoAI { name: "DemoAI here".to_string(), counter: 0 };
    let ai2 = BoxedAIModule::new(DemoAI { name: "DemoAI2 here".to_string(), counter: 0 });
    let mut ai = BoxedAIModule::new(demo);
    println!("ai2.as_raw() = {:p}", ai2.as_raw());
    println!("ai.as_raw() = {:p}", ai.as_raw());
    println!("&ai = {:p}", &ai);
    let wrapper: UniquePtr<ffi::AIModuleWrapper> = ffi::create_ai_module_wrapper(&mut ai);
    std::mem::forget(ai);
    wrapper.into_raw()
}

#[derive(Debug, Clone)]
pub struct DemoAI {
    name: String,
    counter: u32,
}

impl AIModule for DemoAI {
    fn on_event(&mut self, event: Event) {
        let _game: &Game = GAME.lock().unwrap().borrow();
        match event {
            Event::OnStart() => {
                println!("fn on_start()");
            }
            Event::OnEnd(is_winner) => {
                println!("fn on_end(is_winner: {})", is_winner);
            }
            Event::OnFrame() => {
                // println!("fn on_frame()");
            }
            Event::OnSendText(text) => {
                println!("fn on_send_text(text: {})", text);
            }
            Event::OnReceiveText(player, text) => {
                println!("fn on_receive_text(player: {:?}, text: {})", player, text);
            }
            Event::OnPlayerLeft(player) => {
                println!("fn on_player_left(player: {:?})", player);
            }
            Event::OnNukeDetect(target) => {
                println!("fn on_nuke_detect(target: {:?})", target);
            }
            Event::OnUnitDiscover(unit) => {
                println!("fn on_unit_discover(unit: {:?})", unit);
            }
            Event::OnUnitEvade(unit) => {
                println!("fn on_unit_evade(unit: {:?})", unit);
            }
            Event::OnUnitShow(unit) => {
                println!("fn on_unit_show(unit: {:?})", unit);
            }
            Event::OnUnitHide(unit) => {
                println!("fn on_unit_hide(unit: {:?})", unit);
            }
            Event::OnUnitCreate(unit) => {
                println!("fn on_unit_create(unit: {:?})", unit);
            }
            Event::OnUnitDestroy(unit) => {
                println!("fn on_unit_destroy(unit: {:?})", unit);
            }
            Event::OnUnitMorph(unit) => {
                println!("fn on_unit_morph(unit: {:?})", unit);
            }
            Event::OnUnitRenegade(unit) => {
                println!("fn on_unit_renegade(unit: {:?})", unit);
            }
            Event::OnSaveGame(game_name) => {
                println!("fn on_save_game(game_name: {})", game_name);
            }
            Event::OnUnitComplete(unit) => {
                println!("fn on_unit_complete(unit: {:?})", unit);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use library::prelude::*;
    use crate::DemoAI;

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn boxed_test() {

        struct TestAI;

        impl AIModule for TestAI {
            fn on_event(&mut self, event: Event) {
                match event {
                    Event::OnNukeDetect(p) => {
                        println!("nuke = {:?}", p);
                    }
                    _ => {}
                }
            }
        }
        let mut demo = DemoAI { name: "DemoAI here".to_string(), counter: 0 };
        let mut ai = BoxedAIModule::new(demo);
        ai.on_event(Event::OnNukeDetect(Position { x: 11, y: 22 }));
    }
}
