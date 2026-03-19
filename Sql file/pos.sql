-- phpMyAdmin SQL Dump
-- version 4.8.3
-- https://www.phpmyadmin.net/
--
-- Host: 127.0.0.1
-- Generation Time: Nov 26, 2018 at 11:02 AM
-- Server version: 10.1.36-MariaDB
-- PHP Version: 7.2.10

SET SQL_MODE = "NO_AUTO_VALUE_ON_ZERO";
SET AUTOCOMMIT = 0;
START TRANSACTION;
SET time_zone = "+00:00";


/*!40101 SET @OLD_CHARACTER_SET_CLIENT=@@CHARACTER_SET_CLIENT */;
/*!40101 SET @OLD_CHARACTER_SET_RESULTS=@@CHARACTER_SET_RESULTS */;
/*!40101 SET @OLD_COLLATION_CONNECTION=@@COLLATION_CONNECTION */;
/*!40101 SET NAMES utf8mb4 */;

--
-- Database: `pos`
--

-- --------------------------------------------------------

--
-- Table structure for table `bill`
--

CREATE TABLE `bill` (
  `Memo_no` int(50) NOT NULL,
  `Date` date NOT NULL,
  `Customer_Name` varchar(50) NOT NULL,
  `Address` varchar(50) NOT NULL,
  `Description_Of_Goods` varchar(50) NOT NULL,
  `Quantity` varchar(50) NOT NULL,
  `Unit_Price` varchar(50) NOT NULL,
  `Total` varchar(50) NOT NULL,
  `Total_Tk` varchar(50) NOT NULL,
  `Advance` varchar(50) NOT NULL,
  `Balance` int(50) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

--
-- Dumping data for table `bill`
--

INSERT INTO `bill` (`Memo_no`, `Date`, `Customer_Name`, `Address`, `Description_Of_Goods`, `Quantity`, `Unit_Price`, `Total`, `Total_Tk`, `Advance`, `Balance`) VALUES
(1, '2018-11-01', 'Asad', 'Badda,Dhaka', '20 kg red colour', '1', '200', '4000', '4000', '500', 3500),
(0, '0000-00-00', '', '', '', '', '', '', '', '', 0),
(2, '2018-11-02', 'Tanim', 'Uttara', 'colour Brush', '3', '5', '15', '15', '10', 20),
(3, '2018-11-03', 'Rahim', 'Mirpur', 'blue colour', '5', '20', '100', '100', '10', 90),
(0, '0000-00-00', '', '', '', '', '', '0', '0', '', 0),
(5, '2018-11-04', 'Kashem', 'Dhanmondi', 'something', '10', '100', '1000', '1000', '20', 800),
(6, '2018-11-05', 'Kuddus', 'Dhaka', 'colour Brush', '20', '30', '600', '600', '10', 540),
(7, '2018-11-06', 'Kgbgh', 'uttora', 'blue colour', '10', '50', '500', '500', '10', 450),
(13, '2018-11-05', '', 'adsd', 'aasc', '12', '99', '1188', '1188', '10', 1069);

-- --------------------------------------------------------

--
-- Table structure for table `import`
--

CREATE TABLE `import` (
  `Product_ID` int(50) NOT NULL,
  `Product_Name` varchar(50) NOT NULL,
  `Quantity` varchar(50) NOT NULL,
  `Product_Price` varchar(50) NOT NULL,
  `Total_Price` varchar(50) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

--
-- Dumping data for table `import`
--

INSERT INTO `import` (`Product_ID`, `Product_Name`, `Quantity`, `Product_Price`, `Total_Price`) VALUES
(1, 'sdcasc', '3', '12', '1213'),
(123, 'kjnvf', '20', '100', '20000');

-- --------------------------------------------------------

--
-- Table structure for table `import_prouct`
--

CREATE TABLE `import_prouct` (
  `Product_Name` varchar(20) NOT NULL,
  `Product_Id` int(20) NOT NULL,
  `Quantity` varchar(20) NOT NULL,
  `Product_Price` varchar(20) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

-- --------------------------------------------------------

--
-- Table structure for table `price`
--

CREATE TABLE `price` (
  `Product_ID` int(50) NOT NULL,
  `Product_Name` varchar(50) NOT NULL,
  `Price_per_kg` varchar(50) NOT NULL,
  `About` varchar(50) NOT NULL,
  `Total_Price` varchar(50) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

--
-- Dumping data for table `price`
--

INSERT INTO `price` (`Product_ID`, `Product_Name`, `Price_per_kg`, `About`, `Total_Price`) VALUES
(0, 'Red', '12', '', '123'),
(0, 'Red', '12', '', '123'),
(0, 'Red', '12', '', '123'),
(0, 'Red', '12', '', '123'),
(0, 'Red', '12', '', '123'),
(2, 's', '12', '', '131'),
(2, 's', '12', 'dc', '131');

-- --------------------------------------------------------

--
-- Table structure for table `staff`
--

CREATE TABLE `staff` (
  `Staff_Name` varchar(50) NOT NULL,
  `Staff_Address` varchar(50) NOT NULL,
  `Staff_Contact` varchar(50) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

--
-- Dumping data for table `staff`
--

INSERT INTO `staff` (`Staff_Name`, `Staff_Address`, `Staff_Contact`) VALUES
('Kashem', 'dhaka', '01912345678');

-- --------------------------------------------------------

--
-- Table structure for table `user_registration`
--

CREATE TABLE `user_registration` (
  `user_name` varchar(20) NOT NULL,
  `password` varchar(20) NOT NULL
) ENGINE=InnoDB DEFAULT CHARSET=latin1;

--
-- Dumping data for table `user_registration`
--

INSERT INTO `user_registration` (`user_name`, `password`) VALUES
('admin', '1234');

--
-- Indexes for dumped tables
--

--
-- Indexes for table `import_prouct`
--
ALTER TABLE `import_prouct`
  ADD PRIMARY KEY (`Product_Id`);
COMMIT;

/*!40101 SET CHARACTER_SET_CLIENT=@OLD_CHARACTER_SET_CLIENT */;
/*!40101 SET CHARACTER_SET_RESULTS=@OLD_CHARACTER_SET_RESULTS */;
/*!40101 SET COLLATION_CONNECTION=@OLD_COLLATION_CONNECTION */;
