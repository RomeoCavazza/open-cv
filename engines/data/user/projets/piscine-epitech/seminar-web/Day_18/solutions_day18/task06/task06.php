<?php

function sequence(int $number) {

    if ($number < 0) {
        return;
    }

    $starter = "1";
    echo $starter . PHP_EOL;

    for ($i = 0; $i < $number; $i++) {
        $next = "";
        $count = 1;
        $length = strlen($starter);

        for ($j = 0; $j < $length; $j++) {
            if ($j + 1 < $length && $starter[$j] === $starter[$j + 1]) {
                $count++;
            } else {
                $next .= $count . $starter[$j];
                $count = 1;
            }
        }

        $starter = $next;
        echo $starter . PHP_EOL;

    
    }

}

?>
