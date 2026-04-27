<?php
header('Content-Type: application/json');

$name  = $_POST['name']  ?? '';
$email = $_POST['email'] ?? '';

if ($name === '' || $email === '') {
    http_response_code(400);
    echo json_encode(['message' => 'Please fill both fields.']);
    exit;
}

if (!preg_match('/^[^@\s]+@[^@\s]+\.[^@\s]+$/', $email)) {
    http_response_code(400);
    echo json_encode(['message' => 'Invalid email address.']);
    exit;
}

echo json_encode(['message' => "Hello $name, your email ($email) is valid."]);