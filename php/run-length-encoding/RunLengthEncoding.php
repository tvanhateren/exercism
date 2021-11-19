<?php

declare(strict_types=1);

function encode(string $input): string
{
    return preg_replace_callback('/(.)\1+/', fn ($match) => strlen($match[0]) . $match[1], $input);
}

function decode(string $input): string
{
    return preg_replace_callback('/(\d+)(\D)/', fn ($match) => str_repeat($match[2], intval($match[1])), $input);
}
