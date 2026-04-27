<?php

function calc_average(array $numbers) {
    $values = array_sum($numbers) / count($numbers);
    return round($values, 1);
}

?>