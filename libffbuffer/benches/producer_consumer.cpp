#include <iostream>
#include <thread>

#include "../ff_ubuffer.cpp"

using namespace std;

uint64_t N = 1000000;

int main()
{
    void *q = ffubuffer_build(2048);
    if (q == NULL)
    {
        cerr << "fail to create buffer" << endl;
        return 1;
    }

    auto t1 = thread([=]() {
        // cout << "inside t1" << endl;
        for (auto i = 0; i < N; i += 1)
        {
            int64_t *el = new int64_t(i);
            if (!ffubuffer_push(q, el))
            {
                cerr << "fail to push" << endl;
                exit(1);
            }
            // cout << "send element: " << *el << endl;
        }
        if (!ffubuffer_push(q, new int64_t(-1)))
        {
            cerr << "fail to push" << endl;
            exit(1);
        }
    });

    auto t2 = thread([=]() {
        // cout << "inside t2" << endl;
        int64_t count(0);
        while (1)
        {
            int64_t *el = (int64_t *)ffubuffer_pop(q);
            if (el == NULL) continue;
            if (*el == -1) break;
            count += *el;
            free(el);
            // cout << "get element: " << *el << endl;
        };
        cout << "count:" << count << endl;
    });

    t1.join();
    t2.join();
    ffubuffer_destroy(q);

    return 0;
}