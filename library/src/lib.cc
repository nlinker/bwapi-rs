#include <BWAPI.h>
#include <BWAPI/Game.h>
#include <BWAPI/AIModule.h>

#include "lib.h"
#include "library/src/lib.rs.h"

int cpp_main() {
    std::cout << "Hello, there" << std::endl;
    AimBox &box = const_cast<AimBox &>(hack());
    auto ai = new AIModuleWrapper(box);

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

std::unique_ptr <AIModuleWrapper> createAIModuleWrapper(AimBox &box) {
    return std::unique_ptr<AIModuleWrapper>(new AIModuleWrapper(box));
}
//void destroyAIModuleWrapper(std::unique_ptr<AIModuleWrapper> module) {
//    delete reinterpret_cast<AIModuleWrapper*>(module);
//}
