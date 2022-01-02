#include "type.h"

word read(volatile word *addr)
{
    return *addr;
}

void write(volatile word *addr, word val)
{
    *addr = val;
}
