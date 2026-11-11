constexpr int operator""_n(const char* s) { int k = 0; while (s[k]) ++k; return k; }
int main() { return 123456789012345678901234567890_n - 30; }
