#include <BWAPI/Game.h>
#include <BWAPI/Unit.h>
#include "lib.h"
#include "library/src/lib.rs.h"
//#include "../../target/debug/build/library-10a440775d1794e8/out/cxxbridge/include/library/src/lib.rs.h"

int cpp_test() {
    std::cout << "cpp_test started" << std::endl;
    auto box = const_cast<BoxedAIModule &>(hack());
    auto ai = new AIModuleWrapper(box);

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

std::unique_ptr <AIModuleWrapper> createAIModuleWrapper(BoxedAIModule box) {
    return std::unique_ptr<AIModuleWrapper>(new AIModuleWrapper(box));
}

std::unique_ptr<BWAPI::Playerset> _game_debug(const BWAPI::Game &game) {
    using namespace BWAPI;
    BWAPI::Game &g = const_cast<BWAPI::Game&>(game);
    Text::Size::Enum sizes[] = {Text::Size::Enum::Huge, Text::Size::Enum::Large, Text::Size::Enum::Small, Text::Size::Enum::Default};
    int size = sizeof(sizes)/sizeof(sizes[0]);
    for (int i = 0; i < size; i++) {
        g.setTextSize(sizes[i]);
        g.drawText(CoordinateType::Enum::Map, 1800, 1800 + (i * 50), "Hello, SSCAIT from c++!");
    }
    return std::unique_ptr<BWAPI::Playerset>();
}

void _game_debug_fun(const BWAPI::Game &game, UnitFilter fun) {
    BWAPI::Game &g = const_cast<BWAPI::Game&>(game);
    const BWAPI::UnitFilter &pred = nullptr;
    auto left = 100;
    auto right = 1000;
    auto top = 100;
    auto bottom = 1000;
    BWAPI::Unitset set = g.getUnitsInRectangle(left, top, right, bottom, pred);
    //for (auto &p : set) {
    //    std::ostringstream os;
    //    os << "raw: " << p << "id:" << p->getID() << " type:" << p->getType() << " position:" << p->getPosition();
    //    g.sendText(os.str().c_str());
    //}
    //std::ostringstream os;
    //os << "sizeof(UnitCommand) = " << sizeof(BWAPI::UnitCommand);
    //g.sendText(os.str().c_str());
}

// region === === Iterators === ===
std::unique_ptr<BulletsetIterator> createBulletsetIterator(const BWAPI::Bulletset &set) {
    return std::unique_ptr<BulletsetIterator>(new BulletsetIteratorRef(set));
}

std::unique_ptr<ForcesetIterator> createForcesetIterator(const BWAPI::Forceset &set) {
    return std::unique_ptr<ForcesetIterator>(new ForcesetIteratorRef(set));
}

std::unique_ptr<PlayersetIterator> createPlayersetIterator(const BWAPI::Playerset &set) {
    return std::unique_ptr<PlayersetIterator>(new PlayersetIteratorRef(set));
}

std::unique_ptr<RegionsetIterator> createRegionsetIterator(const BWAPI::Regionset &set) {
    return std::unique_ptr<RegionsetIterator>(new RegionsetIteratorRef(set));
}

std::unique_ptr<UnitsetIterator> createUnitsetIterator(const BWAPI::Unitset &set) {
    return std::unique_ptr<UnitsetIterator>(new UnitsetIteratorRef(set));
}
// endregion

// region === === Unitset === ===
const BWAPI::UnitInterface *_unitset_getClosestUnit(const BWAPI::Unitset &set, UnitFilter pred, int radius) {
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
    return game.getBestUnit(nullptr /*todo*/, nullptr /*todo*/, center, radius);
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
            break;
        case 165: // blue
            return BWAPI::Text::Enum::Blue;
            break;
        case 159: // teal
            return BWAPI::Text::Enum::Teal;
            break;
        case 164: // purp
            return BWAPI::Text::Enum::Purple;
            break;
        case 156: // orange
            return BWAPI::Text::Enum::Orange;
            break;
        case 19:  // brown
            return BWAPI::Text::Enum::Brown;
            break;
        case 84:  // white
            return BWAPI::Text::Enum::PlayerWhite;
            break;
        case 135: // yellow
            return BWAPI::Text::Enum::PlayerYellow;
            break;
        case 185: // green p9
            return BWAPI::Text::Enum::DarkGreen;
            break;
        case 136: // p10
            return BWAPI::Text::Enum::LightYellow;
            break;
        case 134: // p11
            return BWAPI::Text::Enum::Tan;
            break;
        case 51:  // p12
            return BWAPI::Text::Enum::GreyBlue;
            break;
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

int Unit_getId(const BWAPI::UnitInterface *unit) {
    return unit->getID();
}

BWAPI::UnitType Unit_getType(const BWAPI::UnitInterface *unit) {
    return unit->getType();
}

BWAPI::Position Unit_getPosition(const BWAPI::UnitInterface *unit) {
    return unit->getPosition();
}
