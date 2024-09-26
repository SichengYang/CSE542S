CSE 542 Fall 2024 Lab 1

## Basic Information about Project

* Group member:
Qinzhou Song qinzhounick@gmail.com
Sicheng Yang sicheng@wustl.edu

* Program Design:
This program will generate scribe text based on configuration and each character's speaking. Our program
will first read the configuration file and collect each character's speaking based on the configuration.
After reading the text, our program will sort the scribe based on order provided and output the scribe on
the screen. If the user enable whinge mode, we will also print out warning message related to error format.

* Insights, observations and questions:
We find the collaboration in project is harder than we thought. I think we can talk about how to write 
collaborate code in class. Especially how to pack everybody's program together(eliminating errors).

## Detailed Instruction

* How to unpack the program: download the lab.zip and unzip the package

* How to build the program:
1. Please put all the file under lab1 folder
2. use "rustc ./src/main.rs" to compile the program
3. you can use "./main <configuration file>" to ignore warning message or 
"./main <configuration file> whinge" to enable warning message. You can also use "cargo run <configuration file>"
or "cargo run <configuration file> whinge"

## Test Cases

1. 100% corrent input and command using example provided by Dr. Gill

[sicheng@iht32-1508.sif lab1]$ ./main config.txt 
Title of the play: Hamlet Prince of Denmark ACT II Scene II A room in the Castle by William Shakespeare

King.
Welcome, dear Rosencrantz and Guildenstern!
Moreover that we much did long to see you,
The need we have to use you did provoke
Our hasty sending. Something have you heard
Of Hamlet's transformation; so I call it,
Since nor the exterior nor the inward man
Resembles that it was. What it should be,
More than his father's death, that thus hath put him
So much from the understanding of himself,
I cannot dream of: I entreat you both
That, being of so young days brought up with him,
And since so neighbour'd to his youth and humour,
That you vouchsafe your rest here in our court
Some little time: so by your companies
To draw him on to pleasures, and to gather,
So much as from occasion you may glean,
Whether aught, to us unknown, afflicts him thus,
That, open'd, lies within our remedy.

Queen.
Good gentlemen, he hath much talk'd of you,
And sure I am two men there are not living
To whom he more adheres. If it will please you
To show us so much gentry and good-will
As to expend your time with us awhile,
For the supply and profit of our hope,
Your visitation shall receive such thanks
As fits a king's remembrance.

Rosencrantz.
Both your majesties
Might, by the sovereign power you have of us,
Put your dread pleasures more into command
Than to entreaty.

Guildenstern.
We both obey,
We both obey,
And here give up ourselves, in the full bent,
To lay our service freely at your feet,
To be commanded.

King.
King.
Thanks, Rosencrantz and gentle Guildenstern.
Thanks, Rosencrantz and gentle Guildenstern.


Queen.
Queen.
Thanks, Guildenstern and gentle Rosencrantz:
And I beseech you instantly to visit
My too-much-changed son.--Go, some of you,
And bring these gentlemen where Hamlet is.

Guildenstern.
Heavens make our presence and our practices
Pleasant and helpful to him!

Queen.
Ay, amen!

2. User input incorrect command
[sicheng@iht32-1508.sif lab1]$ ./main error error error
usage: ./main <configuration_file_name> [whinge]
Error: 1

3. Configuration file have incorrect file name
[sicheng@iht32-1508.sif lab1]$ ./main wrong_config.txt
ghost.txt is not a valid filename
Error: 2

4. Winge mode on without error
[sicheng@iht32-1507.sif lab1]$ ./main config.txt whinge
Title of the play: Hamlet Prince of Denmark ACT II Scene II A room in the Castle by William Shakespeare

King.
Welcome, dear Rosencrantz and Guildenstern!
Moreover that we much did long to see you,
The need we have to use you did provoke
Our hasty sending. Something have you heard
Of Hamlet's transformation; so I call it,
Since nor the exterior nor the inward man
Resembles that it was. What it should be,
More than his father's death, that thus hath put him
So much from the understanding of himself,
I cannot dream of: I entreat you both
That, being of so young days brought up with him,
And since so neighbour'd to his youth and humour,
That you vouchsafe your rest here in our court
Some little time: so by your companies
To draw him on to pleasures, and to gather,
So much as from occasion you may glean,
Whether aught, to us unknown, afflicts him thus,
That, open'd, lies within our remedy.

Queen.
Good gentlemen, he hath much talk'd of you,
And sure I am two men there are not living
To whom he more adheres. If it will please you
To show us so much gentry and good-will
As to expend your time with us awhile,
For the supply and profit of our hope,
Your visitation shall receive such thanks
As fits a king's remembrance.

Rosencrantz.
Both your majesties
Might, by the sovereign power you have of us,
Put your dread pleasures more into command
Than to entreaty.

Guildenstern.
We both obey,
And here give up ourselves, in the full bent,
To lay our service freely at your feet,
To be commanded.

King.
Thanks, Rosencrantz and gentle Guildenstern

Queen.
Thanks, Guildenstern and gentle Rosencrantz:
And I beseech you instantly to visit
My too-much-changed son.--Go, some of you,
And bring these gentlemen where Hamlet is.

Guildenstern.
Heavens make our presence and our practices
Pleasant and helpful to him!

Queen.
Ay, amen!

5. whinge mode on with incorrect formatted line *****************
[sicheng@iht32-1507.sif lab1]$ ./main incorrect_line_config.txt whinge
Config file line "Incorrect" length not equal to 2
* (long essay omitted) *

6. Character's file include incorrect order number token
[sicheng@iht32-1507.sif lab1]$ ./main token_err_config.txt whinge
Token 1a does not represent a value in usize
* (long essay omitted) *

7. Character file include blank line or a line with solely whitespace
[sicheng@iht32-1507.sif lab1]$ ./main whitespace_config.txt whinge
Title of the play: Hamlet Prince of Denmark ACT II Scene II A room in the Castle by William Shakespeare

King.
Welcome, dear Rosencrantz and Guildenstern!
Moreover that we much did long to see you,
The need we have to use you did provoke
Our hasty sending. Something have you heard
Of Hamlet's transformation; so I call it,
Since nor the exterior nor the inward man
Resembles that it was. What it should be,
More than his father's death, that thus hath put him
So much from the understanding of himself,
I cannot dream of: I entreat you both
That, being of so young days brought up with him,
And since so neighbour'd to his youth and humour,
That you vouchsafe your rest here in our court
Some little time: so by your companies
To draw him on to pleasures, and to gather,
So much as from occasion you may glean,
Whether aught, to us unknown, afflicts him thus,
That, open'd, lies within our remedy.

Queen.
Good gentlemen, he hath much talk'd of you,
And sure I am two men there are not living
To whom he more adheres. If it will please you
To show us so much gentry and good-will
As to expend your time with us awhile,
For the supply and profit of our hope,
Your visitation shall receive such thanks
As fits a king's remembrance.

Rosencrantz.
Both your majesties
Might, by the sovereign power you have of us,
Put your dread pleasures more into command
Than to entreaty.

Guildenstern.
We both obey,
And here give up ourselves, in the full bent,
To lay our service freely at your feet,
To be commanded.

King.
Thanks, Rosencrantz and gentle Guildenstern

Queen.
Thanks, Guildenstern and gentle Rosencrantz:
And I beseech you instantly to visit
My too-much-changed son.--Go, some of you,
And bring these gentlemen where Hamlet is.

Guildenstern.
Heavens make our presence and our practices
Pleasant and helpful to him!

Queen.
Ay, amen!

9. Negative number in speaking order
[sicheng@iht32-1507.sif lab1]$ ./main crazy_number_config.txt whinge
Token -1 does not represent a value in usize
* (long essay omitted) *