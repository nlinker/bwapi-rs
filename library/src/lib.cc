//#include <BWAPI.h>
//#include <BWAPI/Game.h>
//#include <BWAPI/AIModule.h>

#include "lib.h"
#include "library/src/lib.rs.h"
#include "BWAPI/Game.h"
#include "BWAPI/Unit.h"

int cpp_test() {
    std::cout << "cpp_test started" << std::endl;
    auto &box = const_cast<BoxedAIModule&>(hack());
    auto ai = new AIModuleWrapper(&box);

    BWAPI::Position target(11, 22);
    auto player = reinterpret_cast<BWAPI::Player>(0xDEADBEEF);
    auto unit0 = reinterpret_cast<BWAPI::Unit>(0xF00D0000);
    auto unit1 = reinterpret_cast<BWAPI::Unit>(0xF00D0001);
    auto unit2 = reinterpret_cast<BWAPI::Unit>(0xF00D0002);
    auto unit3 = reinterpret_cast<BWAPI::Unit>(0xF00D0003);
    auto unit4 = reinterpret_cast<BWAPI::Unit>(0xF00D0004);
    auto unit5 = reinterpret_cast<BWAPI::Unit>(0xF00D0005);
    auto unit6 = reinterpret_cast<BWAPI::Unit>(0xF00D0006);
    auto unit7 = reinterpret_cast<BWAPI::Unit>(0xF00D0007);
    //BWAPI::Unit unit8 = reinterpret_cast<BWAPI::Unit>(0xF00D0008);
    auto unit9 = reinterpret_cast<BWAPI::Unit>(0xF00D0009);
    std::string s1("Hello sent");
    std::string s2("Hello received");
    std::string s3("Saving game name");

    ai->onStart();
    ai->onEnd(true);
    ai->onFrame();
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

std::unique_ptr <AIModuleWrapper> createAIModuleWrapper(rust::Box<BoxedAIModule> box) {
    return std::unique_ptr<AIModuleWrapper>(new AIModuleWrapper(box.into_raw()));
}
//void destroyAIModuleWrapper(std::unique_ptr<AIModuleWrapper> module) {
//    delete reinterpret_cast<AIModuleWrapper*>(module);
//}

void sendText(BWAPI::Game *game, rust::Str text) {
    std::string s(text);
    game->sendText(s.c_str());
}

std::unique_ptr<UnitsetRefIterator> buildUnitset(const BWAPI::Unitset &container) {
    return std::unique_ptr<UnitsetRefIterator>(new RefIterator(container));
}

const BWAPI::UnitInterface *UnitsetRefIterator_next(UnitsetRefIterator &uri) {
    return uri.next();
}

int Unit_getId(const BWAPI::UnitInterface *unit) {
    return unit->getID();
}
