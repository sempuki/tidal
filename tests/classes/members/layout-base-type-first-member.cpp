// tidal-rules: CLS-31 CLS-10
// tidal-expect: accept
// tidal-note: The empty base stays at offset 0, and the member of the same type moves to offset 1. The int member then starts at 4, so the class takes 8 bytes on x86-64 and with -m32.
struct Unit {};
struct Wrapper : Unit { Unit head; int value; };
static_assert(sizeof(Wrapper) == 8);
