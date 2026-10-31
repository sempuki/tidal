// tidal-rules: TRY-15
// tidal-expect: run
int count = 0;
struct T { ~T() { ++count; } };
int main() { try { throw 1; } catch (int) { T t; goto done; } return 1; done: return count == 1 ? 0 : 2; }
