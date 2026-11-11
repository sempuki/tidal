// A delegating constructor and a body that runs after it.
struct Range { int lo, hi, len; Range(int l, int h) : lo(l), hi(h), len(h - l) {} Range(int n) : Range(0, n) { len *= 2; } };
int main() { Range r(5); return r.lo + r.hi + r.len == 15 ? 0 : 1; }
