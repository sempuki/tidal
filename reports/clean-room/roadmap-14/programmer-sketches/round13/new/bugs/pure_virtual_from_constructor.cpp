// A base constructor calls a pure virtual function through a helper.
struct Task {
  Task() { start(); }
  void start() { run(); }
  virtual void run() = 0;
  virtual ~Task() = default;
};
struct Print : Task { void run() override {} };
int main() { Print p; return 0; }
