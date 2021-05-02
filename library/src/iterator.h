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
class BaseIterator {
public:
    virtual ~BaseIterator() = default;
    virtual Out next() = 0;
    virtual unsigned long sizeHint() const = 0;
    virtual Container& underlying() const = 0;
};

template<typename Container, typename Out>
class OwnIterator : public BaseIterator<Container, Out> {
public:
    using IterType = typename std::remove_const<typename Container::const_iterator>::type;
//    using Out = typename std::iterator_traits<typename Container::const_iterator>::value_type;
private:
    Container container;
    IterType iter;
public:
    explicit OwnIterator(Container c) : container(std::move(c)), iter(container.begin()) {}

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

template<typename Container, typename Out>
class RefIterator : public BaseIterator<Container, Out> {
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

using BulletsetIterator = BaseIterator<const BWAPI::Bulletset, const BWAPI::BulletInterface *>;
using BulletsetIteratorOwn = OwnIterator<const BWAPI::Bulletset, const BWAPI::BulletInterface *>;
using BulletsetIteratorRef = RefIterator<const BWAPI::Bulletset, const BWAPI::BulletInterface *>;
//std::unique_ptr<BulletsetIterator> createBulletsetIteratorOwn(const BWAPI::Bulletset &set);
std::unique_ptr<BulletsetIterator> createBulletsetIteratorRef(const BWAPI::Bulletset &set);

using ForcesetIterator = BaseIterator<const BWAPI::Forceset, const BWAPI::ForceInterface *>;
using ForcesetIteratorOwn = OwnIterator<const BWAPI::Forceset, const BWAPI::ForceInterface *>;
using ForcesetIteratorRef = RefIterator<const BWAPI::Forceset, const BWAPI::ForceInterface *>;
//std::unique_ptr<ForcesetIterator> createForcesetIteratorOwn(const BWAPI::Forceset &set);
std::unique_ptr<ForcesetIterator> createForcesetIteratorRef(const BWAPI::Forceset &set);

using PlayersetIterator = BaseIterator<const BWAPI::Playerset, const BWAPI::PlayerInterface *>;
using PlayersetIteratorOwn = OwnIterator<const BWAPI::Playerset, const BWAPI::PlayerInterface *>;
using PlayersetIteratorRef = RefIterator<const BWAPI::Playerset, const BWAPI::PlayerInterface *>;
//std::unique_ptr<PlayersetIterator> createPlayersetIteratorOwn(const BWAPI::Playerset &set);
std::unique_ptr<PlayersetIterator> createPlayersetIteratorRef(const BWAPI::Playerset &set);

using RegionsetIterator = BaseIterator<const BWAPI::Regionset, const BWAPI::RegionInterface *>;
using RegionsetIteratorOwn = OwnIterator<const BWAPI::Regionset, const BWAPI::RegionInterface *>;
using RegionsetIteratorRef = RefIterator<const BWAPI::Regionset, const BWAPI::RegionInterface *>;
//std::unique_ptr<RegionsetIterator> createRegionsetIteratorOwn(const BWAPI::Regionset &set);
std::unique_ptr<RegionsetIterator> createRegionsetIteratorRef(const BWAPI::Regionset &set);

using UnitsetIterator = BaseIterator<const BWAPI::Unitset, const BWAPI::UnitInterface *>;
using UnitsetIteratorOwn = OwnIterator<const BWAPI::Unitset, const BWAPI::UnitInterface *>;
using UnitsetIteratorRef = RefIterator<const BWAPI::Unitset, const BWAPI::UnitInterface *>;
//std::unique_ptr<UnitsetIterator> createUnitsetIteratorOwn(const BWAPI::Unitset &set);
std::unique_ptr<UnitsetIterator> createUnitsetIteratorRef(const BWAPI::Unitset &set);

using EventList = std::list<BWAPI::Event>;
