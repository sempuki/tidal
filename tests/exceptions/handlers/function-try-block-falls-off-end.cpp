// tidal-rules: TRY-9
// tidal-expect: run
int trace = 0;
void f() try { throw 1; } catch (int) { trace = 1; }
int main() { f(); return trace == 1 ? 0 : 1; }
