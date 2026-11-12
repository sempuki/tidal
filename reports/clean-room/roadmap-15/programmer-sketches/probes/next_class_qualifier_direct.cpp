namespace d { struct S { static constexpr int n = 3; }; }
int main() { return d::S::n - 3; }
