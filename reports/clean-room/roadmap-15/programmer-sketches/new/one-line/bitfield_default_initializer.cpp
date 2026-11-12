struct Packed {
  unsigned mode : 3 = 5;
  unsigned flag : 1 {1};
};
static_assert(Packed{}.mode == 5 && Packed{}.flag == 1);
int main() { Packed p; p.mode = 2; return p.mode + p.flag - 3; }
