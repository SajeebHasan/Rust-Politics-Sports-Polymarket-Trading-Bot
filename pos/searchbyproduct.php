<?php require 'asset/header.php' ?>
<html lang="en">
<head>

<script language="javascript">
    function printdiv(printpage)
    {
    var headstr = "<html><head><title>Report</title></head><body>";
    var footstr = "</body>";
    var newstr = document.all.item(printpage).innerHTML;
    var oldstr = document.body.innerHTML;
    document.body.innerHTML = headstr+newstr+footstr;
    window.print();
    document.body.innerHTML = oldstr;
    return ;
    }
  </script>

	
</head>
<body>
<div class="flex-container">
<div id="header">
</div>


<form method="POST" style="text-align:center">
<br><br>
<h3>Product Information </h3><br>
Search By Product id:<br>
 <input name="search_id" value="" type="number"><br>
<br>



<br>
<input type="submit" class="button" value="Search"><br><br>
</form>
</div>
<article class="article">
<div class="myMainContent"  id="div_print">
<center>
  <?php
$servername = "localhost";
$username = "root";
$password = "";
$dbname = "pos";
$id="";
if(isset($_POST["search_id"])){
$id=$_POST["search_id"];
}
// Create connection
$conn = mysqli_connect($servername, $username, $password, $dbname);
// Check connection
if (!$conn) {
    die("Connection failed: " . mysqli_connect_error());
}


$sql = "SELECT * FROM import where Product_Id='$id'";
$result = mysqli_query($conn, $sql);

if (mysqli_num_rows($result) > 0) {
    // output data of each row
    echo "<table border=1>";
	echo "<tr><th>Product Id</th><th> Product Name</th><th>Quantity </th><th>Unit Prize </th></tr>";
    while($row = mysqli_fetch_assoc($result)) {
	echo "<tr>";
        echo "<td>". $row["Product_ID"]. "</td><td>" . $row["Product_Name"]."</td><td>" . $row["Quantity"]."</td><td>" . $row["Product_Price"]."</td>"; 
	echo "</tr>";
    }
echo "</table>";
} else {
    echo "No results Found";
}


mysqli_close($conn);
?> 
</center>
<center><a href="#"  class="no-print" onClick="printdiv('div_print');" ><button>Print</button></a></center>
<br>
<br>
</div>
</div>
<br><br><br><br><br><br><br>
</article>

</body>


 <head>
<style>
.footer {
   position: fixed;
   left: 0;
   bottom: 0;
   width: 100%;
   background-color: black;
   opacity: .7;
   color: white;
   text-align: center;
}
</style>
</head>
<body>
</html>
<?php require 'asset/footer.php' ?>
