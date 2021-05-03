#pragma once

#include <BWAPI/AIModule.h>
#include <BWAPI/Game.h>
#include <BWAPI/Unit.h>
#include <BWAPI/Position.h>
#include "aim.h"
#include "aliases.h"
#include "iterator.h"
#include "library/src/lib.rs.h"
//#include "../../target/debug/build/library-10a440775d1794e8/out/cxxbridge/include/library/src/lib.rs.h"

int cpp_test();

// api-specific stuff below
std::unique_ptr<AIModuleWrapper> createAIModuleWrapper(rust::Box<BoxedAIModule> box);

class AIModuleWrapper : public BWAPI::AIModule {
public:
    BoxedAIModule *moduleBox;
public:
    AIModuleWrapper(BoxedAIModule *box) : moduleBox(box) {}

    BoxedAIModule &getBox() { return *moduleBox; }

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
std::unique_ptr<BWAPI::Unitset> _unitset_getInterceptors(const BWAPI::Unitset &set);
std::unique_ptr<BWAPI::Unitset> _unitset_getLarva(const BWAPI::Unitset &set);
std::unique_ptr<BWAPI::Unitset> _unitset_getLoadedUnits(const BWAPI::Unitset &set);
std::unique_ptr<BWAPI::Unitset> _unitset_getUnitsInRadius(const BWAPI::Unitset &set, int radius, rust::Fn<bool(BWAPI::Unit)> pred);
bool _unitset_move(const BWAPI::Unitset &set, BWAPI::Position target, bool shift_queue_command);

std::unique_ptr<std::string> _force_getName(const BWAPI::ForceInterface& force);
std::unique_ptr<BWAPI::Playerset> _force_getPlayers(const BWAPI::ForceInterface& force);

std::unique_ptr<BWAPI::Playerset> _game_debug(const BWAPI::Game &game);
void _game_debug_fun(const BWAPI::Game &game, UnitFilter fun);
BWAPI::UnitInterface *_game_getBestUnit(const BWAPI::Game &game, BestUnitFilter best, UnitFilter pred, BWAPI::Position center, int radius);
BWAPI::UnitInterface *_game_getClosestUnit(const BWAPI::Game &game, BWAPI::Position center, UnitFilter pred, int radius);
BWAPI::UnitInterface *_game_getClosestUnitInRectangle(const BWAPI::Game &game, BWAPI::Position center, UnitFilter pred, int left, int top, int right, int bottom);
const EventList &_game_getEvents(const BWAPI::Game &game);
rust::Vec<BWAPI::Position> _game_getNukeDots(const BWAPI::Game &game);
rust::Vec<BWAPI::TilePosition> _game_getStartLocations(const BWAPI::Game &game);
std::unique_ptr<BWAPI::Unitset> _game_getUnitsInRadius(const BWAPI::Game &game, BWAPI::Position position, int radius, UnitFilter pred);
std::unique_ptr<BWAPI::Unitset> _game_getUnitsInRectangle(const BWAPI::Game &game, BWAPI::Position topLeft, BWAPI::Position bottomRight, UnitFilter pred);
std::unique_ptr<BWAPI::Unitset> _game_getUnitsOnTile(const BWAPI::Game &game, BWAPI::TilePosition tile, UnitFilter pred);
void _game_enableFlag(BWAPI::Game &game, BWAPI::Flag::Enum flag);
bool _game_isFlagEnabled(const BWAPI::Game &game, BWAPI::Flag::Enum flag);
std::unique_ptr<std::string> _game_mapFileName(const BWAPI::Game &game);
std::unique_ptr<std::string> _game_mapHash(const BWAPI::Game &game);
std::unique_ptr<std::string> _game_mapName(const BWAPI::Game &game);
std::unique_ptr<std::string> _game_mapPathName(const BWAPI::Game &game);
void _game_printf(BWAPI::Game &game, rust::Str text);
BWAPI::PlayerInterface *_game_self(const BWAPI::Game &game);
void _game_sendText(BWAPI::Game &game, rust::Str text);
void _game_sendTextEx(BWAPI::Game &game, bool toAllies, rust::Str text);
bool _game_setMap(BWAPI::Game &game, rust::Str text);
void _game_drawText(BWAPI::Game &game, BWAPI::CoordinateType::Enum ctype, int x, int y, rust::Str text);

std::unique_ptr<std::string> _player_getName(const BWAPI::PlayerInterface &player);
BWAPI::Text::Enum _player_getTextColor(const BWAPI::PlayerInterface &player);

std::unique_ptr<BWAPI::Unitset> _region_getUnits(const BWAPI::RegionInterface &region, UnitFilter pred);

BWAPI::UnitInterface *_unit_getClosestUnit(const BWAPI::UnitInterface &unit, UnitFilter pred, int radius);
std::unique_ptr<::BWAPI::Unitset> _unit_getInterceptors(const BWAPI::UnitInterface &unit);
std::unique_ptr<::BWAPI::Unitset> _unit_getLarva(const BWAPI::UnitInterface &unit);
std::unique_ptr<::BWAPI::Unitset> _unit_getLoadedUnits(const BWAPI::UnitInterface &unit);
rust::Vec<::BWAPI::UnitType> _unit_getTrainingQueue(const BWAPI::UnitInterface &unit);
std::unique_ptr<::BWAPI::Unitset> _unit_getUnitsInRadius(const BWAPI::UnitInterface &unit, int radius, UnitFilter pred);
std::unique_ptr<::BWAPI::Unitset> _unit_getUnitsInWeaponRange(const BWAPI::UnitInterface &unit, BWAPI::WeaponType wType, UnitFilter pred);
bool _unit_move(const ::BWAPI::UnitInterface &unit, BWAPI::Position position, bool shiftQueueCommand);

int Unit_getId(const BWAPI::UnitInterface *unit);
BWAPI::UnitType Unit_getType(const BWAPI::UnitInterface *unit);
BWAPI::Position Unit_getPosition(const BWAPI::UnitInterface *unit);

static_assert(sizeof(BWAPI::BulletType) == 4, "BWAPI::BulletType size is not correct");
static_assert(sizeof(BWAPI::Color) == 4, "BWAPI::Color size is not correct");
static_assert(sizeof(BWAPI::DamageType) == 4, "BWAPI::DamageType size is not correct");
static_assert(sizeof(BWAPI::Error) == 4, "BWAPI::Error size is not correct");
static_assert(sizeof(BWAPI::ExplosionType) == 4, "BWAPI::ExplosionType size is not correct");
static_assert(sizeof(BWAPI::GameType) == 4, "BWAPI::GameType size is not correct");
static_assert(sizeof(BWAPI::Order) == 4, "BWAPI::Order size is not correct");
static_assert(sizeof(BWAPI::PlayerType) == 4, "BWAPI::PlayerType size is not correct");
static_assert(sizeof(BWAPI::Race) == 4, "BWAPI::Race size is not correct");
static_assert(sizeof(BWAPI::TechType) == 4, "BWAPI::TechType size is not correct");
static_assert(sizeof(BWAPI::UnitCommand) == 40, "BWAPI::UnitCommand size is not correct");
static_assert(sizeof(BWAPI::UnitCommandType) == 4, "BWAPI::UnitCommandType size is not correct");
static_assert(sizeof(BWAPI::UnitSizeType) == 4, "BWAPI::UnitSizeType size is not correct");
static_assert(sizeof(BWAPI::UnitType) == 4, "BWAPI::UnitType size is not correct");
static_assert(sizeof(BWAPI::UpgradeType) == 4, "BWAPI::UpgradeType size is not correct");
static_assert(sizeof(BWAPI::WeaponType) == 4, "BWAPI::WeaponType size is not correct");
static_assert(sizeof(BWAPI::Text::Size::Enum) == 4, "BWAPI::Text::Size::Enum size is not correct");
