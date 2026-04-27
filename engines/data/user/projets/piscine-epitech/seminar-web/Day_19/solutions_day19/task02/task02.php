<?php

function render_body(string $page) {
    if (in_array($page, ["home", "php", "sql"])) {
        return file_get_contents("$page.html");
    } else {
        return '<p>Unknown page</p>';
    }
}

?>
