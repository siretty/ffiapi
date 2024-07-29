#pragma once

#if !defined(FFIAPI_V1_BITS)
#error "never include ffiapi/v1/arg.hpp directly; use ffiapi/v1.hpp instead"
#endif

namespace ffiapi::v1 {

template <typename T>
struct obj;

template <typename T>
class arg
{
    friend struct obj<T>;

    T * _pointer{nullptr};

public:
    constexpr arg() noexcept;

    constexpr explicit arg(T * pointer_) noexcept;

    constexpr arg(const arg & other) noexcept = delete;

    constexpr arg(arg && other) noexcept = delete;

    constexpr auto operator=(const arg & other) noexcept -> arg & = delete;

    constexpr auto operator=(arg && other) noexcept -> arg & = delete;

    constexpr explicit operator bool() const noexcept;

    constexpr auto get() const noexcept -> T *;

    constexpr auto release() && noexcept -> T *;

    template <typename U>
    friend constexpr void swap(arg<U> & a, arg<U> & b) noexcept;
};

} // namespace ffiapi::v1