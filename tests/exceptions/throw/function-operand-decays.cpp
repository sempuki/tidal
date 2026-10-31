// tidal-rules: THR-2
// tidal-expect: run
int seven() { return 7; }
int main() { try { throw seven; } catch (int (*p)()) { return p() == 7 ? 0 : 1; } return 2; }
