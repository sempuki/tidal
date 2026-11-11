// A static operator[] (C++23).
struct Squares { static constexpr int operator[](int i) { return i * i; } };
int main() { Squares s; return s[4] - Squares{}[4] + (s[3] - 9); }
