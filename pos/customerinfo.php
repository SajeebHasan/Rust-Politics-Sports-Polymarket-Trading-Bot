<?php require 'asset/header.php' ?>
<?php
	$servername = "localhost";
	$username = "root";
	$password = "";
	$dbname = "pos";
	// Create connection
	$conn = new mysqli($servername, $username, $password,$dbname);

	// Check connection
	if ($conn->connect_error) {
		die("Connection failed: " . $conn->connect_error);
	}
	//echo "Connected successfully\n";

		$sql = "SELECT * FROM bill";

		$result = $conn->query($sql);

echo "<table border='1' cellpadding='10'> ;";

echo "<tr><th>Memo No</th> <th>Customer Name</th> <th>Customer Address</th> </tr>";



// loop through results of database query, displaying them in the table

while ($row = $result->fetch_assoc()) {



// echo out the contents of each row into a table

echo "<tr>";

echo '<td>' . $row['Memo_no'] . '</td>';

echo '<td>' . $row['Customer_Name'] . '</td>';

echo '<td>' . $row['Address'] . '</td>';


echo "</tr>";

}



// close table>

echo "</table>";

?>
<?php require 'asset/footer.php' ?>