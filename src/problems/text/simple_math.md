First off, Welcome to _amplify_!

Let's start with some basic info that will apply to every problem:

- You write your code in the text box provided below, pick the language you want to use then press run or use `CTRL + ENTER`
- Your code is saved when it is run
- Make sure save your code before changing the language or your progress will be deleted, yes i could fix this, no im not going to
- The code is build and run in the server in a docker container and its [STDOUT](stdout) is compared to the correct output
- The tests are supplied to your program as command line arguments separated by a space
- If you don't know how to get command line arguments in any of the supported languages, you can look [here](https://paste.connorcode.com/b/05fa1532-4368-448a-b12c-e8cf1119bc00)
- This is the first time I've done anything like this so if you run into any issues you can file an issue on GitHub [here](https://github.com/Basicprogrammer10/amplify/issues)
- These problems will start simple but get a bit more complicated as time goes on

So now that we got that out of the way on to problem[0]

As an aspiring elementary school teacher you have to check lots of students math homework.
Fortunately most of them _don't do homework_ so you only have to grade 10 problems total.
Being the programmer you are you see an opportunity to make a program that will grade the homework for you that will ultimately take 4 times more time than just grading them normally!

Each argument supplied to your program is a simple math expression consisting of two numbers that are either added, subtracted or multiplied with each other.
Every number will be 3 digits long (100-999) so every test in the following format <code>###(+|-|\*)###</code>.
Print the output of each expression on a new line.
View an example input and output [here](https://paste.connorcode.com/b/8c5fe335-d6f2-447d-9b54-1af176cd0968).

[stdout]: https://en.wikipedia.org/wiki/Standard_streams