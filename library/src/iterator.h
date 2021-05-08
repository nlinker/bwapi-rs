#pragma once

#include "BWAPI/Bullet.h"
#include "BWAPI/Bulletset.h"
#include "BWAPI/Force.h"
#include "BWAPI/Forceset.h"
#include "BWAPI/Player.h"
#include "BWAPI/Playerset.h"
#include "BWAPI/Region.h"
#include "BWAPI/Regionset.h"
#include "BWAPI/Unit.h"
#include "BWAPI/Unitset.h"
#include "BWAPI/Event.h"
#include "BWAPI/Position.h"

template<typename Container, typename Out>
class RefIterator {
public:
    using IterType = typename std::remove_const<typename Container::const_iterator>::type;
//    using Out = typename std::iterator_traits<typename Container::const_iterator>::value_type;
private:
    Container container;
    IterType iter;
public:
    explicit RefIterator(const Container &c) : container(c), iter(container.begin()) {}

    Out next() {
        if (iter != container.end()) {
            auto cur = iter;
            ++iter;
            return *cur;
        } else {
            return nullptr;
        }
    }

    unsigned long sizeHint() const {
        return container.size();
    }

    Container& underlying() const {
        return container;
    }
};


using BulletsetIterator = RefIterator<const BWAPI::Bulletset, const BWAPI::BulletInterface *>;
using ForcesetIterator = RefIterator<const BWAPI::Forceset, const BWAPI::ForceInterface *>;
using PlayersetIterator = RefIterator<const BWAPI::Playerset, const BWAPI::PlayerInterface *>;
using RegionsetIterator = RefIterator<const BWAPI::Regionset, const BWAPI::RegionInterface *>;
using UnitsetIterator = RefIterator<const BWAPI::Unitset, const BWAPI::UnitInterface *>;

std::unique_ptr<BulletsetIterator> createBulletsetIterator(const BWAPI::Bulletset &set);
std::unique_ptr<ForcesetIterator> createForcesetIterator(const BWAPI::Forceset &set);
std::unique_ptr<PlayersetIterator> createPlayersetIterator(const BWAPI::Playerset &set);
std::unique_ptr<RegionsetIterator> createRegionsetIterator(const BWAPI::Regionset &set);
std::unique_ptr<UnitsetIterator> createUnitsetIterator(const BWAPI::Unitset &set);

using EventList = std::list<BWAPI::Event>;
