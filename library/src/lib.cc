#include <BWAPI.h>
#include <BWAPI/Game.h>
#include <BWAPI/AIModule.h>

#include "lib.h"
#include "library/src/lib.rs.h"

int cpp_main() {
    std::cout << "Hello, there" << std::endl;
    auto ai = new AIModuleWrapper();
    ai->onStart();
    ai->onFrame();
    ai->onEnd(true);
    return 0;
}

std::unique_ptr <AIModuleWrapper> createAIModuleWrapper() {
    return std::make_unique<AIModuleWrapper>();
}

//void destroyAIModuleWrapper(/* BWAPI::AIModule* */ void* module) {
//    delete reinterpret_cast<AIModuleWrapper*>(module);
//}


//void AIModuleWrapper::onStart() {
//    ai->on_start();
//}
//
//void AIModuleWrapper::onEnd(bool isWinner) {
//    ai->on_end(isWinner);
//}
//
//void AIModuleWrapper::onFrame() {
//    ai->on_frame();
//}
