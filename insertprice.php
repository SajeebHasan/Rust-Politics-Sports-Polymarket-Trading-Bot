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
$Price_per_kg = mysqli_real_escape_string($link, $_REQUEST['Price_per_kg']);
$About = mysqli_real_escape_string($link, $_REQUEST['About']);
$Total_Price = mysqli_real_escape_string($link, $_REQUEST['Total_price']);
 
// attempt insert query execution
$sql = "INSERT INTO price (Product_ID, Product_Name, Price_per_kg, About, Total_Price) VALUES ('$Product_ID','$Product_Name', '$Price_per_kg', '$About','$Total_Price')";
if(mysqli_query($link, $sql)){
    echo "Records added successfully.";
} else{
    echo "ERROR: Could not able to execute $sql. " . mysqli_error($link);
}
 
// close connection
mysqli_close($link);
?>
<?php require 'asset/footer.php' ?>
