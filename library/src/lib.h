#pragma once
#include "BWAPI/AIModule.h"

extern void cpp_main();

class TestAI : public BWAPI::AIModule {
    void onStart() override;
    void onEnd(bool isWinner) override;
    void onFrame() override;
};
