#include <iostream>
#include <thread>

#include "../ff_ubuffer.hpp"

using namespace std;

extern "C" {
uint64_t bench_producer_consumer_cpp(uint64_t n) {
  void *q = ffubuffer_build(2048);
  if (q == NULL) {
    cerr << "fail to create buffer" << endl;
    return 1;
  }

  auto t1 = thread([=]() {
    // cout << "inside t1" << endl;
    for (auto i = 0u; i < n; i += 1) {
      int64_t *el = new int64_t(i);
      if (!ffubuffer_push(q, el)) {
        cerr << "fail to push" << endl;
        exit(1);
      }
      // cout << "send element: " << *el << endl;
    }
    if (!ffubuffer_push(q, new int64_t(-1))) {
      cerr << "fail to push" << endl;
      exit(1);
    }
  });

  uint64_t count(0);
  auto t2 = thread([&]() {
    // cout << "inside t2" << endl;
    while (1) {
      int64_t *el = (int64_t *)ffubuffer_pop(q);
      if (el == NULL)
        continue;
      if (*el == -1)
        break;
      count += *el;
      free(el);
    };
  });

  t1.join();
  t2.join();
  ffubuffer_destroy(q);

  return count;
}
}

#ifdef WITH_MAIN
int main() {
  auto start = chrono::high_resolution_clock::now();
  auto res = bench_producer_consumer_cpp(1000000);
  auto end = chrono::high_resolution_clock::now();
  auto diff = chrono::duration_cast<chrono::milliseconds>(end - start);
  cerr << diff.count() << "ms" << endl;
  cout <<  res << endl;
}
#endif
