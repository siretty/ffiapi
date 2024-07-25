#include <simple/thing.hpp>

namespace simple {

thing::thing()
{
    create();
}

thing::~thing()
{
    if (pointer != nullptr)
    {
        destroy();
    }
}

void thing::create()
{
    ffi::simple_thing_create(&pointer);
}

void thing::destroy()
{
    ffi::simple_thing_destroy(&pointer);
}

} // namespace simple
