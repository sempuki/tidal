struct Ctrl { unsigned enable : 1; unsigned mode : 3; unsigned : 28; };
static_assert(sizeof(Ctrl) == 4);
Ctrl reg_storage{};
int main() {
  volatile Ctrl* c = &reg_storage;
  c->enable = 1;
  c->mode = 5;
  c->mode += 1;
  return (c->enable == 1 && c->mode == 6) ? 0 : 1;
}
