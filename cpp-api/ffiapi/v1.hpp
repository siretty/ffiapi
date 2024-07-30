#pragma once

#if defined(FFIAPI_V1_BITS)
#error "macro `FFIAPI_V1_BITS` must not defined prior to including ffiapi/v1.hpp"
#endif

#if defined(FFIAPI_V1_INCLUDE_SOURCE) && defined(FFIAPI_V1_COMPILED_SOURCE)
#error "" \
    "macros `FFIAPI_V1_INCLUDE_SOURCE` and `FFIAPI_V1_COMPILED_SOURCE` " \
    "cannot be defined simultaneously prior to including ffiapi/v1.hpp"
#endif

#define FFIAPI_V1_BITS 1

// headers
#include "ffiapi/v1/obj.hpp"

#if defined(FFIAPI_V1_INCLUDE_SOURCE) || !defined(FFIAPI_V1_COMPILED_SOURCE)

// sources
#include "ffiapi/v1/obj.ipp"

#endif

#undef FFIAPI_V1_BITS