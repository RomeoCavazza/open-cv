<?php
header('Content-Type: application/json; charset=utf-8');

$type  = isset($_GET['type'])  ? trim($_GET['type'])  : '';
$brand = isset($_GET['brand']) ? trim($_GET['brand']) : '';

$type_lc  = mb_strtolower($type,  'UTF-8');
$brand_lc = mb_strtolower($brand, 'UTF-8');

$re_type  = '/^[a-z-]{3,10}$/i';
$re_brand = '/^[a-z0-9\-&]{2,20}$/i';

$result = [
  'type'  => ['ok' => false, 'message' => ''],
  'brand' => ['ok' => false, 'message' => ''],
];

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
    echo json_encode([
        'type'  => ['ok' => false, 'message' => 'Database error: cannot connect.'],
        'brand' => ['ok' => false, 'message' => 'Database error: cannot connect.']
    ]);
    exit;
}

if ($type === '') {
    $result['type'] = ['ok' => false, 'message' => 'No type sent yet!'];
} elseif (mb_strlen($type) < 3) {
    $result['type'] = ['ok' => false, 'message' => "$type: this type does not have enough characters."];
} elseif (mb_strlen($type) > 10) {
    $result['type'] = ['ok' => false, 'message' => "$type: this type has too many characters."];
} elseif (!preg_match($re_type, $type)) {
    $result['type'] = ['ok' => false, 'message' => "$type: this type has non-alphabetical characters (different from '-')."];
} else {

    $stmt = $pdo->prepare("SELECT 1 FROM products WHERE LOWER(`type`) = :t LIMIT 1");
    $stmt->execute([':t' => $type_lc]);
    if ($stmt->fetchColumn()) {
        $result['type'] = ['ok' => true, 'message' => "$type: this type exists in databases."]; // exact phrasing from spec
    } else {
        $result['type'] = ['ok' => false, 'message' => "$type: this type doesn't exist in our shop."];
    }
}

if ($brand === '') {
    $result['brand'] = ['ok' => false, 'message' => 'No brand sent yet!'];
} elseif (mb_strlen($brand) < 2) {
    $result['brand'] = ['ok' => false, 'message' => "$brand: this brand does not have enough characters."];
} elseif (mb_strlen($brand) > 20) {
    $result['brand'] = ['ok' => false, 'message' => "$brand: this brand has too many characters."];
} elseif (!preg_match($re_brand, $brand)) {
    $result['brand'] = ['ok' => false, 'message' => "$brand: this brand has invalid characters."];
} else {
   
    $type_exists = ($result['type']['ok'] === true);

    if ($type_exists) {
        $stmt = $pdo->prepare("SELECT 1 FROM products WHERE LOWER(`type`) = :t AND LOWER(`brand`) = :b LIMIT 1");
        $stmt->execute([':t' => $type_lc, ':b' => $brand_lc]);

        if ($stmt->fetchColumn()) {
            $result['brand'] = ['ok' => false, 'message' => "$brand: this brand already exists in databases."];
        } else {
            $result['brand'] = ['ok' => true, 'message' => "$brand: this brand is valid for the type $type."];
        }
    } else {
       
        $result['brand'] = ['ok' => false, 'message' => "$brand: this brand is valid syntactically, but the type is not valid/existing."];
    }
}

echo json_encode($result, JSON_UNESCAPED_UNICODE);
