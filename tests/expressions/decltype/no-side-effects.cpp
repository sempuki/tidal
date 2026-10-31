// tidal-rules: UNEV-1
// tidal-expect: run
int count = 0;
int bump() { return ++count; }
int main() { decltype(bump()) a = 0; auto s = sizeof(bump()); bool n = noexcept(bump()); return count == 0 && a == 0 && s > 0 && !n ? 0 : 1; }
