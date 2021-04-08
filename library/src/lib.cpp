#include "BWAPI.h"
//#include "BWAPI/Game.h"
#include "BWAPI/AIModule.h"
#include "lib.h"
#include "library/src/lib.rs.h"

#define DLLEXPORT
extern "C" DLLEXPORT void gameInit(BWAPI::Game *game) { BWAPI::BroodwarPtr = game; }
extern "C" DLLEXPORT BWAPI::AIModule* newAIModule() { return new TestAI(); }

void cpp_main() {
    std::cout << "Hello, there" << std::endl;
}
