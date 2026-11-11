// Port of Machine.cs. IState is an abstract class with pure virtual
// functions (VIRT-P4), the C# interface. A transition returns a new state
// made with new. The machine deletes the old state when the state changes,
// through the base pointer, so IState has a virtual destructor (VIRT-P3).
// The C# using block becomes a guard object whose destructor calls
// Dispose when control leaves the block by any route (OBJ-P2, JMP-P2).
// Strings are compared by their characters: two string literals with the
// same characters need not be one object (PRIM-9).

static bool same(const char* a, const char* b) {
    int i = 0;
    while (a[i] != '\0' && a[i] == b[i]) ++i;
    return a[i] == b[i];
}

struct Machine;

struct IState {
    virtual IState* On(const char* ev, Machine& m) = 0;
    virtual const char* Name() const = 0;
    virtual ~IState() {}
};

struct Locked : IState {
    IState* On(const char* ev, Machine& m) override;
    const char* Name() const override { return "locked"; }
};

struct Unlocked : IState {
    IState* On(const char* ev, Machine& m) override;
    const char* Name() const override { return "unlocked"; }
};

struct StringQueue {
    const char* const* items;
    int head;
    int tail;
    StringQueue(const char* const* p, int n) : items(p), head(0), tail(n) {}
    int Count() const { return tail - head; }
    const char* Dequeue() { return items[head++]; }
};

struct Machine {
    int Coins = 0, Passes = 0, Alarms = 0, Refunds = 0;
    IState* State = new Locked();
    static inline bool Closed = false;        // INL-4
    Machine() {}
    Machine(const Machine&) = delete;         // the machine owns State
    Machine& operator=(const Machine&) = delete;
    ~Machine() { delete State; }
    void Run(StringQueue& events) {
        while (events.Count() > 0) {
            const char* ev = events.Dequeue();
            if (same(ev, "stop")) return;
            IState* next = State->On(ev, *this);
            if (next != State) {
                delete State;
                State = next;
            }
        }
    }
    void Dispose() { Closed = true; }
};

IState* Locked::On(const char* ev, Machine& m) {
    if (same(ev, "coin")) { m.Coins++; return new Unlocked(); }
    if (same(ev, "push")) m.Alarms++;
    return this;
}

IState* Unlocked::On(const char* ev, Machine& m) {
    if (same(ev, "push")) { m.Passes++; return new Locked(); }
    if (same(ev, "coin")) m.Refunds++;
    return this;
}

// The using statement: Dispose runs when the guard's block ends.
struct Using {
    Machine& m;
    explicit Using(Machine& target) : m(target) {}
    ~Using() { m.Dispose(); }
};

int main() {
    int failures = 0;
    const char* events[] = { "push", "coin", "coin", "push", "push", "coin", "stop", "push" };
    StringQueue q(events, 8);
    Machine m;
    {
        Using scope(m);
        m.Run(q);
    }
    if (m.Coins != 2 || m.Refunds != 1 || m.Passes != 1 || m.Alarms != 2) failures++;
    if (!same(m.State->Name(), "unlocked")) failures++;
    if (!Machine::Closed) failures++;
    if (q.Count() != 1) failures++;
    return failures;
}
