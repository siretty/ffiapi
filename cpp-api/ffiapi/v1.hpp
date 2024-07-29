#pragma once

#if defined(FFIAPI_V1_BITS)
#error "macro `FFIAPI_V1_BITS` must not defined prior to including ffiapi/v1.hpp"
#endif

#define FFIAPI_V1_BITS 1

#include "ffiapi/v1/arg.hpp"
#include "ffiapi/v1/obj.hpp"

#include "ffiapi/v1/arg.ipp"
#include "ffiapi/v1/obj.ipp"

#undef FFIAPI_V1_BITS