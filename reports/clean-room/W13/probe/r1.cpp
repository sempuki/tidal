struct R { unsigned status; unsigned count; };
volatile R* const port = reinterpret_cast<volatile R*>(0x40001000ul);
alignas(R) unsigned char mem[sizeof(R)];
int main() {
  volatile R* r = reinterpret_cast<volatile R*>(mem);
  ++r->count;
  r->status = 1;
  return static_cast<int>(r->count) - 1 + (port == nullptr);
}
