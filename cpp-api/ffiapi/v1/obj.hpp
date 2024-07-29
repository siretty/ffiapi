#pragma once

#if !defined(FFIAPI_V1_BITS)
#error "never include ffiapi/v1/obj.hpp directly; use ffiapi/v1.hpp instead"
#endif

namespace ffiapi::v1 {

template <typename T>
class arg;

template <typename T>
struct obj
{
    T * pointer{nullptr};

    constexpr obj() noexcept;

    constexpr explicit obj(T * pointer_) noexcept;

    constexpr obj(const obj & other) noexcept = delete;

    constexpr obj(obj && other) noexcept;

    constexpr auto operator=(const obj & other) noexcept -> obj & = delete;

    constexpr auto operator=(obj && other) noexcept -> obj &;

    constexpr explicit operator bool() const noexcept;

    constexpr auto operator==(const obj & rhs) const noexcept -> bool;

    constexpr auto operator!=(const obj & rhs) const noexcept -> bool;

    constexpr auto get() const noexcept -> T *;

    constexpr auto release() noexcept -> T *;

    constexpr auto as_arg() const noexcept -> arg<T>;

    template <typename U>
    friend constexpr void swap(obj<U> & a, obj<U> & b) noexcept;
};

} // namespace ffiapi::v1