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
    ai->onSendText(std::string("Hello, this is the text!"));
    return 0;
}


std::unique_ptr <AIModuleWrapper> createAIModuleWrapper() {
    return std::make_unique<AIModuleWrapper>();
}

void AIModuleWrapper::onSendText(std::string text) noexcept {
    onSendText_shim(*this, std::make_unique<std::string>(text));
}

//void onSendText_123(AIModuleWrapper& self, std::string& text) {
//    // ???
//}

//void destroyAIModuleWrapper(/* BWAPI::AIModule* */ void* module) {
//    delete reinterpret_cast<AIModuleWrapper*>(module);
//}
