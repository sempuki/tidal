// tidal-rules: TRY-2
// tidal-expect: run
struct W { W(int) {} };
int main() { try { throw 1; } catch (W) { return 1; } catch (int) { return 0; } return 2; }
