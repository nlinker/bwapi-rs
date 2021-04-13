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
    BWAPI::Player player = reinterpret_cast<BWAPI::Player>(0xDEADBEEF);
    ai->onReceiveText(player, std::string("Hello, this is the received text!"));
    return 0;
}


std::unique_ptr <AIModuleWrapper> createAIModuleWrapper() {
    return std::make_unique<AIModuleWrapper>();
}

void AIModuleWrapper::onStart() noexcept {
    onStart_shim(*this);
}
void AIModuleWrapper::onEnd(bool isWinner) noexcept {
    onEnd_shim(*this, isWinner);
}
void AIModuleWrapper::onFrame() noexcept {
    onFrame_shim(*this);
}
void AIModuleWrapper::onSendText(std::string text) noexcept {
    onSendText_shim(*this, std::make_unique<std::string>(text));
}
void AIModuleWrapper::onReceiveText(BWAPI::Player player, std::string text) noexcept {
    Player p {.raw = player};
    onReceiveText_shim(*this, p, std::make_unique<std::string>(text));
}

//void onSendText_123(AIModuleWrapper& self, std::string& text) {
//    // ???
//}

//void destroyAIModuleWrapper(/* BWAPI::AIModule* */ void* module) {
//    delete reinterpret_cast<AIModuleWrapper*>(module);
//}
