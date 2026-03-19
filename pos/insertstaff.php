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
$Staff_Name = mysqli_real_escape_string($link, $_REQUEST['Staff_Name']);
$Staff_Address = mysqli_real_escape_string($link, $_REQUEST['Staff_Address']);
$Staff_Contact = mysqli_real_escape_string($link, $_REQUEST['Staff_Contact']);
 
// attempt insert query execution
$sql = "INSERT INTO staff (Staff_Name, Staff_Address, Staff_Contact) VALUES ('$Staff_Name', '$Staff_Address', '$Staff_Contact')";
if(mysqli_query($link, $sql)){
    echo "Records added successfully.";
} else{
    echo "ERROR: Could not able to execute $sql. " . mysqli_error($link);
}
 
// close connection
mysqli_close($link);
?>
<?php require 'asset/footer.php' ?>
