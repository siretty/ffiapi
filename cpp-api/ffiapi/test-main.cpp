#include "ffiapi-testing.hpp"

int main()
{
    return ::ffiapi_testing::registry::get().run();
}