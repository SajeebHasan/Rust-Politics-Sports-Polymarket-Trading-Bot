<?php
/* Attempt MySQL server connection. Assuming you are running MySQL
server with default setting (user 'root' with no password) */
$link = mysqli_connect("localhost", "root", "", "pos");
 
// Check connection
if($link === false){
    die("ERROR: Could not connect. " . mysqli_connect_error());
}
 
// Escape user inputs for security
$Memo_no = mysqli_real_escape_string($link, $_REQUEST['Memo_no']);
$Date = mysqli_real_escape_string($link, $_REQUEST['Date']);
$Customer_Name = mysqli_real_escape_string($link, $_REQUEST['Customer_Name']);
$Address = mysqli_real_escape_string($link, $_REQUEST['Address']);
$Description_Of_Goods = mysqli_real_escape_string($link, $_REQUEST['Description_Of_Goods']);
$Quantity = mysqli_real_escape_string($link, $_REQUEST['Quantity']);
$Unit_Price = mysqli_real_escape_string($link, $_REQUEST['Unit_Price']);
//$TotalOne = mysqli_real_escape_string($link, $_REQUEST['Total']);

$Total = $Quantity*$Unit_Price;

$Total_Tk = $Total;

$Advance = mysqli_real_escape_string($link, $_REQUEST['Advance']);
$Balance = $Total-($Advance/100)*$Total;
 
// attempt insert query execution
$sql = "INSERT INTO bill (Memo_no, Date, Customer_Name,Address, Description_Of_Goods,Quantity,Unit_Price,Total,Total_Tk,Advance,Balance) VALUES ('$Memo_no', '$Date', '$Customer_Name', '$Address', '$Description_Of_Goods', '$Quantity', '$Unit_Price', '$Total', '$Total_Tk', '$Advance', '$Balance')";
if(mysqli_query($link, $sql)){
    include("transections.php");
} else{
    echo "ERROR: Could not able to execute $sql. " . mysqli_error($link);
}
 
// close connection
mysqli_close($link);
?>
