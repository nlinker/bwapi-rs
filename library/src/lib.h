#pragma once

#include "BWAPI/AIModule.h"
#include "library/src/lib.rs.h"

int cpp_main();

// NOTE: the implementation of the functions `on_*(*this, ..)` are in Rust,
// so it is expected warnings from C++ compiler/IDE about no implementation found
class AIModuleWrapper : public BWAPI::AIModule {
public:
    AimBox &aimBox;
public:
    AIModuleWrapper(AimBox &box) : aimBox(box) {}

    AimBox &getAimBox() { return aimBox; }

    void onStart() noexcept override { on_start(*this); }

    void onEnd(bool isWinner) noexcept override { on_end(*this, isWinner); }

    void onFrame() noexcept override { on_frame(*this); }

    void onSendText(std::string text) noexcept override { on_send_text(*this, text); }

    void onReceiveText(BWAPI::Player player, std::string text) noexcept override { on_receive_text(*this, player, text); }

    void onPlayerLeft(BWAPI::Player player) noexcept override { on_player_left(*this, player); }

    void onNukeDetect(BWAPI::Position target) noexcept override {
        Position *p = reinterpret_cast<Position *>(&target);
        on_nuke_detect(*this, *p);
    }

    void onUnitDiscover(BWAPI::Unit unit) noexcept override { on_unit_discover(*this, unit); }

    void onUnitEvade(BWAPI::Unit unit) noexcept override { on_unit_evade(*this, unit); }

    void onUnitShow(BWAPI::Unit unit) noexcept override { on_unit_show(*this, unit); }

    void onUnitHide(BWAPI::Unit unit) noexcept override { on_unit_hide(*this, unit); }

    void onUnitCreate(BWAPI::Unit unit) noexcept override { on_unit_create(*this, unit); }

    void onUnitDestroy(BWAPI::Unit unit) noexcept override { on_unit_destroy(*this, unit); }

    void onUnitMorph(BWAPI::Unit unit) noexcept override { on_unit_morph(*this, unit); }

    void onUnitRenegade(BWAPI::Unit unit) noexcept override { on_unit_renegade(*this, unit); }

    void onSaveGame(std::string gameName) noexcept override { on_save_game(*this, gameName); }

    void onUnitComplete(BWAPI::Unit unit) noexcept override { on_unit_complete(*this, unit); }
};

std::unique_ptr <AIModuleWrapper> createAIModuleWrapper(AimBox &box);
