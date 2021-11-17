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

function ctoi(string $char): int
{
    return ord($char) - ord('a');
}

function itoc(int $val): string
{
    return chr($val + ord('a'));
}

function generate_key(int $len): string
{
    $str = "";

    for ($i = 0; $i < $len; $i++) {
        $str .= chr(rand(ord('a'), ord('z')));
    }

    return $str;
}

class SimpleCipher
{
    public function __construct(string $key = null)
    {
        // Test if the key contains only characters between 'a' and 'z'
        if (
            isset($key)
            && count(
                array_filter(
                    str_split($key),
                    fn (string $char): bool => ctoi($char) < ctoi('a') || ctoi($char) > ctoi('z')
                )
            ) > 0
        ) {
            throw new InvalidArgumentException();
        }

        // If no key is provided generate a new key
        $this->key = $key ?? generate_key(10);
        $this->key_len = strlen($this->key);
    }

    public function encode(string $plainText): string
    {
        $len = strlen($plainText);
        $cipherText = "";

        for ($i = 0; $i < $len; $i++) {
            $cipherText[$i] = itoc((ctoi($plainText[$i]) + ctoi($this->key[$i % $this->key_len])) % 26);
        }

        return $cipherText;
    }

    public function decode(string $cipherText): string
    {
        $len = strlen($cipherText);
        $plainText = "";

        for ($i = 0; $i < $len; $i++) {
            $plainText[$i] = itoc((ctoi($cipherText[$i]) - ctoi($this->key[$i % $this->key_len])) % 26);
        }

        return $plainText;
    }
}
