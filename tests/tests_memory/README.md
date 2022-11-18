
Mainly used to test whether there is overflow of memory.
Use ```jemalloc``` to get the memory status, call ckb-vm in a loop, and check whether the output value is abnormal.


## Examples
### check_real_memory
This example can be used to test whether the real memory overflows.
Because different environments will cause errors in real memory usage, it is not included in the test case.
Here use ```ps -p PID -o rss``` to get the memory used by the current process. It can output the memory occupied by the current process after each execution of ckb-vm.
