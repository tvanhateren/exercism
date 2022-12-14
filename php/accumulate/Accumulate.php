<?php

declare(strict_types=1);

function accumulate(array $input, callable $accumulator): array
{
    foreach ($input as $key => $value) {
        $input[$key] = $accumulator($value);
    }

    return $input;
}
