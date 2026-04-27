<!DOCTYPE html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>Task 03</title>
</head>
<body>
  <nav>
    <ul>
      <li><a href="index03.php?page=home">Home</a></li>
      <li><a href="index03.php?page=php">PHP</a></li>
      <li><a href="index03.php?page=sql">SQL</a></li>
    </ul>
  </nav>

  <?php
    require 'task03.php';
    echo dynamic_body();
  ?>
</body>
</html>