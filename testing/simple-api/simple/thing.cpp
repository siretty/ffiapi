#include <simple/thing.hpp>

namespace simple {

thing::thing()
{
    create();
}

thing::~thing()
{
    if (_obj)
    {
        destroy();
    }
}

void thing::create()
{
    simple_ffi::simple_thing_create(&_obj);
}

void thing::destroy()
{
    simple_ffi::simple_thing_destroy(&_obj);
}

} // namespace simple