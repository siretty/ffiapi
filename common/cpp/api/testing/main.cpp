#include "./testing.hpp"

#include "ffiapi/common.hpp"

namespace {

ADD_TEST("object zeroes is actually zero", [] {
    for (const auto byte : ffiapi::object::ZEROS)
    {
        ASSERT_EQ(byte, std::byte{0});
    }
})

ADD_TEST("object default constructor produces zeros", [] {
    ffiapi::object o;
    ASSERT_EQ(o.bytes, ffiapi::object::ZEROS);
})

ADD_TEST("object move constructor leaves zeros", [] {
    std::array<std::pair<std::size_t, std::byte>, 2> data{
        std::make_pair(0, std::byte{121}),
        std::make_pair(sizeof(ffiapi::object::bytes) - 1, std::byte{212}),
    };

    ffiapi::object o1;

    for (const auto [index, value] : data)
    {
        o1.bytes[index] = value;
    }

    ffiapi::object o2{std::move(o1)};

    ASSERT_EQ(o1.bytes, ffiapi::object::ZEROS);

    for (const auto [index, value] : data)
    {
        ASSERT_EQ(o2.bytes[index], value);
    }
})

} // namespace

int main()
{
    return registry::get().run();
}
