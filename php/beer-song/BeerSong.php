<?php

declare(strict_types=1);

function bottles_of_beer(int $number, bool $capital): string
{
    $n = $number == 0 ? (($capital ? "N" : "n") . "o more") : "$number";
    $s = $number == 1 ? "" : "s";

    return "$n bottle$s of beer";
}

function take($number): string
{
    $subj = $number >= 2 ? "one" : "it";
    return $number == 0 ? "Go to the store and buy some more" : "Take $subj down and pass it around";
}

function mod($a, $b)
{
    $result = $a % $b;
    if ($result < 0) $result += $b;
    return $result;
}

class BeerSong
{
    public function verse(int $number): string
    {
        $le = $number == 0 ? "" : "\n";
        return sprintf("%s on the wall, %s.\n", bottles_of_beer($number, true), bottles_of_beer($number, false)) .
            sprintf("%s, %s on the wall.%s", take($number), bottles_of_beer(mod($number - 1, 100), false), $le);
    }

    public function verses(int $start, int $finish): string
    {
        $result = "";

        for ($i = $start; $i > $finish; $i--) {
            $result .= $this->verse($i) . "\n";
        }
        $result .= $this->verse($i);

        return $result;
    }

    public function lyrics(): string
    {
        return $this->verses(99, 0);
    }
}
