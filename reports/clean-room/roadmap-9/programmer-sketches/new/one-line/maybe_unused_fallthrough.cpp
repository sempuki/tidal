int classify(int x) { [[maybe_unused]] int unused = 0; int r = 0; switch (x) { case 0: r += 1; [[fallthrough]]; case 1: r += 10; break; default: r = -1; } return r; }
int main() { return classify(0) == 11 && classify(1) == 10 && classify(7) == -1 ? 0 : 1; }
