<?php

declare(strict_types=1);

const REGEX = "(\p{Lu}+)\P{L}+|(\p{L})\p{Ll}+";

function acronym(string $text): string
{
    mb_ereg_search_init($text, REGEX);

    $acronym = "";
    while ($match = mb_ereg_search_regs()) {
        if ($match[1] && mb_strlen($match[1]) > 1) {
            return $match[1];
        } else {
            $acronym .= $match[2];
        }
    }

    return strlen($acronym) > 1 ? mb_convert_case($acronym, MB_CASE_UPPER) : "";
}
