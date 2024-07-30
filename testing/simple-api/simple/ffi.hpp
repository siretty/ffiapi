#pragma once

#include <ffiapi/v1.hpp>

namespace simple::ffi {

using namespace ffiapi::v1;

struct thing;
extern "C" void simple_thing_create(obj<thing> *);
extern "C" void simple_thing_destroy(obj<thing> *);

} // namespace simple::ffi