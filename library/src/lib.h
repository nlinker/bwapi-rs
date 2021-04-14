#pragma once
#include "BWAPI/AIModule.h"
#include "library/src/lib.rs.h"

int cpp_main();

class AIModuleWrapper: public BWAPI::AIModule {
public:
    AimBox& aimBox;
public:
    AIModuleWrapper(AimBox& box): aimBox(box) {}
    AimBox& getAimBox() { return aimBox; }

    void onStart() noexcept override;
    void onEnd(bool isWinner) noexcept override;
    void onFrame() noexcept override;
    void onSendText(std::string text) noexcept override;
    void onReceiveText(BWAPI::Player player, std::string text) noexcept override;
    void onPlayerLeft(BWAPI::Player player) noexcept override;
    void onNukeDetect(BWAPI::Position target) noexcept override;
    void onUnitDiscover(BWAPI::Unit unit) noexcept override;
    void onUnitEvade(BWAPI::Unit unit) noexcept override;
    void onUnitShow(BWAPI::Unit unit) noexcept override;
    void onUnitHide(BWAPI::Unit unit) noexcept override;
    void onUnitCreate(BWAPI::Unit unit) noexcept override;
    void onUnitDestroy(BWAPI::Unit unit) noexcept override;
    void onUnitMorph(BWAPI::Unit unit) noexcept override;
    void onUnitRenegade(BWAPI::Unit unit) noexcept override;
    void onSaveGame(std::string gameName) noexcept override;
    void onUnitComplete(BWAPI::Unit unit) noexcept override;
};

std::unique_ptr<AIModuleWrapper> createAIModuleWrapper(AimBox& box);

