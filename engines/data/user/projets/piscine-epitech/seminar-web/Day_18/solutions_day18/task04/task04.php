<?php

function calc($operator, $a, $b) {
    $signs = array("+", "-", "*", "/", "%");
    
    if (!in_array($operator, $signs)) {
        return "Unknown operator";
    }

    if (($operator === "/" || $operator === "%") && $b == 0) {
        return "Cannot divide by 0";
    }

    if ($operator === "+") {
        return $a + $b;
    } elseif ($operator === "-") {
        return $a - $b;
    } elseif ($operator === "*") {
        return $a * $b;
    } elseif ($operator === "/") {
        return $a / $b;
    } elseif ($operator === "%") {
        return $a % $b;
    }
}

?>