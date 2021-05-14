#include <BWAPI/Game.h>
#include <BWAPI/Unit.h>
#include "lib.h"
#include "library/src/lib.rs.h"
//#include "../../target/debug/build/library-10a440775d1794e8/out/cxxbridge/include/library/src/lib.rs.h"

int cpp_test() {
    std::cout << "cpp_test started" << std::endl;
    auto &box = const_cast<BoxedAIModule &>(hack());
    auto ai = new AIModuleWrapper(&box);

    BWAPI::Position target(11, 22);
    auto player = reinterpret_cast<BWAPI::Player>(0xDEADBEEF);
    auto unit0 = reinterpret_cast<BWAPI::Unit>(0xF00D0000);
    auto unit1 = reinterpret_cast<BWAPI::Unit>(0xF00D0001);
    auto unit2 = reinterpret_cast<BWAPI::Unit>(0xF00D0002);
    auto unit3 = reinterpret_cast<BWAPI::Unit>(0xF00D0003);
    auto unit4 = reinterpret_cast<BWAPI::Unit>(0xF00D0004);
    auto unit5 = reinterpret_cast<BWAPI::Unit>(0xF00D0005);
    auto unit6 = reinterpret_cast<BWAPI::Unit>(0xF00D0006);
    auto unit7 = reinterpret_cast<BWAPI::Unit>(0xF00D0007);
    //BWAPI::Unit unit8 = reinterpret_cast<BWAPI::Unit>(0xF00D0008);
    auto unit9 = reinterpret_cast<BWAPI::Unit>(0xF00D0009);
    std::string s1("Hello sent");
    std::string s2("Hello received");
    std::string s3("Saving game name");

    ai->onStart();
    ai->onEnd(true);
    ai->onFrame();
    ai->onSendText(s1);
    ai->onReceiveText(player, s2);
    ai->onPlayerLeft(player);
    ai->onNukeDetect(target);
    ai->onUnitDiscover(unit0);
    ai->onUnitEvade(unit1);
    ai->onUnitShow(unit2);
    ai->onUnitHide(unit3);
    ai->onUnitCreate(unit4);
    ai->onUnitDestroy(unit5);
    ai->onUnitMorph(unit6);
    ai->onUnitRenegade(unit7);
    ai->onSaveGame(s3);
    ai->onUnitComplete(unit9);
    return 0;
}

std::unique_ptr <AIModuleWrapper> createAIModuleWrapper(rust::Box <BoxedAIModule> box) {
    return std::unique_ptr<AIModuleWrapper>(new AIModuleWrapper(box.into_raw()));
}

void _game_debug(const BWAPI::Game &game, UnitFilter fun) {
    // rust::Fn<bool(BWAPI::UnitInterface*)> fun
    using namespace BWAPI;
    Game &g = const_cast<Game&>(game);
    Position c(250, 3160);
    auto units = g.getUnitsInRadius(c, 100, *fun);
    for (auto& unit: units) {
        std::ostringstream os;
        os << "in radius unit: " << unit->getID() << ", " << unit->getType();
        g.sendText(os.str().c_str());
    }
}

// region === === Iterators === ===
std::unique_ptr<BulletsetIterator> createBulletsetIterator(const BWAPI::Bulletset &set) {
    return std::unique_ptr<BulletsetIterator>(new BulletsetIterator(set));
}

std::unique_ptr<ForcesetIterator> createForcesetIterator(const BWAPI::Forceset &set) {
    return std::unique_ptr<ForcesetIterator>(new ForcesetIterator(set));
}

std::unique_ptr<PlayersetIterator> createPlayersetIterator(const BWAPI::Playerset &set) {
    return std::unique_ptr<PlayersetIterator>(new PlayersetIterator(set));
}

std::unique_ptr<RegionsetIterator> createRegionsetIterator(const BWAPI::Regionset &set) {
    return std::unique_ptr<RegionsetIterator>(new RegionsetIterator(set));
}

std::unique_ptr<UnitsetIterator> createUnitsetIterator(const BWAPI::Unitset &set) {
    return std::unique_ptr<UnitsetIterator>(new UnitsetIterator(set));
}
// endregion

std::unique_ptr<BWAPI::Bulletset> _bulletset_dummy(const BWAPI::Bulletset &) {
    return std::make_unique<BWAPI::Bulletset>(BWAPI::Bulletset());
}

std::unique_ptr<BWAPI::Forceset> _forceset_dummy(const BWAPI::Forceset &) {
    return std::make_unique<BWAPI::Forceset>(BWAPI::Forceset());
}

std::unique_ptr<BWAPI::Playerset> _forceset_getPlayers(const BWAPI::Forceset &set) {
    return std::make_unique<BWAPI::Playerset>(set.getPlayers());
}

std::unique_ptr<BWAPI::Regionset> _regionset_dummy(const BWAPI::Regionset &) {
    return std::make_unique<BWAPI::Regionset>(BWAPI::Regionset());
}
std::unique_ptr<BWAPI::Unitset> _regionset_getUnits(const BWAPI::Regionset &set, UnitFilter pred) {
    return std::make_unique<BWAPI::Unitset>(set.getUnits(nullptr /*todo*/));
}

// region === === Playerset === ===
rust::Vec<BWAPI::Race> _playerset_getRaces(const BWAPI::Playerset &set) {
    auto races = set.getRaces();
    rust::Vec<BWAPI::Race> xs;
    xs.reserve(races.size());
    for (auto &race: races) {
        xs.push_back(race);
    }
    return xs;
}

std::unique_ptr<BWAPI::Unitset> _playerset_getUnits(const BWAPI::Playerset &set) {
    return std::make_unique<BWAPI::Unitset>(set.getUnits());
}
// endregion

// region === === Unitset === ===
BWAPI::UnitInterface *_unitset_getClosestUnit(const BWAPI::Unitset &set, UnitFilter pred, int radius) {
    return set.getClosestUnit(nullptr /* todo convert predicate */, radius);
}

std::unique_ptr<BWAPI::Unitset> _unitset_getInterceptors(const BWAPI::Unitset &set) {
    return std::make_unique<BWAPI::Unitset>(set.getInterceptors());
}

std::unique_ptr<BWAPI::Unitset> _unitset_getLarva(const BWAPI::Unitset &set) {
    return std::make_unique<BWAPI::Unitset>(set.getLarva());
}

std::unique_ptr<BWAPI::Unitset> _unitset_getLoadedUnits(const BWAPI::Unitset &set) {
    return std::make_unique<BWAPI::Unitset>(set.getLoadedUnits());
}

std::unique_ptr<BWAPI::Unitset> _unitset_getUnitsInRadius(const BWAPI::Unitset &set, int radius, rust::Fn<bool(BWAPI::Unit)> pred) {
    return std::make_unique<BWAPI::Unitset>(set.getUnitsInRadius(radius, nullptr /* todo convert predicate */));
}

bool _unitset_move(const BWAPI::Unitset &set, BWAPI::Position target, bool shift_queue_command) {
    return set.move(target, shift_queue_command);
}
// endregion

// region === === Force === ===
std::unique_ptr<std::string> _force_getName(const BWAPI::ForceInterface& force) {
    return std::make_unique<std::string>(force.getName());
}

std::unique_ptr<BWAPI::Playerset> _force_getPlayers(const BWAPI::ForceInterface& force) {
    return std::make_unique<BWAPI::Playerset>(force.getPlayers());
}
// endregion

// region === === Game === ===
BWAPI::UnitInterface *_game_getBestUnit(const BWAPI::Game &game, BestUnitFilter best, UnitFilter pred, BWAPI::Position center, int radius) {
    return game.getBestUnit(best, pred, center, radius);
}

BWAPI::UnitInterface *_game_getClosestUnit(const BWAPI::Game &game, BWAPI::Position center, UnitFilter pred, int radius) {
    return game.getClosestUnit(center, pred /*todo*/, radius);
}

BWAPI::UnitInterface *_game_getClosestUnitInRectangle(const BWAPI::Game &game, BWAPI::Position center, UnitFilter pred, int left, int top, int right, int bottom) {
    return game.getClosestUnitInRectangle(center, pred /*todo*/, left, top, right, bottom);
}

const EventList& _game_getEvents(const BWAPI::Game &game) {
    return game.getEvents();
}

rust::Vec<BWAPI::Position> _game_getNukeDots(const BWAPI::Game& game) {
    auto &dots = game.getNukeDots();
    rust::Vec<BWAPI::Position> xs;
    xs.reserve(dots.size());
    for (auto &dot: dots) {
        xs.push_back(dot);
    }
    return xs;
}

rust::Vec<BWAPI::TilePosition> _game_getStartLocations(const BWAPI::Game& game) {
    auto &tiles = game.getStartLocations();
    rust::Vec<BWAPI::TilePosition> xs;
    xs.reserve(tiles.size());
    for (auto &tile: tiles) {
        xs.push_back(tile);
    }
    return xs;
}

std::unique_ptr<BWAPI::Unitset> _game_getUnitsInRadius(const BWAPI::Game &game, BWAPI::Position position, int radius, UnitFilter pred) {
    return std::make_unique<BWAPI::Unitset>(game.getUnitsInRadius(position, radius, nullptr /*todo*/));
}

std::unique_ptr<BWAPI::Unitset> _game_getUnitsInRectangle(const BWAPI::Game &game, BWAPI::Position topLeft, BWAPI::Position bottomRight, UnitFilter pred) {
    return std::make_unique<BWAPI::Unitset>(game.getUnitsInRectangle(topLeft, bottomRight, nullptr /*todo*/));
}

std::unique_ptr<BWAPI::Unitset> _game_getUnitsOnTile(const BWAPI::Game &game, BWAPI::TilePosition tile, UnitFilter pred) {
    return std::make_unique<BWAPI::Unitset>(game.getUnitsOnTile(tile, nullptr /*todo*/));
}

void _game_enableFlag(BWAPI::Game &game, BWAPI::Flag::Enum flag) {
    game.enableFlag(static_cast<int>(flag));
}

bool _game_isFlagEnabled(const BWAPI::Game &game, BWAPI::Flag::Enum flag) {
    return game.isFlagEnabled(static_cast<int>(flag));
}

std::unique_ptr<std::string> _game_mapFileName(const BWAPI::Game& game) {
    return std::make_unique<std::string>(game.mapFileName());
}

std::unique_ptr<std::string> _game_mapHash(const BWAPI::Game& game) {
    return std::make_unique<std::string>(game.mapHash());
}

std::unique_ptr<std::string> _game_mapName(const BWAPI::Game& game) {
    return std::make_unique<std::string>(game.mapName());
}

std::unique_ptr<std::string> _game_mapPathName(const BWAPI::Game& game) {
    return std::make_unique<std::string>(game.mapPathName());
}

void _game_printf(BWAPI::Game &game, rust::Str text) {
    std::string s(text);
    game.printf(s.c_str());
}

BWAPI::PlayerInterface *_game_self(const BWAPI::Game &game) {
    return game.self();
}

void _game_sendText(BWAPI::Game &game, rust::Str text) {
    std::string s(text);
    game.sendText(s.c_str());
}

void _game_sendTextEx(BWAPI::Game &game, bool toAllies, rust::Str text) {
    std::string s(text);
    game.sendTextEx(toAllies, s.c_str());
}

bool _game_setMap(BWAPI::Game &game, rust::Str text) {
    std::string s(text);
    return game.setMap(s.c_str());
}

void _game_drawText(BWAPI::Game &game, BWAPI::CoordinateType::Enum ctype, int x, int y, rust::Str text) {
    std::string s(text);
    game.drawText(ctype, x, y, s.c_str());
}
// endregion

// region === === Player === ===
std::unique_ptr<std::string> _player_getName(const BWAPI::PlayerInterface& player) {
    return std::make_unique<std::string>(player.getName());
}

BWAPI::Text::Enum _player_getTextColor(const BWAPI::PlayerInterface& player) {
    switch (static_cast<int>(player.getTextColor())) {
        case 111: // red
            return BWAPI::Text::Enum::BrightRed;
        case 165: // blue
            return BWAPI::Text::Enum::Blue;
        case 159: // teal
            return BWAPI::Text::Enum::Teal;
        case 164: // purp
            return BWAPI::Text::Enum::Purple;
        case 156: // orange
            return BWAPI::Text::Enum::Orange;
        case 19:  // brown
            return BWAPI::Text::Enum::Brown;
        case 84:  // white
            return BWAPI::Text::Enum::PlayerWhite;
        case 135: // yellow
            return BWAPI::Text::Enum::PlayerYellow;
        case 185: // green p9
            return BWAPI::Text::Enum::DarkGreen;
        case 136: // p10
            return BWAPI::Text::Enum::LightYellow;
        case 134: // p11
            return BWAPI::Text::Enum::Tan;
        case 51:  // p12
            return BWAPI::Text::Enum::GreyBlue;
        default:
            return BWAPI::Text::Enum::Default;
    }
}



// endregion

// region === === Region === ===
std::unique_ptr<BWAPI::Unitset> _region_getUnits(const BWAPI::RegionInterface& region, UnitFilter pred) {
    return std::make_unique<BWAPI::Unitset>(region.getUnits(nullptr /*todo*/));
}
// endregion

// region === === Unit === ===
BWAPI::UnitInterface *_unit_getClosestUnit(const BWAPI::UnitInterface &unit, UnitFilter pred, int radius) {
    return unit.getClosestUnit(nullptr /*todo*/, radius);
}
std::unique_ptr<BWAPI::Unitset> _unit_getInterceptors(const BWAPI::UnitInterface &unit) {
    return std::make_unique<BWAPI::Unitset>(unit.getInterceptors());
}
std::unique_ptr<BWAPI::Unitset> _unit_getLarva(const BWAPI::UnitInterface &unit) {
    return std::make_unique<BWAPI::Unitset>(unit.getLarva());
}
std::unique_ptr<BWAPI::Unitset> _unit_getLoadedUnits(const BWAPI::UnitInterface &unit) {
    return std::make_unique<BWAPI::Unitset>(unit.getLoadedUnits());
}
rust::Vec<BWAPI::UnitType> _unit_getTrainingQueue(const BWAPI::UnitInterface &unit) {
    auto uTypes = unit.getTrainingQueue();
    rust::Vec<BWAPI::UnitType> xs;
    xs.reserve(uTypes.size());
    for (auto &tile: uTypes) {
        xs.push_back(tile);
    }
    return xs;
}
std::unique_ptr<BWAPI::Unitset> _unit_getUnitsInRadius(const BWAPI::UnitInterface &unit, int radius, UnitFilter pred) {
    return std::make_unique<BWAPI::Unitset>(unit.getUnitsInRadius(radius, nullptr /*todo*/));
}
std::unique_ptr<BWAPI::Unitset> _unit_getUnitsInWeaponRange(const BWAPI::UnitInterface &unit, BWAPI::WeaponType wType, UnitFilter pred) {
    return std::make_unique<BWAPI::Unitset>(unit.getUnitsInWeaponRange(wType, nullptr /*todo*/));
}
bool _unit_move(BWAPI::UnitInterface &unit, BWAPI::Position target, bool shiftQueueCommand) {
    return unit.move(target, shiftQueueCommand);
}
// endregion