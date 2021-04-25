#pragma once

#include "BWAPI/Bullet.h"
#include "BWAPI/Force.h"
#include "BWAPI/Player.h"
#include "BWAPI/Region.h"
#include "BWAPI/Unit.h"

template <typename Out>
class IteratorBase {
public:
    virtual ~IteratorBase() { }
    virtual Out next() = 0;
    virtual unsigned long sizeHint() const = 0;
};

template<typename Container, typename Out>
class OwnIterator : public IteratorBase<Out> {
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
};

template<typename Container, typename Out>
class RefIterator : public IteratorBase<Out> {
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
};

using BulletIterator = IteratorBase<const BWAPI::BulletInterface *>;
using ForceIterator = IteratorBase<const BWAPI::ForceInterface *>;
using PlayerIterator = IteratorBase<const BWAPI::PlayerInterface *>;
using RegionIterator = IteratorBase<const BWAPI::RegionInterface *>;
using UnitIterator = IteratorBase<const BWAPI::UnitInterface *>;
