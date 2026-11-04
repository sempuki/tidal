// tidal-rules: TRY-2
// tidal-expect: run
// tidal-expect-gcc: run-fail
// tidal-note: Q-0363. The exception object of a thrown array is a pointer (THR-2), so a handler of a reference to an array does not match. Clang does not match. GCC matches the handler, and the program returns 1.
int arr[3] = {1, 2, 3};
int main() { try { throw arr; } catch (int (&)[3]) { return 1; } catch (int*) { return 0; } return 2; }
