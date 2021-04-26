#pragma once

#include "aim.h"
#include "aliases.h"
#include "iterator.h"
#include "BWAPI/AIModule.h"
#include "BWAPI/Game.h"
#include "BWAPI/Unit.h"
#include "library/src/lib.rs.h"

int cpp_test();

// api-specific stuff below
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

using UnitFilter = rust::Fn<bool(BWAPI::Unit)>;

const BWAPI::UnitInterface *getClosestUnit(const BWAPI::Unitset &set, UnitFilter pred, int radius);
std::unique_ptr<UnitIterator> getInterceptors(const BWAPI::Unitset &set);
std::unique_ptr<UnitIterator> getLarva(const BWAPI::Unitset &set);
std::unique_ptr<UnitIterator> getLoadedUnits(const BWAPI::Unitset &set);
std::unique_ptr<UnitIterator> getUnitsInRadius_Unitset(const BWAPI::Unitset &set, int radius, UnitFilter pred);
bool move_(const BWAPI::Unitset &set, BWAPI::Position target, bool shift_queue_command);

void Game_debug(BWAPI::Game *game);
void sendText(BWAPI::Game *game, rust::Str text);
std::unique_ptr<UnitIterator> getAllUnits(BWAPI::Game *game);
std::unique_ptr<UnitIterator> getUnitsInRadius_Game(BWAPI::Game *game, BWAPI::Position position, int radius, UnitFilter pred);

int Unit_getId(const BWAPI::UnitInterface *unit);
BWAPI::UnitType Unit_getType(const BWAPI::UnitInterface *unit);
BWAPI::Position Unit_getPosition(const BWAPI::UnitInterface *unit);
