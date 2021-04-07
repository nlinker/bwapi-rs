#pragma once
#include "BWAPI/AIModule.h"

class TestAI : public BWAPI::AIModule {
    void onStart() override;
    void onEnd(bool isWinner) override;
    void onFrame() override;
};
