<?php

declare(strict_types=1);

function rebase(int $from_base, array $sequence, int $to_base)
{
    if (
        $sequence == [] // Empty sequence
        || reset($sequence) == 0 // Leading digit is zero
        || $from_base < 2 // Invalid source base
        || $to_base < 2 // Invalid destination base
        || count(array_filter($sequence, fn ($num) => $num < 0 || $num >= $from_base)) > 0 // Invalid digit in sequence
    ) {
        return null;
    }

    // Convert sequence to int
    $number = array_reduce($sequence, fn ($acc, $digit) => $acc * $from_base + $digit, 0);

    // Convert int back to sequence of different base
    $result = [];
    while ($number >= $to_base) {
        $result[] = $number % $to_base;
        $number /= $to_base;
    }
    $result[] = (int) $number;

    return array_reverse($result);
}
