
Mainly used to test whether there is overflow of memory.
Use ```jemalloc``` to get the memory status, call ckb-vm in a loop, and check whether the output value is abnormal.

Here use ```ps -p PID -o rss``` to get the memory used by the current process.

Here we can get the actual memory usage of the test case. But because the memory of the test case is unpredictable in actual operation, it is not verified in the test code. If you want to see changes in memory, you can call ```_get_current_memory()```.