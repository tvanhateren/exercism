<?php

declare(strict_types=1);

function char_to_string(string $char, int $count): string
{
    if ($count > 1) {
        return $count . $char;
    } else {
        return $char;
    }
}

function encode(string $input): string
{
    $count = 0;
    $last_c = "";
    $result = "";

    foreach (str_split($input) as $c) {
        if ($count > 0) {
            if ($c == $last_c) {
                $count += 1;
            } else {
                $result .= char_to_string($last_c, $count);
                $last_c = $c;
                $count = 1;
            }
        } else {
            $last_c = $c;
            $count = 1;
        }
    }

    $result .= char_to_string($last_c, $count);

    return $result;
}

function decode(string $input): string
{
    $result = "";
    $count = 0;

    foreach (str_split($input) as $c) {
        $num = intval($c);
        if ($num == 0) {
            $result .= str_repeat($c, max($count, 1));
            $count = 0;
        } else {
            $count = $count * 10 + $num;
        }
    }

    return $result;
}
