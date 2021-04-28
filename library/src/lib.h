#pragma once

#include "aim.h"
#include "aliases.h"
#include "iterator.h"
#include "BWAPI/AIModule.h"
#include "BWAPI/Game.h"
#include "BWAPI/Unit.h"
#include "library/src/lib.rs.h"
#include "../openbw/bwapilib/include/BWAPI/Game.h"

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
using BestUnitFilter = rust::Fn<BWAPI::Unit(BWAPI::Unit, BWAPI::Unit)>;

const BWAPI::UnitInterface *_unitset_getClosestUnit(const BWAPI::Unitset &set, UnitFilter pred, int radius);
std::unique_ptr<UnitsetIterator> _unitset_getInterceptors(const BWAPI::Unitset &set);
std::unique_ptr<UnitsetIterator> _unitset_getLarva(const BWAPI::Unitset &set);
std::unique_ptr<UnitsetIterator> _unitset_getLoadedUnits(const BWAPI::Unitset &set);
std::unique_ptr<UnitsetIterator> _unitset_getUnitsInRadius(const BWAPI::Unitset &set, int radius, UnitFilter pred);
bool _unitset_move(const BWAPI::Unitset &set, BWAPI::Position target, bool shift_queue_command);

void _game_debug(const BWAPI::Game &game);
BWAPI::UnitInterface* _game_getBestUnit(const BWAPI::Game &game, BestUnitFilter best, UnitFilter pred, BWAPI::Position center, int radius);
BWAPI::UnitInterface *_game_getClosestUnit(const BWAPI::Game &game, BWAPI::Position center, UnitFilter pred, int radius);
BWAPI::UnitInterface *_game_getClosestUnitInRectangle(const BWAPI::Game &game, BWAPI::Position center, UnitFilter pred, int left, int top, int right, int bottom);
std::unique_ptr<UnitsetIterator> _game_getUnitsInRadius(const BWAPI::Game &game, BWAPI::Position position, int radius, UnitFilter pred);
std::unique_ptr<UnitsetIterator> _game_getUnitsInRectangle(const BWAPI::Game &game, BWAPI::Position topLeft, BWAPI::Position bottomRight, UnitFilter pred);
std::unique_ptr<UnitsetIterator> _game_getUnitsOnTile(const BWAPI::Game &game, BWAPI::TilePosition tile, UnitFilter pred);
std::unique_ptr<std::string> _game_mapFileName(const BWAPI::Game& game);
std::unique_ptr<std::string> _game_mapHash(const BWAPI::Game& game);
std::unique_ptr<std::string> _game_mapName(const BWAPI::Game& game);
std::unique_ptr<std::string> _game_mapPathName(const BWAPI::Game& game);
void _game_printf(BWAPI::Game &game, rust::Str text);
BWAPI::PlayerInterface *_game_self(const BWAPI::Game &game);
void _game_sendText(BWAPI::Game &game, rust::Str text);
void _game_sendTextEx(BWAPI::Game &game, bool toAllies, rust::Str text);
bool _game_setMap(BWAPI::Game &game, rust::Str text);

int Unit_getId(const BWAPI::UnitInterface *unit);
BWAPI::UnitType Unit_getType(const BWAPI::UnitInterface *unit);
BWAPI::Position Unit_getPosition(const BWAPI::UnitInterface *unit);
