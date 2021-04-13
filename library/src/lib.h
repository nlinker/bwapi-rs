#pragma once
#include "BWAPI/AIModule.h"
#include "library/src/lib.rs.h"

int cpp_main();

// NOTE: the implementation of the class in Rust, so it is expected
// warnings from C++ compiler/IDE about no implementation found
class AIModuleWrapper: public BWAPI::AIModule {
public:
    const AimBox& aimBox;
public:
    AIModuleWrapper(const AimBox& box): aimBox(box) {}
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

std::unique_ptr<AIModuleWrapper> createAIModuleWrapper(const AimBox& box);
const AimBox& getAimBox(const AIModuleWrapper& wrapper);
