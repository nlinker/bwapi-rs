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

template<typename Container, typename Out>
class IteratorBase {
public:
    virtual ~IteratorBase() { }
    virtual Out next() = 0;
    virtual unsigned long sizeHint() const = 0;
    virtual Container& underlying() const = 0;
};

template<typename Container, typename Out>
class OwnIterator : public IteratorBase<Container, Out> {
public:
    using IterType = typename std::remove_const<typename Container::const_iterator>::type;
//    using Out = typename std::iterator_traits<typename Container::const_iterator>::value_type;
private:
    Container container;
    IterType iter;
public:
    OwnIterator(Container c) : container(std::move(c)), iter(container.begin()) {}

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
class RefIterator : public IteratorBase<Container, Out> {
public:
    using IterType = typename std::remove_const<typename Container::const_iterator>::type;
//    using Out = typename std::iterator_traits<typename Container::const_iterator>::value_type;
private:
    Container container;
    IterType iter;
public:
    RefIterator(const Container &c) : container(c), iter(container.begin()) {}

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

using BulletIterator = IteratorBase<const BWAPI::Bulletset, const BWAPI::BulletInterface *>;
using BulletIteratorOwn = OwnIterator<const BWAPI::Bulletset, const BWAPI::BulletInterface *>;
using BulletIteratorRef = RefIterator<const BWAPI::Bulletset, const BWAPI::BulletInterface *>;

using ForceIterator = IteratorBase<const BWAPI::Forceset, const BWAPI::ForceInterface *>;
using ForceIteratorOwn = OwnIterator<const BWAPI::Forceset, const BWAPI::ForceInterface *>;
using ForceIteratorRef = RefIterator<const BWAPI::Forceset, const BWAPI::ForceInterface *>;

using PlayerIterator = IteratorBase<const BWAPI::Playerset, const BWAPI::PlayerInterface *>;
using PlayerIteratorOwn = OwnIterator<const BWAPI::Playerset, const BWAPI::PlayerInterface *>;
using PlayerIteratorRef = RefIterator<const BWAPI::Playerset, const BWAPI::PlayerInterface *>;

using RegionIterator = IteratorBase<const BWAPI::Regionset, const BWAPI::RegionInterface *>;
using RegionIteratorOwn = OwnIterator<const BWAPI::Regionset, const BWAPI::RegionInterface *>;
using RegionIteratorRef = RefIterator<const BWAPI::Regionset, const BWAPI::RegionInterface *>;

using UnitIterator = IteratorBase<const BWAPI::Unitset, const BWAPI::UnitInterface *>;
using UnitIteratorOwn = OwnIterator<const BWAPI::Unitset, const BWAPI::UnitInterface *>;
using UnitIteratorRef = RefIterator<const BWAPI::Unitset, const BWAPI::UnitInterface *>;
