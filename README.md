# rust-interpreter

A simple module that converts bytecode and performs arithmetic operations and looping as well.

```
Suppose we added the following bytecode instructions to our language:


SEND_CHANNEL:
Pops the channel and a value from the stack and sends the value on the channel using a blocking send

RECV_CHANNEL:
Pops the channel from the stack receives a value from the channel(this may block), and push the resulting value back onto the stack

SPAWN:
Pop two functions from the stack and spawn them as concurrent tasks
```

For the above question, the answer is as follows
- SEND_CHANNEL: In this case, I would use single threaded method to perform the operation. I would get a value from the channel and wait for the operation to get completed
 before moving on to the next task. This would enable the blocking send.
- RECV_CHANNEL: In this case, I would use the same single threaded method to perform the operation. I would receive the value from the channel which would cause blocking 
since it is single threaded. Then once the operation is completed, i would just push the value back in to the stack.
- SPAWN: Since we need to run two tasks concurrently, i would use threads to accomplish this task. I would use the thread module to spawn multiple threads for different functions
and use message passing to send data between the threads if needed. Using the threads, concurrent tasks is possible.

