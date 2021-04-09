#include <BWAPI.h>
#include <BWAPI/Game.h>
#include <BWAPI/AIModule.h>

#include "lib.h"
#include "library/src/lib.rs.h"
#include "../bwapilib/include/BWAPI/Game.h"

#define DLLEXPORT
extern "C" {
DLLEXPORT void gameInit(BWAPI::Game *game) { BWAPI::BroodwarPtr = game; }
DLLEXPORT BWAPI::AIModule *newAIModule() { return nullptr; }
};

void cpp_main() {
    std::cout << "Hello, there" << std::endl;
    BWAPI::Game* gameStub = nullptr;
    gameInit(gameStub);
}
