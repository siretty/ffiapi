#include "ffiapi-testing.hpp"

#include "ptr.hpp"

#include <cstdint>

namespace {

ADD_TEST("ptr default constructor is null")
{
    const ffiapi::ptr<void> o;
    ASSERT_FALSE(o);
    ASSERT_TRUE(o.pointer == nullptr);
}

ADD_TEST("ptr move constructor leaves null")
{
    const auto p1 = reinterpret_cast<void *>(std::uintptr_t{12345678});
    ASSERT_TRUE(p1 != nullptr);
    auto o1 = ffiapi::ptr<void>{p1};
    ASSERT_TRUE(o1);
    ASSERT_TRUE(o1.pointer == p1);
    ffiapi::ptr<void> o2{std::move(o1)};
    ASSERT_TRUE(o2);
    ASSERT_TRUE(o2.pointer == p1);
    ASSERT_FALSE(o1);
    ASSERT_TRUE(o1.pointer == nullptr);
}

ADD_TEST("ptr move assignment leaves null")
{
    const auto p1 = reinterpret_cast<void *>(std::uintptr_t{12345678});
    ASSERT_TRUE(p1 != nullptr);
    auto o1 = ffiapi::ptr<void>{p1};
    ASSERT_TRUE(o1);
    ASSERT_TRUE(o1.pointer == p1);
    ffiapi::ptr<void> o2;
    ASSERT_FALSE(o2);
    ASSERT_TRUE(o2.pointer == nullptr);
    o2 = std::move(o1);
    ASSERT_TRUE(o2);
    ASSERT_TRUE(o2.pointer == p1);
    ASSERT_FALSE(o1);
    ASSERT_TRUE(o1.pointer == nullptr);
}

} // namespace

int main()
{
    return ::ffiapi_testing::registry::get().run();
}