<?php
header('Content-Type: application/json');

if (isset($_GET['name']) && $_GET['name'] !== '') {
    $name = htmlspecialchars($_GET['name']);
    echo json_encode(['name' => $name]);
} else {
    http_response_code(400);
    echo json_encode(['error' => 'No name provided']);
}
?>
