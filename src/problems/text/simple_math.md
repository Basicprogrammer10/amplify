First off, Welcome to _amplify_!

Let's start with some basic info that will apply to every problem:

- You write your code in the text box provided below, pick the language you want to use then press run or use `CTRL + ENTER`
- The code is build and run in the server in a docker container and its STDOUT is compared to the correct output
- The tests are supplied to your program as command line arguments separated by a space
- This is the first time I've done anything like this so if you run into any issues you can file an issue on GitHub [here](https://github.com/Basicprogrammer10/amplify/issues)
- These problems will start simple but get a bit more complicated as time goes on

So now that we got that out of the way on to problem[0]

As an aspiring elementary school teacher you have to check lots of students math homework.
Fortunately most of them _don't do homework_ so you only have to grade 10 problems.
Being the programmer you are you see an opportunity to make a program that will grade the homework for you that will ultimately take 4 times more time than just grading them normally.

Each argument supplied to your program is a simple math expression consisting of two numbers that are either added, subtracted or multiplied with each other.
Every number will be 3 digits long (100-999) so every test in the following format <code>###(+|-|\*)###</code>.
Print the output of each expression on a new line.
View an example input and output [here](https://paste.connorcode.com/b/c0747851-eeea-4705-a009-a669b018c1ce).
