// tidal-rules: CLS-33 CLS-34 CLS-10
// tidal-expect: accept
// tidal-note: The empty base takes no bytes (CLS-33). Its offset is unspecified (CLS-34). On the matrix targets the base stays at offset 0, and the member of the same type moves to offset 1. The int member then starts at 4, so the class takes 8 bytes on x86-64 and with -m32.
struct Unit {};
struct Wrapper : Unit { Unit head; int value; };
static_assert(sizeof(Wrapper) == 8);
