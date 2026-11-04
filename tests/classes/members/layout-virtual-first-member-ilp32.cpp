// tidal-rules: CLS-34 CLS-35
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: -m32. The pointer to the table takes 4 bytes, and the int member follows it.
struct Shape { virtual int sides() const { return 0; } int id = 5; };
static_assert(sizeof(void*) == 4 && sizeof(Shape) == 8 && alignof(Shape) == 4);
