struct S { static constexpr int limit = 7; };
const int* other() { return &S::limit; }
