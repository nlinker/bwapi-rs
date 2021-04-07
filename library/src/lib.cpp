#include "BWAPI.h"
//#include "BWAPI/Game.h"
#include "BWAPI/AIModule.h"
#include "lib.h"

#define DLLEXPORT
extern "C" DLLEXPORT void gameInit(BWAPI::Game *game) { BWAPI::BroodwarPtr = game; }
extern "C" DLLEXPORT BWAPI::AIModule* newAIModule() { return new TestAI(); }


