## rustloop 
My reimplementation of https://github.com/lesacar/loop in rust
Rust can get 3200 echoes per 5 seconds, if using 1ms sleep, this number should obviously be 5000\
I can't confirm it, but I suspect this program also doesn't wait for the program to finish executing before looping it again, so you could very easily leak memory (rust btw)\
With cmd.spawn() and cmd.wait() they both get 3200 commands per 5 seconds on 1ms delay, meanwhile\
\
the C version gets 1200 commands per 5 seconds using 1ms delay with waiting for the process being looped to finish\
and without waiting, it gets 5000 processes ran in 5 sec, which is correct ( +-50 )
