<?php
function whoami(): void {
    $name = $_POST['name'] ?? '';
    $age  = $_POST['age']  ?? '';

    if ($name !== '' && is_numeric($age) && (int)$age >= 0) {
        echo "Hi, my name is $name and I'm " . (int)$age . " years old.";
    } elseif ($name !== '') {
        echo "Hi, my name is $name.";
    } elseif (is_numeric($age) && (int)$age >= 0) {
        echo "Hi, I have no name and I'm " . (int)$age . " years old.";
    } else {
        echo "Hi.";
    }
}
?>