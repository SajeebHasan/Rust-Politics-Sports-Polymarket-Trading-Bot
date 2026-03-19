<?php require 'asset/header.php' ?>
<style>
table, th, td {
    border: 1px solid black;
}
</style>
</head>
<body>
<body style='text-align:center';</body>
<style>
// THIS STYLE IS IMPORTANT FOR POPUP UPDATE PAGE 
	.p{
	background: #f5f3f3;
	z-index: 9999;
	height: 300px;
	width: 400px;
	position: absolute;
	margin-left: 20%;
	margin-top: -60px;
	padding-bottom: 35px;
	border: 1px dotted green;
	padding-top: 15px;
}
</style>






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
		echo"<table align=center><tr><th>Memo No</th><th>Date </th><th>Customer Name</th><th>Address </th><th>Description of Goods </th><th>Quantity </th><th>Unit Price </th><th>Total </th><th>Total Tk </th><th>Advance </th><th>Balance </th><th>Edit</th><th>Delete</th><th>Print</th></tr>";

		if ($result->num_rows > 0) {
			while ($row = $result->fetch_assoc()) {
				// HERE EVERY TABLE DATA WILL CONTAIN DIFFRENT ID TO PRINT
				echo "<tr><td id='a".$row['Memo_no']."'>".$row['Memo_no']."</td><td id='b".$row['Memo_no']."'>".$row['Date']."</td>";
				echo "<td id='c".$row['Memo_no']."'>".$row['Customer_Name']."</td>";
				echo "<td id='c".$row['Memo_no']."'>".$row['Address']."</td>";
				echo "<td id='c".$row['Memo_no']."'>".$row['Description_Of_Goods']."</td>";
				echo "<td id='c".$row['Memo_no']."'>".$row['Quantity']."</td>";
				echo "<td id='c".$row['Memo_no']."'>".$row['Unit_Price']."</td>";
				echo "<td id='c".$row['Memo_no']."'>".$row['Total']."</td>";
				echo "<td id='c".$row['Memo_no']."'>".$row['Total_Tk']."</td>";
				echo "<td id='c".$row['Memo_no']."'>".$row['Advance']."</td>";
				echo "<td id='c".$row['Memo_no']."'>".$row['Balance']."</td>";

				// EDIT BUTTON CREATION
				echo "<td><form action='' method='GET'><input type='submit' name=".$row['Memo_no']." value='Edit'></form></td>";
				// DELETE BUTTON CREATION
				echo "<td><form action='' method='GET'><input type='submit' name='delete".$row['Memo_no']."' value='Delete'></form></td>";
				// PDF BUTTON CREATION
				echo "<td><form action='' method='GET'><input type='submit' name='print".$row['Memo_no']."' value='Print'></form></td></tr>";

				// UPDATE CODE STARTS FROM HERE
				if(isset($_GET[$row['Memo_no']])){
					echo"<form action='' method='POST'><div class='p' id='close'>";// CLASS P IS USED TO DECORATION AND ID CLOSE IS USED TO CLOSE THE POPUP PAGE
					echo"<h2>Update Information</h2>";
					echo "Memo No: <input type='text' name='Memo_no' value=".$row['Memo_no'].">";
					echo "</br></br>";
					echo "Date : <input type='Date' name='Date' value=".$row['Date'].">";
					echo "</br></br>";
					echo "Customer Name: <input type='text' name='Customer_Name' value=".$row['Customer_Name'].">";
					echo "</br></br>";
                    echo "Address : <input type='text' name='Address' value=".$row['Address'].">";
					echo "</br></br>";
                    echo "Description Of Goods: <input type='text' name='Description_Of_Goods' value=".$row['Description_Of_Goods'].">";
					echo "</br></br>";
                    echo "Quantity: <input type='text' name='Quantity' value=".$row['Quantity'].">";
					echo "</br></br>";
                    echo "Unit Price : <input type='text' name='Unit_Price' value=".$row['Unit_Price'].">";
					echo "</br></br>";
                    echo "Total : <input type='text' name='Total' value=".$row['Total'].">";
					echo "</br></br>";
                    echo "Total Tk : <input type='text' name='Total_Tk' value=".$row['Total_Tk'].">";
					echo "</br></br>";
                    echo "Advance  : <input type='text' name='Advance' value=".$row['Advance'].">";
					echo "</br></br>";
                    echo "Balance  : <input type='text' name='Balance' value=".$row['Balance'].">";
					echo "</br></br>";
					echo "</br></br>";

					echo"<input type='submit' name = 'submit' value='Update'>";
					echo"<input type='submit' name = 'cancle' value='Cancle'>";
					echo "</div></form>";

					if(isset($_POST['submit'])){
						$Memo_no = $_POST["Memo_no"];
						$Date = $_POST["Date"];
						$Customer_Name = $_POST["Customer_Name"];
						$Address = $_POST["Address"];
						$Description_Of_Goods = $_POST["Description_Of_Goods"];
						$Quantity = $_POST["Quantity"];
						$Unit_Price = $_POST["Unit_Price"];
						$Total = $_POST["Total"];
						$Total_Tk = $_POST["Total_Tk"];
						$Advance = $_POST["Advance"];
						$Balance = $_POST["Balance"];

						$ssql = "UPDATE bill SET Memo_no='$Memo_no', Date='$Date', Customer_Name='$Customer_Name', Address='$Address', Description_Of_Goods='$Description_Of_Goods', Quantity='$Quantity', Unit_Price='$Unit_Price', Total='$Total', Total_Tk='$Total_Tk', Advance='$Advance', Balance='$Balance'
						WHERE Memo_no=".$row['Memo_no']."";
						
						if ($conn->query($ssql) === TRUE) {
						echo "<script type='text/javascript'>alert('Submitted successfully!')</script>";
						} else {
						echo "Upadate Unsucessful". $conn->error;
						}

					}
					if(isset($_POST['cancle'])){
						echo "<script>document.getElementById('close').style.display='none'</script>";
					}
				}

				// DELETE CODE STARTS FORM HERE
				if(isset($_GET['delete'.$row['Memo_no']])){
					$delete = "DELETE FROM bill WHERE Memo_no=".$row['Memo_no']."";
					if ($conn->query($delete) === TRUE) {
					echo "<script type='text/javascript'>alert('Deleted successfully!')</script>";
					echo "<meta http-equiv='refresh' content='0'>"; // THIS IS FOR AUTO REFRESH CURRENT PAGE
					} else {
					echo "Delete Unsucessful". $conn->error;
					}
				}

				// PDF STARTS FROM HERE
				if(isset($_GET['print'.$row['Memo_no']])){

					echo "<script>
					var mywindow = window.open('', 'PRINT', 'height=400,width=600');
					mywindow.document.write('<html><head><title>' + document.title  + '</title>');
					mywindow.document.write('</head><body >');
					mywindow.document.write('<h1>' + 'Reader Information'  + '</h1>');
					mywindow.document.write(document.getElementById('a".$row['Memo_no']."').innerHTML);
					mywindow.document.write(' -- ');
					mywindow.document.write(document.getElementById('b".$row['Memo_no']."').innerHTML);
					mywindow.document.write(' -- ');
					mywindow.document.write(document.getElementById('c".$row['Memo_no']."').innerHTML);
					mywindow.document.write(' -- ');
					mywindow.document.write(document.getElementById('d".$row['Memo_no']."').innerHTML);
					mywindow.document.write(' -- ');
					mywindow.document.write(document.getElementById('e".$row['Memo_no']."').innerHTML);
					mywindow.document.write(' -- ');
					mywindow.document.write(document.getElementById('f".$row['Memo_no']."').innerHTML);
					mywindow.document.write('</body></html>');
					mywindow.document.close(); // necessary for IE >= 10
					mywindow.focus(); // necessary for IE >= 10*/

					mywindow.print();
					mywindow.close();
					history.back(); // IT WILL TAKE YOU BAKE PAGE
					</script>";
				}





			}echo"</table>";
		}else{
				echo "No search found!!!";
		}
	$conn->close();
	 ?>




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

</body>

	
</body>


<br><br>
</br></br></br>





		
		
		
</html>

<?php require 'asset/footer.php' ?>