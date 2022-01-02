#include "type.h"

extern word _STACK, _DATA_START, _DATA_END, _DATA_VAL_START,
    _BSS_START, _BSS_END;

void _reset();
void _nmi();
void _hardfault();
void _svcall(word, word, word, word);
void __init_rust();
void start();

__attribute__((used, section(".vector")))
const word __VECTOR[] = {
    &_STACK,
    _reset,
    _nmi,
    _hardfault,
    0, 0,
    0, 0, 0, 0, 0,
    _svcall
};

__attribute__((noreturn))
void _reset()
{
    for(byte *dest = &_DATA_START, *src = &_DATA_VAL_START; dest != &_DATA_END; dest++, src++)
        *dest = *src;
    for(byte *dest = &_BSS_START; dest != &_BSS_END; dest++)
        *dest = 0;
    __init_rust();
    asm("push {r0}\n\
    ldr r0, =0b101\n\
    msr CONTROL, r0\n\
    pop {r0}");
    start();
    while(1);
}
