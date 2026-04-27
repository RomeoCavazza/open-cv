<?php
function form_is_submitted(): bool {
  return isset($_POST['submit']);
}

function whoami(): void {
  $name       = $_POST['name'] ?? '';
  $age        = $_POST['age'] ?? '';
  $curriculum = $_POST['curriculum'] ?? '';

  $hasName = $name !== '';
  $hasAge  = is_numeric($age) && (int)$age >= 0;

  if ($hasName && $hasAge) {
    $msg = "Hi, my name is $name and I'm " . (int)$age . " years old.";
  } elseif ($hasName) {
    $msg = "Hi, my name is $name.";
  } elseif ($hasAge) {
    $msg = "Hi, I have no name and I'm " . (int)$age . " years old.";
  } else {
    $msg = "Hi.";
  }

  if ($curriculum !== '') {
    $msg .= " I'm a student of " . strtoupper($curriculum) . ".";
  }

  echo $msg;
}
