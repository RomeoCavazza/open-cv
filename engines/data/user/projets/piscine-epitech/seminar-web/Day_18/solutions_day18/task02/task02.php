<?php

function dog_bark(int $woof_nb) {
    if ($woof_nb > 0) {
        for ($i = 0; $i < $woof_nb; $i++) {
            echo "woof ";
        }
        echo "\n";
    } else {
        echo "\n";
    }
}
?>
