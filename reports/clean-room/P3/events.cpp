// Port of Events.cs. A delegate is an Action<Arg>: it owns a closure of any
// type behind a common base with a virtual call operator (LAM-P4, VIRT-P1).
// An Event keeps a list of pointers to Actions. += adds, -= removes the
// first match by identity. Raising the event copies the list first, as a C#
// multicast delegate is immutable, so a handler that removes itself does not
// disturb the handlers after it.

template <class Arg> struct Callable {
    virtual void operator()(Arg a) = 0;
    virtual ~Callable() {}                     // VIRT-P3: delete through the base
};

template <class Arg, class F> struct CallableOf : Callable<Arg> {
    F f;
    explicit CallableOf(F g) : f(g) {}
    void operator()(Arg a) override { f(a); }
};

template <class Arg> class Action {
    Callable<Arg>* target = nullptr;
public:
    Action() {}
    Action(const Action&) = delete;            // one owner of the closure
    Action& operator=(const Action&) = delete;
    // C# writes "onceOnly = v => ...". tidal-fe does not run an operator=
    // whose argument is a class object (see reductions/), so a named
    // member function sets the target.
    template <class F> void Set(F f) {
        delete target;
        target = new CallableOf<Arg, F>(f);
    }
    template <class F> Action(F f) : target(new CallableOf<Arg, F>(f)) {}
    ~Action() { delete target; }
    void Invoke(Arg a) { (*target)(a); }
};

template <class Arg> class Event {
    static const int Capacity = 8;
    Action<Arg>* handlers[Capacity];
    int count = 0;
public:
    Event& operator+=(Action<Arg>& h) {
        if (count < Capacity) handlers[count++] = &h;
        return *this;
    }
    Event& operator-=(Action<Arg>& h) {
        for (int i = 0; i < count; ++i) {
            if (handlers[i] == &h) {
                for (int j = i + 1; j < count; ++j) handlers[j - 1] = handlers[j];
                --count;
                break;
            }
        }
        return *this;
    }
    void Invoke(Arg a) {
        Action<Arg>* snapshot[Capacity];
        int n = count;
        for (int i = 0; i < n; ++i) snapshot[i] = handlers[i];
        for (int i = 0; i < n; ++i) snapshot[i]->Invoke(a);
    }
};

class Sensor {
    int threshold;
    bool above = false;
public:
    Event<int> Crossed;
    explicit Sensor(int t) : threshold(t) {}
    void Read(int value) {
        bool now = value > threshold;
        if (now != above) { above = now; Crossed.Invoke(value); }
    }
};

struct IntList {
    int items[16];
    int Count = 0;
    void Add(int v) { if (Count < 16) items[Count++] = v; }
    int operator[](int i) const { return items[i]; }
};

int main() {
    int failures = 0;
    Sensor sensor(50);
    IntList log;
    int once = 0;
    Action<int> logIt = [&log](int v) { log.Add(v); };
    Action<int> onceOnly;
    onceOnly.Set([&](int) { once++; sensor.Crossed -= onceOnly; });
    sensor.Crossed += logIt;
    sensor.Crossed += onceOnly;
    int values[] = { 10, 60, 70, 40, 55, 20 };
    for (int v : values) sensor.Read(v);
    if (log.Count != 4) failures++;
    if (log.Count == 4 && (log[0] != 60 || log[1] != 40 || log[2] != 55 || log[3] != 20)) failures++;
    if (once != 1) failures++;
    sensor.Crossed -= logIt;
    sensor.Read(99);
    if (log.Count != 4) failures++;
    return failures;
}
