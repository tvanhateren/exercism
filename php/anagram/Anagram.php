<?php

declare(strict_types=1);

function word_info($word)
{
    $word = mb_convert_case($word, MB_CASE_LOWER);
    $count = count_chars($word, 1);
    return [$word, $count];
}

function detectAnagrams(string $word, array $anagrams): array
{
    $result = [];

    $a = word_info($word);
    foreach ($anagrams as $other_word) {
        $b = word_info($other_word);
        if ($a[0] != $b[0] && $a[1] == $b[1]) {
            $result[] = $other_word;
        }
    }

    return $result;
}
