// In this version, you will move the output function (write_to_summary_file) to write to file from lib.rs to the main.rs file.
// 1.	Accept, validate and parse the data folder name (with path) from the command line argument.
// 2.	Check if the output folder exists - if not, create it.
// 3.	Create a channel to communicate with child threads that you will create in the next step.  This channel is what children threads will use to pass the output string that will have  the branch code,  product code and the total sale separated by comma.
// 4.	Split all the branch input folders into 4 groups with 10 folders in each group
// 5.	Start a timer
// 6.	In a loop create 4 threads to run the input processing function process_input_file in lib.rs on each of the group of folders you created in step 4 above.
// a.	The file input function in lib.rs will accept a list of 10 folders (with path) for all the branches.  Remember, in version 1 you already have this implemented.
// b.	Make sure you wait for each thread to finish
// 7.	Once all the threads are done processing their assigned folders,
// a.	In a loop, process all the messages received in the channel from the children threads -
// i.	Go through all the messages in a loop and
// 1.	print them on the console
// 2.	Write them to the output file using the write_to_summary_file file described in Version 1.
// 8.	Stop the timer and print the total time in the console
// 9.	Print the following message to indicate processing of all files are done - “Phew!  I am done.”
