use library::prelude::*;
use library::ffi;
use cxx::UniquePtr;
use std::thread::sleep;
use std::time::Duration;
use library::bw::color::Color;
use library::bw::coordinate_type::CoordinateType;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn newAIModule() -> *mut ffi::AIModuleWrapper {
    println!("newAIModule called!");
    let demo = DemoAI { name: "DemoAI here".to_string(), counter: 0 };
    let ai = BoxedAIModule::new(demo);
    let wrapper: UniquePtr<ffi::AIModuleWrapper> = ffi::create_ai_module_wrapper(Box::new(ai));
    wrapper.into_raw()
}

#[derive(Debug, Clone)]
pub struct DemoAI {
    name: String,
    counter: u32,
}

impl AIModule for DemoAI {
    fn on_event(&mut self, event: Event) {
        let game = GAME.lock().unwrap();
        match event {
            Event::OnStart() => {
                println!("fn on_start()");
            }
            Event::OnEnd(is_winner) => {
                println!("fn on_end(is_winner: {})", is_winner);
            }
            Event::OnFrame() => {
                game.debug();
                let colors = [Color::Black, Color::Brown, Color::Grey, Color::Red, Color::Green, Color::Cyan,
                    Color::Yellow, Color::Teal, Color::Purple, Color::Blue, Color::Orange, Color::White,
                ];
                for i in 0..colors.len() {
                    let delta = (i * 37) as i32;
                    game.draw_box(CoordinateType::Map, 100 + delta, 100 + delta, 200 + delta, 200 + delta, colors[i], true);
                    game.draw_box(CoordinateType::Map, 300 + delta, 100 + delta, 400 + delta, 200 + delta, colors[i], false);
                }
                // let sizes = [TextSize::Huge, TextSize::Large, TextSize::Small, TextSize::Default];
                // for i in 0..sizes.len() {
                //     game.set_text_size(sizes[i]);
                //     game.draw_text(CoordinateType::Map, 1800, 1800 + (i as i32 * 50), "Hello, SSCAIT!");
                // }

                // println!("fn on_frame()");
                let fc = game.get_frame_count();
                if fc % 20 == 0 {
                    // game.allies();
                    game.send_text(&format!("Unitset size_hint: {:?}", game.get_all_units().size_hint()));
                    let xs = game.get_nuke_dots();
                    println!("get_nuke_dots = {:?}", xs);
                    let xs = game.get_start_locations();
                    println!("get_start_locations = {:?}", xs);
                    // for u in game.get_all_units() {
                    //     println!("All list: unit = {:?} with id {}, type: {:?}, pos: {:?}", u, u.get_id(), u.get_type(), u.get_position());
                    // }
                    // let c = Position { x: 250, y: 3160 };
                    // let mut inr = game.get_units_in_radius(c, 100, |_| true).collect::<Vec<_>>();
                    // if let Some(h) = inr.iter().find(|u| u.get_type() == UnitType::Zerg_Hatchery) {
                    //     let drones = inr.iter().filter(|u| u.get_type() == UnitType::Zerg_Drone).collect::<Vec<_>>();
                    // }
                    // game.send_text(&format!("In radius size_hint: {:?}", &inr.size_hint()));
                    // for u in inr {
                    //     println!("In radius: unit with id {:0>3}, type: {:?}, pos: {:?}", u.get_id(), u.get_type(), u.get_position());
                    // }
                    // game.debug();
                    game.send_text(&format!("Hello, SSCAIT!, frame count = {}", fc));
                    sleep(Duration::from_millis(100));
                }
            }
            Event::OnSendText(text) => {
                println!("fn on_send_text(text: {})", text);
            }
            Event::OnReceiveText(player, text) => {
                println!("fn on_receive_text(player: {:?}, text: {})", player, text);
            }
            _ => {
                // ignore the rest
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
