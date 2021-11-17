<?php

/*
 * By adding type hints and enabling strict type checking, code can become
 * easier to read, self-documenting and reduce the number of potential bugs.
 * By default, type declarations are non-strict, which means they will attempt
 * to change the original type to match the type specified by the
 * type-declaration.
 *
 * In other words, if you pass a string to a function requiring a float,
 * it will attempt to convert the string value to a float.
 *
 * To enable strict mode, a single declare directive must be placed at the top
 * of the file.
 * This means that the strictness of typing is configured on a per-file basis.
 * This directive not only affects the type declarations of parameters, but also
 * a function's return type.
 *
 * For more info review the Concept on strict type checking in the PHP track
 * <link>.
 *
 * To disable strict typing, comment out the directive below.
 */

declare(strict_types=1);

function double_luhn_digits(array $digits, int $len): array
{
    for ($i = $len - 2; $i >= 0; $i -= 2) {
        $digits[$i] *= 2;
        if ($digits[$i] > 9) {
            $digits[$i] -= 9;
        }
    }

    return $digits;
}

function isValid(string $number): bool
{
    // Test for invalid characters
    if (mb_ereg("[^0-9\s]", $number)) {
        return false;
    }

    // Remove any whitespace
    $number = mb_ereg_replace("\s", "", $number);

    $digits = str_split($number);
    $len = count($digits);

    if ($len <= 1) {
        return false;
    }

    $digits = double_luhn_digits($digits, $len);

    return array_sum($digits) % 10 === 0;
}
