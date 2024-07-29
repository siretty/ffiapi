#pragma once

#if !defined(FFIAPI_V1_BITS)
#error "never include ffiapi/v1/arg.ipp directly; use ffiapi/v1.hpp instead"
#endif

#include "ffiapi/v1/obj.hpp"

#include <utility>

namespace ffiapi::v1 {

template <typename T>
constexpr arg<T>::arg() noexcept = default;

template <typename T>
constexpr arg<T>::arg(T * pointer_) noexcept
    : _pointer{pointer_}
{
}

template <typename T>
constexpr arg<T>::operator bool() const noexcept
{
    return _pointer != nullptr;
}

template <typename T>
constexpr auto arg<T>::get() const noexcept -> T *
{
    return _pointer;
}

template <typename T>
constexpr auto arg<T>::release() && noexcept -> T *
{
    arg result;
    swap(result, *this);
    return result._pointer;
}

template <typename T>
constexpr void swap(arg<T> & a, arg<T> & b) noexcept
{
    using std::swap;
    swap(a._pointer, b._pointer);
}

} // namespace ffiapi::v1