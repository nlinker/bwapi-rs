#pragma once

// https://github.com/dtolnay/cxx/issues/796
using c_void = void;

#include "nameof.hpp"
#include "BWAPI/AIModule.h"
#include "library/src/lib.rs.h"
#include "../openbw/bwapilib/include/BWAPI/Game.h"

int cpp_test();

// helper for debugging on the c++ side
template <class T>
void dump(T const *t) {
    auto p = reinterpret_cast<unsigned long const *>(t);
    for (size_t n = 0 ; n < sizeof(T) ; ++n)
        printf("0x%012lx ", p[n]);
    printf("\n");
}

std::unique_ptr <AIModuleWrapper> createAIModuleWrapper(rust::Box<BoxedAIModule> box);

class AIModuleWrapper : public BWAPI::AIModule {
public:
    BoxedAIModule *moduleBox;
public:
    AIModuleWrapper(BoxedAIModule *box): moduleBox(box)  {}

    BoxedAIModule& getBox() { return *moduleBox; }

    void onStart() noexcept override { on_start(*this); }

    void onEnd(bool isWinner) noexcept override { on_end(*this, isWinner); }

    void onFrame() noexcept override { on_frame(*this); }

    void onSendText(std::string text) noexcept override { on_send_text(*this, text); }

    void onReceiveText(BWAPI::Player player, std::string text) noexcept override { on_receive_text(*this, player, text); }

    void onPlayerLeft(BWAPI::Player player) noexcept override { on_player_left(*this, player); }

    void onNukeDetect(BWAPI::Position target) noexcept override { on_nuke_detect(*this, target); }

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

void sendText(BWAPI::Game *game, rust::Str text);
//int getFrameCount(BWAPI::Game* game);
// void printTypeInfo(const void *obj);

template <typename T>
void printTypeInfo(T obj) {
    std::cout << "typeof(" << obj << ") = " << NAMEOF_TYPE(T) << std::endl;
}
