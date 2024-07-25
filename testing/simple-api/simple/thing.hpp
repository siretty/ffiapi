#pragma once

#include <ffiapi/v1/ptr.hpp>

namespace simple {

namespace ffi {
struct thing;
extern "C" void simple_thing_create(thing **);
extern "C" void simple_thing_destroy(thing **);
} // namespace ffi

struct thing : protected ffiapi::v1::ptr<ffi::thing>
{
    thing();
    ~thing();

    void create();
    void destroy();
};

} // namespace simple