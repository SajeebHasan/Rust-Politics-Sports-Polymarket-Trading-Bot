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

		$sql = "SELECT * FROM staff";

		$result = $conn->query($sql);

echo "<table border='1' cellpadding='10'> ;";

echo "<tr><th>Staff Name</th> <th>Staff Address</th> <th> Staff Contact</th> </tr>";



// loop through results of database query, displaying them in the table

while ($row = $result->fetch_assoc()) {



// echo out the contents of each row into a table

echo "<tr>";

echo '<td>' . $row['Staff_Name'] . '</td>';

echo '<td>' . $row['Staff_Address'] . '</td>';

echo '<td>' . $row['Staff_Contact'] . '</td>';


echo "</tr>";

}



// close table>

echo "</table>";

?>
<?php require 'asset/footer.php' ?>