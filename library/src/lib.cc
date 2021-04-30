#include "lib.h"
#include "library/src/lib.rs.h"
#include "BWAPI/Game.h"
#include "BWAPI/Unit.h"
#include "../openbw/bwapilib/include/BWAPI/Game.h"
#include "../openbw/bwapilib/include/BWAPI/Filters.h"

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

// region === === Iterators === ===
std::unique_ptr<BulletsetIterator> createBulletsetIteratorRef(const BWAPI::Bulletset &set) {
    return std::unique_ptr<BulletsetIterator>(new BulletsetIteratorRef(set));
}

std::unique_ptr<ForcesetIterator> createForcesetIteratorRef(const BWAPI::Forceset &set) {
    return std::unique_ptr<ForcesetIterator>(new ForcesetIteratorRef(set));
}

std::unique_ptr<PlayersetIterator> createPlayersetIteratorRef(const BWAPI::Playerset &set) {
    return std::unique_ptr<PlayersetIterator>(new PlayersetIteratorRef(set));
}

std::unique_ptr<RegionsetIterator> createRegionsetIteratorRef(const BWAPI::Regionset &set) {
    return std::unique_ptr<RegionsetIterator>(new RegionsetIteratorRef(set));
}

std::unique_ptr<UnitsetIterator> createUnitsetIteratorRef(const BWAPI::Unitset &set) {
    return std::unique_ptr<UnitsetIterator>(new UnitsetIteratorRef(set));
}
// endregion

// region === === Unitset === ===
const BWAPI::UnitInterface *_unitset_getClosestUnit(const BWAPI::Unitset &set, UnitFilter pred, int radius) {
    return set.getClosestUnit(nullptr /* todo convert predicate */, radius);
}

std::unique_ptr <UnitsetIterator> _unitset_getInterceptors(const BWAPI::Unitset &set) {
    const BWAPI::Unitset xs = set.getInterceptors();
    return std::unique_ptr<UnitsetIterator>(new UnitsetIteratorOwn(std::move(xs)));
}

std::unique_ptr <UnitsetIterator> _unitset_getLarva(const BWAPI::Unitset &set) {
    const BWAPI::Unitset xs = set.getLarva();
    return std::unique_ptr<UnitsetIterator>(new UnitsetIteratorOwn(std::move(xs)));
}

std::unique_ptr <UnitsetIterator> _unitset_getLoadedUnits(const BWAPI::Unitset &set) {
    const BWAPI::Unitset xs = set.getLoadedUnits();
    return std::unique_ptr<UnitsetIterator>(new UnitsetIteratorOwn(std::move(xs)));
}

std::unique_ptr <UnitsetIterator> _unitset_getUnitsInRadius(const BWAPI::Unitset &set, int radius, UnitFilter pred) {
    const BWAPI::Unitset xs = set.getUnitsInRadius(radius, nullptr /* todo convert predicate */);
    return std::unique_ptr<UnitsetIterator>(new UnitsetIteratorOwn(std::move(xs)));
}

bool _unitset_move(const BWAPI::Unitset &set, BWAPI::Position target, bool shift_queue_command) {
    return set.move(target, shift_queue_command);
}
// endregion

// region ======== Game ========
void _game_debug(const BWAPI::Game &game) {
    using namespace BWAPI;
    BWAPI::Game &g = const_cast<BWAPI::Game&>(game);
    Text::Size::Enum sizes[] = {Text::Size::Enum::Huge, Text::Size::Enum::Large, Text::Size::Enum::Small, Text::Size::Enum::Default};
    int size = sizeof(sizes)/sizeof(sizes[0]);
    for (int i = 0; i < size; i++) {
        int delta = i * 37;
        g.setTextSize(sizes[i]);
        g.drawText(CoordinateType::Enum::Map, 1800, 1800 + (i * 50), "Hello, SSCAIT from c++!");
    }
}

void _game_debug_fun(const BWAPI::Game &game, UnitFilter fun) {
    BWAPI::Game &g = const_cast<BWAPI::Game&>(game);
    const BWAPI::UnitFilter &pred = nullptr;
    auto left = 100;
    auto right = 1000;
    auto top = 100;
    auto bottom = 1000;
    BWAPI::Unitset set = g.getUnitsInRectangle(left, top, right, bottom, pred);
    for (auto &p : set) {
        std::ostringstream os;
        os << "raw: " << p << "id:" << p->getID() << " type:" << p->getType() << " position:" << p->getPosition();
        g.sendText(os.str().c_str());
    }
    //std::ostringstream os;
    //os << "sizeof(UnitCommand) = " << sizeof(BWAPI::UnitCommand);
    //g.sendText(os.str().c_str());
}

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

std::unique_ptr<UnitsetIterator> _game_getUnitsInRadius(const BWAPI::Game &game, BWAPI::Position position, int radius, UnitFilter pred) {
    const BWAPI::Unitset xs = game.getUnitsInRadius(position, radius, nullptr /*todo*/);
    return std::unique_ptr<UnitsetIterator>(new UnitsetIteratorOwn(std::move(xs)));
}

std::unique_ptr<UnitsetIterator> _game_getUnitsInRectangle(const BWAPI::Game &game, BWAPI::Position topLeft, BWAPI::Position bottomRight, UnitFilter pred) {
    const BWAPI::Unitset xs = game.getUnitsInRectangle(topLeft, bottomRight, nullptr /*todo*/);
    return std::unique_ptr<UnitsetIterator>(new UnitsetIteratorOwn(std::move(xs)));
}

std::unique_ptr<UnitsetIterator> _game_getUnitsOnTile(const BWAPI::Game &game, BWAPI::TilePosition tile, UnitFilter pred) {
    const BWAPI::Unitset xs = game.getUnitsOnTile(tile, nullptr /*todo*/);
    return std::unique_ptr<UnitsetIterator>(new UnitsetIteratorOwn(std::move(xs)));
}

std::unique_ptr<std::string> _game_mapFileName(const BWAPI::Game& game) {
    return std::unique_ptr<std::string>(new std::string(game.mapFileName()));
}

std::unique_ptr<std::string> _game_mapHash(const BWAPI::Game& game) {
    return std::unique_ptr<std::string>(new std::string(game.mapHash()));
}

std::unique_ptr<std::string> _game_mapName(const BWAPI::Game& game) {
    return std::unique_ptr<std::string>(new std::string(game.mapName()));
}

std::unique_ptr<std::string> _game_mapPathName(const BWAPI::Game& game) {
    return std::unique_ptr<std::string>(new std::string(game.mapPathName()));
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


int Unit_getId(const BWAPI::UnitInterface *unit) {
    return unit->getID();
}

BWAPI::UnitType Unit_getType(const BWAPI::UnitInterface *unit) {
    return unit->getType();
}

BWAPI::Position Unit_getPosition(const BWAPI::UnitInterface *unit) {
    return unit->getPosition();
}
