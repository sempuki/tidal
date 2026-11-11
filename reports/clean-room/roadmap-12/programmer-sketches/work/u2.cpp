template <char... Cs> constexpr int operator""_n() { return sizeof...(Cs); }
int main() { return 123456789012345678901234567890_n - 30; }
