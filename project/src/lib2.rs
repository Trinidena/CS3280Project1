// C.	The process_input_file function
// IN this version, this function will change a little bit.  Instead of returning a message to the calling function, it will have to write the message to the channel received from the main thread.
// a.	Accept the list folders as parameters.
// b.	Loop through the folder list and do the following -
// i.	Open the input file in the folder
// ii.	Read each line and extract the quantity sold and add it to a sum variable
// iii.	Send a string that will have  the branch code,  product code and the total sale separated by comma to the calling thread (main) through the channel.
// Example: “ALBNM, PROD001, 32”
// iv.	Log messages in a log file using any of the logging crates available for RUST.  The name of the log file should be log.txt and should be placed in the root folder of the app.
// 1.	You should log any errors
// 2.	You should log the name of each of the folders you have processed in that thread.
// c.	Once the loop is done return either of the following string message to the calling function depending on the status of the file processing - “OK”,  “ERROR”
