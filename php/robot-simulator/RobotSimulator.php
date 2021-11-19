<?php

declare(strict_types=1);

function mod(int $num, int $mod): int
{
    $result = $num % $mod;

    if ($result < 0) {
        $result += $mod;
    }

    return $result;
}

class Robot
{
    const DIRECTION_NORTH = "\0";
    const DIRECTION_EAST = "\1";
    const DIRECTION_SOUTH = "\2";
    const DIRECTION_WEST = "\3";

    /**
     *
     * @var int[]
     */
    protected $position;

    /**
     *
     * @var string
     */
    protected $direction;

    public function __construct(array $position, string $direction)
    {
        $this->position = $position;
        $this->direction = $direction;
    }

    public function __get($name)
    {
        switch ($name) {
            case "position":
                return $this->position;
            case "direction":
                return $this->direction;
        }
    }

    public function turnRight(): self
    {
        $this->direction = chr(mod(ord($this->direction) + 1, 4));
        return $this;
    }

    public function turnLeft(): self
    {
        $this->direction = chr(mod(ord($this->direction) - 1, 4));
        return $this;
    }

    public function advance(): self
    {
        switch ($this->direction) {
            case self::DIRECTION_NORTH:
                $this->position[1] += 1;
                break;
            case self::DIRECTION_EAST:
                $this->position[0] += 1;
                break;
            case self::DIRECTION_SOUTH:
                $this->position[1] -= 1;
                break;
            case self::DIRECTION_WEST:
                $this->position[0] -= 1;
                break;
        }
        return $this;
    }

    public function instructions(string $commands): self
    {
        foreach (mb_str_split($commands) as $command) {
            switch ($command) {
                case "L":
                    $this->turnLeft();
                    break;
                case "R":
                    $this->turnRight();
                    break;
                case "A":
                    $this->advance();
                    break;
                default:
                    throw new InvalidArgumentException("Invalid instruction: " . $command);
            }
        }

        return $this;
    }
}
