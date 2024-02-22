# Local test server 

Load-testing is not as complex as other task, yet requires a lot of different scenarion to cover.

This small express test server helps us to go it

Simply start the server and access by http://localhost:8083/
```bash
npm run-script dev 
```

# Test URLs

There are few URLs type:
- returning specific HTTP code w/o delay
    - http://localhost:8083/code/201 - return HTTP 201

- returning HTTP 200, with requested delay or delay range
    - http://localhost:8083/delay/100 – 100 ms delay
    - http://localhost:8083/delay/100-500 – delay between 100 and 500 ms

- returning random HTTP code w/o delay
    - http://localhost:8083/random

- returning random HTTP code, with requested delay or delay range
    - http://localhost:8083/random – no delay
    - http://localhost:8083/random/delay/100 – 100 ms delay
    - http://localhost:8083/random/delay/100-500 – delay between 100 and 500 ms
