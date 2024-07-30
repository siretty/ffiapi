#pragma once

#if !defined(FFIAPI_V1_BITS)
#error "never include ffiapi/v1/obj.ipp directly; use ffiapi/v1.hpp instead"
#endif

#include <utility>

namespace ffiapi::v1 {

template <typename T>
constexpr obj<T>::obj() noexcept = default;

template <typename T>
constexpr obj<T>::obj(T * pointer_) noexcept
    : _pointer{pointer_}
{
}

template <typename T>
constexpr obj<T>::obj(obj && other) noexcept
    : obj()
{
    using std::swap;
    swap(*this, other);
}

template <typename T>
constexpr auto obj<T>::operator=(obj && other) noexcept -> obj &
{
    using std::swap;
    swap(*this, other);
    return *this;
}

template <typename T>
constexpr obj<T>::operator bool() const noexcept
{
    return _pointer != nullptr;
}

template <typename T>
constexpr auto obj<T>::operator==(const obj &) const noexcept -> bool = default;

template <typename T>
constexpr auto obj<T>::operator!=(const obj &) const noexcept -> bool = default;

template <typename T>
constexpr auto obj<T>::get() const noexcept -> T *
{
    return _pointer;
}

template <typename T>
constexpr auto obj<T>::release() noexcept -> T *
{
    obj result;
    swap(*this, result);
    return result._pointer;
}

template <typename T>
constexpr void swap(obj<T> & a, obj<T> & b) noexcept
{
    using std::swap;
    swap(a._pointer, b._pointer);
}

} // namespace ffiapi::v1
