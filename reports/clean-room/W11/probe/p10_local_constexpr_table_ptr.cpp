constexpr const char* names[] = { "a", "b" };
int main() { constexpr const char* const* p = names; return p[0][0] == 'a' ? 0 : 1; }
