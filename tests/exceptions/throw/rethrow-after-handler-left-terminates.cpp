// tidal-rules: THR-7
// tidal-expect: run-fail
void later() { throw; }
int main() { try { throw 1; } catch (int) { } later(); return 0; }
