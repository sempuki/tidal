inline int live = 0;
struct Part { Part() { ++live; } ~Part() { --live; } };
struct Whole { Part a, b; Whole(bool fail) { if (fail) throw 1; } };
int main() { try { Whole w(true); } catch (int) { return live == 0 ? 0 : 1; } return 2; }
