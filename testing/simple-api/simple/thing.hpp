#pragma once

#include <ffiapi/v1.hpp>

namespace simple_ffi {
using namespace ffiapi::v1;

struct thing;
extern "C" void simple_thing_create(obj<thing> *);
extern "C" void simple_thing_destroy(obj<thing> *);
} // namespace simple_ffi

namespace simple {

namespace ffi = simple_ffi;

struct thing
{
    thing();
    ~thing();

    void create();
    void destroy();

protected:
    ffi::obj<ffi::thing> _obj;
};

} // namespace simple