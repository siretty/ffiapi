#pragma once

#include <array>
#include <utility>

#include <cstddef>
#include <cstdint>
#include <cstring>

namespace ffiapi::v1 {

template <typename T>
struct ptr
{
    T * pointer{nullptr};

    constexpr ptr() noexcept;

    constexpr explicit ptr(T * pointer_) noexcept;

    constexpr ptr(const ptr & other) noexcept = delete;

    constexpr ptr(ptr && other) noexcept;

    constexpr auto operator=(const ptr & other) noexcept -> ptr & = delete;

    constexpr auto operator=(ptr && other) noexcept -> ptr &;

    constexpr explicit operator bool() const noexcept;

    constexpr auto operator==(const ptr & rhs) const noexcept -> bool;

    constexpr auto operator!=(const ptr & rhs) const noexcept -> bool;

    constexpr void swap(ptr & other) noexcept;
};

template <typename T>
constexpr ptr<T>::ptr() noexcept = default;

template <typename T>
constexpr ptr<T>::ptr(T * pointer_) noexcept
    : pointer{pointer_}
{
}

template <typename T>
constexpr ptr<T>::ptr(ptr && other) noexcept
    : ptr()
{
    swap(other);
}

template <typename T>
constexpr auto ptr<T>::operator=(ptr && other) noexcept -> ptr &
{
    swap(other);
    return *this;
}

template <typename T>
constexpr ptr<T>::operator bool() const noexcept
{
    return pointer != nullptr;
}

template <typename T>
constexpr auto ptr<T>::operator==(const ptr &) const noexcept -> bool = default;

template <typename T>
constexpr auto ptr<T>::operator!=(const ptr &) const noexcept -> bool = default;

template <typename T>
constexpr void ptr<T>::swap(ptr & other) noexcept
{
    using std::swap;
    swap(pointer, other.pointer);
}

} // namespace ffiapi::v1

namespace ffiapi {
using namespace v1;
} // namespace ffiapi
