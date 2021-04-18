use library::prelude::{AIModule, BoxedAIModule, Event};
use library::HACK_BOX;

struct TestAI;
impl AIModule for TestAI {
    fn on_event(&mut self, event: Event) {
        match event {
            Event::OnStart() => {
                println!("fn on_start()");
            }
            Event::OnEnd(is_winner) => {
                println!("fn on_end(is_winner: {})", is_winner);
            }
            Event::OnFrame() => {
                println!("fn on_frame()");
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

#[test]
fn all_events() {
    HACK_BOX.set(BoxedAIModule::new(TestAI)).unwrap_or(());
    println!("BoxedAIModule size = {}", std::mem::size_of::<BoxedAIModule>());
    library::ffi_test::cpp_test();
}