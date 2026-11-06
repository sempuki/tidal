// tidal-rules: CMP-15 PRIM-9
// tidal-expect: run
// tidal-note: Roadmap round 9, programmer reduction literal_pointer_copy_compare.cpp. One evaluation of a string literal gives one pointer, and a copy of that pointer holds the same address. PRIM-9 covers two literals or two evaluations, so it gives no choice here.
int main() {
  const char* first = "tide";
  const char* second = first;
  return first == second ? 0 : 1;
}
