// From bigint_first_try.cpp. A literal with the suffix of a raw literal
// operator passes its digits as a string (OPR-8), so no integer type
// needs to hold them. tidal-fe rejects with PRIM-2.
constexpr int operator""_n(const char* s) { int k = 0; while (s[k]) ++k; return k; }
int main() { return 123456789012345678901234567890_n - 30; }
