
<?php
			
	$df_name = "Admin";
	$df_pass = "1234";
	
	if($_SERVER["REQUEST_METHOD"] == "POST"){
		$name = $_POST['username'];
		$password = $_POST['password'];
		
		if($df_name == $name && $df_pass == $password){
            echo include("extra.php");
		}else{
			echo "Error";
		}
	}
?>
	
	<center>
		<div class="wrapper">
			<h2>Login</h2>
			<p>Please fill in your credentials to login.</p><br>
			<form action="" method="POST">
				<label>Username</label>
				<input type="text" name="username" class="form-control" value=""><br><br>
				<label>Password</label>
				<input type="password" name="password" class="form-control" value=""><br><br>
				<input type="submit" class="btn" value="Login">
			</form>
		</div>
	</center>
</body>
</html>