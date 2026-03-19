<?php require 'asset/header.php' ?>

<div class="form-area">
    
<div class="container">
  <h2>Cash Memo</h2>
  <form action="insertbill.php">
    <div class="form-group">
      <label for="Memo_no">Memo no:</label>
      <input type="number" class="form-control" id="" placeholder="Memo No" name="Memo_no">
    </div>
    <div class="form-group">
      <label for="Date">Date:</label>
      <input type="Date" class="form-control" id="Date" placeholder="Date" name="Date">
    </div>
          <div class="form-group">
      <label for="Customer_Name">Customer Name:</label>
      <input type="varchar" class="form-control" id="Customer_Name" placeholder="Customer_Name" name="Customer_Name">
    </div>
          <div class="form-group">
      <label for="Address"> Address:</label>
      <input type="varchar" class="form-control" id="Address" placeholder="Address" name="Address">
    </div>
          <div class="form-group">
      <label for="Description_Of_Goods">Description Of Goods:</label>
      <input type="varchar" class="form-control" id="Description_Of_Goods" placeholder="Description_Of_Goods" name="Description_Of_Goods">
    </div>
                <div class="form-group">
      <label for="Quantity">Quantity:</label>
      <input type="price" class="form-control" id="Quantity" placeholder="Quantity" name="Quantity">
    </div>
             <div class="form-group">
      <label for="Unit_Price">Unit Price:</label>
      <input type="number" class="form-control" id="Unit_Price" placeholder="Unit_Price" name="Unit_Price">
    </div>
	
	<!--
                   <div class="form-group">
      <label for="Total">Total:</label>
      <input type="price" class="form-control" id="" placeholder="Total" name="Total">
    </div>
                         <div class="form-group">
      <label for="Total_Tk">Total_Tk:</label>
      <input type="price" class="form-control" id="Total_Tk" placeholder="Total_Tk" name="Total_Tk">
    </div> -->
                               <div class="form-group">
      <label for="Advance">Advance:</label>
      <input type="price" class="form-control" id="" placeholder="Advance" name="Advance">
    </div>
                            <div class="form-group">
      <!--<label for="Balance">Balance:</label>
      <input type="Balance" class="form-control" id="" placeholder="Balance" name="Balance">
    </div>-->
    <div class="checkbox">
      <label><input type="checkbox" name="remember"> Remember me</label>
    </div>
    <button type="submit" class="btn btn-default">Submit</button>
  </form>
</div>

</div>

<?php require 'asset/footer.php' ?>