// tidal-rules: LIST-2 CONV-21
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: With -m32, int holds every value of long (TYP-1), so the conversion is not narrowing. narrowing-nonconstant-int.cpp is the x86-64 case.
static_assert(sizeof(long) == sizeof(int));
long l = 1;
int x{l};
