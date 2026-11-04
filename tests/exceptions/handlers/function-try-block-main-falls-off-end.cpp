// tidal-rules: TRY-9
// tidal-expect: run
// tidal-note: Control reaches the end of a handler of the function try block of main, which returns 0 as the end of main does.
int main() try { throw 1; } catch (int) { }
