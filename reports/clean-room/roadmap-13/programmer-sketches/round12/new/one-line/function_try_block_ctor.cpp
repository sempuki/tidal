// A function try block on a constructor rethrows after its handler.
inline int seen = 0;
struct Part { Part(int v) { if (v < 0) throw v; } };
struct Whole { Part p; Whole(int v) try : p(v) {} catch (int) { ++seen; } };
int main() { try { Whole w(-1); } catch (int e) { return e + 1 + (seen - 1); } return 1; }
