<?php

declare(strict_types=1);

function encode(string $text): string
{
    $text = strtolower(preg_replace("/[^a-zA-Z\d]+/", "", $text));
    $text = preg_replace_callback(
        "/[a-z]/",
        fn ($c) => chr((25 - (ord($c[0]) - ord('a')) % 26) + ord('a')),
        $text
    );
    return substr(chunk_split($text, 5, " "), 0, -1);
}
