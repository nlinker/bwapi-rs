use cxx::UniquePtr;
use library::bw::color::Color;
use library::bw::coordinate_type::CoordinateType;
use library::bw::unit_type::UnitType;
use library::ffi;
use library::prelude::*;
use std::sync::MutexGuard;
use std::thread::sleep;
use std::time::Duration;

#[no_mangle]
#[allow(non_snake_case)]
pub unsafe extern "C" fn newAIModule() -> *mut ffi::AIModuleWrapper {
    println!("newAIModule called!");
    let demo = DemoAI {
        name: "DemoAI here".to_string(),
        counter: 0,
        is_burrowed: false,
    };
    let ai = Box::new(BoxedAIModule::new(demo));
    let wrapper: UniquePtr<ffi::AIModuleWrapper> = ffi::create_ai_module_wrapper(ai);
    wrapper.into_raw()
}

#[derive(Debug, Clone)]
pub struct DemoAI {
    name: String,
    counter: u32,
    is_burrowed: bool,
}

impl AIModule for DemoAI {
    fn on_event(&mut self, event: Event) {
        let game: MutexGuard<Game> = GAME.lock().unwrap();
        match event {
            Event::OnStart => {
                println!("fn on_start()");
            }
            Event::OnEnd { is_winner } => {
                println!("fn on_end(is_winner: {})", is_winner);
            }
            Event::OnFrame => {
                let colors = [
                    Color::Black,
                    Color::Brown,
                    Color::Grey,
                    Color::Red,
                    Color::Green,
                    Color::Cyan,
                    Color::Yellow,
                    Color::Teal,
                    Color::Purple,
                    Color::Blue,
                    Color::Orange,
                    Color::White,
                ];
                for i in 0..colors.len() {
                    let delta = (i * 37) as i32;
                    game.draw_box(
                        CoordinateType::Map,
                        100 + delta,
                        100 + delta,
                        200 + delta,
                        200 + delta,
                        colors[i],
                        true,
                    );
                    game.draw_box(
                        CoordinateType::Map,
                        300 + delta,
                        100 + delta,
                        400 + delta,
                        200 + delta,
                        colors[i],
                        false,
                    );
                }
                // let sizes = [TextSize::Huge, TextSize::Large, TextSize::Small, TextSize::Default];
                // for i in 0..sizes.len() {
                //     game.set_text_size(sizes[i]);
                //     game.draw_text(CoordinateType::Map, 1800, 1800 + (i as i32 * 50), "Hello, SSCAIT!");
                // }
                // println!("fn on_frame()");
                let fc = game.get_frame_count();
                if fc % 50 == 0 {
                    let xs = game.get_start_locations();
                    let home = xs.iter().filter(|t| {
                        let p = Position { x: t.x * 32, y: t.y * 32 };
                        !game.get_units_in_radius(p, 100, |u| u.get_type() == UnitType::Zerg_Hatchery).is_empty()
                    }).copied().next().unwrap();
                    let home_pos = Position { x: home.x * 32, y: home.y * 32 };
                    println!("home = {:?}, home position = {:?}", home, home_pos);

                    if !self.is_burrowed {
                        if let Some(hatchery) = game.get_closest_unit(home_pos, 100, |x| x.get_type() == UnitType::Zerg_Hatchery) {
                            let drones = game.get_units_in_radius(home_pos, 200, |x| x.get_type() == UnitType::Zerg_Drone);
                            for drone in drones.iter() {
                                drone.attack_u(&hatchery, false);
                            }
                        }
                        self.is_burrowed = true;
                    } else {
                        if let Some(hatchery) = game.get_closest_unit(home_pos, 100, |x| x.get_type() == UnitType::Zerg_Hatchery) {
                            println!("frame = {}, hatchery hit points = {:?}", fc, hatchery.get_hit_points());
                            game.set_frame_skip(100);
                            game.set_local_speed(0);
                        }
                    }
                }

                // let c = Position { x: 250, y: 3160 };
                // let unit_opt = game.get_best_unit(
                //     |u1, u2| if u1.get_id() < u2.get_id() { u1 } else { u2 },
                //     |u| u.get_type() == UnitType::Zerg_Drone,
                //     c,
                //     100,
                // );

                // if let Some(unit) = unit_opt {
                // if !self.is_attacking {
                //     let s = TilePosition { x: 117, y: 13 };
                //     let s = Position { x: s.x * 32, y: s.y * 32 };
                //     unit.attack_p(s, false);
                //     self.is_attacking = true;
                // }
                // } else {
                //     game.send_text("The best unit not found");
                // }

                // let forces = game.get_forces();
                // for force in forces.iter() {
                //     println!("force: {} {}", force.get_id(), force.get_name());
                // for player in force.get_players().iter() {
                //     println!("  player: {:?}", player);
                // }
                // }
                // println!("game.allies = {:?}", game.allies().into_iter().next());
                // game.send_text(&format!("Unitset size: {:?}", game.get_all_units().len()));
                // let xs = game.get_nuke_dots();
                // println!("get_nuke_dots = {:?}", xs);
                // let xs = game.get_start_locations();
                // println!("get_start_locations = {:?}", xs);
                // for u in game.get_all_units().iter() {
                // println!("All list: unit = {:?} with id {}, type: {:?}, pos: {:?}", u, u.get_id(), u.get_type(), u.get_position());
                // }
                // if let Some(h) = inr.iter().find(|u| u.get_type() == UnitType::Zerg_Hatchery) {
                //     let drones = inr.iter().filter(|u| u.get_type() == UnitType::Zerg_Drone).collect::<Vec<_>>();
                // }
                // game.send_text(&format!("Hello, SSCAIT!, frame count = {}", fc));
                sleep(Duration::from_millis(100));
            }
            Event::OnSendText { text } => {
                println!("fn on_send_text(text: {})", text);
            }
            Event::OnReceiveText { player, text } => {
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
    use crate::DemoAI;
    use library::prelude::*;

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
        let mut demo = DemoAI {
            name: "DemoAI here".to_string(),
            counter: 0,
        };
        let mut ai = BoxedAIModule::new(demo);
        ai.on_event(Event::OnNukeDetect(Position { x: 11, y: 22 }));
    }
}
