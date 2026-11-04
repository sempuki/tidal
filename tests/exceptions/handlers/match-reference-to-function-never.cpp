// tidal-rules: TRY-2
// tidal-expect: run
// tidal-expect-gcc: run-fail
// tidal-note: Q-0363. The exception object of a thrown function is a pointer (THR-2), so a handler of a reference to a function does not match. Clang does not match. GCC matches the handler, and the program returns 1.
int f() { return 7; }
int main() { try { throw f; } catch (int (&)()) { return 1; } catch (int (*)()) { return 0; } return 2; }
