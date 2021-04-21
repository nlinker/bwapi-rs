//#include <BWAPI.h>
//#include <BWAPI/Game.h>
//#include <BWAPI/AIModule.h>

#include "lib.h"
#include "library/src/lib.rs.h"
#include "../openbw/bwapilib/include/BWAPI/Game.h"

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
    using namespace BWAPI;

    printTypeInfo(&Broodwar);
    printTypeInfo(&game->getForces());
    printTypeInfo(&game->getAllUnits());
    printTypeInfo(game->self());

    //    printTypeInfo(reinterpret_cast<const void*>(game));
    //    printTypeInfo(reinterpret_cast<const void*>(&game->getForces()));
    //    printTypeInfo(reinterpret_cast<const void*>(&game->getAllUnits()));
    //    printTypeInfo(reinterpret_cast<const void*>(game->self()));
    //    printTypeInfo(reinterpret_cast<const void*>(&game->self()->getUnits()));
    //    std::cout << game->getForces();

    std::string s(text);
    game->sendText(s.c_str());
}

//int getFrameCount(BWAPI::Game *game) {
//    return game->getFrameCount();
//}

//    for (auto &u : game->self()->getUnits()) {
//        // First line is command
//        std::ostringstream debug;
//        debug << "cmd=" << u->getLastCommand().getType() << ";f="
//              << (Broodwar->getFrameCount() - u->getLastCommandFrame());
//        if (u->getLastCommand().getTarget()) {
//            debug << ";tgt=" << u->getLastCommand().getTarget()->getType()
//                  << "#" << u->getLastCommand().getTarget()->getID()
//                  << "@" << WalkPosition(u->getLastCommand().getTarget()->getPosition())
//                  << ";d=" << u->getLastCommand().getTarget()->getDistance(u);
//        } else if (u->getLastCommand().getTargetPosition()) {
//            debug << ";tgt=" << WalkPosition(u->getLastCommand().getTargetPosition());
//        }
//
//        // Next line is order
//        debug << "\nord=" << u->getOrder() << ";t=" << u->getOrderTimer();
//        if (u->getOrderTarget()) {
//            debug << ";tgt=" << u->getOrderTarget()->getType()
//                  << "#" << u->getOrderTarget()->getID()
//                  << "@" << WalkPosition(u->getOrderTarget()->getPosition())
//                  << ";d=" << u->getOrderTarget()->getDistance(u);
//        } else if (u->getOrderTargetPosition()) {
//            debug << ";tgt=" << WalkPosition(u->getOrderTargetPosition());
//        }
//
//        // Last line is movement data
//        debug << "\n";
//        if (u->getType().topSpeed() > 0.001) {
//            auto speed = sqrt(u->getVelocityX() * u->getVelocityX()
//                              + u->getVelocityY() * u->getVelocityY());
//            debug << "spd=" << ((int) (100.0 * speed / u->getType().topSpeed()))
//                  << ";mvng=" << u->isMoving()
//                  << ";stk=" << u->isStuck();
//        }
//
//        std::cout << debug.str() << std::endl;
//    }
