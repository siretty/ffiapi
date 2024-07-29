#include "ffiapi-testing.hpp"

#include "ffiapi/v1.hpp"

#include <cstdint>

namespace {

static_assert(sizeof(ffiapi::v1::obj<void>) == sizeof(void *));
static_assert(alignof(ffiapi::v1::obj<void>) == alignof(void *));

ADD_TEST("obj default constructor is null")
{
    const ffiapi::v1::obj<void> o;

    ASSERT(!o);
    ASSERT(o.pointer == nullptr);
}

ADD_TEST("obj _pointer constructor")
{
    const auto p = reinterpret_cast<void *>(std::uintptr_t{12345678});
    ASSERT(p != nullptr);

    ffiapi::v1::obj<void> o{p};

    ASSERT(o);
    ASSERT(o.pointer == p);
}

ADD_TEST("obj move constructor leaves null")
{
    const auto p = reinterpret_cast<void *>(std::uintptr_t{12345678});
    ASSERT(p != nullptr);

    auto o_src = ffiapi::v1::obj<void>{p};
    ASSERT(o_src);
    ASSERT(o_src.pointer == p);

    ffiapi::v1::obj<void> o_dst{std::move(o_src)};

    ASSERT(!o_src);
    ASSERT(o_src.pointer == nullptr);
    ASSERT(o_dst);
    ASSERT(o_dst.pointer == p);
}

ADD_TEST("obj move assignment leaves null")
{
    const auto p = reinterpret_cast<void *>(std::uintptr_t{12345678});
    ASSERT(p != nullptr);

    auto o_src = ffiapi::v1::obj<void>{p};
    ASSERT(o_src);
    ASSERT(o_src.pointer == p);

    ffiapi::v1::obj<void> o_dst;
    ASSERT(!o_dst);
    ASSERT(o_dst.pointer == nullptr);

    o_dst = std::move(o_src);

    ASSERT(!o_src);
    ASSERT(o_src.pointer == nullptr);
    ASSERT(o_dst);
    ASSERT(o_dst.pointer == p);
}

} // namespace