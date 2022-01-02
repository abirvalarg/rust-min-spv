#include "type.h"

void nop()
{}

void svc(word a, word b, word c, word d)
{
    asm("svc 0");
}
