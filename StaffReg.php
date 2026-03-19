<?php require 'asset/header.php' ?>

<div class="form-area">
    
<div class="container">
  <h2>Staff Registration Form</h2>
  <form action="insertstaff.php">
    <div class="form-group">
      <label for="Staff_Name">Staff Name:</label>
      <input type="varchar" class="form-control" id="Staff_Name" placeholder="Staff_Name" name="Staff_Name">
    </div>
    <div class="form-group">
      <label for="Staff_Address">Staff_Address:</label>
      <input type="Address" class="form-control" id="Staff_Address" placeholder="Staff_Address" name="Staff_Address">
    </div>
          <div class="form-group">
      <label for="Staff_Contact">Staff_Contact:</label>
      <input type="contact" class="form-control" id="Staff_Contact" placeholder="Staff_Contact" name="Staff_Contact">
    </div>
    <div class="checkbox">
      <label><input type="checkbox" name="remember"> Remember me</label>
    </div>
    <button type="submit" class="btn btn-default">Submit</button>
  </form>
</div>

</div>

<?php require 'asset/footer.php' ?>