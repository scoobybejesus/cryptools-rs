// Copyright (c) 2017-2019, scoobybejesus

// Redistributions must include the license: https://github.com/scoobybejesus/cryptools-rs/blob/master/LEGAL.txt

---

# Examples for using cryptools-rs

The sample input files and the resulting reports are in the `/examples/resources/` directory.

## Using the wizard

##### First, preview the input file `faker1__sample_input.csv`.

&nbsp;&nbsp;&nbsp;&nbsp; (You'll see it is the same (or roughly the same) as the README example.)

##### We're going to pass in that file as a command-line argument (no flags are required).
Enter the following:

##### &nbsp;&nbsp;&nbsp;&nbsp;`cargo run -- ./examples/resources/faker1__sample_input.csv`

&nbsp;&nbsp;&nbsp;&nbsp; (Substitute a Windows-style file path, if necessary.)

Running that command takes you to the wizard.

&nbsp;&nbsp;&nbsp;&nbsp; (Note: You can simply run **`cargo run`** instead, in which case after answering yes to "Shall we proceed," you will have to enter the absolute path of the input file, but it's easier to pass in the relative path.)

##### Type `<Enter>` to accept default responses to the first three prompts, which are:

* Shall we proceed?
* Choose the lot inventory costing method.
* Continue without like-kind treatment?

##### The final question asks if and where you'd like to save the reports.

The default is the current directory, which is probably undesirable.
Type `c` and `<Enter>` to change the directory.
Then tab-complete your way through `/Users/<your-username>/Documents`*, for example, and then `<Enter>`.

&nbsp;&nbsp;&nbsp;&nbsp;\* This would be different for Windows, of course.

##### Now the program has ended, and you should have reports in the directory you provided.

The reports should match those in the `examples/resources` directory.


## Skipping the wizard

This time around, we'll pass command-line parameters to skip past the wizard.

##### Again, preview the input file `faker2__sample_input.csv`.

You'll see it's similar to the README example, except that there is a slightly wider variety of transactions, plus the memos are more descriptive.

##### Run **`cargo run -- --help`** to see descriptions for the parameters we can use, or just enter:

##### &nbsp;&nbsp;&nbsp;&nbsp;`cargo run -- -a -o ~/Documents ./examples/resources/faker2__sample_input.csv`

&nbsp;&nbsp;&nbsp;&nbsp;\* Substitute `~/Documents` with your desired output directory. Substitute a Windows-style file path, if necessary.

##### Again, the program runs, and you should have reports in the location you provided.

We bypassed the wizard because:

1. The  required parameters were passed.
2. Default values were used for parameters not passed.
3. The `-a` flag was set which accepts all parameters without asking twice (i.e., skips the wizard).

The only "required" parameter is the input file.
All other parameters have default values.
The default values may not be desirable for your use case, however.
You may want FIFO instead of LIFO.
You may need EUR instead of USD.
You may want to apply like-kind exchange treatment through a particular date.
These parameters can be set via command line parameters.

## Notes on the reports

There are two style of reports: Accounts and Transactions.

### Accounts

* This style of report shows you what you have.
* *Account* reports reflect the balances in each account along with the cost basis of those holdings.  (The exception being the home currency accounts.  In that case, it merely reflects the net of how much was deposited/spent.)
* Future reports will be written that provide details of every deposit and every withdrawal, on a movement-by-movement basis, into and out of every lot, for each account.

### Transactions

* This style of report show you how your holdings performed (not including unrealized gains/losses).
* *Transaction* reports reflect the result of each transaction (specifically, each transaction which results in a gain/loss or income/expense).
* Future reports will be written that provide a more detailed breakdown of the income/expense/gain/loss from every movement (or possibly a more summarized version, too).

#### Other notes and limitations

* There is no place where transaction fees are recorded.
Whether in an exchange transaction or when sending to yourself, in the input file you simply reflect the net amount leaving one account and the net amount entering the other account.
That "cost" simply remains as cost basis until the holding is disposed.
* Transactions in which cryptocurrency is spent reflect a gain/loss ***and*** an expense.
Perhaps your goverment does not require you to reflect any gain or loss resulting from purchases, in which case you would presumably separate expense those rows from the rest.
* There is currently no way to provide sub-classifications for income and expenses.
For example, in `faker2`, there is mining income and there is remote tech support income.  Also, there is a coffee mug purchase and a donation.
The details are easily found in the memo field, but it is up to the user to "parse" those items for their use.
* As noted in the README, adjustments occasionally may need to be made to the output reports.
For example, in `faker2`, a donation was made.
You may choose (or your accountant may decide) that this transaction (#11) is analogous to donating an appreciated security.
In that case, you may decide to manually adjust the **Proceeds** to match the **Cost basis**, and then update the **Gain/loss** to be zero.
* You can't spend or exchange something that you don't have.
The program is designed to crash if you try to do this.
That is, the program is unable to function properly if your holdings appear to go negative.
The only accounts that are permitted to go negative are home currency accounts.
The balances of all other currencies **must** remain greater than or equal to 0.
(The exception to this is margin accounts, since the liability reflects as a negative balance.)