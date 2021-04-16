//
// detail/cstdint.hpp
// ~~~~~~~~~~~~~~~~~~
//
// Copyright (c) 2003-2016 Christopher M. Kohlhoff (chris at kohlhoff dot com)
//
// Distributed under the Boost Software License, Version 1.0. (See accompanying
// file LICENSE_1_0.txt or copy at http://www.boost.org/LICENSE_1_0.txt)
//

#ifndef ASIO_DETAIL_CSTDINT_HPP
#define ASIO_DETAIL_CSTDINT_HPP

#if defined(_MSC_VER) && (_MSC_VER >= 1200)
# pragma once
#endif // defined(_MSC_VER) && (_MSC_VER >= 1200)

#include "./config.hpp"

#if defined(ASIO_HAS_CSTDINT)
# include <cstdint>
#else // defined(ASIO_HAS_CSTDINT)
# include <boost/cstdint.hpp>
#endif // defined(ASIO_HAS_CSTDINT)

namespace asio {

#if defined(ASIO_HAS_CSTDINT)
using std::int16_t;
using std::uint16_t;
using std::int32_t;
using std::uint32_t;
using std::int64_t;
using std::uint64_t;
#else // defined(ASIO_HAS_CSTDINT)
using boost::int16_t;
using boost::uint16_t;
using boost::int32_t;
using boost::uint32_t;
using boost::int64_t;
using boost::uint64_t;
#endif // defined(ASIO_HAS_CSTDINT)

} // namespace asio

#endif // ASIO_DETAIL_CSTDINT_HPP
