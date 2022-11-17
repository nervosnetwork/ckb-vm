
Mainly used to test whether there is overflow of memory.
Use ```jemalloc``` to get the memory status, call ckb-vm in a loop, and check whether the output value is abnormal.

Here use ```ps -p PID -o rss``` to get the memory used by the current process.