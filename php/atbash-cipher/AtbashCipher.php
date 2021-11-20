<?php

declare(strict_types=1);

function encode(string $text): string
{
    $text = strtolower(preg_replace("/[^a-zA-Z\d]+/", "", $text));
    $chars = implode('', range('a', 'z'));
    $text = strtr($text, $chars, strrev($chars));
    return wordwrap($text, 5, " ", true);
}
