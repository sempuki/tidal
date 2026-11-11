// Planted bug: the base constructor calls a pure virtual function through a helper.
struct Task {
  Task() { start(); }
  void start() { run(); }
  virtual int run() = 0;
  virtual ~Task() = default;
};
struct Print : Task { int run() override { return 1; } };
int main() { Print p; return p.run() == 1 ? 0 : 1; }
