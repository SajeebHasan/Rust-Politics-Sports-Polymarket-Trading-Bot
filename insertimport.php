<?php require 'asset/header.php' ?>
<?php
/* Attempt MySQL server connection. Assuming you are running MySQL
server with default setting (user 'root' with no password) */
$link = mysqli_connect("localhost", "root", "", "pos");
 
// Check connection
if($link === false){
    die("ERROR: Could not connect. " . mysqli_connect_error());
}
 
// Escape user inputs for security
$Product_ID = mysqli_real_escape_string($link, $_REQUEST['Product_ID']);
$Product_Name = mysqli_real_escape_string($link, $_REQUEST['Product_Name']);
$Quantity = mysqli_real_escape_string($link, $_REQUEST['Quantity']);
$Product_Price = mysqli_real_escape_string($link, $_REQUEST['Product_Price']);
$Total_Price = mysqli_real_escape_string($link, $_REQUEST['Total_Price']);
 
// attempt insert query execution
$sql = "INSERT INTO import (Product_ID, Product_Name, Quantity, Product_Price, Total_Price) VALUES ('$Product_ID','$Product_Name', '$Quantity', '$Product_Price','$Total_Price')";
if(mysqli_query($link, $sql)){
    echo "Records added successfully.";
} else{
    echo "ERROR: Could not able to execute $sql. " . mysqli_error($link);
}
 
// close connection
mysqli_close($link);
?>
<?php require 'asset/footer.php' ?>
