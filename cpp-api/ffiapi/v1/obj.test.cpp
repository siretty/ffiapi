#include "ffiapi-testing.hpp"

#include "ffiapi/v1.hpp"

#include <cstdint>

namespace {

static_assert(sizeof(ffiapi::v1::obj<void>) == sizeof(void *));
static_assert(alignof(ffiapi::v1::obj<void>) == alignof(void *));

ADD_TEST("obj default constructed converts to false")
{
    const ffiapi::v1::obj<void> o;

    ASSERT(!o);
}

ADD_TEST("obj default constructor is null")
{
    const ffiapi::v1::obj<void> o;

    ASSERT(o.get() == nullptr);
}

ADD_TEST("obj pointer constructor converts to true")
{
    const auto p = reinterpret_cast<void *>(std::uintptr_t{12345678});
    ASSERT(p != nullptr);

    ffiapi::v1::obj<void> o{p};

    ASSERT(o);
    ASSERT(o.get() == p);
}

ADD_TEST("obj pointer constructor stores the pointer")
{
    const auto p = reinterpret_cast<void *>(std::uintptr_t{12345678});
    ASSERT(p != nullptr);

    ffiapi::v1::obj<void> o{p};

    ASSERT(o.get() == p);
}

ADD_TEST("obj move constructor leaves null")
{
    const auto p = reinterpret_cast<void *>(std::uintptr_t{12345678});
    ASSERT(p != nullptr);

    auto o_src = ffiapi::v1::obj<void>{p};
    ASSERT(o_src.get() == p);

    ffiapi::v1::obj<void> o_dst{std::move(o_src)};

    ASSERT(o_src.get() == nullptr);
    ASSERT(o_dst.get() == p);
}

ADD_TEST("obj move assignment leaves null")
{
    const auto p = reinterpret_cast<void *>(std::uintptr_t{12345678});
    ASSERT(p != nullptr);

    auto o_src = ffiapi::v1::obj<void>{p};
    ASSERT(o_src.get() == p);

    ffiapi::v1::obj<void> o_dst;
    ASSERT(o_dst.get() == nullptr);

    o_dst = std::move(o_src);

    ASSERT(o_src.get() == nullptr);
    ASSERT(o_dst.get() == p);
}

ADD_TEST("obj release leaves null")
{
    const auto p = reinterpret_cast<void *>(std::uintptr_t{12345678});
    ASSERT(p != nullptr);

    auto o_src = ffiapi::v1::obj<void>{p};
    ASSERT(o_src.get() == p);

    auto q = o_src.release();
    ASSERT(q == p);

    ASSERT(o_src.get() == nullptr);
}

} // namespace