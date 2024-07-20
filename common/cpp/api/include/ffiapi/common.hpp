#pragma once

#include <array>
#include <utility>

#include <cstddef>
#include <cstdint>
#include <cstring>

namespace ffiapi::v1 {

struct object
{
    constexpr static std::array<std::byte, 16> ZEROS{};

    std::array<std::byte, 16> bytes{ZEROS};

    constexpr object() noexcept;

    constexpr object(const object & other) noexcept = delete;

    constexpr object(object && other) noexcept;

    constexpr auto operator=(const object & other) noexcept -> object & = delete;

    constexpr auto operator=(object && other) noexcept -> object &;

    [[nodiscard]] constexpr auto is_empty() const noexcept -> bool;

    constexpr void swap(object & other) noexcept;
};

constexpr object::object() noexcept = default;

constexpr object::object(object && other) noexcept
    : object()
{
    swap(other);
}

constexpr auto object::operator=(object && other) noexcept -> object &
{
    swap(other);
    return *this;
}

constexpr auto object::is_empty() const noexcept -> bool
{
    return bytes == ZEROS;
}

constexpr void object::swap(object & other) noexcept
{
    bytes.swap(other.bytes);
}

} // namespace ffiapi::v1

namespace ffiapi {
using namespace v1;
} // namespace ffiapi
