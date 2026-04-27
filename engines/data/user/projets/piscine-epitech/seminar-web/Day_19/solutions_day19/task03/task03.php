<?php

function dynamic_body() {
    $page = $_GET['page'] ?? null;
    if (in_array($page, ["home", "php", "sql"], true)) {
        return file_get_contents("$page.html");
    }
    return '<p>Unknown page</p>';
}

?>