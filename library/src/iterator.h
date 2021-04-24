#pragma once

template<class Container>
class OwnIterator {
public:
    using IterType = typename std::remove_const<typename Container::const_iterator>::type;
    using Out = typename std::iterator_traits<typename Container::const_iterator>::value_type;
private:
    Container container;
    IterType iter;
public:
    OwnIterator(Container c) : container(std::move(c)), iter(container.begin()) {}

    Out next() const {
        if (iter != container.end()) {
            auto item = iter;
            ++iter;
            return *item;
        } else {
            return nullptr;
        }
    }

    unsigned long size() const {
        return container.size();
    }
};

template<class Container>
class RefIterator {
public:
    using IterType = typename std::remove_const<typename Container::const_iterator>::type;
    using Out = typename std::iterator_traits<typename Container::const_iterator>::value_type;
private:
    Container container;
    IterType iter;
public:
    RefIterator(const Container &c) : container(c), iter(container.begin()) {}

    Out next() {
        if (iter != container.end()) {
            auto item = iter;
            ++iter;
            return *item;
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

