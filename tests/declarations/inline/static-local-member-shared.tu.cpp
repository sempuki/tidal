struct Counter {
  static int tick() { static int n = 0; return ++n; }
};
int other() { return Counter::tick(); }
