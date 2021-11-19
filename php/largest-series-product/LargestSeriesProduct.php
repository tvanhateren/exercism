<?php

declare(strict_types=1);

class Series
{
    private int $len;
    private array $series;

    public function __construct(string $series)
    {
        if (preg_match('/\D/', $series)) {
            throw new InvalidArgumentException();
        }

        $this->series = str_split($series);
        $this->len = strlen($series);
    }

    public function largestProduct(int $span): int
    {
        if ($span < 0 || $span > $this->len) {
            throw new InvalidArgumentException();
        } elseif ($span == 0) {
            return 1;
        }

        $max = 0;

        for ($i = 0; $i + $span <= $this->len; $i++) {
            $max = max(array_product(array_slice($this->series, $i, $span)), $max);
        }

        return $max;
    }
}
