<?php

declare(strict_types=1);

function isIsogram(string $word): bool
{
    $word = mb_ereg_replace("[\s-]+", "", $word);
    $word = mb_convert_case($word, MB_CASE_LOWER);
    $letters_seen = [];

    foreach (mb_str_split($word) as $letter) {
        if (isset($letters_seen[$letter])) {
            return false;
        } else {
            $letters_seen[$letter] = true;
        }
    }

    return true;
}
