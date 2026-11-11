constexpr int parse(const char* s) { int v = 0; bool neg = *s == 45; if (neg) ++s; while (*s >= 48 && *s <= 57) v = v * 10 + (*s++ - 48); return neg ? -v : v; }
static_assert(parse("-123") == -123);
int main() { return parse("4096") == 4096 ? 0 : 1; }
