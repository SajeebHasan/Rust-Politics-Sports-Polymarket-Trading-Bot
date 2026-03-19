<?php require 'asset/header.php' ?>

<div class="form-area">
    
<div class="container">
  <h2>Price List</h2>
  <form action="insertprice.php">
    <div class="form-group">
      <label for="Product_ID">Product Id:</label>
      <input type="id" class="form-control" id="Product_ID" placeholder="Product_ID" name="Product_ID">
    </div>
    <div class="form-group">
      <label for="Product_Name">Product Name:</label>
      <input type="varchar" class="form-control" id="Product_Name" placeholder="Product_Name" name="Product_Name">
    </div>
          <div class="form-group">
      <label for="Price_per_kg">Price_per_kg:</label>
      <input type="varchar" class="form-control" id="Price_per_kg" placeholder="Price_per_kg" name="Price_per_kg">
    </div>
          <div class="form-group">
      <label for="About">About:</label>
      <input type="text" class="form-control" id="About" placeholder="About" name="About">
    </div>
                <div class="form-group">
      <label for="About">Total Price:</label>
      <input type="text" class="form-control" id="Total_price" placeholder="Total_price" name="Total_price">
    </div>
    <div class="checkbox">
      <label><input type="checkbox" name="remember"> Remember me</label>
    </div>
    <button type="submit" class="btn btn-default">Submit</button>
  </form>
</div>

</div>

<?php require 'asset/footer.php' ?>