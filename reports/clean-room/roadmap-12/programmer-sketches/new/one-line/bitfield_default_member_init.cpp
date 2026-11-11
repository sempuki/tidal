// Default member initializers on bit-fields (C++20).
struct Flags { unsigned ready : 1 = 1; unsigned level : 3 = 5; };
int main() { Flags f; f.level += 2; return (f.ready == 1 && f.level == 7) ? 0 : 1; }
