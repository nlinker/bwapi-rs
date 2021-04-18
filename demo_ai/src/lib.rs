use library::prelude::*;
use library::ffi;
use cxx::UniquePtr;

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
                // println!("fn on_frame()");
                let fc = game.get_frame_count();
                if fc % 50 == 0 {
                    game.send_text(&format!("Hello, SSCAIT!, frame count = {}", fc));
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
