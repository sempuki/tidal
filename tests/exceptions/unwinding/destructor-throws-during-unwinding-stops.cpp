// tidal-rules: UNW-7 UNW-5
// tidal-expect: run-fail
// tidal-note: A destructor throws during unwinding, and the program ends at that point. The destructor of o would end the program with status 0. It does not run, because no further unwinding happens.
extern "C" [[noreturn]] void _Exit(int) noexcept;
struct Outer { ~Outer() { _Exit(0); } };
struct Bad { ~Bad() noexcept(false) { throw 2; } };
void f() { Outer o; { Bad b; throw 1; } }
int main() { try { f(); } catch (...) {} return 3; }
