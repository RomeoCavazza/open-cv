<?php
function get_shortest(array $words) {
    $lengths = array_map('strlen', $words);
    $minLength = min($lengths);
    $index = array_search($minLength, $lengths);
    return $words[$index];
}
?>
