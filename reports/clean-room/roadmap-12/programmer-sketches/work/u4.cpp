template <char... Cs> constexpr int operator""_n() { return ((Cs - '0') + ...); }
int main() { return 1234_n - 10; }
