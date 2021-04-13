#include <BWAPI.h>
#include <BWAPI/Game.h>
#include <BWAPI/AIModule.h>

#include "lib.h"
#include "library/src/lib.rs.h"

int cpp_main() {
    std::cout << "Hello, there" << std::endl;
    auto ai = new AIModuleWrapper();

    BWAPI::Position target(11, 22);
    BWAPI::Player player = reinterpret_cast<BWAPI::Player>(0xDEADBEEF);
    BWAPI::Unit unit0 = reinterpret_cast<BWAPI::Unit>(0xF00D0000);
    BWAPI::Unit unit1 = reinterpret_cast<BWAPI::Unit>(0xF00D0001);
    BWAPI::Unit unit2 = reinterpret_cast<BWAPI::Unit>(0xF00D0002);
    BWAPI::Unit unit3 = reinterpret_cast<BWAPI::Unit>(0xF00D0003);
    BWAPI::Unit unit4 = reinterpret_cast<BWAPI::Unit>(0xF00D0004);
    BWAPI::Unit unit5 = reinterpret_cast<BWAPI::Unit>(0xF00D0005);
    BWAPI::Unit unit6 = reinterpret_cast<BWAPI::Unit>(0xF00D0006);
    BWAPI::Unit unit7 = reinterpret_cast<BWAPI::Unit>(0xF00D0007);
    //BWAPI::Unit unit8 = reinterpret_cast<BWAPI::Unit>(0xF00D0008);
    BWAPI::Unit unit9 = reinterpret_cast<BWAPI::Unit>(0xF00D0009);
    std::string s1("Hello, this is the text!");
    std::string s2("Hello, this is the received text!");
    std::string s3("This is game name");

    ai->onStart();
    ai->onFrame();
    ai->onEnd(true);
    ai->onSendText(s1);
    ai->onReceiveText(player, s2);
    ai->onPlayerLeft(player);
    ai->onNukeDetect(target);
    ai->onUnitDiscover(unit0);
    ai->onUnitEvade(unit1);
    ai->onUnitShow(unit2);
    ai->onUnitHide(unit3);
    ai->onUnitCreate(unit4);
    ai->onUnitDestroy(unit5);
    ai->onUnitMorph(unit6);
    ai->onUnitRenegade(unit7);
    ai->onSaveGame(s3);
    ai->onUnitComplete(unit9);
    return 0;
}

std::unique_ptr<AIModuleWrapper> createAIModuleWrapper(const AiPlaceholder& userModule) {
    return std::make_unique<AIModuleWrapper>();
}
//void destroyAIModuleWrapper(std::unique_ptr<AIModuleWrapper> module) {
//    delete reinterpret_cast<AIModuleWrapper*>(module);
//}

// region --------------------- AIModuleWrapper shims ---------------------
void AIModuleWrapper::onStart() noexcept {
    on_start(*this);
}

void AIModuleWrapper::onEnd(bool isWinner) noexcept {
    on_end(*this, isWinner);
}

void AIModuleWrapper::onFrame() noexcept {
    on_frame(*this);
}

void AIModuleWrapper::onSendText(std::string text) noexcept {
    on_send_text(*this, text);
}

void AIModuleWrapper::onReceiveText(BWAPI::Player player, std::string text) noexcept {
    const Player p {.raw = player};
    on_receive_text(*this, p, text);
}

void AIModuleWrapper::onPlayerLeft(BWAPI::Player player) noexcept {
    const Player p {.raw = player};
    on_player_left(*this, p);
}

void AIModuleWrapper::onNukeDetect(BWAPI::Position target) noexcept {
    Position* p = reinterpret_cast<Position*>(&target);
    on_nuke_detect(*this, *p);
}

void AIModuleWrapper::onUnitDiscover(BWAPI::Unit unit) noexcept {
    const Unit u {.raw = unit};
    on_unit_discover(*this, u);
}

void AIModuleWrapper::onUnitEvade(BWAPI::Unit unit) noexcept {
    const Unit u {.raw = unit};
    on_unit_evade(*this, u);
}

void AIModuleWrapper::onUnitShow(BWAPI::Unit unit) noexcept {
    const Unit u {.raw = unit};
    on_unit_show(*this, u);
}

void AIModuleWrapper::onUnitHide(BWAPI::Unit unit) noexcept {
    const Unit u {.raw = unit};
    on_unit_hide(*this, u);
}

void AIModuleWrapper::onUnitCreate(BWAPI::Unit unit) noexcept {
    const Unit u {.raw = unit};
    on_unit_create(*this, u);
}

void AIModuleWrapper::onUnitDestroy(BWAPI::Unit unit) noexcept {
    const Unit u {.raw = unit};
    on_unit_destroy(*this, u);
}

void AIModuleWrapper::onUnitMorph(BWAPI::Unit unit) noexcept {
    const Unit u {.raw = unit};
    on_unit_morph(*this, u);
}

void AIModuleWrapper::onUnitRenegade(BWAPI::Unit unit) noexcept {
    const Unit u {.raw = unit};
    on_unit_renegade(*this, u);
}

void AIModuleWrapper::onSaveGame(std::string gameName) noexcept {
    on_save_game(*this, gameName);
}

void AIModuleWrapper::onUnitComplete(BWAPI::Unit unit) noexcept {
    Unit u {.raw = unit};
    on_unit_complete(*this, u);
}
// --------------------- endregion -------------------------
