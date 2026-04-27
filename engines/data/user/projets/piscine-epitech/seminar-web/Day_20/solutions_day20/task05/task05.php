<?php
header('Content-Type: application/json; charset=utf-8');

$type   = isset($_GET['type'])   ? trim($_GET['type'])   : '';
$brand  = isset($_GET['brand'])  ? trim($_GET['brand'])  : '';
$priceS = isset($_GET['price'])  ? trim($_GET['price'])  : '';
$number = isset($_GET['number']) ? trim($_GET['number']) : '';

$type_lc  = mb_strtolower($type,  'UTF-8');
$brand_lc = mb_strtolower($brand, 'UTF-8');

$re_type  = '/^[a-z-]{3,10}$/i';
$re_brand = '/^[a-z0-9\-&]{2,20}$/i';

$re_price = '/^[<>=][0-9]+$/';
$errors   = [];

$DB_HOST = '127.0.0.1';
$DB_NAME = 'ajax_products';
$DB_USER = 'romeo';
$DB_PASS = 'secret';

try {
  $pdo = new PDO(
    "mysql:host=$DB_HOST;dbname=$DB_NAME;charset=utf8mb4",
    $DB_USER, $DB_PASS,
    [ PDO::ATTR_ERRMODE => PDO::ERRMODE_EXCEPTION ]
  );
} catch (Throwable $e) {
  echo json_encode(['error' => 'Database error: cannot connect.']);
  exit;
}

if ($type === '') {
  $errors[] = 'Error ($type): this type does not have enough characters.'; // minimal hint; spec lists exact errors
} elseif (mb_strlen($type) < 3) {
  $errors[] = "Error (\$type): this type does not have enough characters.";
} elseif (mb_strlen($type) > 10) {
  $errors[] = "Error (\$type): this type has too many characters.";
} elseif (!preg_match($re_type, $type)) {
  $errors[] = "Error (\$type): this type has non-alphabetical characters (different from '-').";
} else {
  $stmt = $pdo->prepare("SELECT 1 FROM products WHERE LOWER(`type`) = :t LIMIT 1");
  $stmt->execute([':t' => $type_lc]);
  if (!$stmt->fetchColumn()) {
    $errors[] = "Error (\$type): this type doesn't exist in our shop.";
  }
}

if ($brand === '') {
  $errors[] = "Error (\$brand): this brand does not have enough characters.";
} elseif (mb_strlen($brand) < 2) {
  $errors[] = "Error (\$brand): this brand does not have enough characters.";
} elseif (mb_strlen($brand) > 20) {
  $errors[] = "Error (\$brand): this brand has too much characters.";
} elseif (!preg_match($re_brand, $brand)) {
  $errors[] = "Error (\$brand): this brand has invalid characters.";
} else {
  $stmt = $pdo->prepare("SELECT 1 FROM products WHERE LOWER(`brand`) = :b LIMIT 1");
  $stmt->execute([':b' => $brand_lc]);
  if (!$stmt->fetchColumn()) {
    $errors[] = "Error (\$brand): this brand doesn't exist in our database.";
  }
}

if ($priceS === '') {
  $errors[] = "Error (\$price): this price does not have enough characters.";
} elseif (mb_strlen($priceS) < 2) {
  $errors[] = "Error (\$price): this price does not have enough characters.";
} elseif (mb_strlen($priceS) > 5) {
  $errors[] = "Error (\$price): this price has too many characters.";
} elseif (!preg_match($re_price, $priceS)) {
  $errors[] = "Error (\$price): we cannot define a price - string invalid.";
}

$op = null; $priceVal = null;
if (empty($errors)) {
  $op = $priceS[0];
  $priceVal = (int)substr($priceS, 1);
}

if ($number === '' || !ctype_digit($number)) {
  $errors[] = "Error (\$number): not a positive number.";
}
$qty = (int)$number;

if (!empty($errors)) {
  echo json_encode(['error' => implode(' ', $errors)], JSON_UNESCAPED_UNICODE);
  exit;
}

$priceSql = ($op === '=') ? 'price = :p' : (($op === '>') ? 'price > :p' : 'price < :p');

$sql = "SELECT `type`, `brand`, `price`, `stock`
        FROM products
        WHERE LOWER(`type`) = :t
          AND LOWER(`brand`) = :b
          AND $priceSql";

$stmt = $pdo->prepare($sql);
$stmt->execute([':t' => $type_lc, ':b' => $brand_lc, ':p' => $priceVal]);

$rows = $stmt->fetchAll(PDO::FETCH_ASSOC);

if (!$rows || count($rows) === 0) {

    echo json_encode(['error' => "Error (\$price): no products found at this price."], JSON_UNESCAPED_UNICODE);
  exit;
}

$products = [];
foreach ($rows as $r) {
  if ($qty > (int)$r['stock']) {
    echo json_encode(['error' => "Error (\$number): sorry, we don't have enough stock, we only have {$r['stock']} in stock."], JSON_UNESCAPED_UNICODE);
    exit;
  }

  $products[] = [
    'type'   => $r['type'],
    'brand'  => $r['brand'],
    'price'  => (int)$r['price'],
    'number' => $qty,
    'stock'  => (int)$r['stock'],
  ];
}

echo json_encode(['products' => $products], JSON_UNESCAPED_UNICODE);