<?php

declare(strict_types=1);

function generateNumber(): string
{
    return chr(random_int(ord('0'), ord('9')));
}

function generateChar(): string
{
    return chr(random_int(ord('A'), ord('Z')));
}

function generateName(): string
{
    return generateChar() . generateChar() . generateNumber() . generateNumber() . generateNumber();
}

function generateUniqueName(): string
{
    static $in_use_names = [];

    do {
        $name = generateName();
    } while (array_key_exists($name, $in_use_names));

    $in_use_names[$name] = true;

    return $name;
}

class Robot
{
    private string $name;

    public function getName(): string
    {
        $this->name ?? $this->reset();
        return $this->name;
    }

    public function reset(): void
    {
        $this->name = generateUniqueName();
    }
}
