// From bigint_first_try.cpp. A numeric literal operator template with a
// pack of char. OPR-8 and OPR-10 do not list this form. GCC, Clang, and
// EDG accept it. tidal-fe stops with "literal operator template".
template <char... Cs> constexpr int operator""_n() { return sizeof...(Cs); }
int main() { return 12_n - 2; }
