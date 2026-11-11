template <char... Cs> constexpr int operator""_n() { return sizeof...(Cs); }
int main() { return 12_n - 2; }
