#include "ffiapi-testing.hpp"

#include "ffiapi/v1.hpp"

#include <cstdint>

namespace {

static_assert(sizeof(ffiapi::v1::arg<void>) == sizeof(void *));
static_assert(alignof(ffiapi::v1::arg<void>) == alignof(void *));

ADD_TEST("arg default constructor is null")
{
    constexpr ffiapi::v1::arg<void> a;

    ASSERT_FALSE(a);
    ASSERT(a.get() == nullptr);
}

ADD_TEST("arg pointer constructor")
{
    const auto p = reinterpret_cast<void *>(std::uintptr_t{12345678});
    ASSERT(p != nullptr);

    ffiapi::v1::arg<void> a{p};

    ASSERT(a);
    ASSERT(a.get() == p);
}

ADD_TEST("swap works for arg")
{
    const auto p = reinterpret_cast<void *>(std::uintptr_t{12345678});
    const auto q = reinterpret_cast<void *>(std::uintptr_t{87654321});
    ASSERT(p != nullptr);
    ASSERT(q != nullptr);
    ASSERT(p != q);

    ffiapi::v1::arg<void> a_one{p};
    ffiapi::v1::arg<void> a_two{q};

    ASSERT(a_one.get() == p);
    ASSERT(a_two.get() == q);

    swap(a_one, a_two);

    ASSERT(a_one.get() == q);
    ASSERT(a_two.get() == p);
}

} // namespace