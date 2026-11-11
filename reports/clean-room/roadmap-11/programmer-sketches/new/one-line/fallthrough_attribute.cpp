// [[fallthrough]] in a switch that counts the cases it passes.
int steps(int n) { int s = 0; switch (n) { case 3: ++s; [[fallthrough]]; case 2: ++s; [[fallthrough]]; case 1: ++s; break; default: s = -1; } return s; }
int main() { return steps(3) + steps(1) + steps(9) == 3 ? 0 : 1; }
