#pragma once

#include <iostream>
#include <chrono>
#include <ctime>

class IteratorBase {
public:
    virtual ~IteratorBase() {
        std::time_t now = std::time(0);   // get time now
        std::cout << std::ctime(&now) << std::endl;
    }
    virtual const void *next() = 0;
    virtual unsigned long size() const = 0;
};

template<class Container>
class OwnIterator : public IteratorBase {
public:
    using IterType = typename std::remove_const<typename Container::const_iterator>::type;
private:
    Container container;
    IterType iter;
public:
    OwnIterator(Container c) : container(std::move(c)), iter(container.begin()) {}

    const void *next() {
        if (iter != container.end()) {
            auto cur = iter;
            ++iter;
            return reinterpret_cast<void *>(*cur);
        } else {
            return nullptr;
        }
    }

    unsigned long size() const {
        return container.size();
    }
};

template<class Container>
class RefIterator : public IteratorBase {
public:
    using IterType = typename std::remove_const<typename Container::const_iterator>::type;
private:
    Container container;
    IterType iter;
public:
    RefIterator(const Container &c) : container(c), iter(container.begin()) {}

    const void *next() {
        if (iter != container.end()) {
            auto cur = iter;
            ++iter;
            return reinterpret_cast<void *>(*cur);
        } else {
            return nullptr;
        }
    }

    unsigned long size() const {
        return container.size();
    }
};


//template<class OutIterator, class Container>
//OutIterator* into_iter(Container container) {
//    //const auto type = static_cast<IteratorType>(GetIterType<Out>::value);
//    //static_assert(type != itUnknown, "Out must be registered, see GetIterType<T>");
//    //using type = std::remove_const<typename Container::const_iterator>::type;
//    auto *iter = new OwningIterator<Container>(std::move(container));
//    return reinterpret_cast<OutIterator*>(iter);
//}
//
//template<class OutIterator, class Container>
//OutIterator* as_iter(const Container& container) {
//    //const auto type = static_cast<IteratorType>(GetIterType<Out>::value);
//    //static_assert(type != itUnknown, "Out must be registered, see GetIterType<T>");
//    //using itemType = typename std::remove_const<typename Container::const_iterator>::type;
//    auto *iter = new BorrowingIterator<Container>(container);
//    return reinterpret_cast<OutIterator*>(iter);
//}

