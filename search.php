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
<h3>Bill Information </h3><br>
Search By Memo no:<br>
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


$sql = "SELECT * FROM bill where Memo_no='$id'";
$result = mysqli_query($conn, $sql);

if (mysqli_num_rows($result) > 0) {
    // output data of each row
    echo "<table border=1>";
	echo "<tr><th>Memo no</th><th> Date</th><th>Customer Name</th><th>Address </th><th>Description</th><th>Quantity</th>
    <th>Unit Price</th><th>Total</th><th>Total Tk</th><th>Advance</th><th>Balance</th></tr>";
    while($row = mysqli_fetch_assoc($result)) {
	echo "<tr>";
        echo "<td>". $row["Memo_no"]. "</td><td>" . $row["Date"]."</td><td>" . $row["Customer_Name"]."</td><td>" . $row["Address"]."</td><td>" . $row["Description_Of_Goods"]."</td><td>" . $row["Quantity"]."</td><td>" . $row["Unit_Price"]."</td><td>" . $row["Total"]."</td><td>" . $row["Total_Tk"]."</td><td>" . $row["Advance"]."</td><td>" . $row["Balance"]."</td>"; 
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
