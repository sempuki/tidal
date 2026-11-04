// tidal-rules: CAST-12
// tidal-expect: accept
// tidal-flags: -m32
// tidal-note: -m32. Every compiler accepts the conversions in both directions, as on x86-64. A pointer to a function has the size of a pointer to an object.
int target() { return 4; }
static_assert(sizeof(&target) == sizeof(void*) && sizeof(void*) == 4);
void* to_object() { return reinterpret_cast<void*>(&target); }
int (*to_function(void* v))() { return reinterpret_cast<int (*)()>(v); }
char* to_bytes() { return reinterpret_cast<char*>(&target); }
