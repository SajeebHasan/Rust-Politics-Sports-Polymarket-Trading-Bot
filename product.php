<?php require 'asset/header.php' ?>

<div class="form-area">
    
<div class="container">
  <h2>Import Product List</h2>
  <form action="insertimport.php">
    <div class="form-group">
    <div class="form-group">
      <label for="Product_ID">Product Id:</label>
      <input type="id" class="form-control" id="Product_ID" placeholder="Product_ID" name="Product_ID">
    </div>
      <label for="Product_Name">Product Name:</label>
      <input type="varchar" class="form-control" id="Product_Name" placeholder="Product Name" name="Product_Name">
    </div>
          <div class="form-group">
      <label for="Quantity">Quantity:</label>
      <input type="number" class="form-control" id="Quantity" placeholder="Quantity" name="Quantity">
    </div>
          <div class="form-group">
      <label for="Product_Price">Product price:</label>
      <input type="number" class="form-control" id="" placeholder="Product_Price" name="Product_Price">
    </div>
                <div class="form-group">
      <label for="Total_Price">Total Price:</label>
      <input type="number" class="form-control" id="Total_Price" placeholder="Total_Price" name="Total_Price">
    </div>

    <div class="checkbox">
      <label><input type="checkbox" name="remember"> Remember me</label>
    </div>
    <button type="submit" class="btn btn-default">Submit</button>
  </form>
</div>

</div>

<?php require 'asset/footer.php' ?>