// tidal-rules: CONV-8
// tidal-expect: accept
int* a = nullptr;
int* b = 0;
int* c = 0L;
void (*f)() = nullptr;
static_assert(static_cast<int*>(0) == nullptr);
