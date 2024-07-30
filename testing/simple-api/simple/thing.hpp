#pragma once

#include <simple/ffi.hpp>

namespace simple {

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